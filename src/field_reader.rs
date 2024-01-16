use nohash_hasher::IntMap;
use strum::IntoEnumIterator;
use crate::reader::{Reader, ReaderMethods};
use crate::field_op::FieldOp;
use crate::field_path::FieldPath;
use crate::field_state::FieldState;
use crate::huffman_tree::{build_huffman_tree, EHTree};
use crate::serializer::Serializer;

pub struct FieldReader {
    table: IntMap<usize, FieldOp>,
    tree: EHTree,
}

impl FieldReader {
    pub fn new() -> Self {
        let tree = build_huffman_tree(FieldOp::iter().map(|op| op.weight() as i32).collect()).unwrap();
        let mut table = IntMap::default();
        table.extend(FieldOp::iter().enumerate());
        FieldReader {
            table,
            tree,
        }
    }

    pub fn read_field_paths(&self, reader: &mut Reader) -> Vec<FieldPath> {
        let mut fp = FieldPath::new();
        let mut paths: Vec<FieldPath> = Vec::new();
        let mut node = &self.tree;
        let mut next = &self.tree;
        loop {
            next = match reader.read_bool() {
                true => node.right(),
                false => node.left(),
            };
            match next {
                EHTree::Leaf { value, .. } => {
                    node = &self.tree;
                    let op = &self.table[&(*value as usize)];
                    op.execute(reader, &mut fp);
                    if let FieldOp::FieldPathEncodeFinish = op {
                        break
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

    pub fn read_fields(&self, reader: &mut Reader, s: &Serializer, st: &mut FieldState) {
        let fps = self.read_field_paths(reader);
        for fp in fps {
            let decoder = s.get_decoder_for_field_path(&fp, 0);
            let val = decoder.decode(reader);
            st.set(&fp, val);
        }
    }
}