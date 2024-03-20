use anyhow::{anyhow, bail, Result};
use lazy_static::lazy_static;
use nohash_hasher::IntMap;
use prost::Message;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::class::{Class, Classes};
use crate::entity::{Entities, Entity, EntityEvent};
use crate::field::FIELD_PATCHES;
use crate::field::{Field, FieldModels};
use crate::field_reader::FieldReader;
use crate::field_type::FieldType;
use crate::reader::Reader;
use crate::serializer::Serializer;
use crate::string_table::{StringTable, StringTables};

use proto::{
    CDemoClassInfo, CDemoFullPacket, CDemoPacket, CDemoSendTables, CnetMsgTick,
    CsvcMsgCreateStringTable, CsvcMsgFlattenedSerializer, CsvcMsgPacketEntities, CsvcMsgServerInfo,
    CsvcMsgUpdateStringTable, EBaseEntityMessages, EBaseGameEvents, EBaseUserMessages,
    EDemoCommands, EDotaUserMessages, NetMessages, SvcMessages,
};

use crate::combat_log::CombatLog;
use proto::CMsgDotaCombatLogEntry;

#[derive(Debug)]
struct OuterMessage {
    tick: u32,
    msg_type: EDemoCommands,
    buf: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PendingMessage {
    pub tick: u32,
    pub msg_type: i32,
    pub buf: Vec<u8>,
}

impl PendingMessage {
    pub fn new(tick: u32, msg_type: i32, buf: Vec<u8>) -> Self {
        PendingMessage {
            tick,
            msg_type,
            buf,
        }
    }
}

pub struct Parser<'a> {
    reader: Reader<'a>,
    field_reader: FieldReader,

    pending_messages: VecDeque<PendingMessage>,
    serializers: FxHashMap<Box<str>, Rc<Serializer>>,

    pub(crate) observers: Vec<Rc<RefCell<dyn Observer + 'static>>>,

    combat_log: VecDeque<CMsgDotaCombatLogEntry>,

    pub classes: Classes,
    pub entities: Entities,
    pub string_tables: StringTables,

