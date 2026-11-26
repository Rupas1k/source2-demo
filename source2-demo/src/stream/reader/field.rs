#[cfg(feature = "entity-field-updates")]
use crate::entity::field::FieldValue;
use crate::entity::field::{Decode, FieldPath, FieldState, Serializer};
use crate::reader::{BitsReader, SliceReader};
use crate::stream::field_path::{FieldOp, FieldPathCodec};

pub(crate) struct FieldReader {
    codec: FieldPathCodec,
    paths_buf: [FieldPath; 8192],
}

impl Default for FieldReader {
    fn default() -> Self {
        FieldReader {
            codec: FieldPathCodec::default(),
            paths_buf: [FieldPath::default(); 8192],
        }
    }
}

impl FieldReader {
    #[inline]
    pub(crate) fn read_fields(
        &mut self,
        reader: &mut SliceReader,
        serializer: &Serializer,
        state: &mut FieldState,
    ) {
        let count = self.read_paths(reader);
        self.paths_buf[..count]
            .iter_mut()
            .for_each(|fp| state.set(fp, serializer.get_decoder(fp).decode(reader)));
    }

    #[cfg(feature = "entity-field-updates")]
    pub(crate) fn read_fields_with(
        &mut self,
        reader: &mut SliceReader,
        serializer: &Serializer,
        state: &mut FieldState,
        mut on_write: impl FnMut(&FieldPath, &FieldState, &FieldValue),
    ) {
        let count = self.read_paths(reader);
        for path in &self.paths_buf[..count] {
            let value = serializer.get_decoder(path).decode(reader);
            on_write(path, state, &value);
            state.set(path, value);
        }
    }

    #[inline(always)]
    fn read_paths(&mut self, reader: &mut SliceReader) -> usize {
        let mut i = 0;
        let mut fp = FieldPath::default();
        reader.refill();
        loop {
            let op = self.codec.read_op(reader);
            if let FieldOp::FieldPathEncodeFinish = op {
                break;
            }
            op.execute(reader, &mut fp);
            self.paths_buf[i] = fp;
            i += 1;
            reader.refill();
        }
        i
    }
}
