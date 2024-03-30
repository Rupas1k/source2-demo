use crate::field::FieldPath;
use crate::field::FieldState;
use crate::operation::FieldOp;
use crate::serializer::Serializer;
use crate::utils::Reader;
use crate::utils::{build_huffman_tree, HTree};
use strum::IntoEnumIterator;

pub struct FieldReader {
    tree: HTree,
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
                HTree::Leaf { value, .. } => {
                    let op = FieldOp::from_position(*value);
                    op.execute(reader, &mut fp);
                    if let FieldOp::FieldPathEncodeFinish = op {
                        break;
                    }
                    paths.push(fp);
                    node = &self.tree;
                }
                HTree::Node { .. } => {
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
