use crate::class::{Class, Classes};
use crate::combat_log::CombatLog;
use crate::decoder::Decoders;
use crate::entity::{Entities, Entity, EntityAction};
use crate::field::{Encoder, Field, FieldModels, FieldProperties, FieldType, FIELD_PATCHES};
use crate::field_reader::FieldReader;
use crate::proto::*;
use crate::reader::Reader;
use crate::serializer::Serializer;
use crate::string_table::{StringTable, StringTables};
use anyhow::{bail, Result};
use nohash_hasher::IntMap;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
struct OuterMessage {
    // tick: u32,
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
    baselines: IntMap<i32, Rc<Vec<u8>>>,

    pub(crate) observers: Vec<Rc<RefCell<dyn Observer + 'a>>>,

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
            baselines: IntMap::default(),

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

    pub fn register_observer(&mut self, obs: Rc<RefCell<dyn Observer + 'a>>) {
        self.observers.push(obs);
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
            self.net_tick = CnetMsgTick::decode(msg)?.tick();
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

        let fs: CsvcMsgFlattenedSerializer = CsvcMsgFlattenedSerializer::decode(buf.as_slice())?;

        let resolve = |p: Option<i32>| -> Box<str> {
            if let Some(i) = p {
                return fs.symbols[i as usize].clone().into();
            }
            "".to_string().into()
        };

        let pointer_types: FxHashSet<&'static str> = [
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
        .copied()
        .collect();

        let patches = FIELD_PATCHES
            .iter()
            .filter(|patch| patch.should_apply(self.game_build.unwrap()))
            .collect::<Vec<_>>();

        let mut fields = IntMap::<i32, Rc<Field>>::default();
        let mut field_types = FxHashMap::<Box<str>, Rc<FieldType>>::default();

        for s in fs.serializers.iter() {
            let serializer_name = fs.symbols[s.serializer_name_sym() as usize].clone();
            let mut serializer = Serializer::new();

            for i in s.fields_index.iter() {
                let current_field = &fs.fields[*i as usize];
                let field_serializer_name = resolve(current_field.field_serializer_name_sym);

                if !fields.contains_key(i) {
                    let var_type_str = resolve(current_field.var_type_sym);
                    let current_field_serializer =
                        self.serializers.get(&field_serializer_name).cloned();

                    if !field_types.contains_key(&var_type_str) {
                        field_types.insert(
                            var_type_str.clone(),
                            Rc::new(FieldType::new(var_type_str.clone().as_ref())),
                        );
                    }

                    let field_type = field_types[&var_type_str].clone();

                    let current_field_model = if current_field_serializer.is_some() {
                        if field_type.pointer || pointer_types.contains(field_type.base.as_ref()) {
                            FieldModels::FixedTable
                        } else {
                            FieldModels::VariableTable
                        }
                    } else if field_type.count.is_some()
                        && field_type.count.unwrap() > 0
                        && field_type.base.as_ref() != "char"
                    {
                        FieldModels::FixedArray
                    } else if field_type.base.as_ref() == "CUtlVector"
                        || field_type.base.as_ref() == "CNetworkUtlVectorBase"
                    {
                        FieldModels::VariableArray
                    } else {
                        FieldModels::Simple
                    };

                    let mut field = Field {
                        var_name: resolve(current_field.var_name_sym),
                        properties: FieldProperties {
                            encoder: Encoder::from_str(&resolve(current_field.var_encoder_sym)),
                            encoder_flags: current_field.encode_flags(),
                            bit_count: current_field.bit_count(),
                            low_value: current_field.low_value(),
                            high_value: current_field.high_value(),
                        },
                        field_type: field_type.clone(),
                        serializer: current_field_serializer,
                        model: current_field_model,

                        decoder: Decoders::Default,
                        base_decoder: Decoders::Default,
                        child_decoder: Decoders::Default,
                    };

                    for patch in patches.iter() {
                        (patch.patch)(&mut field);
                    }

                    match field.model {
                        FieldModels::FixedArray => {
                            field.decoder = Decoders::from_field(&field, false);
                        }
                        FieldModels::FixedTable => field.base_decoder = Decoders::Boolean,
                        FieldModels::VariableArray => {
                            field.base_decoder = Decoders::Unsigned32;
                            field.child_decoder = Decoders::from_field(&field, true)
                        }
                        FieldModels::VariableTable => {
                            field.base_decoder = Decoders::Unsigned32;
                        }
                        FieldModels::Simple => {
                            field.decoder = Decoders::from_field(&field, false);
                        }
                    };
                    fields.insert(*i, Rc::new(field));
                }
                serializer.fields.push(fields[i].clone());
            }
            let ser_name = serializer_name.clone();
            self.serializers
                .insert(ser_name.clone().into(), Rc::new(serializer));
        }
        Ok(())
    }

