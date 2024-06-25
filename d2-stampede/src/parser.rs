use crate::class::{Class, Classes};
use crate::combat_log::CombatLog;
use crate::decoder::Decoder;
use crate::entity::{Entities, Entity, EntityEvents};
use crate::field::{Encoder, Field, FieldModel, FieldProperties, FieldType, FieldVector};
use crate::field_reader::FieldReader;
use crate::proto::*;
use crate::reader::Reader;
use crate::serializer::Serializer;
use crate::string_table::{StringTable, StringTableEntry, StringTables};
use crate::try_observers;
use anyhow::{bail, Result};
use hashbrown::{HashMap, HashSet};
use prettytable::{row, Table};
use regex::Regex;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::mem;
use std::rc::Rc;

pub struct Parser<'a> {
    reader: Reader<'a>,
    field_reader: FieldReader,
    observers: Vec<Rc<RefCell<dyn Observer + 'a>>>,
    start_offset: usize,

    combat_log: VecDeque<CMsgDotaCombatLogEntry>,

    prologue_completed: bool,
    processing_deltas: bool,

    pub context: Context,
}

pub(crate) struct Baselines {
    field_reader: FieldReader,
    baselines: HashMap<i32, Rc<Vec<u8>>>,
    states: HashMap<i32, FieldVector>,
}

impl Baselines {
    pub(crate) fn add_baseline(&mut self, id: i32, baseline: Rc<Vec<u8>>) {
        self.baselines.insert(id, baseline);
    }

    pub(crate) fn read_baseline(&mut self, class: &Class) {
        let mut state = FieldVector::new();
        self.field_reader.read_fields(
            &mut Reader::new(&self.baselines[&class.id]),
            &class.serializer,
            &mut state,
        );

        self.states.insert(class.id, state);
    }
}

pub struct Context {
    pub(crate) classes: Classes,
    pub(crate) entities: Entities,
    pub(crate) string_tables: StringTables,
    pub(crate) replay_info: CDemoFileInfo,

    pub(crate) tick: u32,

    pub(crate) net_tick: u32,
    pub(crate) game_build: u32,

    baselines: Baselines,
    serializers: HashMap<Box<str>, Rc<Serializer>>,
    last_full_packet_tick: u32,
}

impl Context {
    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    pub fn string_tables(&self) -> &StringTables {
        &self.string_tables
    }

    pub fn replay_info(&self) -> &CDemoFileInfo {
        &self.replay_info
    }

    pub fn tick(&self) -> u32 {
        self.tick
    }

    pub fn net_tick(&self) -> u32 {
        self.net_tick
    }

    pub fn game_build(&self) -> u32 {
        self.game_build
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["Classes", self.classes.classes_vec.len()]);
        table.add_row(row!["Entities", self.entities.entities_vec.len()]);
        table.add_row(row!["String Tables", self.string_tables.tables.len()]);
        table.add_row(row!["Tick", self.tick]);
        table.add_row(row!["Net Tick", self.net_tick]);
        table.add_row(row!["Game Build", format!("{:?}", self.game_build)]);
        write!(f, "{}", table)
    }
}

