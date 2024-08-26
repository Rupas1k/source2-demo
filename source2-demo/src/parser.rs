use crate::baseline::Baselines;
use crate::class::{Class, ClassError, Classes};
use crate::decoder::Decoder;
use crate::entity::{Entities, Entity, EntityError, EntityEvents};
use crate::field::{Encoder, Field, FieldModel, FieldProperties, FieldState, FieldType};
use crate::field_reader::FieldReader;
use crate::field_value::FieldValueError;
use crate::game_event::{GameEvent, GameEventError, GameEventList};
use crate::proto::*;
use crate::reader::Reader;
use crate::serializer::Serializer;
use crate::string_table::{StringTable, StringTableError, StringTableRow, StringTables};
use crate::ObserverResult;
use hashbrown::HashMap;
use prettytable::{row, Table};
use std::cell::RefCell;
use std::cmp::min;

use std::fmt::{Display, Formatter};
use std::mem;
use std::rc::Rc;

#[cfg(feature = "dota")]
use crate::combat_log::{CombatLogEntry, CombatLogError};
#[cfg(feature = "dota")]
use std::collections::VecDeque;

macro_rules! try_observers {
    ($self:ident, $method:ident ( $($arg:expr),* )) => {
        $self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().$method($($arg),*))
    };
}

