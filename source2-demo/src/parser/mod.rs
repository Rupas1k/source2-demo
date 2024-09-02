mod context;
mod demo;
mod observer;

pub use context::*;
pub use demo::runner::*;
pub use observer::*;

use crate::error::*;
use crate::proto::*;
use crate::reader::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::parser::demo::DemoCommands;
use crate::try_observers;
#[cfg(feature = "dota")]
use std::collections::VecDeque;

pub struct Parser<'a> {
    pub(crate) reader: Reader<'a>,
    pub(crate) field_reader: FieldReader,
    pub(crate) observers: Vec<Rc<RefCell<dyn Observer + 'a>>>,

    #[cfg(feature = "dota")]
    pub(crate) combat_log: VecDeque<CMsgDotaCombatLogEntry>,

    pub(crate) prologue_completed: bool,
    pub(crate) skip_deltas: bool,

    pub(crate) replay_info: CDemoFileInfo,
    pub(crate) last_tick: u32,
    pub(crate) context: Context,
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

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn replay_info(&self) -> &CDemoFileInfo {
        &self.replay_info
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

    pub(crate) fn prologue(&mut self) -> Result<(), ParserError> {
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

    pub(crate) fn on_demo_command(
        &mut self,
        msg_type: EDemoCommands,
        msg: &[u8],
    ) -> Result<(), ParserError> {
        match msg_type {
            EDemoCommands::DemSendTables => {
                self.dem_send_tables(CDemoSendTables::decode(msg)?)?;
            }
            EDemoCommands::DemClassInfo => {
                self.dem_class_info(CDemoClassInfo::decode(msg)?)?;
            }
            EDemoCommands::DemPacket | EDemoCommands::DemSignonPacket => {
                self.dem_packet(CDemoPacket::decode(msg)?)?;
            }
            EDemoCommands::DemFullPacket => self.dem_full_packet(CDemoFullPacket::decode(msg)?)?,
            EDemoCommands::DemStringTables => {
                self.dem_string_tables(CDemoStringTables::decode(msg)?)?
            }
            EDemoCommands::DemStop => {
                self.dem_stop()?;
            }
            _ => {}
        };

        try_observers!(self, on_demo_command(&self.context, msg_type, msg))?;
        Ok(())
    }

    #[cfg(feature = "deadlock")]
    pub fn deadlock_match_details(&mut self) -> Result<CMsgMatchMetaDataContents, ParserError> {
        self.reader.read_deadlock_match_details()
    }
}