    pub tick: u32,
    pub net_tick: u32,
    pub game_build: Option<u32>,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Parser {
            reader: Reader::new(buf),
            field_reader: FieldReader::new(),

            pending_messages: VecDeque::new(),
            serializers: FxHashMap::default(),

            classes: Classes::new(),
            entities: Entities::new(),
            string_tables: StringTables::new(),

            observers: Vec::new(),

            combat_log: VecDeque::new(),

            tick: 0,
            net_tick: 0,
            game_build: None,
        }
    }

    pub fn register_observer<T: Observer + 'static>(&mut self, obs: Rc<RefCell<T>>) {
        self.observers.push(obs)
    }

    fn on_packet(&mut self, cmd: EDemoCommands, msg: &[u8]) -> Result<()> {
        match cmd {
            EDemoCommands::DemSendTables => self.send_tables(msg)?,
            EDemoCommands::DemClassInfo => self.class_info(msg)?,
            EDemoCommands::DemPacket | EDemoCommands::DemSignonPacket => self.dem_packet(msg)?,
            EDemoCommands::DemFullPacket => self.full_packet(msg)?,
            _ => {}
        };

        self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().on_packet(self, cmd, msg))
    }

    fn on_net_message(&mut self, p: NetMessages, msg: &[u8]) -> Result<()> {
        if p == NetMessages::NetTick {
            let t = CnetMsgTick::decode(msg)?;
            self.net_tick = t.tick();
        }

        self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().on_net_message(self, p, msg))
    }

    fn on_svc_message(&mut self, p: SvcMessages, msg: &[u8]) -> Result<()> {
        match p {
            SvcMessages::SvcServerInfo => self.server_info(msg)?,
            SvcMessages::SvcCreateStringTable => self.create_string_table(msg)?,
            SvcMessages::SvcUpdateStringTable => self.update_string_table(msg)?,
            SvcMessages::SvcPacketEntities => self.packet_entities(msg)?,
            _ => {}
        }

        self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().on_svc_message(self, p, msg))
    }

    fn on_base_user_message(&mut self, p: EBaseUserMessages, msg: &[u8]) -> Result<()> {
        self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().on_base_user_message(self, p, msg))
    }

    fn on_base_entity_message(&mut self, p: EBaseEntityMessages, msg: &[u8]) -> Result<()> {
        self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().on_base_entity_message(self, p, msg))
    }

    fn on_base_game_event(&mut self, p: EBaseGameEvents, msg: &[u8]) -> Result<()> {
        self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().on_base_game_event(self, p, msg))
    }

    fn on_dota_user_message(&mut self, p: EDotaUserMessages, msg: &[u8]) -> Result<()> {
        if p == EDotaUserMessages::DotaUmCombatLogDataHltv {
            let entry = CMsgDotaCombatLogEntry::decode(msg)?;
            self.combat_log.push_back(entry);
        }
        self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().on_dota_user_message(self, p, msg))
    }

    pub fn process(&mut self) -> Result<()> {
        self.reader.read_bytes(16);
        while let Some(message) = self.read_outer_message()? {
            self.on_tick_start()?;
            self.on_packet(message.msg_type, message.buf.as_slice())?;
            self.on_tick_end()?;
        }

        self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().epilogue(self))
    }

    fn send_tables(&mut self, msg: &[u8]) -> Result<()> {
        let send_tables = CDemoSendTables::decode(msg)?;
        let mut r = Reader::new(send_tables.data());
        let amount = r.read_var_u32();
        let buf = r.read_bytes(amount);

        let fs = CsvcMsgFlattenedSerializer::decode(buf.as_slice())?;

        let patches = FIELD_PATCHES
            .iter()
            .filter(|patch| patch.should_apply(self.game_build.unwrap()))
            .collect::<Vec<_>>();

        let mut fields = FxHashMap::<i32, Rc<Field>>::default();
        let mut field_types = FxHashMap::<String, Rc<FieldType>>::default();

        for s in fs.serializers.iter() {
            let mut serializer = Serializer::new(
                fs.symbols[s.serializer_name_sym.unwrap() as usize].clone(),
                s.serializer_version.unwrap(),
            );

            for i in s.fields_index.iter() {
                if fields.get(i).is_none() {
                    let mut field = Field::new(fs.clone(), fs.fields[*i as usize].clone());
                    if field_types.get(field.var_type.as_ref()).is_none() {
                        field_types.insert(
                            field.var_type.clone().parse()?,
                            Rc::new(FieldType::new(&field.var_type)),
                        );
                    }
                    field.field_type = Some(Rc::clone(&field_types[field.var_type.as_ref()]));

                    if field.serializer_name.as_ref() != "" {
                        field.serializer =
                            self.serializers.get(&field.serializer_name).map(Rc::clone);
                    }

                    for patch in patches.iter() {
                        (patch.patch)(&mut field);
                    }

                    if field.serializer.is_some() {
                        if field.field_type.as_ref().unwrap().pointer
                            || POINTER_TYPES
                                .get(field.field_type.as_ref().unwrap().base.as_str())
                                .is_some()
                        {
                            field.set_model(FieldModels::FixedTable);
                        } else {
                            field.set_model(FieldModels::VariableTable);
                        }
                    } else if field.field_type.as_ref().unwrap().count.is_some()
                        && field.field_type.as_ref().unwrap().count.unwrap() > 0
                        && field.field_type.as_ref().unwrap().base != "char"
                    {
                        field.set_model(FieldModels::FixedArray);
                    } else if field.field_type.as_ref().unwrap().base == "CUtlVector"
                        || field.field_type.as_ref().unwrap().base == "CNetworkUtlVectorBase"
                    {
                        field.set_model(FieldModels::VariableArray);
                    } else {
                        field.set_model(FieldModels::Simple);
                    }
                    fields.insert(*i, Rc::new(field));
                }
                serializer.fields.push(Rc::clone(&fields[i]));
            }
            let ser_name = serializer.name.clone();
            self.serializers
                .insert(serializer.name.clone(), Rc::new(serializer));
            if let Ok(x) = self.classes.get_by_name_mut(ser_name.as_ref()) {
                x.borrow_mut().serializer = Rc::clone(self.serializers.get(&ser_name).unwrap());
            }
        }
        Ok(())
    }

    fn class_info(&mut self, msg: &[u8]) -> Result<()> {
        let info = CDemoClassInfo::decode(msg)?;
        for class in info.classes {
            let class_id = class.class_id.unwrap();
            let network_name = class.network_name.unwrap();
            let c = Rc::new(RefCell::new(Class::new(
                class_id,
                network_name.as_str(),
                Rc::clone(&self.serializers[&network_name.clone().into_boxed_str()]),
            )));
            self.classes.classes_by_id.insert(class_id, Rc::clone(&c));
            self.classes
                .classes_by_name
                .insert(network_name.into(), Rc::clone(&c));
        }
        self.classes.class_info = true;
        Ok(())
    }

    fn dem_packet(&mut self, msg: &[u8]) -> Result<()> {
        let packet = CDemoPacket::decode(msg)?;
        let mut packet_reader = Reader::new(packet.data());
        while packet_reader.remain_bytes() > 0 {
            let t = packet_reader.read_ubit_var() as i32;
            let size = packet_reader.read_var_u32();
            let packet_buf = packet_reader.read_bytes(size);
            let message = PendingMessage::new(self.tick, t, packet_buf);
            self.pending_messages.push_back(message);
        }
        self.process_messages()
    }

    fn full_packet(&mut self, msg: &[u8]) -> Result<()> {
        let packet = CDemoFullPacket::decode(msg)?;
        self.on_packet(
            EDemoCommands::DemPacket,
            packet.packet.unwrap().encode_to_vec().as_slice(),
        )
    }

    fn read_outer_message(&mut self) -> Result<Option<OuterMessage>> {
        let reader = &mut self.reader;

        if reader.remain_bytes() == 0 {
            return Ok(None);
        }

        let raw_command = reader.read_var_u32() as i32;
        let msg_type =
            EDemoCommands::try_from(raw_command & !(EDemoCommands::DemIsCompressed as i32))?;
        let msg_compressed = (raw_command & EDemoCommands::DemIsCompressed as i32)
            == EDemoCommands::DemIsCompressed as i32;
        let tick = match reader.read_var_u32() {
            0xffffffff => 0,
            x => x,
        };

        let size = reader.read_var_u32();
        let mut buf = reader.read_bytes(size);

        if msg_compressed {
            let mut decoder = snap::raw::Decoder::new();
            buf = decoder.decompress_vec(&buf)?;
        }

        self.tick = tick;

        Ok(Some(OuterMessage {
            tick,
            msg_type,
            buf,
        }))
    }

    fn process_messages(&mut self) -> Result<()> {
        while let Some(message) = self.pending_messages.pop_front() {
            if let Ok(msg) = NetMessages::try_from(message.msg_type) {
                self.on_net_message(msg, &message.buf)?;
            } else if let Ok(msg) = EBaseUserMessages::try_from(message.msg_type) {
                self.on_base_user_message(msg, &message.buf)?;
            } else if let Ok(msg) = EBaseGameEvents::try_from(message.msg_type) {
                self.on_base_game_event(msg, &message.buf)?;
            } else if let Ok(msg) = EBaseEntityMessages::try_from(message.msg_type) {
                self.on_base_entity_message(msg, &message.buf)?;
            } else if let Ok(msg) = SvcMessages::try_from(message.msg_type) {
                self.on_svc_message(msg, &message.buf)?;
            } else if let Ok(msg) = EDotaUserMessages::try_from(message.msg_type) {
                self.on_dota_user_message(msg, &message.buf)?;
            }
        }
        Ok(())
    }

    fn packet_entities(&mut self, msg: &[u8]) -> Result<()> {
        {
            let packet = CsvcMsgPacketEntities::decode(msg)?;
            let packet_entity = packet.entity_data.unwrap();

            let mut r = Reader::new(packet_entity.as_slice());

            let updates = packet.updated_entries.unwrap();

            let mut index = -1;
            let mut op: isize;

            if !packet.is_delta.unwrap() {
                if self.entities.entity_full_packets > 0 {
                    return Ok(());
                }
                self.entities.entity_full_packets += 1;
            }

            let baselines = self.string_tables.get_by_name("instancebaseline")?;
            for _ in 0..updates {
                index += r.read_ubit_var() as i32 + 1;

                let cmd = r.read_bits(2);

                if cmd & 0x01 == 0 {
                    if cmd & 0x02 != 0 {
                        let class_id = r.read_bits(self.classes.class_id_size.unwrap()) as i32;
                        let serial = r.read_bits(17) as i32;

                        r.read_var_u32();

                        let class = Rc::clone(self.classes.get_by_id_mut(&class_id)?);
                        let baseline = baselines
                            .items
                            .values()
                            .find(|x| x.key.parse::<i32>().unwrap() == class_id)
                            .ok_or(anyhow!("No baseline for given class"))?
                            .value
                            .as_slice();

                        self.entities
                            .index_to_entity
                            .insert(index, Entity::new(index, serial, Rc::clone(&class)));
                        let e = self.entities.index_to_entity.get_mut(&index).unwrap();

                        self.field_reader.read_fields(
                            &mut Reader::new(baseline),
                            &e.class.borrow().serializer,
                            &mut e.state,
                        );

                        self.field_reader.read_fields(
                            &mut r,
                            &e.class.borrow().serializer,
                            &mut e.state,
                        );

                        op = EntityEvent::Created as isize | EntityEvent::Entered as isize;
                    } else {
                        op = EntityEvent::Updated as isize;
                        let e = self.entities.index_to_entity.get_mut(&index).unwrap();
                        if !e.active {
                            e.active = true;
                            op |= EntityEvent::Entered as isize;
                        }
                        self.field_reader.read_fields(
                            &mut r,
                            &e.class.borrow().serializer,
                            &mut e.state,
                        );
                    }
                } else {
                    op = EntityEvent::Left as isize;
                    if cmd & 0x02 != 0 {
                        op |= EntityEvent::Deleted as isize;
                    }
                }
                self.entities.undone_entities.push_back((index, op));
            }
        }

        self.process_entities()
    }

    fn update_string_table(&mut self, msg: &[u8]) -> Result<()> {
        let st = CsvcMsgUpdateStringTable::decode(msg)?;
        let mut t = self
            .string_tables
            .tables
            .get(&st.table_id.unwrap())
            .unwrap()
            .borrow_mut();

        for item in t.parse(
            st.string_data.unwrap().as_slice(),
            st.num_changed_entries.unwrap(),
        )? {
            match t.items.get_mut(&item.index) {
                Some(x) => {
                    if !item.key.is_empty() && item.key != x.key {
                        x.key = item.key;
                    }
                    if item.value.len() > 0 {
                        x.value = item.value;
                    }
                }
                None => {
                    t.items.insert(item.index, item);
                }
            }
        }
        Ok(())
    }

    fn create_string_table(&mut self, msg: &[u8]) -> Result<()> {
        let st = CsvcMsgCreateStringTable::decode(msg)?;

        let mut t = StringTable {
            index: self.string_tables.next_index,
            name: st.name().into(),
            items: IntMap::default(),
            user_data_fixed_size: st.user_data_fixed_size.unwrap(),
            user_data_size: st.user_data_size.unwrap(),
            flags: st.flags.unwrap() as u32,
            var_int_bit_counts: st.using_varint_bitcounts(),
        };

        self.string_tables.next_index += 1;

        let buf = match st.data_compressed.unwrap() {
            true => {
                let mut decoder = snap::raw::Decoder::new();
                decoder.decompress_vec(st.string_data.unwrap().as_slice())?
            }
            false => st.string_data.unwrap(),
        };

        for item in t.parse(buf.as_slice(), st.num_entries.unwrap())? {
            t.items.insert(item.index, item);
        }

        let rc = Rc::new(RefCell::new(t));
        self.string_tables
            .tables
            .insert(rc.borrow().index, Rc::clone(&rc));
        self.string_tables
            .names_to_table
            .insert(rc.borrow().name.clone().into(), Rc::clone(&rc));
        Ok(())
    }

    fn server_info(&mut self, msg: &[u8]) -> Result<()> {
        let info = CsvcMsgServerInfo::decode(msg)?;
        self.classes.class_id_size = Some((f64::log2(info.max_classes() as f64) + 1.0) as u32);

        let game_build_regexp = Regex::new(r"/dota_v(\d+)/")?;

        if let Some(captures) = game_build_regexp.captures(info.game_dir()) {
            if let Some(build_match) = captures.get(1) {
                let build_str = build_match.as_str();
                let build = build_str.parse::<u32>()?;
                self.game_build = Some(build);
            } else {
                // panic!("No build number found in regex capture");
                bail!("No build number found in regex capture");
            }
        } else {
            // panic!("Failed to parse build number: '{}'", info.game_dir());
            bail!("Failed to parse build number: '{}'", info.game_dir());
        }
        Ok(())
    }

    fn process_entities(&mut self) -> Result<()> {
        let throw_event = |ctx: &Parser, index: &i32, event: EntityEvent| -> Result<()> {
            self.observers.iter().try_for_each(|obs| {
                obs.borrow_mut()
                    .on_entity(ctx, event, &ctx.entities.index_to_entity[index])
            })
        };

        while let Some((index, op)) = self.entities.undone_entities.pop_front() {
            if op & EntityEvent::Created as isize != 0 {
                throw_event(self, &index, EntityEvent::Created)?;
            }
            if op & EntityEvent::Entered as isize != 0 {
                throw_event(self, &index, EntityEvent::Entered)?;
            }
            if op & EntityEvent::Updated as isize != 0 {
                throw_event(self, &index, EntityEvent::Updated)?;
            }
            if op & EntityEvent::Left as isize != 0 {
                throw_event(self, &index, EntityEvent::Left)?;
            }
            if op & EntityEvent::Deleted as isize != 0 {
                throw_event(self, &index, EntityEvent::Deleted)?;
                self.entities.index_to_entity.remove(&index);
            }
        }
        Ok(())
    }

    pub(crate) fn on_tick_start(&mut self) -> Result<()> {
        self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().on_tick_start(self))
    }

    pub(crate) fn on_tick_end(&mut self) -> Result<()> {
        if let Ok(names) = self.string_tables.get_by_name("CombatLogNames") {
            while let Some(entry) = self.combat_log.pop_front() {
                let log = CombatLog {
                    names: &names,
                    log: entry,
                };
                self.on_combat_log(&log)?;
            }
        }

        self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().on_tick_end(self))
    }

    pub(crate) fn on_combat_log(&self, entry: &CombatLog) -> Result<()> {
        self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().on_combat_log(self, entry))
    }
}

