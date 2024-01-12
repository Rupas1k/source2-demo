use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Deref;
use std::rc::Rc;
use std::time::Instant;
use protobuf::{Enum, Message};
use protogen::demo::{CDemoClassInfo, CDemoFullPacket, CDemoPacket, CDemoSendTables, EDemoCommands};
use protogen::dota_usermessages::{EDotaUserMessages};
use protogen::gameevents::EBaseGameEvents;
use protogen::netmessages::{CSVCMsg_CreateStringTable, CSVCMsg_FlattenedSerializer, CSVCMsg_PacketEntities, CSVCMsg_ServerInfo, CSVCMsg_UpdateStringTable, SVC_Messages};
use protogen::networkbasetypes::*;
use protogen::usermessages::{EBaseEntityMessages, EBaseUserMessages};
use regex::Regex;
use crate::class::Class;
use crate::demo::{PendingMessage};
use crate::entities::{Entity, EntityOperations};
use crate::field::{Field, FIELD_PATCHES, FieldModels, FieldPatch, FieldReader, FieldType};
use crate::reader::{Reader, ReaderMethods};
use crate::serializer::Serializer;
use crate::string_table::{StringTable, StringTables};


#[derive(Debug)]
struct OuterMessage {
    tick: u32,
    msg_type: EDemoCommands,
    buf: Vec<u8>
}


pub struct Stampede<'a> {
    reader: Reader,
    field_reader: FieldReader,
    pending_messages: VecDeque<PendingMessage>,
    tick: u32,
    // net_tick: u32,
    game_build: Option<u32>,
    class_base_lines: HashMap<i32, Vec<u8>>,
    classes_by_id: HashMap<i32, Rc<RefCell<Class>>>,
    classes_by_name: HashMap<String, Rc<RefCell<Class>>>,
    class_id_size: Option<u32>,
    class_info: bool,

    entities: HashMap<i32, Entity>,
    undone_entities: Vec<(i32, EntityOperations)>,
    entity_full_packets: i32,

    serializers: HashMap<String, Rc<RefCell<Serializer>>>,
    string_tables: StringTables,
    external: &'a dyn External,
}

