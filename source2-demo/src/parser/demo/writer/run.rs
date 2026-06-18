use super::packet::PacketDataRewrite;
use super::{DemoWriter, MessageRewrite, RewriteInterests};
use crate::error::ParserError;
use crate::parser::demo::DemoCommands;
use crate::proto::{
    CDemoClassInfo, CDemoFullPacket, CDemoPacket, CDemoSendTables, CDemoStringTables,
    EDemoCommands, Message,
};
use crate::reader::{BitsReader, MessageReader};
use std::io::{Seek, Write};

impl<'a, R, W> DemoWriter<'a, R, W>
where
    R: BitsReader + MessageReader,
    W: Write + Seek,
{
    /// Parses the demo while writing the rewritten stream.
    pub fn run(&mut self) -> Result<(), ParserError> {
        self.parser.reader.seek(0);
        let header = self.parser.reader.read_bytes(16);
        self.writer.write_all(&header)?;
        self.bytes_written = header.len() as u64;

        'messages: while let Some(message) = self.read_next_raw_message()? {
            self.update_context_tick(message.tick);

            let mut payload = None;
            if self.has_rewriters(RewriteInterests::DEMO_MESSAGE) {
                let mut decoded = Self::decode_raw_payload(&message)?;
                let ctx = &self.parser.context;
                for rewriter in self.rewriters.iter_mut().filter(|rewriter| {
                    rewriter
                        .interests()
                        .contains(RewriteInterests::DEMO_MESSAGE)
                }) {
                    match rewriter.rewrite_demo_message(
                        ctx,
                        message.tick,
                        message.msg_type,
                        decoded.as_slice(),
                    )? {
                        MessageRewrite::Drop => continue 'messages,
                        MessageRewrite::Replace(bytes) => decoded = bytes,
                        MessageRewrite::Keep | MessageRewrite::Rewrite => {}
                    }
                }
                payload = Some(decoded);
            }

            match message.msg_type {
                EDemoCommands::DemPacket | EDemoCommands::DemSignonPacket => {
                    if !self.needs_packet_scan() {
                        self.write_message_or_raw(&message, payload.as_deref())?;
                        continue;
                    }

                    let can_write_raw = payload.is_none();
                    let payload = Self::materialize_payload(&mut payload, &message)?;
                    let mut packet = CDemoPacket::decode(payload.as_slice())?;
                    let rewritten = self.rewrite_packet_data_if_changed(
                        message.tick,
                        packet.data(),
                        self.needs_packet_state(),
                    )?;
                    match rewritten {
                        PacketDataRewrite::Unchanged if can_write_raw => {
                            self.write_raw_demo_message(
                                message.msg_type,
                                message.tick,
                                message.raw_payload.as_slice(),
                                message.compressed,
                            )?;
                            continue;
                        }
                        PacketDataRewrite::Unchanged => {}
                        PacketDataRewrite::Changed(rewritten) => packet.data = Some(rewritten),
                    }
                    let packet_bytes = packet.encode_to_vec();
                    self.write_demo_message_with_compression(
                        message.msg_type,
                        message.tick,
                        &packet_bytes,
                        message.compressed,
                    )?;
                }
                EDemoCommands::DemFullPacket => {
                    if !self.needs_packet_scan() && !self.needs_demo_string_table_scan() {
                        self.write_message_or_raw(&message, payload.as_deref())?;
                        continue;
                    }

                    let can_write_raw = payload.is_none();
                    let payload = Self::materialize_payload(&mut payload, &message)?;
                    let mut full = CDemoFullPacket::decode(payload.as_slice())?;
                    let should_process = self.parser.context.last_full_packet_tick == u32::MAX
                        || self.parser.skip_deltas;
                    let mut changed = false;

                    if let Some(table) = full.string_table.take() {
                        if self.needs_demo_string_table_scan() {
                            if let Some((rewritten, table_changed)) =
                                self.rewrite_string_tables(message.tick, table)?
                            {
                                if should_process && self.needs_demo_string_table_state() {
                                    self.parser.dem_string_tables(rewritten.clone())?;
                                }
                                full.string_table = Some(rewritten);
                                changed |= table_changed;
                            } else {
                                changed = true;
                            }
                        } else {
                            full.string_table = Some(table);
                        }
                    }

                    if let Some(packet) = full.packet.as_mut() {
                        if self.needs_packet_scan() {
                            let rewritten = self.rewrite_packet_data_if_changed(
                                message.tick,
                                packet.data(),
                                should_process && self.needs_packet_state(),
                            )?;
                            if let PacketDataRewrite::Changed(rewritten) = rewritten {
                                packet.data = Some(rewritten);
                                changed = true;
                            }
                        }
                    }

                    self.parser.context.last_full_packet_tick = message.tick;
                    if can_write_raw && !changed {
                        self.write_raw_demo_message(
                            message.msg_type,
                            message.tick,
                            message.raw_payload.as_slice(),
                            message.compressed,
                        )?;
                        continue;
                    }
                    let full_bytes = full.encode_to_vec();
                    self.write_demo_message_with_compression(
                        message.msg_type,
                        message.tick,
                        &full_bytes,
                        message.compressed,
                    )?;
                }
                EDemoCommands::DemSendTables if self.needs_class_metadata() => {
                    let payload = Self::materialize_payload(&mut payload, &message)?;
                    let msg = CDemoSendTables::decode(payload.as_slice())?;
                    self.parser.dem_send_tables(msg)?;
                    self.write_demo_message_with_compression(
                        message.msg_type,
                        message.tick,
                        &payload,
                        message.compressed,
                    )?;
                }
                EDemoCommands::DemClassInfo if self.needs_class_metadata() => {
                    let payload = Self::materialize_payload(&mut payload, &message)?;
                    let msg = CDemoClassInfo::decode(payload.as_slice())?;
                    self.parser.dem_class_info(msg)?;
                    self.write_demo_message_with_compression(
                        message.msg_type,
                        message.tick,
                        &payload,
                        message.compressed,
                    )?;
                }
                EDemoCommands::DemStringTables => {
                    if !self.needs_demo_string_table_scan() {
                        self.write_message_or_raw(&message, payload.as_deref())?;
                        continue;
                    }

                    let payload = Self::materialize_payload(&mut payload, &message)?;
                    let mut msg = CDemoStringTables::decode(payload.as_slice())?;
                    let mut changed = false;
                    if self.rewrites_entity_fields() {
                        changed |= self.rewrite_instance_baselines(&mut msg)?;
                    }
                    changed |= self.rewrite_demo_string_table_entries(message.tick, &mut msg)?;
                    let mut out_payload: Option<Vec<u8>> = None;
                    let ctx = &self.parser.context;
                    for rewriter in self.rewriters.iter_mut().filter(|rewriter| {
                        rewriter
                            .interests()
                            .contains(RewriteInterests::DEMO_STRING_TABLES)
                    }) {
                        match rewriter.rewrite_demo_string_tables(ctx, message.tick, &mut msg)? {
                            MessageRewrite::Drop => continue 'messages,
                            MessageRewrite::Keep => {}
                            MessageRewrite::Rewrite => {
                                changed = false;
                                out_payload = Some(msg.encode_to_vec());
                            }
                            MessageRewrite::Replace(bytes) => {
                                msg = CDemoStringTables::decode(bytes.as_slice())?;
                                changed = false;
                                out_payload = Some(bytes);
                            }
                        }
                    }
                    let payload = out_payload.unwrap_or_else(|| {
                        if changed {
                            msg.encode_to_vec()
                        } else {
                            payload
                        }
                    });
                    if self.needs_demo_string_table_state() {
                        self.parser.dem_string_tables(msg)?;
                    }
                    self.write_demo_message_with_compression(
                        message.msg_type,
                        message.tick,
                        &payload,
                        message.compressed,
                    )?;
                }
                _ => {
                    self.write_message_or_raw(&message, payload.as_deref())?;
                }
            }
        }

        self.finalize_file_info_offset()?;
        Ok(())
    }

    fn update_context_tick(&mut self, tick: u32) {
        self.parser.context.previous_tick = self.parser.context.tick;
        self.parser.context.tick = tick;
    }
}
