mod huff;
mod op;

use huff::*;
use op::*;

use crate::field::{FieldPath, FieldState, Serializer};
use crate::reader::{BitsReader, Reader};
use std::cell::RefCell;

pub(crate) struct FieldReader {
    tree: HTree,
    paths_buf: RefCell<[FieldPath; 4096]>,
}

impl Default for FieldReader {
    fn default() -> Self {
        FieldReader {
            tree: HTree::default(),
            paths_buf: RefCell::new([FieldPath::default(); 4096]),
        }
    }
}

impl FieldReader {
    #[inline]
    pub(crate) fn read_fields(
        &self,
        reader: &mut Reader,
        serializer: &Serializer,
        state: &mut FieldState,
    ) {
        let mut paths = self.paths_buf.borrow_mut();
        let mut node = &self.tree;
        let mut i = 0;
        let mut fp = FieldPath::default();
        reader.refill();
        loop {
            let next = match reader.read_bool() {
                true => node.right(),
                false => node.left(),
            };
            match next {
                HTree::Leaf { value, .. } => {
                    let op = OPERATIONS[*value as usize].0;
                    op.execute(reader, &mut fp);
                    if let FieldOp::FieldPathEncodeFinish = op {
                        break;
                    }
                    paths[i] = fp;
                    i += 1;
                    node = &self.tree;
                    reader.refill();
                }
                HTree::Node { .. } => {
                    node = next;
                }
            }
        }

        paths[..i]
            .iter_mut()
            .for_each(|fp| state.set(fp, serializer.get_decoder_for_field_path(fp).decode(reader)))
    }
}