impl<'a> Stampede<'a> {
    pub fn new(reader: Reader, external: &'a (dyn External + 'a)) -> Self {
        Stampede {
            reader,
            external,
            pending_messages: VecDeque::new(),
            field_reader: FieldReader::new(),
            game_build: None,
            class_base_lines: HashMap::new(),
            classes_by_id: HashMap::new(),
            classes_by_name: HashMap::new(),
            class_id_size: None,
            class_info: false,
            entities: HashMap::new(),
            undone_entities: Vec::new(),
            entity_full_packets: 0,
            serializers: HashMap::new(),
            string_tables: StringTables::new(),
            tick: 0,
        }
    }

    pub fn update_instance_baseline(&mut self) {
        if !self.class_info { return }
        if let Some(st) = self.string_tables.get_table_by_name("instancebaseline") {
            for (_, item) in &st.items {
                let class_id = item.key.parse::<i32>().unwrap();
                self.class_base_lines.insert(class_id, item.value.clone());
            }

        }

    }

    pub fn process(&mut self) {
        let reader = &mut self.reader;
        let _ = reader.read_bytes(16);
        let start = Instant::now();
        loop {
            match self.read_outer_message() {
                Some(message) => {
                    self.on_packet(message.msg_type, message.buf)
                }
                None => {
                    break;
                }
            }
        }
        println!("{:?}", start.elapsed())
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


    // fn process_messages(&mut self, messages: PendingMessages) {
    fn process_messages(&mut self) {
        // for message in messages.messages {
        while let Some(message) = self.pending_messages.pop_front() {
            // println!("{}", message.msg_type);

            match NET_Messages::from_i32(message.msg_type) {
                Some(msg) => self.on_net_message(msg, &message.buf),
                None => {}
            }

            match EBaseUserMessages::from_i32(message.msg_type) {
                Some(msg) => self.on_base_user_message(msg, &message.buf),
                None => {}
            }

            match EBaseGameEvents::from_i32(message.msg_type) {
                Some(msg) => self.on_base_game_event(msg, &message.buf),
                None => {}
            }

            match EBaseEntityMessages::from_i32(message.msg_type) {
                Some(msg) => self.on_base_entity_message(msg, &message.buf),
                None => {}
            }

            match SVC_Messages::from_i32(message.msg_type) {
                Some(msg) => self.on_svc_message(msg, &message.buf),
                None => {}
            }

            match EDotaUserMessages::from_i32(message.msg_type) {
                Some(msg) => self.on_dota_user_message(msg, &message.buf),
                None => {}
            }
        }
    }


    fn on_packet(&mut self, p: EDemoCommands, msg: Vec<u8>) {
        match p {
            EDemoCommands::DEM_Error => {}
            EDemoCommands::DEM_Stop => {}
            EDemoCommands::DEM_FileHeader => {}
            EDemoCommands::DEM_FileInfo => {}
            EDemoCommands::DEM_SyncTick => {}
            EDemoCommands::DEM_SendTables => {
                let mut send_tables = CDemoSendTables::new();
                send_tables.merge_from_bytes(&msg);
                let mut r = Reader::new(send_tables.data().to_vec());
                let amount = r.read_var_u32();
                let buf = r.read_bytes(amount);

                let mut fs = CSVCMsg_FlattenedSerializer::new();
                fs.merge_from_bytes(&buf);

                let mut patches: Vec<&FieldPatch> = vec![];
                for patch in &FIELD_PATCHES {
                    if patch.should_apply(self.game_build.unwrap()) {
                        patches.push(patch);
                    }
                }

                let mut fields = HashMap::<i32, Field>::new();
                let mut field_types = HashMap::<String, FieldType>::new();

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

                for s in fs.serializers.iter() {
                    let mut serializer = Serializer::new(fs.symbols[s.serializer_name_sym.unwrap() as usize].to_string(), s.serializer_version.unwrap());

                    for i in s.fields_index.iter() {
                        if fields.get(i).is_none() {
                            let mut field = Field::new(fs.clone(), fs.fields[*i as usize].clone());

                            if self.game_build.unwrap() <= 990 {
                                field.parent = Some(serializer.name.clone());
                            }

                            // println!("{:?}", serializer);

                            if field_types.get(&field.var_type).is_none() {
                                field_types.insert(field.var_type.clone(), FieldType::new(&field.var_type));
                            }
                            field.field_type = Some(field_types.get(&field.var_type).unwrap().clone());

                            if &field.serializer_name != "" {
                                // field.serializer = Some(&self.serializers.get(field.serializer_name).unwrap());
                                field.serializer = match self.serializers.get(field.serializer_name.as_str()) {
                                    Some(s) => Some(Rc::clone(s)),
                                    None => None
                                };
                                // println!("{:?}", field);
                                // field.serializer = None;
                            }

                            for patch in patches.iter() {
                                (patch.patch)(&mut field);
                            }

                            if field.serializer.is_some() {
                                if field.field_type.as_ref().unwrap().pointer || pointer_types.get(field.field_type.as_ref().unwrap().base.as_str()).is_some() {
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

                            // println!("{:?}", field);

                            // println!("{:?}", &fields);
                            fields.insert(*i, field);
                        }
                        serializer.fields.push(fields[i].clone());
                    }
                    // println!("{:?}", &serializer);
                    // println!("\n");
                    self.serializers.insert(serializer.name.clone(), Rc::new(RefCell::new(serializer.clone())));
                    if self.classes_by_name.get(&serializer.name).is_some() {
                        self.classes_by_name.get_mut(&serializer.name).unwrap().borrow_mut().serializer = Rc::clone(self.serializers.get(&serializer.name).unwrap());
                    }
                }
            }
            EDemoCommands::DEM_ClassInfo => {
                let mut info = CDemoClassInfo::new();
                info.merge_from_bytes(&msg);

                for class in info.classes.iter() {
                    let class_id = class.class_id.unwrap();
                    let network_name = class.network_name.clone().unwrap().to_string();
                    let c = Rc::new(RefCell::new(Class::new(class_id, network_name.clone(), Rc::clone(&self.serializers[&network_name]))));
                    self.classes_by_id.insert(class_id, Rc::clone(&c));
                    self.classes_by_name.insert(network_name, Rc::clone(&c));
                }
                self.class_info = true;
                self.update_instance_baseline();
            }
            EDemoCommands::DEM_StringTables => {}
            EDemoCommands::DEM_Packet | EDemoCommands::DEM_SignonPacket => {
                let mut packet = CDemoPacket::new();
                let _ = packet.merge_from_bytes(&msg);
                let mut packet_reader = Reader::new(packet.data().to_vec());
                // let mut messages = PendingMessages::new();
                while packet_reader.remain_bytes() > 0 {
                    let t = packet_reader.read_ubit_var() as i32;
                    let size = packet_reader.read_var_u32();
                    let packet_buf = packet_reader.read_bytes(size);
                    let message = PendingMessage::new(self.tick, t, packet_buf);
                    // println!("{}", t);
                    self.pending_messages.push_back(message)
                    // messages.push(message);
                    // self.pending_messages.push(message);
                }
                // println!();

                // println!("{}", messages.len());

                // messages.sort();
                // self.pending_messages.sort();

                self.process_messages();
                // self.process_messages(messages);
                // println!("{:?}", packet.data());
            }
            EDemoCommands::DEM_ConsoleCmd => {}
            EDemoCommands::DEM_CustomData => {}
            EDemoCommands::DEM_CustomDataCallbacks => {}
            EDemoCommands::DEM_UserCmd => {}
            EDemoCommands::DEM_FullPacket => {
                let mut packet = CDemoFullPacket::new();
                packet.merge_from_bytes(&msg);

                self.on_packet(EDemoCommands::DEM_StringTables, packet.string_table.unwrap().write_to_bytes().unwrap());
                self.on_packet(EDemoCommands::DEM_Packet, packet.packet.unwrap().write_to_bytes().unwrap());
            }
            EDemoCommands::DEM_SaveGame => {}
            EDemoCommands::DEM_SpawnGroups => {}
            EDemoCommands::DEM_Max => {}
            EDemoCommands::DEM_IsCompressed => {}
        }

        self.external.on_packet(&self, p, &msg);
    }

    fn on_net_message(&self, p: NET_Messages, msg: &Vec<u8>) {
        if let NET_Messages::net_Tick = p {
            self.on_tick_start();
        }
        self.external.on_net_message(self, p, msg);
    }

    fn on_svc_message(&mut self, p: SVC_Messages, msg: &Vec<u8>) {
        match p {
            SVC_Messages::svc_ServerInfo => {
                let mut info = CSVCMsg_ServerInfo::new();
                info.merge_from_bytes(msg).expect("Error");

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
            SVC_Messages::svc_CreateStringTable => {
                let mut st = CSVCMsg_CreateStringTable::new();
                st.merge_from_bytes(&msg);

                let mut t = StringTable {
                    index: self.string_tables.next_index,
                    name: st.name.unwrap().to_string(),
                    items: HashMap::new(),
                    user_data_fixed_size: st.user_data_fixed_size.unwrap(),
                    user_data_size: st.user_data_size.unwrap(),
                    flags: st.flags.unwrap() as u32,
                    var_int_bit_counts: st.using_varint_bitcounts.unwrap(),
                };

                self.string_tables.next_index += 1;

                let mut buf = st.string_data.unwrap().to_vec();

                if st.data_compressed.unwrap() {
                    let mut decoder = snap::raw::Decoder::new();
                    buf = decoder.decompress_vec(&buf).expect("Error");
                    // let mut r = Reader::new(buf.clone());
                    // if r.read_string_n(4) != "LZSS" {
                    //     let mut decoder = snap::raw::Decoder::new();
                    //     buf = decoder.decompress_vec(&buf).expect("Error");
                    // }
                }

                // let items = t.parse(buf, st.num_entries());
                match t.parse(buf, st.num_entries.unwrap()) {
                    Some(items) => {
                        for item in items {
                            t.items.insert(item.index, item);
                        }

                        self.string_tables.tables.insert(t.index, t.clone());
                        self.string_tables.name_index.insert(t.name.clone(), t.index);

                    }
                    // None => println!("{}", t.name)
                    None => {}
                }
                if t.name == "instancebaseline" {
                    self.update_instance_baseline();
                }
            }
            SVC_Messages::svc_UpdateStringTable => {
                let mut st = CSVCMsg_UpdateStringTable::new();
                st.merge_from_bytes(&msg);
                let mut t = match self.string_tables.tables.get_mut(&st.table_id.unwrap()) {
                    Some(x) => x,
                    None => return
                };

                // let items = &mut t.parse(st.string_data().to_vec(), st.num_changed_entries());
                match &mut t.parse(st.string_data.unwrap().to_vec(), st.num_changed_entries.unwrap()) {
                    Some(items) => {
                        for item in items {
                            let index = item.index;
                            match t.items.get(&index) {
                                Some(x) => {
                                    if item.key != "" && item.key != t.items.get(&index).unwrap().key {
                                        t.items.get_mut(&index).unwrap().key = item.key.clone();
                                    }
                                    if item.value.len() > 0 {
                                        t.items.get_mut(&index).unwrap().value = item.value.clone();
                                    }
                                }
                                None => {
                                    t.items.insert(index, item.clone());
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
            SVC_Messages::svc_PacketEntities => {
                let mut packet = CSVCMsg_PacketEntities::new();
                packet.merge_from_bytes(&msg);

                let mut r = Reader::new(packet.entity_data.unwrap().to_vec());

                let updates = packet.updated_entries.unwrap();
                let mut index = -1;
                let mut cmd: u32;
                let mut class_id: i32;
                let mut serial: i32;
                let mut e: &mut Entity;
                let mut op: EntityOperations;

                if !packet.is_delta.unwrap() {
                    if self.entity_full_packets > 0 {
                        return;
                    }
                    self.entity_full_packets += 1;
                }

                for i in 0..updates {
                    index += r.read_ubit_var() as i32 + 1;
                    op = EntityOperations::None;

                    cmd = r.read_bits(2);

                    if cmd & 0x01 == 0 {
                        if cmd & 0x02 != 0 {
                            class_id = r.read_bits(self.class_id_size.unwrap()) as i32;
                            serial = r.read_bits(17) as i32;
                            r.read_var_u32();

                            let class = Rc::clone(self.classes_by_id.get(&class_id).unwrap());
                            let baseline = self.class_base_lines.get(&class_id).unwrap().clone();

                            self.entities.insert(index, Entity::new(index, serial, Rc::clone(&class)));
                            e = self.entities.get_mut(&index).unwrap();

                            self.field_reader.read_fields(&mut Reader::new(baseline.clone()), &e.class.borrow().serializer.borrow(), &mut e.state);

                            self.field_reader.read_fields(&mut r, &e.class.borrow().serializer.borrow(), &mut e.state);

                            op = EntityOperations::CreatedEntered;
                        } else {
                            op = EntityOperations::Updated;
                            e = self.entities.get_mut(&index).unwrap();
                            if !e.active {
                                e.active = true;
                                op = EntityOperations::UpdatedEntered;
                            }
                            self.field_reader.read_fields(&mut r, &e.class.borrow().serializer.borrow(), &mut e.state);
                        }
                    } else {
                        op = EntityOperations::Left;
                        if cmd & 0x02 != 0 {
                            op = EntityOperations::DeletedLeft;
                        }
                    }
                    self.undone_entities.push((index, op));
                }
                self.process_entities();
            }
            _ => {}
        }


        self.external.on_svc_message(self, p, msg);
    }

    fn on_base_user_message(&self, p: EBaseUserMessages, msg: &Vec<u8>) {
        self.external.on_base_user_message(self, p, msg);
    }

    fn on_base_entity_message(&self, p: EBaseEntityMessages, msg: &Vec<u8>) {
        self.external.on_base_entity_message(self, p, msg);
    }

    fn on_base_game_event(&self, p: EBaseGameEvents, msg: &Vec<u8>) {
        self.external.on_base_game_event(self, p, msg);
    }

    fn on_dota_user_message(&self, p: EDotaUserMessages, msg: &Vec<u8>) {
        self.external.on_dota_user_message(self, p, msg);
    }

    fn process_entities(&mut self) {
        while let Some((index, op)) = self.undone_entities.pop() {
            self.external.on_entity(&self, &op, self.entities.get(&index).unwrap());
            if let EntityOperations::DeletedLeft = op {
                // println!("{:?}", self.entities.get(&index).unwrap().class.borrow().name);
                // println!("{:?}", self.entities.get(&index).unwrap().map());
                self.entities.remove(&index);
            }
        }
    }

    pub fn on_tick_start(&self) {
        self.external.on_tick_start(&self)
    }

    pub fn get_entity_by_id(&self, id: i32) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entities_by_class_name(&self, name: &str) -> Vec<&Entity> {
        let mut result = Vec::<&Entity>::new();
        for (_, entity) in &self.entities {
            if entity.class.borrow().name == name {
                result.push(entity);
            }
        }
        result
    }

    pub fn get_first_entity_by_class_name(&self, name: &str) -> Option<&Entity> {
        for (_, entity) in &self.entities {
            if entity.class.borrow().name == name {
                return Some(entity)
            }
        }
        None
    }

    pub fn get_class_by_name(&self, name: &str) -> Option<Ref<Class>> {
        if let Some(class) = self.classes_by_name.get(name) {
            Some(class.borrow())
        } else {
            None
        }
    }

    pub fn get_class_by_id(&self, id: i32) -> Option<Ref<Class>> {
        if let Some(class) = self.classes_by_id.get(&id) {
            Some(class.borrow())
        } else {
            None
        }
    }
}

pub trait External {
    fn on_packet(&self, ctx: &Stampede, p: EDemoCommands, msg: &Vec<u8>) {}
    fn on_net_message(&self, ctx: &Stampede, p: NET_Messages, msg: &Vec<u8>) {}
    fn on_svc_message(&self, ctx: &Stampede, p: SVC_Messages, msg: &Vec<u8>) {}
    fn on_base_user_message(&self, ctx: &Stampede, p: EBaseUserMessages, msg: &Vec<u8>) {}
    fn on_base_entity_message(&self, ctx: &Stampede, p: EBaseEntityMessages, msg: &Vec<u8>) {}
    fn on_base_game_event(&self, ctx: &Stampede, p: EBaseGameEvents, msg: &Vec<u8>) {}
    fn on_dota_user_message(&self, ctx: &Stampede, p: EDotaUserMessages, msg: &Vec<u8>) {}
    fn on_entity(&self, ctx: &Stampede, ev: &EntityOperations/* EntityEvent */, e: &Entity) {}
    fn on_tick_start(&self, ctx: &Stampede) {}
}