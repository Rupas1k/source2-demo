use crate::error::ParserError;
#[cfg(feature = "deadlock")]
use crate::proto::{
    CCitadelUserMsgPostMatchDetails, CDemoPacket, CMsgMatchMetaDataContents, CitadelUserMessageIds,
};
use crate::proto::{CDemoFileInfo, EDemoCommands, Message};
use crate::reader::bits::BitsReader;
use crate::reader::Reader;

pub(crate) struct OuterMessage {
    pub(crate) msg_type: EDemoCommands,
    pub(crate) tick: u32,
    pub(crate) buf: Vec<u8>,
}

pub(crate) trait MessageReader {
    fn read_next_message(&mut self) -> Result<Option<OuterMessage>, ParserError>;

    fn read_replay_info(&mut self) -> Result<CDemoFileInfo, ParserError>;

    #[cfg(feature = "deadlock")]
    fn read_deadlock_match_details(&mut self) -> Result<CMsgMatchMetaDataContents, ParserError>;
}

impl MessageReader for Reader<'_> {
    #[inline]
    fn read_next_message(&mut self) -> Result<Option<OuterMessage>, ParserError> {
        if self.bytes_remaining() == 0 {
            return Ok(None);
        }

        let cmd = self.read_var_u32() as i32;
        let tick = self.read_var_u32();
        let size = self.read_var_u32();

        let msg_type =
            EDemoCommands::try_from(cmd & !(EDemoCommands::DemIsCompressed as i32)).unwrap();
        let msg_compressed = cmd & EDemoCommands::DemIsCompressed as i32 != 0;

        let buf = if msg_compressed {
            let buf = self.read_bytes(size);
            let mut decoder = snap::raw::Decoder::new();
            decoder.decompress_vec(&buf)?
        } else {
            self.read_bytes(size)
        };

        Ok(Some(OuterMessage {
            msg_type,
            tick,
            buf,
        }))
    }

    fn read_replay_info(&mut self) -> Result<CDemoFileInfo, ParserError> {
        let offset = u32::from_le_bytes(self.buf[8..12].try_into().unwrap()) as usize;

        if self.buf.len() < offset {
            return Err(ParserError::ReplayEncodingError);
        }

        let mut reader = Reader::new(&self.buf[offset..]);
        Ok(CDemoFileInfo::decode(
            reader.read_next_message()?.unwrap().buf.as_slice(),
        )?)
    }

    #[cfg(feature = "deadlock")]
    fn read_deadlock_match_details(&mut self) -> Result<CMsgMatchMetaDataContents, ParserError> {
        let mut temp_reader = Reader::new(self.buf);
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
}
