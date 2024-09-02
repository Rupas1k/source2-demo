use crate::error::ParserError;
use crate::parser::demo::DemoMessages;
use crate::parser::Parser;
use crate::proto::*;
use crate::reader::*;
use std::cmp::min;
use std::mem;

pub trait DemoRunner {
    /// Moves to the end of replay. Last packet is [`CDemoFileInfo`].
    fn run_to_end(&mut self) -> Result<(), ParserError>;

    /// Moves to target tick.
    fn run_to_tick(&mut self, target_tick: u32) -> Result<(), ParserError>;

    /// Moves to target tick without calling observers and processing delta
    /// packets.
    fn jump_to_tick(&mut self, target_tick: u32) -> Result<(), ParserError>;
}

impl DemoRunner for Parser<'_> {
    fn run_to_end(&mut self) -> Result<(), ParserError> {
        self.prologue()?;

        while let Some(message) = self.reader.read_next_message()? {
            self.on_tick_start(message.tick)?;
            self.on_demo_command(message.msg_type, message.buf.as_slice())?;
        }
        self.on_tick_end()?;

        Ok(())
    }

    fn run_to_tick(&mut self, target_tick: u32) -> Result<(), ParserError> {
        assert!(target_tick > self.context.tick || self.context.tick == u32::MAX);

        self.prologue()?;

        let target_tick = min(target_tick, self.last_tick);

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

    fn jump_to_tick(&mut self, target_tick: u32) -> Result<(), ParserError> {
        let fp_delta = if cfg!(feature = "deadlock") {
            3600
        } else {
            1800
        };

        let target_tick = min(target_tick, self.last_tick);

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
}
