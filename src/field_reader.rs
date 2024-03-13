use crate::field_op::FieldOp;
use crate::field_path::FieldPath;
use crate::field_state::FieldState;
use crate::huffman_tree::{build_huffman_tree, EHTree};
use crate::reader::Reader;
use crate::serializer::Serializer;
use strum::IntoEnumIterator;

pub struct FieldReader {
    tree: EHTree,
}

impl FieldReader {
    pub fn new() -> Self {
        let tree =
            build_huffman_tree(FieldOp::iter().map(|op| op.weight() as i32).collect()).unwrap();
        FieldReader { tree }
    }

    pub(crate) fn read_field_paths(&self, reader: &mut Reader) -> Vec<FieldPath> {
        let mut fp = FieldPath::new();
        let mut paths: Vec<FieldPath> = Vec::new();
        let mut node = &self.tree;
        let mut next = &self.tree;
        let mut i = -1;
        loop {
            i += 1;
            next = match reader.read_bool() {
                true => node.right(),
                false => node.left(),
            };
            match next {
                EHTree::Leaf { value, .. } => {
                    node = &self.tree;
                    let op = FieldOp::from_position(*value);
                    op.execute(reader, &mut fp);
                    if let FieldOp::FieldPathEncodeFinish = op {
                        break;
                    }
                    paths.push(fp.clone());
                }
                EHTree::Node { .. } => {
                    node = next;
                }
            }
        }
        paths
    }

    pub(crate) fn read_fields(&self, reader: &mut Reader, s: &Serializer, st: &mut FieldState) {
        let fps = self.read_field_paths(reader);
        for fp in fps.iter() {
            let decoder = s.get_decoder_for_field_path(fp, 0);
            let val = decoder.decode(reader);
            st.set(fp, val);
        }
    }
}
