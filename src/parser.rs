use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use nohash_hasher::IntMap;
use protobuf::{Enum, Message};
use protogen::demo::{CDemoClassInfo, CDemoFullPacket, CDemoPacket, CDemoSendTables, EDemoCommands};
use protogen::dota_shared_enums::CMsgDOTACombatLogEntry;
use protogen::dota_usermessages::{EDotaUserMessages};
use protogen::gameevents::EBaseGameEvents;
use protogen::netmessages::{CSVCMsg_CreateStringTable, CSVCMsg_FlattenedSerializer, CSVCMsg_PacketEntities, CSVCMsg_ServerInfo, CSVCMsg_UpdateStringTable, SVC_Messages};
use protogen::networkbasetypes::*;
use protogen::usermessages::{EBaseEntityMessages, EBaseUserMessages};
use regex::Regex;
use rustc_hash::FxHashMap;
use crate::class::Class;
use crate::combat_log::CombatLog;
use crate::entitiy::{Entity, EntityOperation};
use crate::field::{Field, FieldModels};
use crate::reader::Reader;
use crate::field_patch::{FIELD_PATCHES, FieldPatch};
use crate::field_reader::FieldReader;
use crate::field_type::FieldType;
use crate::serializer::Serializer;
use crate::string_table::{StringTable, StringTables};


#[derive(Debug)]
struct OuterMessage {
    tick: u32,
    msg_type: EDemoCommands,
    buf: Vec<u8>
}

#[derive(Debug, Eq, PartialEq)]
pub struct PendingMessage {
    pub tick: u32,
    pub msg_type: i32,
    pub buf: Vec<u8>
}

impl PendingMessage {
    pub fn new(tick: u32, msg_type: i32, buf: Vec<u8>) -> Self {
        PendingMessage {
            tick,
            msg_type,
            buf
        }
    }
}

pub struct Stampede<'a, T> {
    reader: Reader<'a>,
    field_reader: FieldReader,
    pending_messages: VecDeque<PendingMessage>,
    pub tick: u32,
    game_build: Option<u32>,
    class_base_lines: IntMap<i32, Rc<Vec<u8>>>,
    classes_by_id: IntMap<i32, Rc<RefCell<Class>>>,
    classes_by_name: FxHashMap<String, Rc<RefCell<Class>>>,
    class_id_size: Option<u32>,
    class_info: bool,

    entities: IntMap<i32, Entity>,
    undone_entities: Vec<(i32, EntityOperation)>,
    entity_full_packets: i32,

    pointer_types: HashSet<&'a str>,

    serializers: FxHashMap<Box<str>, Rc<Serializer>>,
    pub string_tables: StringTables,

    combat_log: VecDeque<CMsgDOTACombatLogEntry>,

    pub visitor: Arc<Mutex<Option<T>>>,

    pub parser_start: Instant
}

