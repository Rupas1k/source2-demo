use super::SliceReader;
use crate::error::ParserError;
use crate::proto::{CDemoFullPacket, CDemoPacket, EDemoCommands, Message};
use crate::stream::bits::BitsReader;
use std::{borrow::Cow, ops::Range};

/// Location of one outer command in the original replay.
#[derive(Debug, Clone)]
pub struct Command {
    /// Recorded tick, including -1 for sign-on.
    pub tick: i32,
    /// Numeric wire identifier.
    pub id: u32,
    /// Whether the stored payload uses Snappy compression.
    pub compressed: bool,
    /// Byte range of the stored payload, relative to the replay start.
    pub range: Range<usize>,
}

impl Command {
    /// Whether the command can contain packet messages.
    pub fn is_packet(&self) -> bool {
        matches!(
            EDemoCommands::try_from(self.id as i32),
            Ok(EDemoCommands::DemPacket
                | EDemoCommands::DemSignonPacket
                | EDemoCommands::DemFullPacket)
        )
    }

    /// Read this command from the replay used to build the index.
    pub fn payload<'a>(&self, file: &'a [u8]) -> Result<Cow<'a, [u8]>, ParserError> {
        let data = file
            .get(self.range.clone())
            .ok_or(ParserError::InvalidFrame("command range outside replay"))?;
        if self.compressed {
            snap::raw::Decoder::new()
                .decompress_vec(data)
                .map(Cow::Owned)
                .map_err(ParserError::from)
        } else {
            Ok(Cow::Borrowed(data))
        }
    }
}

pub(crate) struct CommandHeader {
    pub id: u32,
    pub compressed: bool,
    pub tick: u32,
    pub size: u32,
}

impl CommandHeader {
    pub fn read(reader: &mut impl BitsReader) -> Self {
        let flags = reader.read_var_u32();
        let compression = EDemoCommands::DemIsCompressed as u32;
        Self {
            id: flags & !compression,
            compressed: flags & compression != 0,
            tick: reader.read_var_u32(),
            size: reader.read_var_u32(),
        }
    }
}

/// Index command headers in file order without decompressing their payloads.
/// Command headers are assumed to be correctly encoded.
pub fn commands(file: &[u8]) -> Result<Vec<Command>, ParserError> {
    if file.len() < 16 || !file.starts_with(b"PBDEMS2\0") {
        return Err(ParserError::WrongMagic);
    }
    let mut reader = SliceReader::new(file);
    reader.seek(16);
    let mut result = Vec::new();
    while reader.remaining_bytes() != 0 {
        let header = CommandHeader::read(&mut reader);
        let pos = file.len() - reader.remaining_bytes();
        let end = pos
            .checked_add(header.size as usize)
            .filter(|end| *end <= file.len())
            .ok_or(ParserError::InvalidFrame("truncated demo command"))?;
        result.push(Command {
            tick: header.tick as i32,
            id: header.id,
            compressed: header.compressed,
            range: pos..end,
        });
        reader.seek(end);
    }
    Ok(result)
}

/// A packet message before game-specific ID dispatch.
#[derive(Debug)]
pub struct PacketMessage {
    /// Numeric wire ID, including IDs unknown to this library.
    pub id: u32,
    /// Original protobuf payload.
    pub payload: Vec<u8>,
}

/// Decode a packet-bearing command and return its contained packet bytes.
pub fn packet_data(id: u32, payload: &[u8]) -> Result<Option<Vec<u8>>, ParserError> {
    let packet = match EDemoCommands::try_from(id as i32) {
        Ok(EDemoCommands::DemPacket | EDemoCommands::DemSignonPacket) => {
            Some(CDemoPacket::decode(payload)?)
        }
        Ok(EDemoCommands::DemFullPacket) => CDemoFullPacket::decode(payload)?.packet,
        _ => None,
    };
    Ok(packet.map(|p| p.data.unwrap_or_default()))
}

/// Iterate over framed packet messages in wire order.
/// Message headers are assumed to be correctly encoded. A short payload terminates the iterator.
pub fn packet_messages(
    data: &[u8],
) -> impl Iterator<Item = Result<PacketMessage, ParserError>> + '_ {
    let mut reader = SliceReader::new(data);
    let mut failed = false;
    std::iter::from_fn(move || {
        if failed {
            return None;
        }
        let mut payload = Vec::new();
        match reader.read_packet_message_into(&mut payload) {
            Ok(Some(id)) => Some(Ok(PacketMessage { id, payload })),
            Ok(None) => None,
            Err(error) => {
                failed = true;
                Some(Err(error))
            }
        }
    })
}
