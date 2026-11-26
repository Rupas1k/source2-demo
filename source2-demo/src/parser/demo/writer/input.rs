use crate::error::ParserError;
use crate::parser::demo::writer::DemoWriter;
use crate::reader::{read_raw_command, BitsReader, MessageReader, RawDemoMessage};
use std::io::{Seek, Write};

impl<'a, R, W> DemoWriter<'a, R, W>
where
    R: BitsReader + MessageReader,
    W: Write + Seek,
{
    pub(crate) fn read_next_raw_message(&mut self) -> Result<Option<RawDemoMessage>, ParserError> {
        read_raw_command(&mut self.parser.reader)
    }

    pub(crate) fn decode_raw_payload(message: &RawDemoMessage) -> Result<Vec<u8>, ParserError> {
        if message.compressed {
            let mut decoder = snap::raw::Decoder::new();
            Ok(decoder.decompress_vec(message.raw_payload.as_slice())?)
        } else {
            Ok(message.raw_payload.clone())
        }
    }

    pub(crate) fn materialize_payload(
        payload: &mut Option<Vec<u8>>,
        message: &RawDemoMessage,
    ) -> Result<Vec<u8>, ParserError> {
        payload
            .take()
            .map(Ok)
            .unwrap_or_else(|| Self::decode_raw_payload(message))
    }
}