    fn class_info(&mut self, msg: &[u8]) -> Result<()> {
        let info = CDemoClassInfo::decode(msg)?;
        for class in info.classes {
            let class_id = class.class_id();
            let network_name = class.network_name();
            let c: Rc<Class> = Rc::new(Class::new(
                class_id,
                network_name,
                self.serializers[network_name].clone(),
            ));
            self.classes.classes_by_id.insert(class_id, c.clone());
            self.classes
                .classes_by_name
                .insert(network_name.into(), c.clone());
        }
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
        if self.reader.remain_bytes() == 0 {
            return Ok(None);
        }

        let raw_command = self.reader.read_var_u32() as i32;
        let msg_type =
            EDemoCommands::try_from(raw_command & !(EDemoCommands::DemIsCompressed as i32))?;
        let msg_compressed = (raw_command & EDemoCommands::DemIsCompressed as i32)
            == EDemoCommands::DemIsCompressed as i32;
        let tick = match self.reader.read_var_u32() {
            0xffffffff => 0,
            x => x,
        };

        let size = self.reader.read_var_u32();
        let mut buf = self.reader.read_bytes(size);

        if msg_compressed {
            let mut decoder = snap::raw::Decoder::new();
            buf = decoder.decompress_vec(&buf)?;
        }

        self.tick = tick;

        Ok(Some(OuterMessage {
            // tick,
            msg_type,
            buf,
        }))
    }

    fn process_messages(&mut self) -> Result<()> {
        while let Some(message) = self.pending_messages.pop_front() {
            if let Ok(msg) = EDotaUserMessages::try_from(message.msg_type) {
                self.on_dota_user_message(msg, &message.buf)?;
            } else if let Ok(msg) = SvcMessages::try_from(message.msg_type) {
                self.on_svc_message(msg, &message.buf)?;
            } else if let Ok(msg) = EBaseUserMessages::try_from(message.msg_type) {
                self.on_base_user_message(msg, &message.buf)?;
            } else if let Ok(msg) = EBaseGameEvents::try_from(message.msg_type) {
                self.on_base_game_event(msg, &message.buf)?;
            } else if let Ok(msg) = NetMessages::try_from(message.msg_type) {
                self.on_net_message(msg, &message.buf)?;
            } else if let Ok(msg) = EBaseEntityMessages::try_from(message.msg_type) {
                self.on_base_entity_message(msg, &message.buf)?;
            }
        }
        Ok(())
    }

    fn packet_entities(&mut self, msg: &[u8]) -> Result<()> {
        let packet = CsvcMsgPacketEntities::decode(msg)?;
        let mut r = Reader::new(packet.entity_data());

        let updates = packet.updated_entries();

        let mut index = -1;
        let mut op: isize;

        if !packet.legacy_is_delta() {
            if self.entities.entity_full_packets > 0 {
                return Ok(());
            }
            self.entities.entity_full_packets += 1;
        }

        for _ in 0..updates {
            index += r.read_ubit_var() as i32 + 1;

            let cmd = r.read_bits(2);

            if cmd & 0x01 == 0 {
                if cmd & 0x02 != 0 {
                    let class_id = r.read_bits(self.classes.class_id_size.unwrap()) as i32;
                    let serial = r.read_bits(17) as i32;

                    r.read_var_u32();

                    let class = Rc::clone(self.classes.get_by_id_rc(&class_id)?);
                    let baseline = self.baselines[&class_id].as_slice();

                    self.entities
                        .index_to_entity
                        .insert(index, Entity::new(index, serial, Rc::clone(&class)));
                    let e = self.entities.index_to_entity.get_mut(&index).unwrap();

                    self.field_reader.read_fields(
                        &mut Reader::new(baseline),
                        &e.class.serializer,
                        &mut e.state,
                    );

                    self.field_reader
                        .read_fields(&mut r, &e.class.serializer, &mut e.state);

                    op = EntityAction::Created as isize | EntityAction::Entered as isize;
                } else {
                    op = EntityAction::Updated as isize;
                    let e = self.entities.index_to_entity.get_mut(&index).unwrap();
                    if packet.has_pvs_vis_bits() != 0 {
                        let pvs = r.read_bits(2);
                        e.active = (pvs & 0x02) != 0;
                        if (pvs & 0x01) == 1 {
                            continue;
                        }
                    }
                    if !e.active {
                        e.active = true;
                        op |= EntityAction::Entered as isize;
                    }
                    self.field_reader
                        .read_fields(&mut r, &e.class.serializer, &mut e.state);
                }
            } else {
                op = EntityAction::Left as isize;
                if cmd & 0x02 != 0 {
                    op |= EntityAction::Deleted as isize;
                }
            }
            self.entities.undone_entities.push_back((index, op));
        }
        self.process_entities()
    }