impl<'a, T: Visitor + 'a> Stampede<'a, T> {
    pub fn new(buf: &'a [u8], external: T) -> Self {

        // TODO: Move it somewhere else
        let pointer_types: HashSet<&str> = [
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
        ].iter().cloned().collect();

        Stampede {
            reader: Reader::new(buf),
            visitor: Arc::new(Mutex::new(Some(external))),
            pending_messages: VecDeque::new(),
            field_reader: FieldReader::new(),
            game_build: None,
            class_base_lines: IntMap::default(),
            classes_by_id: IntMap::default(),
            classes_by_name: FxHashMap::default(),
            class_id_size: None,
            class_info: false,
            entities: IntMap::default(),
            undone_entities: Vec::new(),
            entity_full_packets: 0,
            serializers: FxHashMap::default(),
            string_tables: StringTables::new(),

            combat_log: VecDeque::new(),

            pointer_types,

            tick: 0,

            parser_start: Instant::now()
        }
    }

    fn on_packet(&mut self, cmd: EDemoCommands, msg: Vec<u8>) {
        match cmd {
            EDemoCommands::DEM_SendTables => { self.send_tables(&msg) }
            EDemoCommands::DEM_ClassInfo => { self.class_info(&msg) }
            EDemoCommands::DEM_Packet | EDemoCommands::DEM_SignonPacket => { self.dem_packet(&msg) }
            EDemoCommands::DEM_FullPacket => { self.full_packet(&msg) }
            _ => {}
        }
        self.visitor.lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .on_packet(self, cmd, &msg);
    }

    fn on_net_message(&mut self, p: NET_Messages, msg: &Vec<u8>) {
        self.visitor.lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .on_net_message(self, p, msg);
    }

    fn on_svc_message(&mut self, p: SVC_Messages, msg: &Vec<u8>) {
        match p {
            SVC_Messages::svc_ServerInfo        => { self.server_info(&msg) }
            SVC_Messages::svc_CreateStringTable => { self.create_string_table(&msg) }
            SVC_Messages::svc_UpdateStringTable => { self.update_string_table(&msg) }
            SVC_Messages::svc_PacketEntities    => { self.packet_entities(&msg )}
            _ => {}
        }
        self.visitor.lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .on_svc_message(self, p, msg);
    }

    fn on_base_user_message(&mut self, p: EBaseUserMessages, msg: &Vec<u8>) {
        self.visitor.lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .on_base_user_message(self, p, msg);
    }

    fn on_base_entity_message(&mut self, p: EBaseEntityMessages, msg: &Vec<u8>) {
        self.visitor.lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .on_base_entity_message(self, p, msg);
    }

    fn on_base_game_event(&mut self, p: EBaseGameEvents, msg: &Vec<u8>) {
        self.visitor.lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .on_base_game_event(self, p, msg);
    }

    fn on_dota_user_message(&mut self, p: EDotaUserMessages, msg: &Vec<u8>) {
        if p == EDotaUserMessages::DOTA_UM_CombatLogDataHLTV {
            let entry = CMsgDOTACombatLogEntry::parse_from_bytes(&msg).unwrap();
            self.combat_log.push_back(entry);
        }
        self.visitor.lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .on_dota_user_message(self, p, msg);
    }

    pub fn process(mut self) -> T {
        let reader = &mut self.reader;
        let _ = reader.read_bytes(16);
        let start = Instant::now();
        loop {
            match self.read_outer_message() {
                Some(message) => {
                    self.on_tick_start();
                    self.on_packet(message.msg_type, message.buf);
                    self.on_tick_end();
                }
                None => {
                    break;
                }
            }
        }
        println!("{:?}", start.elapsed());
        self.visitor.lock()
            .unwrap()
            .take()
            .unwrap()
    }

    fn send_tables(&mut self, msg: &Vec<u8>) {
        let send_tables = CDemoSendTables::parse_from_bytes(&msg).unwrap();
        let mut r = Reader::new(send_tables.data());
        let amount = r.read_var_u32();
        let buf = r.read_bytes(amount);

        let fs = CSVCMsg_FlattenedSerializer::parse_from_bytes(&buf).unwrap();

        let mut patches: Vec<&FieldPatch> = vec![];
        for patch in &FIELD_PATCHES {
            if patch.should_apply(self.game_build.unwrap()) {
                patches.push(patch);
            }
        }

        let mut fields = FxHashMap::<i32, Rc<Field>>::default();
        let mut field_types = FxHashMap::<String, Rc<FieldType>>::default();

        for s in fs.serializers.iter() {
            let mut serializer = Serializer::new(fs.symbols[s.serializer_name_sym.unwrap() as usize].clone(), s.serializer_version.unwrap());

            for i in s.fields_index.iter() {
                if fields.get(i).is_none() {
                    let mut field = Field::new(fs.clone(), fs.fields[*i as usize].clone());
                    if field_types.get(field.var_type.as_ref()).is_none() {
                        field_types.insert(field.var_type.clone().parse().unwrap(), Rc::new(FieldType::new(&field.var_type)));
                    }
                    field.field_type = Some(Rc::clone(&field_types[field.var_type.as_ref()]));

                    if field.serializer_name.as_ref() != "" {
                        field.serializer = self.serializers.get(&field.serializer_name).map(Rc::clone);
                    }

                    for patch in patches.iter() {
                        (patch.patch)(&mut field);
                    }

                    if field.serializer.is_some() {
                        if field.field_type.as_ref().unwrap().pointer || self.pointer_types.get(field.field_type.as_ref().unwrap().base.as_str()).is_some() {
                            field.set_model(FieldModels::FixedTable);
                        } else {
                            field.set_model(FieldModels::VariableTable);
                        }
                    } else if field.field_type.as_ref().unwrap().count.is_some() && field.field_type.as_ref().unwrap().count.unwrap() > 0 && field.field_type.as_ref().unwrap().base != "char" {
                        field.set_model(FieldModels::FixedArray);
                    } else if field.field_type.as_ref().unwrap().base == "CUtlVector" || field.field_type.as_ref().unwrap().base == "CNetworkUtlVectorBase" {
                        field.set_model(FieldModels::VariableArray);
                    } else {
                        field.set_model(FieldModels::Simple);
                    }
                    fields.insert(*i, Rc::new(field));
                }
                // serializer.fields.push(fields[i].clone());
                serializer.fields.push(Rc::clone(&fields[i]));
            }
            let ser_name = serializer.name.clone();
            self.serializers.insert(serializer.name.clone(), Rc::new(serializer));
            if let Some(x) = self.classes_by_name.get(ser_name.as_ref()) {
                x.borrow_mut().serializer = Rc::clone(self.serializers.get(&ser_name).unwrap());
            }
        }
    }

    fn class_info(&mut self, msg: &Vec<u8>) {
        let info = CDemoClassInfo::parse_from_bytes(&msg).unwrap();
        for class in info.classes.iter() {
            let class_id = class.class_id.unwrap();
            let network_name = class.network_name.as_ref().unwrap().clone();
            let c = Rc::new(RefCell::new(Class::new(class_id, network_name.clone(), Rc::clone(&self.serializers[&network_name.clone().into_boxed_str()]))));
            self.classes_by_id.insert(class_id, Rc::clone(&c));
            self.classes_by_name.insert(network_name.clone(), Rc::clone(&c));
        }
        self.class_info = true;
        self.update_instance_baseline();
    }


    fn dem_packet(&mut self, msg: &Vec<u8>) {
        let packet = CDemoPacket::parse_from_bytes(&msg).unwrap();
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


    fn full_packet(&mut self, msg: &Vec<u8>) {
        let packet = CDemoFullPacket::parse_from_bytes(&msg).unwrap();
        // self.on_packet(EDemoCommands::DEM_StringTables, packet.string_table.unwrap().write_to_bytes().unwrap());
        self.on_packet(EDemoCommands::DEM_Packet, packet.packet.unwrap().write_to_bytes().unwrap());
    }

    pub fn update_instance_baseline(&mut self) {
        if self.class_info {
            if let Some(st) = self.string_tables.get_table_by_name("instancebaseline") {
                for (_, item) in st.items.iter() {
                    let class_id = item.key.parse::<i32>().unwrap();
                    self.class_base_lines.insert(class_id, Rc::clone(&item.value));
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
        let msg_type = EDemoCommands::from_i32(raw_command & !EDemoCommands::DEM_IsCompressed.value()).unwrap();
        let msg_compressed = (raw_command & EDemoCommands::DEM_IsCompressed.value()) == EDemoCommands::DEM_IsCompressed.value();
        let tick = match reader.read_var_u32() {
            x if x == 4294967295 => 0,
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
            if let Some(msg) = NET_Messages::from_i32(message.msg_type) {
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

            if let Some(msg) = SVC_Messages::from_i32(message.msg_type) {
                self.on_svc_message(msg, &message.buf);
            }

            if let Some(msg) = EDotaUserMessages::from_i32(message.msg_type) {
                self.on_dota_user_message(msg, &message.buf);
            }
        }
    }

    fn packet_entities(&mut self, msg: &Vec<u8>) {
        let packet = CSVCMsg_PacketEntities::parse_from_bytes(&msg).unwrap();
        let packet_entity = packet.entity_data.unwrap();

        let mut r = Reader::new(packet_entity.as_slice());

        let updates = packet.updated_entries.unwrap();
        let mut index = -1;
        let mut cmd: u32;
        let mut class_id: i32;
        let mut serial: i32;
        let mut e: &mut Entity;
        let mut op: EntityOperation;

        if !packet.is_delta.unwrap() {
            if self.entity_full_packets > 0 {
                return;
            }
            self.entity_full_packets += 1;
        }

        for _ in 0..updates {
            index += r.read_ubit_var() as i32 + 1;
            op = EntityOperation::None;

            cmd = r.read_bits(2);

            if cmd & 0x01 == 0 {
                if cmd & 0x02 != 0 {
                    class_id = r.read_bits(self.class_id_size.unwrap()) as i32;
                    serial = r.read_bits(17) as i32;
                    r.read_var_u32();

                    let class = Rc::clone(self.classes_by_id.get(&class_id).unwrap());
                    let baseline = Rc::clone(&self.class_base_lines[&class_id]);

                    self.entities.insert(index, Entity::new(index, serial, Rc::clone(&class)));
                    e = self.entities.get_mut(&index).unwrap();

                    self.field_reader.read_fields(&mut Reader::new(baseline.as_slice()), &e.class.borrow().serializer, &mut e.state);

                    self.field_reader.read_fields(&mut r, &e.class.borrow().serializer, &mut e.state);

                    op = EntityOperation::CreatedEntered;
                } else {
                    op = EntityOperation::Updated;
                    e = self.entities.get_mut(&index).unwrap();
                    if !e.active {
                        e.active = true;
                        op = EntityOperation::UpdatedEntered;
                    }
                    self.field_reader.read_fields(&mut r, &e.class.borrow().serializer, &mut e.state);
                }
            } else {
                op = EntityOperation::Left;
                if cmd & 0x02 != 0 {
                    op = EntityOperation::DeletedLeft;
                }
            }
            self.undone_entities.push((index, op));
        }
        self.process_entities();
    }

    fn update_string_table(&mut self, msg: &Vec<u8>) {
        let st = CSVCMsg_UpdateStringTable::parse_from_bytes(&msg).unwrap();
        let t = match self.string_tables.tables.get_mut(&st.table_id.unwrap()) {
            Some(x) => x,
            None => panic!()
        };

        // let items = &mut t.parse(st.string_data().to_vec(), st.num_changed_entries());
        match t.parse(st.string_data.unwrap().as_slice(), st.num_changed_entries.unwrap()) {
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
            None => println!("{}", t.name)
        }
        if t.name == "instancebaseline" {
            self.update_instance_baseline();
        }
    }

    fn create_string_table(&mut self, msg: &Vec<u8>) {
        let mut st = CSVCMsg_CreateStringTable::parse_from_bytes(&msg).unwrap();

        let mut t = StringTable {
            index: self.string_tables.next_index,
            name: st.name.unwrap().to_string(),
            // items: FxHashMap::default(),
            items: IntMap::default(),
            user_data_fixed_size: st.user_data_fixed_size.unwrap(),
            user_data_size: st.user_data_size.unwrap(),
            flags: st.flags.unwrap() as u32,
            var_int_bit_counts: st.using_varint_bitcounts.unwrap_or_default(),
        };

        self.string_tables.next_index += 1;


        let buf = match st.data_compressed.unwrap() {
            true => {
                let mut decoder = snap::raw::Decoder::new();
                decoder.decompress_vec(st.string_data.unwrap().as_slice()).unwrap()
            }
            false => {
                st.string_data.unwrap()
            }
        };


        let name = t.name.clone();
        match t.parse(buf.as_slice(), st.num_entries.unwrap()) {
            Some(items) => {
                for item in items {
                    t.items.insert(item.index, item);
                }

                self.string_tables.name_index.insert(name.clone(), t.index);
                self.string_tables.tables.insert(t.index, t);

            }
            None => {}
        }
        if name == "instancebaseline" {
            self.update_instance_baseline();
        }
    }

    fn server_info(&mut self, msg: &Vec<u8>) {
        let info = CSVCMsg_ServerInfo::parse_from_bytes(&msg).unwrap();
        self.class_id_size = Some((f64::log2(info.max_classes() as f64) + 1.0) as u32);

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
        while let Some((index, op)) = self.undone_entities.pop() {
            self.visitor.lock()
                .unwrap()
                .as_ref()
                .unwrap()
                .on_entity(&self, &op, &self.entities[&index]);
            if let EntityOperation::DeletedLeft = op {
                self.entities.remove(&index);
            }
        }
    }

    pub fn on_tick_start(&mut self) {
        self.visitor.lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .on_tick_start(self);
    }
    pub fn on_tick_end(&mut self) {
        while let Some(entry) = self.combat_log.pop_front() {
            let log = CombatLog {
                names: self.string_tables.get_table_by_name("CombatLogNames").unwrap(),
                log: entry
            };
            self.on_combat_log(log);
        }

        self.visitor.lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .on_tick_end(self);
    }

    pub fn on_combat_log(&self, entry: CombatLog) {
        self.visitor.lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .on_combat_log(&self, &entry);
    }

    pub fn get_entity_by_id(&self, id: i32) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entities_by_class_name(&self, name: &str) -> Vec<&Entity> {
        let mut result = Vec::<&Entity>::new();
        for (_, entity) in &self.entities {
            if entity.class.borrow().name.as_ref() == name {
                result.push(entity);
            }
        }
        result
    }

    pub fn get_first_entity_by_class_name(&self, name: &str) -> Option<&Entity> {
        for (_, entity) in self.entities.iter() {
            if entity.class.borrow().name.as_ref() == name {
                return Some(entity)
            }
        }
        None
    }

    pub fn get_class_by_name(&self, name: &str) -> Option<Rc<RefCell<Class>>> {
        if let Some(class) = self.classes_by_name.get(name) {
            Some(Rc::clone(&class))
        } else {
            None
        }
    }

    pub fn get_class_by_id(&self, id: i32) -> Option<Rc<RefCell<Class>>> {
        if let Some(class) = self.classes_by_id.get(&id) {
            Some(Rc::clone(&class))
        } else {
            None
        }
    }
}

pub trait Visitor: Sized {
    fn on_packet(&self, ctx: &Stampede<Self>, p: EDemoCommands, msg: &Vec<u8>) {}
    fn on_net_message(&self, ctx: &Stampede<Self>, p: NET_Messages, msg: &Vec<u8>) {}
    fn on_svc_message(&self, ctx: &Stampede<Self>, p: SVC_Messages, msg: &Vec<u8>) {}
    fn on_base_user_message(&self, ctx: &Stampede<Self>, p: EBaseUserMessages, msg: &Vec<u8>) {}
    fn on_base_entity_message(&self, ctx: &Stampede<Self>, p: EBaseEntityMessages, msg: &Vec<u8>) {}
    fn on_base_game_event(&self, ctx: &Stampede<Self>, p: EBaseGameEvents, msg: &Vec<u8>) {}
    fn on_dota_user_message(&self, ctx: &Stampede<Self>, p: EDotaUserMessages, msg: &Vec<u8>) {}
    fn on_entity(&self, ctx: &Stampede<Self>, ev: &EntityOperation, e: &Entity) {}
    fn on_tick_start(&mut self, ctx: &Stampede<Self>) {}
    fn on_tick_end(&mut self, ctx: &Stampede<Self>) {}
    fn on_combat_log(&mut self, ctx: &Stampede<Self>, combat_log: &CombatLog) {}
}


