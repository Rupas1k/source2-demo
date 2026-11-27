use crate::error::ParserError;
#[cfg(feature = "deadlock")]
use crate::proto::{
    CCitadelUserMsgPostMatchDetails, CDemoPacket, CMsgMatchMetaDataContents, CitadelUserMessageIds,
};
use crate::proto::{CDemoFileInfo, EDemoCommands, Message};
use crate::stream::bits::BitsReader;
use crate::stream::msg::{MessageReader, OuterMessage, ReplayInfoReader};
use crate::stream::reader::{SeekableReader, SliceReader};
use std::io::{Read, Seek};

pub(crate) struct RawDemoMessage {
    pub(crate) msg_type: EDemoCommands,
    pub(crate) tick: u32,
    pub(crate) raw_payload: Vec<u8>,
    pub(crate) compressed: bool,
}

pub(crate) fn read_raw_command(
    reader: &mut impl BitsReader,
) -> Result<Option<RawDemoMessage>, ParserError> {
    reader.refill();
    if reader.remaining_bytes() == 0 {
        return Ok(None);
    }
    let header = super::framing::CommandHeader::read(reader);
    let msg_type = EDemoCommands::try_from(header.id as i32)?;
    Ok(Some(RawDemoMessage {
        msg_type,
        tick: header.tick,
        compressed: header.compressed,
        raw_payload: reader.read_bytes(header.size),
    }))
}

fn read_outer_message(reader: &mut impl BitsReader) -> Result<Option<OuterMessage>, ParserError> {
    let Some(RawDemoMessage {
        msg_type,
        tick,
        compressed,
        raw_payload,
    }) = read_raw_command(reader)?
    else {
        return Ok(None);
    };
    let buf = if compressed {
        snap::raw::Decoder::new().decompress_vec(&raw_payload)?
    } else {
        raw_payload
    };
    Ok(Some(OuterMessage {
        msg_type,
        tick,
        buf,
        compressed,
    }))
}

impl MessageReader for SliceReader<'_> {
    fn read_next_message(&mut self) -> Result<Option<OuterMessage>, ParserError> {
        read_outer_message(self)
    }
}

impl<R: Read + Seek> ReplayInfoReader for SeekableReader<R> {
    fn read_replay_info(&mut self) -> Result<CDemoFileInfo, ParserError> {
        self.seek(8);
        let offset_bytes = self.read_bytes(4);
        let offset = u32::from_le_bytes([
            offset_bytes[0],
            offset_bytes[1],
            offset_bytes[2],
            offset_bytes[3],
        ]) as usize;

        self.seek(offset);

        if let Some(msg) = self.read_next_message()? {
            Ok(CDemoFileInfo::decode(msg.buf.as_slice())?)
        } else {
            Err(ParserError::ReplayEncodingError)
        }
    }

    #[cfg(feature = "deadlock")]
    fn read_deadlock_match_details(&mut self) -> Result<CMsgMatchMetaDataContents, ParserError> {
        self.seek(16);

        while let Some(message) = self.read_next_message()? {
            if message.msg_type != EDemoCommands::DemPacket {
                continue;
            }

            let packet = CDemoPacket::decode(message.buf.as_slice())?;
            let mut packet_reader = SliceReader::new(packet.data());

            let mut packet_buf = Vec::new();
            while let Some(msg_type) = packet_reader.read_packet_message_into(&mut packet_buf)? {
                if msg_type == CitadelUserMessageIds::KEUserMsgPostMatchDetails as u32 {
                    return Ok(CMsgMatchMetaDataContents::decode(
                        CCitadelUserMsgPostMatchDetails::decode(packet_buf.as_slice())?
                            .match_details(),
                    )?);
                }
            }
        }

        Err(ParserError::MatchDetailsNotFound)
    }
}

impl<'a> ReplayInfoReader for SliceReader<'a> {
    fn read_replay_info(&mut self) -> Result<CDemoFileInfo, ParserError> {
        let source_data = self.source_buffer;
        let offset = u32::from_le_bytes(source_data[8..12].try_into().unwrap()) as usize;

        if source_data.len() < offset {
            return Err(ParserError::ReplayEncodingError);
        }

        let mut reader = SliceReader::new(&source_data[offset..]);
        Ok(CDemoFileInfo::decode(
            reader.read_next_message()?.unwrap().buf.as_slice(),
        )?)
    }

    #[cfg(feature = "deadlock")]
    fn read_deadlock_match_details(&mut self) -> Result<CMsgMatchMetaDataContents, ParserError> {
        let source_data = self.source_buffer;
        let mut temp_reader = SliceReader::new(source_data);
        temp_reader.seek(16);
        while let Some(message) = temp_reader.read_next_message()? {
            if message.msg_type != EDemoCommands::DemPacket {
                continue;
            }

            let packet = CDemoPacket::decode(message.buf.as_slice())?;
            let mut packet_reader = SliceReader::new(packet.data());
            let mut packet_buf = Vec::new();
            while let Some(msg_type) = packet_reader.read_packet_message_into(&mut packet_buf)? {
                if msg_type == CitadelUserMessageIds::KEUserMsgPostMatchDetails as u32 {
                    return Ok(CMsgMatchMetaDataContents::decode(
                        CCitadelUserMsgPostMatchDetails::decode(packet_buf.as_slice())?
                            .match_details(),
                    )?);
                }
            }
        }

        Err(ParserError::ReplayEncodingError)
    }
}

impl<R: Read + Seek> MessageReader for SeekableReader<R> {
    #[inline]
    fn read_next_message(&mut self) -> Result<Option<OuterMessage>, ParserError> {
        read_outer_message(self)
    }
}