#[allow(unused_variables)]
pub trait Observer {
    fn on_packet(&mut self, ctx: &Parser, cmd: EDemoCommands, msg: &[u8]) -> Result<()> {
        Ok(())
    }
    fn on_net_message(&mut self, ctx: &Parser, emsg: NetMessages, msg: &[u8]) -> Result<()> {
        Ok(())
    }
    fn on_svc_message(&mut self, ctx: &Parser, emsg: SvcMessages, msg: &[u8]) -> Result<()> {
        Ok(())
    }
    fn on_base_user_message(
        &mut self,
        ctx: &Parser,
        emsg: EBaseUserMessages,
        msg: &[u8],
    ) -> Result<()> {
        Ok(())
    }
    fn on_base_entity_message(
        &mut self,
        ctx: &Parser,
        emsg: EBaseEntityMessages,
        msg: &[u8],
    ) -> Result<()> {
        Ok(())
    }
    fn on_base_game_event(
        &mut self,
        ctx: &Parser,
        emsg: EBaseGameEvents,
        msg: &[u8],
    ) -> Result<()> {
        Ok(())
    }
    fn on_dota_user_message(
        &mut self,
        ctx: &Parser,
        emsg: EDotaUserMessages,
        msg: &[u8],
    ) -> Result<()> {
        Ok(())
    }
    fn on_tick_start(&mut self, ctx: &Parser) -> Result<()> {
        Ok(())
    }
    fn on_tick_end(&mut self, ctx: &Parser) -> Result<()> {
        Ok(())
    }
    fn on_entity(&mut self, ctx: &Parser, event: EntityEvent, entity: &Entity) -> Result<()> {
        Ok(())
    }
    fn on_combat_log(&mut self, ctx: &Parser, combat_log: &CombatLog) -> Result<()> {
        Ok(())
    }
    fn epilogue(&mut self, ctx: &Parser) -> Result<()> {
        Ok(())
    }
}
pub enum ParseError {}

lazy_static! {
    static ref POINTER_TYPES: FxHashSet<&'static str> = [
        "PhysicsRagdollPose_t",
        "CBodyComponent",
        "CEntityIdentity",
        "CPhysicsComponent",
        "CRenderComponent",
        "CDOTAGamerules",
        "CDOTAGameManager",
        "CDOTASpectatorGraphManager",
        "CPlayerLocalData",
        "CPlayer_CameraServices",
        "CDOTAGameRules",
    ]
    .iter()
    .cloned()
    .collect();
}
