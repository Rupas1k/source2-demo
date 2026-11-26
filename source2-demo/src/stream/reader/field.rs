use crate::entity::field::{Decode, FieldPath, FieldState, FieldValue, Serializer};
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
        self.read_fields_with(reader, serializer, state, |_, _, _| {});
    }

    pub(crate) fn read_fields_with(
        &mut self,
        reader: &mut SliceReader,
        serializer: &Serializer,
        state: &mut FieldState,
        mut on_write: impl FnMut(&FieldPath, &FieldState, &FieldValue),
    ) {
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

        for path in &self.paths_buf[..i] {
            let value = serializer.get_decoder(path).decode(reader);
            on_write(path, state, &value);
            state.set(path, value);
        }
    }
}