/// Main error type
#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    #[error(transparent)]
    ProtobufDecode(#[from] prost::DecodeError),

    #[error(transparent)]
    SnapDecompress(#[from] snap::Error),

    #[error(transparent)]
    StringTable(#[from] StringTableError),

    #[error(transparent)]
    Class(#[from] ClassError),

    #[error(transparent)]
    Entity(#[from] EntityError),

    #[error(transparent)]
    FieldValue(#[from] FieldValueError),

    #[error(transparent)]
    GameEvent(#[from] GameEventError),

    #[error(transparent)]
    ObserverError(#[from] anyhow::Error),

    #[error("Wrong CDemoFileInfo offset")]
    ReplayEncodingError,

    #[error("Supports only Source 2 replays")]
    WrongMagic,

    #[cfg(feature = "dota")]
    #[error(transparent)]
    CombatLog(#[from] CombatLogError),

    #[cfg(feature = "deadlock")]
    #[error("CCitadelUserMsgPostMatchDetails not found")]
    MatchDetailsNotFound,
}

pub struct Parser<'a> {
    reader: Reader<'a>,
    field_reader: FieldReader,
    observers: Vec<Rc<RefCell<dyn Observer + 'a>>>,

    #[cfg(feature = "dota")]
    combat_log: VecDeque<CMsgDotaCombatLogEntry>,

    prologue_completed: bool,
    skip_deltas: bool,

    replay_info: CDemoFileInfo,
    last_tick: u32,
    context: Context,
}

/// Current replay state.
pub struct Context {
    pub(crate) classes: Classes,
    pub(crate) entities: Entities,
    pub(crate) string_tables: StringTables,
    pub(crate) game_events: GameEventList,

    pub(crate) tick: u32,
    pub(crate) previous_tick: u32,

    pub(crate) net_tick: u32,
    pub(crate) game_build: u32,

    baselines: Baselines,
    serializers: HashMap<Box<str>, Rc<Serializer>>,
    last_full_packet_tick: u32,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            classes: Classes::default(),
            entities: Entities::default(),
            string_tables: StringTables::default(),
            game_events: Default::default(),
            tick: u32::MAX,
            previous_tick: u32::MAX,
            net_tick: u32::MAX,
            game_build: 0,
            baselines: Baselines::default(),
            serializers: HashMap::default(),
            last_full_packet_tick: u32::MAX,
        }
    }
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

    pub fn game_events(&self) -> &GameEventList {
        &self.game_events
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
        table.add_row(row![
            "Entities",
            self.entities
                .entities_vec
                .iter()
                .flatten()
                .collect::<Vec<_>>()
                .len()
        ]);
        table.add_row(row!["String Tables", self.string_tables.tables.len()]);
        table.add_row(row!["Tick", self.tick]);
        table.add_row(row!["Net Tick", self.net_tick]);
        table.add_row(row!["Game Build", format!("{:?}", self.game_build)]);
        write!(f, "{}", table)
    }
}

pub(crate) struct OuterMessage {
    pub(crate) msg_type: EDemoCommands,
    pub(crate) tick: u32,
    pub(crate) buf: Vec<u8>,
}

impl<'a> Parser<'a> {
    /// Creates new instance of parser and performs validation of replay file.
    pub fn new(replay: &'a [u8]) -> Result<Self, ParserError> {
        let mut reader = Reader::new(replay);

        if replay.len() < 16 || reader.read_bytes(8) != b"PBDEMS2\0" {
            return Err(ParserError::WrongMagic);
        };

        reader.read_bytes(8);

        let replay_info = reader.read_replay_info()?;
        let last_tick = replay_info.playback_ticks() as u32;

        Ok(Parser {
            reader,
            field_reader: FieldReader::default(),
            observers: Vec::default(),

            #[cfg(feature = "dota")]
            combat_log: VecDeque::default(),

            prologue_completed: false,
            skip_deltas: false,

            replay_info,
            last_tick,

            context: Context::default(),
        })
    }

    /// Registers new observer and returns `Rc<RefCell<T>>` of it.
    /// Observer struct must implement Observer and Default traits.
    pub fn register_observer<T>(&mut self) -> Rc<RefCell<T>>
    where
        T: Observer + Default + 'a,
    {
        let rc = Rc::new(RefCell::new(T::default()));
        self.observers.push(rc.clone());
        rc.clone()
    }

    fn prologue(&mut self) -> Result<(), ParserError> {
        if self.prologue_completed && self.context.tick != u32::MAX {
            return Ok(());
        }

        while let Some(message) = self.reader.read_next_message()? {
            if self.prologue_completed
                && (message.msg_type == EDemoCommands::DemSendTables
                    || message.msg_type == EDemoCommands::DemClassInfo)
            {
                continue;
            }

            self.on_demo_command(message.msg_type, message.buf.as_slice())?;

            if message.msg_type == EDemoCommands::DemSyncTick {
                self.prologue_completed = true;
                break;
            }
        }

        Ok(())
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn replay_info(&self) -> &CDemoFileInfo {
        &self.replay_info
    }

    /// Moves to the end of replay. Last packet is [`CDemoFileInfo`].
    pub fn run_to_end(&mut self) -> Result<(), ParserError> {
        self.prologue()?;

        while let Some(message) = self.reader.read_next_message()? {
            self.on_tick_start(message.tick)?;
            self.on_demo_command(message.msg_type, message.buf.as_slice())?;
        }
        self.on_tick_end()?;

        try_observers!(self, epilogue(&self.context))?;
        Ok(())
    }

    /// Moves to target tick without calling observers and processing delta
    /// packets.
    pub fn jump_to_tick(&mut self, mut target_tick: u32) -> Result<(), ParserError> {
        let fp_delta = if cfg!(feature = "deadlock") {
            3600
        } else {
            1800
        };

        target_tick = min(target_tick, self.last_tick);

        if target_tick < self.context.tick {
            self.context.last_full_packet_tick = u32::MAX;
            self.context.tick = u32::MAX;
            self.context.net_tick = u32::MAX;
            self.reader.reset_to(16);

            self.context.entities.entities_vec = vec![None; 8192];

            self.context.string_tables.tables.clear();
            self.context.string_tables.name_to_table.clear();
            self.context.game_events.list.clear();
        }

        self.prologue()?;

        self.skip_deltas = true;
        let observers = mem::take(&mut self.observers);

        let mut first_fp_checked = self.context.last_full_packet_tick != u32::MAX;
        let mut last_fp_checked = false;

        while let Some(mut message) = self.reader.read_next_message()? {
            self.context.previous_tick = self.context.tick;
            self.context.tick = message.tick;

            if message.msg_type == EDemoCommands::DemFullPacket {
                self.context.last_full_packet_tick = self.context.tick;
            }

            let next_fp = self.context.last_full_packet_tick == u32::MAX
                || self.context.last_full_packet_tick < target_tick
                    && (target_tick - self.context.last_full_packet_tick) > fp_delta;

            if message.msg_type == EDemoCommands::DemFullPacket {
                if next_fp && first_fp_checked {
                    message.msg_type = EDemoCommands::DemStringTables;
                    message.buf = CDemoFullPacket::decode(message.buf.as_slice())?
                        .string_table
                        .unwrap()
                        .encode_to_vec();
                }

                self.on_demo_command(message.msg_type, message.buf.as_slice())?;
            }

            if last_fp_checked {
                self.on_demo_command(message.msg_type, message.buf.as_slice())?;
            }

            if message.msg_type == EDemoCommands::DemFullPacket && !first_fp_checked {
                first_fp_checked = true;
            }

            if message.msg_type == EDemoCommands::DemFullPacket && !next_fp {
                last_fp_checked = true;
                self.skip_deltas = false;
            }

            if self.context.tick >= target_tick && first_fp_checked {
                break;
            }
        }

        self.skip_deltas = false;
        self.observers = observers;

        Ok(())
    }

    /// Moves to target tick.
    pub fn run_to_tick(&mut self, mut target_tick: u32) -> Result<(), ParserError> {
        assert!(target_tick > self.context.tick || self.context.tick == u32::MAX);

        self.prologue()?;

        target_tick = min(target_tick, self.last_tick);

        while let Some(message) = self.reader.read_next_message()? {
            self.on_tick_start(message.tick)?;
            self.on_demo_command(message.msg_type, message.buf.as_slice())?;
            if self.context.tick >= target_tick {
                self.on_tick_end()?;
                break;
            }
        }

        Ok(())
    }

    fn on_demo_command(&mut self, msg_type: EDemoCommands, msg: &[u8]) -> Result<(), ParserError> {
        match msg_type {
            EDemoCommands::DemSendTables => self.dem_send_tables(msg)?,
            EDemoCommands::DemClassInfo => self.dem_class_info(msg)?,
            EDemoCommands::DemPacket | EDemoCommands::DemSignonPacket => self.dem_packet(msg)?,
            EDemoCommands::DemFullPacket => self.dem_full_packet(msg)?,
            EDemoCommands::DemStringTables => self.dem_string_tables(msg)?,
            _ => {}
        };

        try_observers!(self, on_demo_command(&self.context, msg_type, msg))?;
        Ok(())
    }

    fn on_net_message(&mut self, msg_type: NetMessages, msg: &[u8]) -> Result<(), ParserError> {
        if msg_type == NetMessages::NetTick {
            self.context.net_tick = CNetMsgTick::decode(msg)?.tick();
        }

        try_observers!(self, on_net_message(&self.context, msg_type, msg))?;
        Ok(())
    }

    fn on_svc_message(&mut self, msg_type: SvcMessages, msg: &[u8]) -> Result<(), ParserError> {
        match msg_type {
            SvcMessages::SvcServerInfo => self.server_info(msg)?,
            SvcMessages::SvcCreateStringTable => self.create_string_table(msg)?,
            SvcMessages::SvcUpdateStringTable => self.update_string_table(msg)?,
            SvcMessages::SvcPacketEntities => self.packet_entities(msg)?,
            _ => {}
        }

        try_observers!(self, on_svc_message(&self.context, msg_type, msg))?;
        Ok(())
    }

    fn on_base_user_message(
        &mut self,
        msg_type: EBaseUserMessages,
        msg: &[u8],
    ) -> Result<(), ParserError> {
        try_observers!(self, on_base_user_message(&self.context, msg_type, msg))?;
        Ok(())
    }

    fn on_base_game_event(
        &mut self,
        msg_type: EBaseGameEvents,
        msg: &[u8],
    ) -> Result<(), ParserError> {
        if msg_type == EBaseGameEvents::GeSource1LegacyGameEventList {
            self.context.game_events = GameEventList::new(CSvcMsgGameEventList::decode(msg)?);
        }

        if msg_type == EBaseGameEvents::GeSource1LegacyGameEvent {
            let ge = GameEvent::new(&self.context.game_events, CSvcMsgGameEvent::decode(msg)?);
            try_observers!(self, on_game_event(&self.context, &ge))?;
        }

        try_observers!(self, on_base_game_event(&self.context, msg_type, msg))?;
        Ok(())
    }

    #[cfg(feature = "dota")]
    fn on_dota_user_message(
        &mut self,
        msg_type: EDotaUserMessages,
        msg: &[u8],
    ) -> Result<(), ParserError> {
        if msg_type == EDotaUserMessages::DotaUmCombatLogDataHltv {
            let entry = CMsgDotaCombatLogEntry::decode(msg)?;
            self.combat_log.push_back(entry);
        }

        try_observers!(self, on_dota_user_message(&self.context, msg_type, msg))?;
        Ok(())
    }

    #[cfg(feature = "deadlock")]
    fn on_citadel_game_event(
        &mut self,
        msg_type: ECitadelGameEvents,
        msg: &[u8],
    ) -> Result<(), ParserError> {
        try_observers!(self, on_citadel_game_event(&self.context, msg_type, msg))?;
        Ok(())
    }

    #[cfg(feature = "deadlock")]
    fn on_citadel_user_message(
        &mut self,
        msg_type: CitadelUserMessageIds,
        msg: &[u8],
    ) -> Result<(), ParserError> {
        try_observers!(self, on_citadel_user_message(&self.context, msg_type, msg))?;
        Ok(())
    }

    pub(crate) fn on_tick_start(&mut self, msg_tick: u32) -> Result<(), ParserError> {
        if msg_tick > self.context.tick {
            self.on_tick_end()?;
        }

        self.context.previous_tick = self.context.tick;
        self.context.tick = msg_tick;

        if self.context.previous_tick == msg_tick {
            return Ok(());
        }

        try_observers!(self, on_tick_start(&self.context))?;
        Ok(())
    }

    pub(crate) fn on_tick_end(&mut self) -> Result<(), ParserError> {
        #[cfg(feature = "dota")]
        if let Ok(names) = self.context.string_tables.get_by_name("CombatLogNames") {
            while let Some(log) = self.combat_log.pop_front() {
                let entry = CombatLogEntry { names, log };
                try_observers!(self, on_combat_log(&self.context, &entry))?;
            }
        }

        try_observers!(self, on_tick_end(&self.context))?;
        Ok(())
    }

    fn dem_send_tables(&mut self, msg: &[u8]) -> Result<(), ParserError> {
        let send_tables = CDemoSendTables::decode(msg)?;
        let mut reader = Reader::new(send_tables.data());
        let amount = reader.read_var_u32();
        let buf = reader.read_bytes(amount);

        let fs = CSvcMsgFlattenedSerializer::decode(buf.as_slice())?;

        let resolve = |p: Option<i32>| -> Box<str> {
            if let Some(i) = p {
                return fs.symbols[i as usize].clone().into();
            }
            "".into()
        };

        let mut fields: Vec<Rc<Field>> = vec![];
        let mut field_types: HashMap<Box<str>, Rc<FieldType>> = HashMap::default();

        for s in fs.serializers.iter() {
            let serializer_name = fs.symbols[s.serializer_name_sym() as usize].clone();
            let mut serializer = Serializer::default();

            for i in s.fields_index.iter().map(|x| *x as usize) {
                let current_field = &fs.fields[i];
                let field_serializer_name = resolve(current_field.field_serializer_name_sym);

                if i >= fields.len() {
                    let var_type_str = resolve(current_field.var_type_sym);
                    let var_name = resolve(current_field.var_name_sym);

                    let current_field_serializer = self
                        .context
                        .serializers
                        .get(&field_serializer_name)
                        .cloned();

                    let field_type = field_types
                        .entry(var_type_str.clone())
                        .or_insert_with(|| FieldType::new(var_type_str.clone().as_ref()).into())
                        .clone();

                    let properties = FieldProperties {
                        encoder: match var_name.as_ref() {
                            "m_flSimulationTime" | "m_flAnimTime" => Some(Encoder::SimTime),
                            "m_flRuneTime" => Some(Encoder::RuneTime),
                            _ => Encoder::from_str(&resolve(current_field.var_encoder_sym)),
                        },
                        encoder_flags: current_field.encode_flags(),
                        bit_count: current_field.bit_count(),
                        low_value: current_field.low_value(),
                        high_value: current_field.high_value(),
                    };

                    let model = if let Some(serializer) = current_field_serializer {
                        if field_type.pointer {
                            FieldModel::FixedTable(serializer)
                        } else {
                            FieldModel::VariableTable(serializer)
                        }
                    } else if field_type.count.is_some_and(|x| x > 0)
                        && field_type.base.as_ref() != "char"
                    {
                        FieldModel::FixedArray
                    } else if field_type.base.as_ref() == "CUtlVector"
                        || field_type.base.as_ref() == "CNetworkUtlVectorBase"
                        || field_type.base.as_ref() == "CUtlVectorEmbeddedNetworkVar"
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
                    fields.push(field.into());
                }
                serializer.fields.push(fields[i].clone());
            }
            self.context
                .serializers
                .insert(serializer_name.into(), serializer.into());
        }
        Ok(())
    }

    fn dem_class_info(&mut self, msg: &[u8]) -> Result<(), ParserError> {
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

    #[cfg(feature = "deadlock")]
    pub fn deadlock_match_details(&mut self) -> Result<CMsgMatchMetaDataContents, ParserError> {
        let mut temp_reader = Reader::new(self.reader.buf);
        temp_reader.reset_to(16);
        while let Some(message) = temp_reader.read_next_message()? {
            if EDemoCommands::try_from(message.msg_type) != Ok(EDemoCommands::DemPacket) {
                continue;
            }

            let packet = CDemoPacket::decode(message.buf.as_slice())?;
            let mut packet_reader = Reader::new(packet.data());
            while packet_reader.bytes_remaining() != 0 {
                let msg_type = packet_reader.read_ubit_var() as i32;
                let size = packet_reader.read_var_u32();
                let packet_buf = packet_reader.read_bytes(size);

                if CitadelUserMessageIds::try_from(msg_type)
                    == Ok(CitadelUserMessageIds::KEUserMsgPostMatchDetails)
                {
                    return Ok(CMsgMatchMetaDataContents::decode(
                        CCitadelUserMsgPostMatchDetails::decode(packet_buf.as_slice())?
                            .match_details(),
                    )?);
                }
            }
        }

        Err(ParserError::MatchDetailsNotFound)
    }

    #[allow(unreachable_code)]
    fn dem_packet(&mut self, msg: &[u8]) -> Result<(), ParserError> {
        let packet = CDemoPacket::decode(msg)?;
        let mut packet_reader = Reader::new(packet.data());
        while packet_reader.bytes_remaining() != 0 {
            let msg_type = packet_reader.read_ubit_var() as i32;
            let size = packet_reader.read_var_u32();
            let packet_buf = packet_reader.read_bytes(size);

            #[cfg(any(
                all(feature = "dota", feature = "deadlock"),
                all(not(feature = "dota"), not(feature = "deadlock"))
            ))]
            {
                if let Ok(msg) = SvcMessages::try_from(msg_type) {
                    self.on_svc_message(msg, &packet_buf)?;
                } else if let Ok(msg) = EBaseUserMessages::try_from(msg_type) {
                    self.on_base_user_message(msg, &packet_buf)?;
                } else if let Ok(msg) = EBaseGameEvents::try_from(msg_type) {
                    self.on_base_game_event(msg, &packet_buf)?;
                } else if let Ok(msg) = NetMessages::try_from(msg_type) {
                    self.on_net_message(msg, &packet_buf)?;
                }
                continue;
            }

            #[cfg(feature = "dota")]
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

            #[cfg(feature = "deadlock")]
            if let Ok(msg) = CitadelUserMessageIds::try_from(msg_type) {
                self.on_citadel_user_message(msg, &packet_buf)?;
            } else if let Ok(msg) = ECitadelGameEvents::try_from(msg_type) {
                self.on_citadel_game_event(msg, &packet_buf)?;
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

    fn dem_full_packet(&mut self, msg: &[u8]) -> Result<(), ParserError> {
        let packet = CDemoFullPacket::decode(msg)?;

        if self.context.last_full_packet_tick == u32::MAX || self.skip_deltas {
            self.on_demo_command(
                EDemoCommands::DemStringTables,
                &packet.string_table.unwrap().encode_to_vec(),
            )?;

            self.on_demo_command(
                EDemoCommands::DemPacket,
                &packet.packet.unwrap().encode_to_vec(),
            )?;
        }

        self.context.last_full_packet_tick = self.context.tick;

        Ok(())
    }

    fn dem_string_tables(&mut self, msg: &[u8]) -> Result<(), ParserError> {
        let cmd = CDemoStringTables::decode(msg)?;
        for table in cmd.tables.iter() {
            let x = self
                .context
                .string_tables
                .get_by_name_mut(table.table_name())?;
            if table.items.len() < x.items.len() {
                return Ok(());
            }
            x.items
                .resize_with(table.items.len(), StringTableRow::default);
            for (i, item) in table.items.iter().enumerate() {
                x.items[i].index = i as i32;
                x.items[i].key = item.str().to_string();
                x.items[i].value = Rc::new(item.data().to_vec()).into();
                if table.table_name() == "instancebaseline" {
                    self.context.baselines.add_baseline(
                        item.str().parse().unwrap(),
                        x.items[i].value.as_ref().unwrap().clone(),
                    );
                }
            }
        }

        Ok(())
    }

    fn packet_entities(&mut self, msg: &[u8]) -> Result<(), ParserError> {
        let packet = CSvcMsgPacketEntities::decode(msg)?;
        let mut reader = Reader::new(packet.entity_data());

        let mut index = usize::MAX;

        for _ in 0..packet.updated_entries() {
            index = index.wrapping_add((reader.read_ubit_var() + 1) as usize);

            let cmd = reader.read_bits(2);
            if cmd == 1 {
                continue;
            }

            match EntityEvents::from_cmd(cmd) {
                EntityEvents::Created => {
                    let class_id = reader.read_bits(self.context.classes.class_id_size) as i32;
                    let serial = reader.read_bits(17);
                    let _ = reader.read_var_u32();

                    let class = self.context.classes.get_by_id_rc(class_id as usize).clone();

                    let entity_baseline = self
                        .context
                        .baselines
                        .states
                        .entry(class_id)
                        .or_insert_with(|| {
                            let mut state = FieldState::default();
                            self.field_reader.read_fields(
                                &mut Reader::new(&self.context.baselines.baselines[&class_id]),
                                &class.serializer,
                                &mut state,
                            );
                            state
                        })
                        .clone();

                    self.context.entities.entities_vec[index] = Some(Entity::new(
                        index as u32,
                        serial,
                        class.clone(),
                        entity_baseline,
                    ));

                    let entity = self.context.entities.entities_vec[index].as_mut().unwrap();

                    self.field_reader.read_fields(
                        &mut reader,
                        &entity.class.serializer,
                        &mut entity.state,
                    );

                    try_observers!(
                        self,
                        on_entity(
                            &self.context,
                            EntityEvents::Created,
                            self.context.entities.entities_vec[index].as_ref().unwrap()
                        )
                    )?;
                }
                EntityEvents::Updated => {
                    let entity = self.context.entities.entities_vec[index].as_mut().unwrap();

                    self.field_reader.read_fields(
                        &mut reader,
                        &entity.class.serializer,
                        &mut entity.state,
                    );

                    try_observers!(
                        self,
                        on_entity(
                            &self.context,
                            EntityEvents::Updated,
                            self.context.entities.entities_vec[index].as_ref().unwrap()
                        )
                    )?;
                }
                EntityEvents::Deleted => {
                    if let Some(entity) = self.context.entities.entities_vec[index].as_ref() {
                        try_observers!(
                            self,
                            on_entity(&self.context, EntityEvents::Deleted, entity)
                        )?;
                    }
                    self.context.entities.entities_vec[index] = None;
                }
            }
        }

        Ok(())
    }

    fn update_string_table(&mut self, msg: &[u8]) -> Result<(), ParserError> {
        let table_msg = CSvcMsgUpdateStringTable::decode(msg)?;

        let modified = {
            let table = self
                .context
                .string_tables
                .tables
                .get_mut(table_msg.table_id() as usize)
                .unwrap();

            table.parse(
                &mut self.context.baselines,
                table_msg.string_data(),
                table_msg.num_changed_entries(),
            )?
        };

        try_observers!(
            self,
            on_string_table(
                &self.context,
                &self.context.string_tables.tables[table_msg.table_id() as usize],
                modified.as_slice()
            )
        )?;

        Ok(())
    }

    fn create_string_table(&mut self, msg: &[u8]) -> Result<(), ParserError> {
        let table_msg = CSvcMsgCreateStringTable::decode(msg)?;

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

        let table_index = table.index as usize;

        let buf = if table_msg.data_compressed() {
            let mut decoder = snap::raw::Decoder::new();
            decoder.decompress_vec(table_msg.string_data())?
        } else {
            table_msg.string_data().into()
        };

        let modified = table.parse(
            &mut self.context.baselines,
            buf.as_slice(),
            table_msg.num_entries(),
        )?;

        self.context
            .string_tables
            .name_to_table
            .insert(table.name().into(), table.index as usize);
        self.context.string_tables.tables.push(table);

        try_observers!(
            self,
            on_string_table(
                &self.context,
                &self.context.string_tables.tables[table_index],
                modified.as_slice()
            )
        )?;

        Ok(())
    }

    fn server_info(&mut self, msg: &[u8]) -> Result<(), ParserError> {
        let info = CSvcMsgServerInfo::decode(msg)?;
        self.context.classes.class_id_size = (f64::log2(info.max_classes() as f64) + 1.0) as u32;

        let game_dir = info.game_dir();

        let game_prefix = if cfg!(feature = "dota") {
            "dota_v"
        } else if cfg!(feature = "deadlock") {
            "citadel_v"
        } else {
            "unknown"
        };

        if let Some(start) = game_dir.find(game_prefix) {
            let start = start + game_prefix.len();
            if let Some(end) = game_dir[start..].find('/') {
                let build_str = &game_dir[start..start + end];
                let build = build_str.parse::<u32>().unwrap();
                self.context.game_build = build;
            }
        }
        Ok(())
    }
}

/// A trait defining methods for handling game events and protobuf messages. Can
/// be attached to [`Parser`] instance with [`Parser::register_observer`]
/// method.
#[allow(unused_variables)]
pub trait Observer {
    fn on_demo_command(
        &mut self,
        ctx: &Context,
        msg_type: EDemoCommands,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    fn on_net_message(
        &mut self,
        ctx: &Context,
        msg_type: NetMessages,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    fn on_svc_message(
        &mut self,
        ctx: &Context,
        msg_type: SvcMessages,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    fn on_base_user_message(
        &mut self,
        ctx: &Context,
        msg_type: EBaseUserMessages,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    fn on_base_game_event(
        &mut self,
        ctx: &Context,
        msg_type: EBaseGameEvents,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    fn on_tick_start(&mut self, ctx: &Context) -> ObserverResult {
        Ok(())
    }

    fn on_tick_end(&mut self, ctx: &Context) -> ObserverResult {
        Ok(())
    }

    fn on_entity(&mut self, ctx: &Context, event: EntityEvents, entity: &Entity) -> ObserverResult {
        Ok(())
    }

    fn on_game_event(&mut self, ctx: &Context, ge: &GameEvent) -> ObserverResult {
        Ok(())
    }

    fn on_string_table(
        &mut self,
        ctx: &Context,
        st: &StringTable,
        modified: &[i32],
    ) -> ObserverResult {
        Ok(())
    }

    #[cfg(feature = "dota")]
    fn on_combat_log(&mut self, ctx: &Context, cle: &CombatLogEntry) -> ObserverResult {
        Ok(())
    }

    #[cfg(feature = "dota")]
    fn on_dota_user_message(
        &mut self,
        ctx: &Context,
        msg_type: EDotaUserMessages,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    #[cfg(feature = "deadlock")]
    fn on_citadel_game_event(
        &mut self,
        ctx: &Context,
        msg_type: ECitadelGameEvents,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    #[cfg(feature = "deadlock")]
    fn on_citadel_user_message(
        &mut self,
        ctx: &Context,
        msg_type: CitadelUserMessageIds,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    fn epilogue(&mut self, ctx: &Context) -> ObserverResult {
        Ok(())
    }
}
