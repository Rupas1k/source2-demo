use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::time::Instant;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::class::{Class, Classes};
use crate::entity::{Entities, Entity, EntityEvent};
use crate::field::{Field, FieldModels};
use crate::field_patch::{FieldPatch, FIELD_PATCHES};
use crate::field_reader::FieldReader;
use crate::field_type::FieldType;
use crate::reader::Reader;
use crate::serializer::Serializer;
use crate::string_table::{StringTable, StringTables};
use lazy_static::lazy_static;
use nohash_hasher::IntMap;
use prost::Message;
use proto::{
    CsvcMsgCreateStringTable, CsvcMsgFlattenedSerializer, CsvcMsgPacketEntities, CsvcMsgServerInfo,
    CsvcMsgUpdateStringTable, SvcMessages, CDemoClassInfo, CDemoFullPacket, CDemoPacket, CDemoSendTables, CnetMsgTick, EDemoCommands,
    NetMessages, EDotaUserMessages, EBaseGameEvents, EBaseEntityMessages, EBaseUserMessages
};


// #[cfg(feature = "combat_log")]
use crate::combat_log::CombatLog;
// #[cfg(feature = "combat_log")]
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

    // #[cfg(feature = "combat_log")]
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

            // observers: Observers::new(),
            observers: Vec::new(),

            // #[cfg(feature = "combat_log")]
            combat_log: VecDeque::new(),

            tick: 0,
            net_tick: 0,
            game_build: None,
        }
    }

    pub fn register_observer<T: Observer + 'static>(&mut self, obs: Rc<RefCell<T>>) {
        self.observers.push(obs)
    }

    fn on_packet(&mut self, cmd: EDemoCommands, msg: &[u8]) {
        match cmd {
            EDemoCommands::DemSendTables => self.send_tables(msg),
            EDemoCommands::DemClassInfo => self.class_info(msg),
            EDemoCommands::DemPacket | EDemoCommands::DemSignonPacket => self.dem_packet(msg),
            EDemoCommands::DemFullPacket => self.full_packet(msg),
            _ => {}
        }

        self.observers.iter().for_each(|obs| obs.borrow_mut().on_packet(self, cmd, msg))
    }

    fn on_net_message(&mut self, p: NetMessages, msg: &[u8]) {
        if p == NetMessages::NetTick {
            let t = CnetMsgTick::decode(msg).unwrap();
            self.net_tick = t.tick();
        }

        self.observers.iter().for_each(|obs| obs.borrow_mut().on_net_message(self, p, msg))

    }

    fn on_svc_message(&mut self, p: SvcMessages, msg: &[u8]) {
        match p {
            SvcMessages::SvcServerInfo => self.server_info(msg),
            SvcMessages::SvcCreateStringTable => self.create_string_table(msg),
            SvcMessages::SvcUpdateStringTable => self.update_string_table(msg),
            SvcMessages::SvcPacketEntities => self.packet_entities(msg),
            _ => {}
        }
        self.observers.iter().for_each(|obs| obs.borrow_mut().on_svc_message(self, p, msg))
    }

    fn on_base_user_message(&mut self, p: EBaseUserMessages, msg: &[u8]) {
        self.observers.iter().for_each(|obs| obs.borrow_mut().on_base_user_message(self, p, msg))
    }

    fn on_base_entity_message(&mut self, p: EBaseEntityMessages, msg: &[u8]) {
        self.observers.iter().for_each(|obs| obs.borrow_mut().on_base_entity_message(self, p, msg))

    }

    fn on_base_game_event(&mut self, p: EBaseGameEvents, msg: &[u8]) {
        self.observers.iter().for_each(|obs| obs.borrow_mut().on_base_game_event(self, p, msg))
    }

    fn on_dota_user_message(&mut self, p: EDotaUserMessages, msg: &[u8]) {
        // #[cfg(feature = "combat_log")]
        if p == EDotaUserMessages::DotaUmCombatLogDataHltv {
            let entry = CMsgDotaCombatLogEntry::decode(msg).unwrap();
            self.combat_log.push_back(entry);
        }
        self.observers.iter().for_each(|obs| obs.borrow_mut().on_dota_user_message(self, p, msg))
    }

    pub fn process(&mut self) {
        let reader = &mut self.reader;
        let _ = reader.read_bytes(16);
        let start = Instant::now();
        while let Some(message) = self.read_outer_message() {
            self.on_tick_start();
            self.on_packet(message.msg_type, message.buf.as_slice());
            self.on_tick_end();
        }
        // self.observers.epilogue(self);
        self.observers.iter().for_each(|obs| obs.borrow_mut().epilogue(self));
        println!("{:?}", start.elapsed());
    }

    fn send_tables(&mut self, msg: &[u8]) {
        let send_tables = CDemoSendTables::decode(msg).unwrap();
        let mut r = Reader::new(send_tables.data());
        let amount = r.read_var_u32();
        let buf = r.read_bytes(amount);

        let fs = CsvcMsgFlattenedSerializer::decode(buf.as_slice()).unwrap();

        let mut patches: Vec<&FieldPatch> = vec![];
        for patch in &FIELD_PATCHES {
            if patch.should_apply(self.game_build.unwrap()) {
                patches.push(patch);
            }
        }

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
                            field.var_type.clone().parse().unwrap(),
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
            if let Some(x) = self.classes.get_by_name_mut(ser_name.as_ref()) {
                x.borrow_mut().serializer = Rc::clone(self.serializers.get(&ser_name).unwrap());
            }
        }
    }

    fn class_info(&mut self, msg: &[u8]) {
        let info = CDemoClassInfo::decode(msg).unwrap();
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
                .insert(network_name.into_boxed_str(), Rc::clone(&c));
        }
        self.classes.class_info = true;
        self.update_instance_baseline();
    }

    fn dem_packet(&mut self, msg: &[u8]) {
        let packet = CDemoPacket::decode(msg).unwrap();
        let mut packet_reader = Reader::new(packet.data());
        while packet_reader.remain_bytes() > 0 {
            let t = packet_reader.read_ubit_var() as i32;
            let size = packet_reader.read_var_u32();
            let packet_buf = packet_reader.read_bytes(size);
            let message = PendingMessage::new(self.tick, t, packet_buf);
            self.pending_messages.push_back(message)
        }
        self.process_messages();
    }

    fn full_packet(&mut self, msg: &[u8]) {
        let packet = CDemoFullPacket::decode(msg).unwrap();
        self.on_packet(
            EDemoCommands::DemPacket,
            packet.packet.unwrap().encode_to_vec().as_slice(),
        );
    }

    pub(crate) fn update_instance_baseline(&mut self) {
        if self.classes.class_info {
            if let Some(st) = self.string_tables.get_by_name("instancebaseline") {
                for item in st.items.values() {
                    let class_id = item.key.parse::<i32>().unwrap();
                    self.classes
                        .class_base_lines
                        .insert(class_id, Rc::clone(&item.value));
                }
            }
        }
    }

    fn read_outer_message(&mut self) -> Option<OuterMessage> {
        let reader = &mut self.reader;

        if reader.remain_bytes() == 0 {
            return None;
        }

        let raw_command = reader.read_var_u32() as i32;
        let msg_type = EDemoCommands::from_i32(raw_command & !(EDemoCommands::DemIsCompressed as i32)).unwrap();
        let msg_compressed = (raw_command & EDemoCommands::DemIsCompressed as i32) == EDemoCommands::DemIsCompressed as i32;
        let tick = match reader.read_var_u32() {
            4294967295 => 0,
            x => x,
        };

        let size = reader.read_var_u32();
        let mut buf = reader.read_bytes(size);

        if msg_compressed {
            let mut decoder = snap::raw::Decoder::new();
            buf = decoder.decompress_vec(&buf).expect("Error");
        }

        self.tick = tick;

        Some(OuterMessage {
            tick,
            msg_type,
            buf,
        })
    }

    fn process_messages(&mut self) {
        while let Some(message) = self.pending_messages.pop_front() {
            if let Some(msg) = NetMessages::from_i32(message.msg_type) {
                self.on_net_message(msg, &message.buf);
            }

            if let Some(msg) = EBaseUserMessages::from_i32(message.msg_type) {
                self.on_base_user_message(msg, &message.buf);
            }

            if let Some(msg) = EBaseGameEvents::from_i32(message.msg_type) {
                self.on_base_game_event(msg, &message.buf);
            }

            if let Some(msg) = EBaseEntityMessages::from_i32(message.msg_type) {
                self.on_base_entity_message(msg, &message.buf);
            }

            if let Some(msg) = SvcMessages::from_i32(message.msg_type) {
                self.on_svc_message(msg, &message.buf);
            }

            if let Some(msg) = EDotaUserMessages::from_i32(message.msg_type) {
                self.on_dota_user_message(msg, &message.buf);
            }
        }
    }

    fn packet_entities(&mut self, msg: &[u8]) {
        let packet = CsvcMsgPacketEntities::decode(msg).unwrap();
        let packet_entity = packet.entity_data.unwrap();

        let mut r = Reader::new(packet_entity.as_slice());

        let updates = packet.updated_entries.unwrap();
        let mut index = -1;
        let mut cmd: u32;
        let mut class_id: i32;
        let mut serial: i32;
        let mut e: &mut Entity;
        let mut op: isize;

        if !packet.is_delta.unwrap() {
            if self.entities.entity_full_packets > 0 {
                return;
            }
            self.entities.entity_full_packets += 1;
        }

        for _ in 0..updates {
            index += r.read_ubit_var() as i32 + 1;

            cmd = r.read_bits(2);

            if cmd & 0x01 == 0 {
                if cmd & 0x02 != 0 {
                    class_id = r.read_bits(self.classes.class_id_size.unwrap()) as i32;
                    serial = r.read_bits(17) as i32;
                    r.read_var_u32();

                    let class = Rc::clone(self.classes.get_by_id_mut(class_id).unwrap());
                    let baseline = Rc::clone(&self.classes.class_base_lines[&class_id]);

                    self.entities
                        .entities
                        .insert(index, Entity::new(index, serial, Rc::clone(&class)));
                    e = self.entities.entities.get_mut(&index).unwrap();

                    self.field_reader.read_fields(
                        &mut Reader::new(baseline.as_slice()),
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
                    e = self.entities.entities.get_mut(&index).unwrap();
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
        self.process_entities();
    }

    fn update_string_table(&mut self, msg: &[u8]) {
        let st = CsvcMsgUpdateStringTable::decode(msg).unwrap();
        let t = match self.string_tables.tables.get_mut(&st.table_id.unwrap()) {
            Some(x) => x,
            None => panic!(),
        };

        match t.parse(
            st.string_data.unwrap().as_slice(),
            st.num_changed_entries.unwrap(),
        ) {
            Some(items) => {
                for item in items {
                    let index = item.index;
                    match t.items.get_mut(&index) {
                        Some(x) => {
                            if item.key != "" && item.key != x.key {
                                x.key = item.key;
                            }
                            if item.value.len() > 0 {
                                x.value = item.value;
                            }
                        }
                        None => {
                            t.items.insert(index, item);
                        }
                    }
                }
            }
            None => println!("{}", t.name),
        }
        if t.name == "instancebaseline" {
            self.update_instance_baseline();
        }
    }

    fn create_string_table(&mut self, msg: &[u8]) {
        let mut st = CsvcMsgCreateStringTable::decode(msg).unwrap();

        let mut t = StringTable {
            index: self.string_tables.next_index,
            name: st.name().to_string(),
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
                decoder
                    .decompress_vec(st.string_data.unwrap().as_slice())
                    .unwrap()
            }
            false => st.string_data.unwrap(),
        };

        let name = t.name.clone();
        if let Some(items) = t.parse(buf.as_slice(), st.num_entries.unwrap()) {
            for item in items {
                t.items.insert(item.index, item);
            }

            self.string_tables.name_index.insert(name.clone(), t.index);
            self.string_tables.tables.insert(t.index, t);
        }
        if name == "instancebaseline" {
            self.update_instance_baseline();
        }
    }

    fn server_info(&mut self, msg: &[u8]) {
        let info = CsvcMsgServerInfo::decode(msg).unwrap();
        self.classes.class_id_size = Some((f64::log2(info.max_classes() as f64) + 1.0) as u32);

        let game_build_regexp = Regex::new(r"/dota_v(\d+)/").unwrap();

        if let Some(captures) = game_build_regexp.captures(info.game_dir()) {
            if let Some(build_match) = captures.get(1) {
                let build_str = build_match.as_str();
                let build = build_str.parse::<u32>().unwrap();
                self.game_build = Some(build);
            } else {
                panic!("No build number found in regex capture");
            }
        } else {
            panic!("Failed to parse build number: '{}'", info.game_dir());
        }
    }

    fn process_entities(&mut self) {
        while let Some((index, op)) = self.entities.undone_entities.pop_front() {
            if op & EntityEvent::Created as isize != 0 {
                self.observers
                    .iter()
                    .for_each(|obs| obs
                        .borrow_mut()
                        .on_entity(
                            self,
                            EntityEvent::Created,
                            &self.entities.entities[&index])
                    )
            }
            if op & EntityEvent::Entered as isize != 0 {
                self.observers
                    .iter()
                    .for_each(|obs| obs
                        .borrow_mut()
                        .on_entity(
                            self,
                            EntityEvent::Entered,
                            &self.entities.entities[&index])
                    )
            }
            if op & EntityEvent::Updated as isize != 0 {
                self.observers
                    .iter()
                    .for_each(|obs| obs
                        .borrow_mut()
                        .on_entity(
                            self,
                            EntityEvent::Entered,
                            &self.entities.entities[&index])
                    )
            }
            if op & EntityEvent::Left as isize != 0 {
                self.observers
                    .iter()
                    .for_each(|obs| obs
                        .borrow_mut()
                        .on_entity(
                            self,
                            EntityEvent::Left,
                            &self.entities.entities[&index])
                    )
            }
            if op & EntityEvent::Deleted as isize != 0 {
                self.observers
                    .iter()
                    .for_each(|obs| obs
                        .borrow_mut()
                        .on_entity(
                            self,
                            EntityEvent::Deleted,
                            &self.entities.entities[&index])
                    )
            }

            if op & EntityEvent::Deleted as isize != 0 {
                self.entities.entities.remove(&index);
            }
        }
    }

    pub(crate) fn on_tick_start(&mut self) {
        self.observers.iter().for_each(|obs| obs.borrow_mut().on_tick_start(self))
    }

    pub(crate) fn on_tick_end(&mut self) {
        // #[cfg(feature = "combat_log")]
        while let Some(entry) = self.combat_log.pop_front() {
            let log = CombatLog {
                names: self
                    .string_tables
                    .get_by_name("CombatLogNames")
                    .unwrap(),
                log: entry,
            };
            self.on_combat_log(&log);
        }

        self.observers.iter().for_each(|obs| obs.borrow_mut().on_tick_end(self))
    }

    // #[cfg(feature = "combat_log")]
    pub(crate) fn on_combat_log(&self, entry: &CombatLog) {
        self.observers.iter().for_each(|obs| obs.borrow_mut().on_combat_log(self, entry))

    }
}

#[allow(unused_variables)]
pub trait Observer {
    // Raw data

    /// Function that will be called each time DemoCommand is received.
    /// Provides reference to the parser instance, enum variant of demo command and raw data
    /// ```
    /// use stampede::prelude::*;
    /// use stampede::proto::*;
    ///
    /// struct MyObserver {
    ///     packet_amount: u32
    /// }
    ///
    /// impl Observer for MyObserver {
    ///     fn on_packet(&mut self, ctx: &Parser, p: EDemoCommands, msg: &[u8]) {
    ///         if p == EDemoCommands::DemPacket {
    ///             let packet = CDemoPacket::decode(msg)?;
    ///             self.packet_amount += 1;
    ///         }
    ///     }
    ///
    ///     fn epilogue(&mut self, ctx: &Parser) {
    ///         println!("Total packets: {}", self.packet_amount);
    ///     }
    /// }
    /// ```
    fn on_packet(&mut self, ctx: &Parser, cmd: EDemoCommands, msg: &[u8]) {}

    // on_message?
    fn on_net_message(&mut self, ctx: &Parser, emsg: NetMessages, msg: &[u8]) {}
    fn on_svc_message(&mut self, ctx: &Parser, emsg: SvcMessages, msg: &[u8]) {}
    fn on_base_user_message(&mut self, ctx: &Parser, emsg: EBaseUserMessages, msg: &[u8]) {}
    fn on_base_entity_message(&mut self, ctx: &Parser, emsg: EBaseEntityMessages, msg: &[u8]) {}
    fn on_base_game_event(&mut self, ctx: &Parser, emsg: EBaseGameEvents, msg: &[u8]) {}
    fn on_dota_user_message(&mut self, ctx: &Parser, emsg: EDotaUserMessages, msg: &[u8]) {}
    fn on_tick_start(&mut self, ctx: &Parser) {}
    fn on_tick_end(&mut self, ctx: &Parser) {}
    fn on_entity(&mut self, ctx: &Parser, event: EntityEvent, entity: &Entity) {}

    // #[cfg(feature = "combat_log")]
    fn on_combat_log(&mut self, ctx: &Parser, combat_log: &CombatLog) {}

    // TODO: on_game_event
    // fn on_game_event(&mut self, ctx: &Parser, combat_log: &CombatLog) {}

    fn epilogue(&mut self, ctx: &Parser) {}
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