    fn update_string_table(&mut self, msg: &[u8]) -> Result<()> {
        let st = CsvcMsgUpdateStringTable::decode(msg)?;
        let mut t = self.string_tables.tables[&st.table_id()].borrow_mut();

        let is_baseline = t.name == "instancebaseline";

        for item in t.parse(st.string_data(), st.num_changed_entries())? {
            match t.items.get_mut(&item.index) {
                Some(x) => {
                    if is_baseline {
                        self.baselines
                            .insert(item.key.parse::<i32>()?, item.value.clone());
                    }
                    if item.value.len() > 0 {
                        x.value = item.value;
                    }
                    if !item.key.is_empty() && item.key != x.key {
                        x.key = item.key;
                    }
                }
                None => {
                    if is_baseline {
                        self.baselines
                            .insert(item.key.parse::<i32>()?, item.value.clone());
                    }
                    t.items.insert(item.index, item);
                }
            }
        }
        Ok(())
    }

    fn create_string_table(&mut self, msg: &[u8]) -> Result<()> {
        let st = CsvcMsgCreateStringTable::decode(msg)?;

        if st.name() == "decalprecache" {
            self.string_tables.next_index += 1;
            return Ok(());
        }

        let mut t = StringTable {
            index: self.string_tables.next_index,
            name: st.name().into(),
            items: IntMap::default(),
            user_data_fixed_size: st.user_data_fixed_size(),
            user_data_size: st.user_data_size(),
            flags: st.flags() as u32,
            var_int_bit_counts: st.using_varint_bitcounts(),
        };

        self.string_tables.next_index += 1;

        let buf = match st.data_compressed() {
            true => {
                let mut decoder = snap::raw::Decoder::new();
                decoder.decompress_vec(st.string_data())?
            }
            false => st.string_data().into(),
        };

        let is_baseline = t.name == "instancebaseline";

        for item in t.parse(buf.as_slice(), st.num_entries())? {
            if is_baseline {
                self.baselines
                    .insert(item.key.parse::<i32>()?, item.value.clone());
            }
            t.items.insert(item.index, item);
        }

        let rc = Rc::new(RefCell::new(t));
        self.string_tables
            .tables
            .insert(rc.borrow().index, rc.clone());
        self.string_tables
            .names_to_table
            .insert(rc.borrow().name.clone().into(), rc.clone());

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
                bail!("No build number found in regex capture");
            }
        } else {
            bail!("Failed to parse build number: '{}'", info.game_dir());
        }
        Ok(())
    }

    fn process_entities(&mut self) -> Result<()> {
        let throw_event = |ctx: &Parser, index: &i32, event: EntityAction| -> Result<()> {
            self.observers.iter().try_for_each(|obs| {
                obs.borrow_mut()
                    .on_entity(ctx, event, &ctx.entities.index_to_entity[index])
            })
        };

        while let Some((index, op)) = self.entities.undone_entities.pop_front() {
            if op & EntityAction::Created as isize != 0 {
                throw_event(self, &index, EntityAction::Created)?;
            }
            if op & EntityAction::Entered as isize != 0 {
                throw_event(self, &index, EntityAction::Entered)?;
            }
            if op & EntityAction::Updated as isize != 0 {
                throw_event(self, &index, EntityAction::Updated)?;
            }
            if op & EntityAction::Left as isize != 0 {
                throw_event(self, &index, EntityAction::Left)?;
            }
            if op & EntityAction::Deleted as isize != 0 {
                throw_event(self, &index, EntityAction::Deleted)?;
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
        msg_type: EBaseUserMessages,
        msg: &[u8],
    ) -> Result<()> {
        Ok(())
    }
    fn on_base_entity_message(
        &mut self,
        ctx: &Parser,
        msg_type: EBaseEntityMessages,
        msg: &[u8],
    ) -> Result<()> {
        Ok(())
    }
    fn on_base_game_event(
        &mut self,
        ctx: &Parser,
        msg_type: EBaseGameEvents,
        msg: &[u8],
    ) -> Result<()> {
        Ok(())
    }
    fn on_dota_user_message(
        &mut self,
        ctx: &Parser,
        msg_type: EDotaUserMessages,
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
    fn on_entity(&mut self, ctx: &Parser, event: EntityAction, entity: &Entity) -> Result<()> {
        Ok(())
    }
    fn on_combat_log(&mut self, ctx: &Parser, combat_log: &CombatLog) -> Result<()> {
        Ok(())
    }
    fn epilogue(&mut self, ctx: &Parser) -> Result<()> {
        Ok(())
    }
}