struct OuterMessage {
    msg_type: EDemoCommands,
    size: usize,
    tick: u32,
    buf: Vec<u8>,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a [u8]) -> Result<Self> {
    pub fn new(replay: &'a [u8]) -> Result<Self> {
        let baselines = Baselines {
            field_reader: FieldReader::new(),
            baselines: HashMap::default(),
            states: HashMap::default(),
        };

        let mut reader = Reader::new(replay);

        if replay.len() < 16 {
            bail!("Couldn't validate file header")
        }

        if reader.read_bytes(8) != b"PBDEMS2\0" {
            bail!("Supports only Source 2 replays")
        };

        reader.read_bytes(8);

        let replay_info = Self::replay_info(&mut reader)?;

        Ok(Parser {
            reader,
            field_reader: FieldReader::new(),
            observers: Vec::new(),
            combat_log: VecDeque::new(),
            prologue_completed: false,
            start_offset: 0,
            processing_deltas: true,

            context: Context {
                classes: Classes::new(),
                entities: Entities::new(),
                string_tables: StringTables::new(),

                replay_info,

                tick: u32::MAX,
                net_tick: u32::MAX,
                last_full_packet_tick: u32::MAX,

                game_build: 0,

                baselines,
                serializers: HashMap::default(),
            },
        })
    }

    pub fn register_observer<T>(&mut self) -> Rc<RefCell<T>>
    where
        T: Observer + Default + 'static,
    {
        let rc = Rc::new(RefCell::new(T::default()));
        self.observers.push(rc.clone());
        rc.clone()
    }

    fn replay_info(reader: &mut Reader) -> Result<CDemoFileInfo> {
        let offset = u32::from_le_bytes(reader.buf[8..12].try_into()?) as usize;
        if reader.buf.len() < offset {
            bail!("Buf is too small")
        }
        let mut reader = Reader::new(&reader.buf[offset..]);
        Ok(CDemoFileInfo::decode(
            Self::read_message(&mut reader)?.unwrap().buf.as_slice(),
        )?)
    }

    fn prologue(&mut self) -> Result<()> {
        if self.prologue_completed {
            return Ok(());
        }

        let mut offset: usize = 16;
        while let Some(message) = Self::read_message(&mut self.reader)? {
            self.context.tick = message.tick;
            self.on_tick_start()?;
            self.on_packet(message.msg_type, message.buf.as_slice())?;
            self.on_tick_end()?;

            offset += message.size;

            if message.msg_type == EDemoCommands::DemSyncTick {
                self.prologue_completed = true;
                self.start_offset = offset;
                break;
            }
        }

        Ok(())
    }

    pub fn run_to_end(&mut self) -> Result<()> {
        self.prologue()?;

        while let Some(message) = Self::read_message(&mut self.reader)? {
            self.context.tick = message.tick;
            self.on_tick_start()?;
            self.on_packet(message.msg_type, message.buf.as_slice())?;
            self.on_tick_end()?;
        }

        try_observers!(self, epilogue(&self.context))
    }

    pub fn jump_to_tick(&mut self, target_tick: u32) -> Result<()> {
        self.prologue()?;

        if target_tick < self.context.tick {
            self.context.last_full_packet_tick = u32::MAX;
            self.context.tick = u32::MAX;
            self.context.net_tick = u32::MAX;
            self.reader.reset_to(self.start_offset);
        }

        self.processing_deltas = false;

        let observers = mem::take(&mut self.observers);

        let mut first_fp_checked = true;
        let mut last_fp_checked = false;

        while let Some(mut message) = Self::read_message(&mut self.reader)? {
            let next_fp = self.context.last_full_packet_tick == u32::MAX
                || (target_tick - self.context.last_full_packet_tick) > 1800;
            self.context.tick = message.tick;
            if message.msg_type == EDemoCommands::DemFullPacket {
                if next_fp && first_fp_checked {
                    message.msg_type = EDemoCommands::DemStringTables;
                    message.buf = CDemoFullPacket::decode(message.buf.as_slice())?
                        .string_table
                        .unwrap()
                        .encode_to_vec();
                }

                self.on_packet(message.msg_type, message.buf.as_slice())?;
            }

            if last_fp_checked {
                self.on_packet(message.msg_type, message.buf.as_slice())?;
            }

            if message.msg_type == EDemoCommands::DemFullPacket && !first_fp_checked {
                first_fp_checked = true;
            }

            if message.msg_type == EDemoCommands::DemFullPacket && !next_fp {
                last_fp_checked = true;
                self.processing_deltas = true;
            }

            if self.context.tick >= target_tick {
                break;
            }
        }

        self.observers = observers;

        try_observers!(self, epilogue(&self.context))
    }

    pub fn run_to_tick(&mut self, tick: u32) -> Result<()> {
        assert!(tick > self.context.tick);
    pub fn run_to_tick(&mut self, target_tick: u32) -> Result<()> {
        assert!(target_tick > self.context.tick);

        self.prologue()?;

        while let Some(message) = Self::read_message(&mut self.reader)? {
            self.context.tick = message.tick;
            self.on_tick_start()?;
            self.on_packet(message.msg_type, message.buf.as_slice())?;
            self.on_tick_end()?;
            if self.context.tick >= target_tick {
                break;
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn read_message(reader: &mut Reader) -> Result<Option<OuterMessage>> {
        if reader.bytes_remaining() == 0 {
            return Ok(None);
        }

        let start = reader.bytes_remaining();

        let cmd = reader.read_var_u32() as i32;
        let tick = reader.read_var_u32();
        let size = reader.read_var_u32();

        let end = start - reader.bytes_remaining();

        let msg_type = EDemoCommands::try_from(cmd & !(EDemoCommands::DemIsCompressed as i32))?;
        let msg_compressed = cmd & EDemoCommands::DemIsCompressed as i32 != 0;

        let buf = if msg_compressed {
            let buf = reader.read_bytes(size);
            let mut decoder = snap::raw::Decoder::new();
            decoder.decompress_vec(&buf)?
        } else {
            reader.read_bytes(size)
        };

        Ok(Some(OuterMessage {
            size: end + size as usize,
            msg_type,
            tick,
            buf,
        }))
    }

    #[inline(always)]
    fn on_packet(&mut self, msg_type: EDemoCommands, msg: &[u8]) -> Result<()> {
        match msg_type {
            EDemoCommands::DemSendTables => self.dem_send_tables(msg)?,
            EDemoCommands::DemClassInfo => self.dem_class_info(msg)?,
            EDemoCommands::DemPacket | EDemoCommands::DemSignonPacket => self.dem_packet(msg)?,
            EDemoCommands::DemFullPacket => self.dem_full_packet(msg)?,
            EDemoCommands::DemStringTables => self.dem_string_tables(msg)?,
            _ => {}
        };

        try_observers!(self, on_packet(&self.context, msg_type, msg))
    }

    #[inline(always)]
    fn on_net_message(&mut self, msg_type: NetMessages, msg: &[u8]) -> Result<()> {
        if msg_type == NetMessages::NetTick {
            self.context.net_tick = CnetMsgTick::decode(msg)?.tick();
        }

        try_observers!(self, on_net_message(&self.context, msg_type, msg))
    }

    #[inline(always)]
    fn on_svc_message(&mut self, msg_type: SvcMessages, msg: &[u8]) -> Result<()> {
        match msg_type {
            SvcMessages::SvcServerInfo => self.server_info(msg)?,
            SvcMessages::SvcCreateStringTable => self.create_string_table(msg)?,
            SvcMessages::SvcUpdateStringTable => self.update_string_table(msg)?,
            SvcMessages::SvcPacketEntities => self.packet_entities(msg)?,
            _ => {}
        }

        try_observers!(self, on_svc_message(&self.context, msg_type, msg))
    }

    #[inline(always)]
    fn on_base_user_message(&mut self, msg_type: EBaseUserMessages, msg: &[u8]) -> Result<()> {
        try_observers!(self, on_base_user_message(&self.context, msg_type, msg))
    }

    #[inline(always)]
    fn on_base_game_event(&mut self, msg_type: EBaseGameEvents, msg: &[u8]) -> Result<()> {
        try_observers!(self, on_base_game_event(&self.context, msg_type, msg))
    }

    #[inline(always)]
    fn on_dota_user_message(&mut self, msg_type: EDotaUserMessages, msg: &[u8]) -> Result<()> {
        if msg_type == EDotaUserMessages::DotaUmCombatLogDataHltv {
            let entry = CMsgDotaCombatLogEntry::decode(msg)?;
            self.combat_log.push_back(entry);
        }

        try_observers!(self, on_dota_user_message(&self.context, msg_type, msg))
    }

    #[inline(always)]
    pub(crate) fn on_tick_start(&mut self) -> Result<()> {
        try_observers!(self, on_tick_start(&self.context))
    }

    #[inline(always)]
    pub(crate) fn on_tick_end(&mut self) -> Result<()> {
        if let Ok(names) = self.context.string_tables.get_by_name("CombatLogNames") {
            while let Some(entry) = self.combat_log.pop_front() {
                let log = CombatLog {
                    names: &names,
                    log: entry,
                };
                self.on_combat_log(&log)?;
            }
        }

        try_observers!(self, on_tick_end(&self.context))
    }

    #[inline(always)]
    pub(crate) fn on_combat_log(&self, entry: &CombatLog) -> Result<()> {
        try_observers!(self, on_combat_log(&self.context, entry))
    }

    fn dem_send_tables(&mut self, msg: &[u8]) -> Result<()> {
        let send_tables = CDemoSendTables::decode(msg)?;
        let mut reader = Reader::new(send_tables.data());
        let amount = reader.read_var_u32();
        let buf = reader.read_bytes(amount);

        let fs = CsvcMsgFlattenedSerializer::decode(buf.as_slice())?;

        let resolve = |p: Option<i32>| -> Box<str> {
            if let Some(i) = p {
                return fs.symbols[i as usize].clone().into();
            }
            "".into()
        };

        let pointer_types: HashSet<&'static str> = [
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

        let mut fields = vec![];
        let mut field_types = HashMap::<Box<str>, Rc<FieldType>>::default();

        for s in fs.serializers.iter() {
            let serializer_name = fs.symbols[s.serializer_name_sym() as usize].clone();
            let mut serializer = Serializer::new();

            for i in s.fields_index.iter() {
                let current_field = &fs.fields[*i as usize];
                let field_serializer_name = resolve(current_field.field_serializer_name_sym);

                if *i as usize >= fields.len() {
                    let var_type_str = resolve(current_field.var_type_sym);
                    let current_field_serializer = self
                        .context
                        .serializers
                        .get(&field_serializer_name)
                        .cloned();

                    if !field_types.contains_key(&var_type_str) {
                        field_types.insert(
                            var_type_str.clone(),
                            Rc::new(FieldType::new(var_type_str.clone().as_ref())),
                        );
                    }

                    let var_name = resolve(current_field.var_name_sym);
                    let field_type = field_types[&var_type_str].clone();
                    let mut properties = FieldProperties {
                        encoder: Encoder::from_str(&resolve(current_field.var_encoder_sym)),
                        encoder_flags: current_field.encode_flags(),
                        bit_count: current_field.bit_count(),
                        low_value: current_field.low_value(),
                        high_value: current_field.high_value(),
                    };

                    match var_name.as_ref() {
                        "m_flSimulationTime" | "m_flAnimTime" => {
                            properties.encoder = Some(Encoder::SimTime);
                        }
                        "m_flRuneTime" => {
                            properties.encoder = Some(Encoder::RuneTime);
                        }
                        _ => {}
                    }

                    let model = if let Some(serializer) = current_field_serializer {
                        if field_type.pointer || pointer_types.contains(field_type.base.as_ref()) {
                            FieldModel::FixedTable(serializer)
                        } else {
                            FieldModel::VariableTable(serializer)
                        }
                    } else if field_type.count.is_some()
                        && field_type.count.unwrap() > 0
                        && field_type.base.as_ref() != "char"
                    {
                        FieldModel::FixedArray
                    } else if field_type.base.as_ref() == "CUtlVector"
                        || field_type.base.as_ref() == "CNetworkUtlVectorBase"
                    {
                        FieldModel::VariableArray(Decoder::from_field(
                            field_type.generic.as_ref().unwrap(),
                            properties,
                        ))
                    } else {
                        FieldModel::Simple
                    };

                    let decoder = match model {
                        FieldModel::Simple | FieldModel::FixedArray => {
                            Decoder::from_field(&field_type, properties)
                        }
                        FieldModel::VariableArray(_) => Decoder::Unsigned32,
                        FieldModel::FixedTable(_) => Decoder::Boolean,
                        FieldModel::VariableTable(_) => Decoder::Unsigned32,
                    };

                    let field = Field {
                        var_name,
                        field_type,
                        model,

                        decoder,
                    };
                    fields.push(Rc::new(field));
                }
                serializer.fields.push(fields[*i as usize].clone());
            }
            self.context
                .serializers
                .insert(serializer_name.into(), Rc::new(serializer));
        }
        Ok(())
    }

    #[inline(always)]
    fn dem_class_info(&mut self, msg: &[u8]) -> Result<()> {
        let info = CDemoClassInfo::decode(msg)?;
        for class in info.classes {
            let class_id = class.class_id();
            let network_name = class.network_name();

            let serializer = self.context.serializers[network_name].clone();

            let class = Rc::new(Class::new(class_id, network_name.into(), serializer));

            self.context.classes.classes_vec.push(class.clone());
            self.context
                .classes
                .classes_by_name
                .insert(network_name.into(), class);
        }
        Ok(())
    }

    #[inline(always)]
    fn dem_packet(&mut self, msg: &[u8]) -> Result<()> {
        let packet = CDemoPacket::decode(msg)?;
        let mut packet_reader = Reader::new(packet.data());
        while packet_reader.bytes_remaining() != 0 {
            let msg_type = packet_reader.read_ubit_var() as i32;
            let size = packet_reader.read_var_u32();
            let packet_buf = packet_reader.read_bytes(size);

            if let Ok(msg) = EDotaUserMessages::try_from(msg_type) {
                self.on_dota_user_message(msg, &packet_buf)?;
            } else if let Ok(msg) = SvcMessages::try_from(msg_type) {
                self.on_svc_message(msg, &packet_buf)?;
            } else if let Ok(msg) = EBaseUserMessages::try_from(msg_type) {
                self.on_base_user_message(msg, &packet_buf)?;
            } else if let Ok(msg) = EBaseGameEvents::try_from(msg_type) {
                self.on_base_game_event(msg, &packet_buf)?;
            } else if let Ok(msg) = NetMessages::try_from(msg_type) {
                self.on_net_message(msg, &packet_buf)?;
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn dem_full_packet(&mut self, msg: &[u8]) -> Result<()> {
        let packet = CDemoFullPacket::decode(msg)?;

        if !self.processing_deltas {
            self.on_packet(
                EDemoCommands::DemStringTables,
                &packet.string_table.unwrap().encode_to_vec(),
            )?;
        }

        self.on_packet(
            EDemoCommands::DemPacket,
            &packet.packet.unwrap().encode_to_vec(),
        )?;

        self.context.last_full_packet_tick = self.context.tick;

        Ok(())
    }

    #[inline(always)]
    fn packet_entities(&mut self, msg: &[u8]) -> Result<()> {
        let packet = CsvcMsgPacketEntities::decode(msg)?;
        let mut entities_reader = Reader::new(packet.entity_data());

        let updates = packet.updated_entries();

        let mut index = u32::MAX;
        let mut op: isize;

        if packet.max_entries() as usize > self.context.entities.entities_vec.len() {
            self.context
                .entities
                .entities_vec
                .resize_with(packet.max_entries() as usize, || None);
        }

        if self.processing_deltas
            && !packet.legacy_is_delta()
            && self.context.last_full_packet_tick != u32::MAX
        {
            return Ok(());
        }

        let throw_event = |ctx: &Context, index: u32, event: EntityEvents| -> Result<()> {
            try_observers!(
                self,
                on_entity(
                    ctx,
                    event,
                    ctx.entities.entities_vec[index as usize].as_ref().unwrap()
                )
            )
        };

        for _ in 0..updates {
            index = index.wrapping_add(entities_reader.read_ubit_var() + 1);

            let cmd = entities_reader.read_bits(2);

            if cmd & 0x01 == 0 {
                if cmd & 0x02 != 0 {
                    let class_id = entities_reader
                        .read_bits(self.context.classes.class_id_size.unwrap())
                        as i32;
                    let serial = entities_reader.read_bits(17);

                    entities_reader.read_var_u32();

                    let class = self
                        .context
                        .classes
                        .get_by_id_rc(class_id as usize)?
                        .clone();

                    if !self.context.baselines.states.contains_key(&class_id) {
                        self.context.baselines.read_baseline(&class)
                    }

                    let entity_baseline = self.context.baselines.states[&class_id].clone();

                    self.context.entities.entities_vec[index as usize] =
                        Some(Entity::new(index, serial, class.clone(), entity_baseline));

                    let e = self.context.entities.entities_vec[index as usize]
                        .as_mut()
                        .unwrap();

                    self.field_reader.read_fields(
                        &mut entities_reader,
                        &e.class.serializer,
                        &mut e.state,
                    );

                    op = EntityEvents::Created as isize | EntityEvents::Entered as isize;
                } else {
                    op = EntityEvents::Updated as isize;
                    let e = self.context.entities.entities_vec[index as usize]
                        .as_mut()
                        .unwrap();

                    self.field_reader.read_fields(
                        &mut entities_reader,
                        &e.class.serializer,
                        &mut e.state,
                    );
                }
            } else {
                op = EntityEvents::Left as isize;
                if cmd & 0x02 != 0 {
                    op |= EntityEvents::Deleted as isize;
                }
            }

            if op & EntityEvents::Created as isize != 0 {
                throw_event(&self.context, index, EntityEvents::Created)?;
            }
            if op & EntityEvents::Entered as isize != 0 {
                throw_event(&self.context, index, EntityEvents::Entered)?;
            }
            if op & EntityEvents::Updated as isize != 0 {
                throw_event(&self.context, index, EntityEvents::Updated)?;
            }
            if op & EntityEvents::Left as isize != 0 {
                throw_event(&self.context, index, EntityEvents::Left)?;
            }
            if op & EntityEvents::Deleted as isize != 0 {
                throw_event(&self.context, index, EntityEvents::Deleted)?;
                self.context.entities.entities_vec[index as usize] = None;
            }
        }

        Ok(())
    }

    fn update_string_table(&mut self, msg: &[u8]) -> Result<()> {
        let table_msg = CsvcMsgUpdateStringTable::decode(msg)?;
        let mut table =
            self.context.string_tables.tables[table_msg.table_id() as usize].borrow_mut();

        table.parse(
            &mut self.context.baselines,
            table_msg.string_data(),
            table_msg.num_changed_entries(),
        )
    }

    fn create_string_table(&mut self, msg: &[u8]) -> Result<()> {
        let table_msg = CsvcMsgCreateStringTable::decode(msg)?;

        let mut table = StringTable {
            index: self.context.string_tables.tables.len() as i32,
            name: table_msg.name().into(),
            items: vec![],
            user_data_fixed_size: table_msg.user_data_fixed_size(),
            user_data_size: table_msg.user_data_size(),
            flags: table_msg.flags() as u32,
            var_int_bit_counts: table_msg.using_varint_bitcounts(),
            keys: RefCell::new(vec![String::default(); 32]),
        };

        let buf = if table_msg.data_compressed() {
            let mut decoder = snap::raw::Decoder::new();
            decoder.decompress_vec(table_msg.string_data())?
        } else {
            table_msg.string_data().into()
        };

        if table.name != "decalprecache" {
            table.parse(
                &mut self.context.baselines,
                buf.as_slice(),
                table_msg.num_entries(),
            )?;
        }

        let rc = Rc::new(RefCell::new(table));
        self.context.string_tables.tables.push(rc.clone());
        self.context
            .string_tables
            .name_to_table
            .insert(rc.borrow().name.clone().into(), rc.clone());

        Ok(())
    }

    fn dem_string_tables(&mut self, msg: &[u8]) -> Result<()> {
        let cmd = CDemoStringTables::decode(msg)?;
        for table in cmd.tables.iter() {
            let mut x = self
                .context
                .string_tables
                .name_to_table
                .get_mut(table.table_name())
                .unwrap()
                .borrow_mut();
            if table.items.len() < x.items.len() {
                return Ok(());
            }
            x.items
                .resize_with(table.items.len(), StringTableEntry::default);
            for (i, item) in table.items.iter().enumerate() {
                x.items[i].index = i as i32;
                x.items[i].key = item.str().to_string();
                x.items[i].value = Rc::new(item.data().to_vec()).into();
                if table.table_name() == "instancebaseline" {
                    self.context.baselines.add_baseline(
                        item.str().parse()?,
                        x.items[i].value.as_ref().unwrap().clone(),
                    );
                }
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn server_info(&mut self, msg: &[u8]) -> Result<()> {
        let info = CsvcMsgServerInfo::decode(msg)?;
        self.context.classes.class_id_size =
            Some((f64::log2(info.max_classes() as f64) + 1.0) as u32);

        let game_build_regexp = Regex::new(r"/dota_v(\d+)/")?;

        if let Some(captures) = game_build_regexp.captures(info.game_dir()) {
            if let Some(build_match) = captures.get(1) {
                let build_str = build_match.as_str();
                let build = build_str.parse::<u32>()?;
                self.context.game_build = build;
            } else {
                bail!("No build number found in regex capture");
            }
        } else {
            bail!("Failed to parse build number: '{}'", info.game_dir());
        }
        Ok(())
    }
}

#[allow(unused_variables)]
pub trait Observer {
    fn on_packet(&mut self, ctx: &Context, msg_type: EDemoCommands, msg: &[u8]) -> Result<()> {
        Ok(())
    }

    fn on_net_message(&mut self, ctx: &Context, msg_type: NetMessages, msg: &[u8]) -> Result<()> {
        Ok(())
    }

    fn on_svc_message(&mut self, ctx: &Context, msg_type: SvcMessages, msg: &[u8]) -> Result<()> {
        Ok(())
    }

    fn on_base_user_message(
        &mut self,
        ctx: &Context,
        msg_type: EBaseUserMessages,
        msg: &[u8],
    ) -> Result<()> {
        Ok(())
    }

    fn on_base_game_event(
        &mut self,
        ctx: &Context,
        msg_type: EBaseGameEvents,
        msg: &[u8],
    ) -> Result<()> {
        Ok(())
    }

    fn on_dota_user_message(
        &mut self,
        ctx: &Context,
        msg_type: EDotaUserMessages,
        msg: &[u8],
    ) -> Result<()> {
        Ok(())
    }

    fn on_tick_start(&mut self, ctx: &Context) -> Result<()> {
        Ok(())
    }

    fn on_tick_end(&mut self, ctx: &Context) -> Result<()> {
        Ok(())
    }

    fn on_entity(&mut self, ctx: &Context, event: EntityEvents, entity: &Entity) -> Result<()> {
        Ok(())
    }

    fn on_combat_log(&mut self, ctx: &Context, combat_log: &CombatLog) -> Result<()> {
        Ok(())
    }

    fn epilogue(&mut self, ctx: &Context) -> Result<()> {
        Ok(())
    }
}
