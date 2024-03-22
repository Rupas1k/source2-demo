use crate::field_path::FieldOp;
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
        loop {
            let next = match reader.read_bool() {
                true => node.right(),
                false => node.left(),
            };
            match next {
                EHTree::Leaf { value, .. } => {
                    let op = FieldOp::from_position(*value);
                    op.execute(reader, &mut fp);
                    if let FieldOp::FieldPathEncodeFinish = op {
                        break;
                    }
                    paths.push(fp.clone());
                    node = &self.tree;
                }
                EHTree::Node { .. } => {
                    node = next;
                }
            }
        }
        paths
    }

    pub(crate) fn read_fields(&self, reader: &mut Reader, s: &Serializer, st: &mut FieldState) {
        self.read_field_paths(reader)
            .iter()
            .for_each(|fp| st.set(fp, s.get_decoder_for_field_path(fp, 0).decode(reader)))
    }
}
