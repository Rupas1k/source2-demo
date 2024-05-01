use crate::field::{FieldPath, FieldState};
use crate::reader::Reader;
use crate::serializer::Serializer;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct FieldReader {
    tree: HTree,
}

impl FieldReader {
    pub fn new() -> Self {
        let op_iter = [
            FieldOp::PlusOne,
            FieldOp::PlusTwo,
            FieldOp::PlusThree,
            FieldOp::PlusFour,
            FieldOp::PlusN,
            FieldOp::PushOneLeftDeltaZeroRightZero,
            FieldOp::PushOneLeftDeltaZeroRightNonZero,
            FieldOp::PushOneLeftDeltaOneRightZero,
            FieldOp::PushOneLeftDeltaOneRightNonZero,
            FieldOp::PushOneLeftDeltaNRightZero,
            FieldOp::PushOneLeftDeltaNRightNonZero,
            FieldOp::PushOneLeftDeltaNRightNonZeroPack6Bits,
            FieldOp::PushOneLeftDeltaNRightNonZeroPack8Bits,
            FieldOp::PushTwoLeftDeltaZero,
            FieldOp::PushTwoPack5LeftDeltaZero,
            FieldOp::PushThreeLeftDeltaZero,
            FieldOp::PushThreePack5LeftDeltaZero,
            FieldOp::PushTwoLeftDeltaOne,
            FieldOp::PushTwoPack5LeftDeltaOne,
            FieldOp::PushThreeLeftDeltaOne,
            FieldOp::PushThreePack5LeftDeltaOne,
            FieldOp::PushTwoLeftDeltaN,
            FieldOp::PushTwoPack5LeftDeltaN,
            FieldOp::PushThreeLeftDeltaN,
            FieldOp::PushThreePack5LeftDeltaN,
            FieldOp::PushN,
            FieldOp::PushNAndNonTopological,
            FieldOp::PopOnePlusOne,
            FieldOp::PopOnePlusN,
            FieldOp::PopAllButOnePlusOne,
            FieldOp::PopAllButOnePlusN,
            FieldOp::PopAllButOnePlusNPack3Bits,
            FieldOp::PopAllButOnePlusNPack6Bits,
            FieldOp::PopNPlusOne,
            FieldOp::PopNPlusN,
            FieldOp::PopNAndNonTopographical,
            FieldOp::NonTopoComplex,
            FieldOp::NonTopoPenultimatePlusOne,
            FieldOp::NonTopoComplexPack4Bits,
            FieldOp::FieldPathEncodeFinish,
        ];

        let tree = build_huffman_tree(op_iter.map(|op| op.weight() as i32).into());
        FieldReader { tree }
    }

    pub(crate) fn read_field_paths(&self, reader: &mut Reader) -> Vec<FieldPath> {
        let mut fp = FieldPath::new();
        let mut paths = vec![];
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
            .for_each(|fp| st.set(fp, s.get_decoder_for_field_path(fp).decode(reader)))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HTree {
    Leaf {
        weight: i32,
        value: i32,
    },
    Node {
        weight: i32,
        value: i32,
        left: Box<HTree>,
        right: Box<HTree>,
    },
}

impl Ord for HTree {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.weight().cmp(&other.weight()) {
            Ordering::Equal => self.value().cmp(&other.value()),
            ord => ord.reverse(),
        }
    }
}

impl PartialOrd for HTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HTree {
    pub fn weight(&self) -> i32 {
        match self {
            HTree::Leaf { weight, .. } | HTree::Node { weight, .. } => *weight,
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            HTree::Leaf { value, .. } | HTree::Node { value, .. } => *value,
        }
    }

    pub fn left(&self) -> &HTree {
        match self {
            HTree::Node { left, .. } => left,
            HTree::Leaf { .. } => panic!(),
        }
    }

    pub fn right(&self) -> &HTree {
        match self {
            HTree::Node { right, .. } => right,
            HTree::Leaf { .. } => panic!(),
        }
    }
}

pub fn build_huffman_tree(frequencies: Vec<i32>) -> HTree {
    let mut trees = frequencies
        .iter()
        .enumerate()
        .map(|(v, w)| HTree::Leaf {
            value: v as i32,
            weight: if *w == 0 { 1 } else { *w },
        })
        .collect::<BinaryHeap<HTree>>();

    let mut n = 40;

    while trees.len() > 1 {
        let a = trees.pop().unwrap();
        let b = trees.pop().unwrap();

        trees.push(HTree::Node {
            weight: a.weight() + b.weight(),
            value: n,
            left: Box::new(a),
            right: Box::new(b),
        });

        n += 1;
    }

    trees.pop().unwrap()
}

pub(crate) enum FieldOp {
    PlusOne,
    PlusTwo,
    PlusThree,
    PlusFour,
    PlusN,
    PushOneLeftDeltaZeroRightZero,
    PushOneLeftDeltaZeroRightNonZero,
    PushOneLeftDeltaOneRightZero,
    PushOneLeftDeltaOneRightNonZero,
    PushOneLeftDeltaNRightZero,
    PushOneLeftDeltaNRightNonZero,
    PushOneLeftDeltaNRightNonZeroPack6Bits,
    PushOneLeftDeltaNRightNonZeroPack8Bits,
    PushTwoLeftDeltaZero,
    PushTwoPack5LeftDeltaZero,
    PushThreeLeftDeltaZero,
    PushThreePack5LeftDeltaZero,
    PushTwoLeftDeltaOne,
    PushTwoPack5LeftDeltaOne,
    PushThreeLeftDeltaOne,
    PushThreePack5LeftDeltaOne,
    PushTwoLeftDeltaN,
    PushTwoPack5LeftDeltaN,
    PushThreeLeftDeltaN,
    PushThreePack5LeftDeltaN,
    PushN,
    PushNAndNonTopological,
    PopOnePlusOne,
    PopOnePlusN,
    PopAllButOnePlusOne,
    PopAllButOnePlusN,
    PopAllButOnePlusNPack3Bits,
    PopAllButOnePlusNPack6Bits,
    PopNPlusOne,
    PopNPlusN,
    PopNAndNonTopographical,
    NonTopoComplex,
    NonTopoPenultimatePlusOne,
    NonTopoComplexPack4Bits,
    FieldPathEncodeFinish,
}

impl FieldOp {
    pub(crate) fn execute(&self, r: &mut Reader, fp: &mut FieldPath) {
        match &self {
            FieldOp::PlusOne => fp.inc_curr(1),
            FieldOp::PlusTwo => fp.inc_curr(2),
            FieldOp::PlusThree => fp.inc_curr(3),
            FieldOp::PlusFour => fp.inc_curr(4),
            FieldOp::PlusN => fp.inc_curr(r.read_ubit_var_fp() as u8 + 5),
            FieldOp::PushOneLeftDeltaZeroRightZero => {
                fp.last += 1;
                fp.path[fp.last] = 0;
            }
            FieldOp::PushOneLeftDeltaZeroRightNonZero => {
                fp.last += 1;
                fp.path[fp.last] = r.read_ubit_var_fp() as u8;
            }
            FieldOp::PushOneLeftDeltaOneRightZero => {
                fp.inc_curr(1);
                fp.last += 1;
                fp.path[fp.last] = 0;
            }
            FieldOp::PushOneLeftDeltaOneRightNonZero => {
                fp.inc_curr(1);
                fp.last += 1;
                fp.path[fp.last] = r.read_ubit_var_fp() as u8;
            }
            FieldOp::PushOneLeftDeltaNRightZero => {
                fp.inc_curr(r.read_ubit_var_fp() as u8);
                fp.last += 1;
                fp.path[fp.last] = 0;
            }
            FieldOp::PushOneLeftDeltaNRightNonZero => {
                fp.inc_curr(r.read_ubit_var_fp() as u8 + 2);
                fp.last += 1;
                fp.path[fp.last] = r.read_ubit_var_fp() as u8 + 1;
            }
            FieldOp::PushOneLeftDeltaNRightNonZeroPack6Bits => {
                fp.inc_curr(r.read_bits(3) as u8 + 2);
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(3) as u8 + 1;
            }
            FieldOp::PushOneLeftDeltaNRightNonZeroPack8Bits => {
                fp.inc_curr(r.read_bits(4) as u8 + 2);
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(4) as u8 + 1;
            }
            FieldOp::PushTwoLeftDeltaZero => {
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
            }
            FieldOp::PushTwoPack5LeftDeltaZero => {
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
            }
            FieldOp::PushThreeLeftDeltaZero => {
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
            }
            FieldOp::PushThreePack5LeftDeltaZero => {
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
            }
            FieldOp::PushTwoLeftDeltaOne => {
                fp.inc_curr(1);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
            }
            FieldOp::PushTwoPack5LeftDeltaOne => {
                fp.inc_curr(1);
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
            }
            FieldOp::PushThreeLeftDeltaOne => {
                fp.inc_curr(1);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
            }
            FieldOp::PushThreePack5LeftDeltaOne => {
                fp.inc_curr(1);
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
            }
            FieldOp::PushTwoLeftDeltaN => {
                fp.inc_curr(r.read_ubit_var() as u8 + 2);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
            }
            FieldOp::PushTwoPack5LeftDeltaN => {
                fp.inc_curr(r.read_ubit_var() as u8 + 2);
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
            }
            FieldOp::PushThreeLeftDeltaN => {
                fp.inc_curr(r.read_ubit_var() as u8 + 2);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
                fp.last += 1;
                fp.inc_curr(r.read_ubit_var_fp() as u8);
            }
            FieldOp::PushThreePack5LeftDeltaN => {
                // fp.path[fp.last] += r.read_ubit_var() as u8 + 2;
                fp.inc_curr(r.read_ubit_var() as u8 + 2);
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as u8;
            }
            FieldOp::PushN => {
                let n = r.read_ubit_var() as i32;
                fp.inc_curr(r.read_ubit_var() as u8);
                for _ in 0..n {
                    fp.last += 1;
                    fp.inc_curr(r.read_ubit_var_fp() as u8);
                }
            }
            FieldOp::PushNAndNonTopological => {
                for i in 0..=fp.last {
                    if r.read_bool() {
                        fp.inc(i, r.read_var_i32() as u8 + 1);
                    }
                }
                let count = r.read_ubit_var() as usize;
                for _ in 0..count {
                    fp.last += 1;
                    fp.path[fp.last] = r.read_ubit_var_fp() as u8;
                }
            }
            FieldOp::PopOnePlusOne => {
                fp.pop(1);
                fp.inc_curr(1);
            }

            FieldOp::PopOnePlusN => {
                fp.pop(1);
                fp.inc_curr(r.read_ubit_var_fp() as u8 + 1);
            }
            FieldOp::PopAllButOnePlusOne => {
                fp.pop(fp.last);
                fp.inc(0, 1);
            }
            FieldOp::PopAllButOnePlusN => {
                fp.pop(fp.last);
                fp.inc(0, r.read_ubit_var_fp() as u8 + 1);
            }
            FieldOp::PopAllButOnePlusNPack3Bits => {
                fp.pop(fp.last);
                fp.inc(0, r.read_bits(3) as u8 + 1);
            }
            FieldOp::PopAllButOnePlusNPack6Bits => {
                fp.pop(fp.last);
                fp.inc(0, r.read_bits(6) as u8 + 1);
            }
            FieldOp::PopNPlusOne => {
                fp.pop(r.read_ubit_var_fp() as usize);
                fp.inc_curr(1);
            }
            FieldOp::PopNPlusN => {
                fp.pop(r.read_ubit_var_fp() as usize);
                fp.inc_curr(r.read_var_i32() as u8);
            }
            FieldOp::PopNAndNonTopographical => {
                fp.pop(r.read_ubit_var_fp() as usize);
                for i in 0..=fp.last {
                    if r.read_bool() {
                        fp.inc(i, r.read_var_i32() as u8);
                    }
                }
            }
            FieldOp::NonTopoComplex => {
                for i in 0..=fp.last {
                    if r.read_bool() {
                        fp.inc(i, r.read_var_i32() as u8);
                    }
                }
            }
            FieldOp::NonTopoPenultimatePlusOne => {
                fp.inc(fp.last - 1, 1);
            }
            FieldOp::NonTopoComplexPack4Bits => {
                for i in 0..=fp.last {
                    if r.read_bool() {
                        fp.inc(i, r.read_bits(4) as u8);
                        fp.sub(i, 7);
                    }
                }
            }
            FieldOp::FieldPathEncodeFinish => {}
        }
    }
    pub fn weight(&self) -> u32 {
        match self {
            FieldOp::PlusOne => 36271,
            FieldOp::PlusTwo => 10334,
            FieldOp::PlusThree => 1375,
            FieldOp::PlusFour => 646,
            FieldOp::PlusN => 4128,
            FieldOp::PushOneLeftDeltaZeroRightZero => 35,
            FieldOp::PushOneLeftDeltaZeroRightNonZero => 3,
            FieldOp::PushOneLeftDeltaOneRightZero => 521,
            FieldOp::PushOneLeftDeltaOneRightNonZero => 2942,
            FieldOp::PushOneLeftDeltaNRightZero => 560,
            FieldOp::PushOneLeftDeltaNRightNonZero => 471,
            FieldOp::PushOneLeftDeltaNRightNonZeroPack6Bits => 10530,
            FieldOp::PushOneLeftDeltaNRightNonZeroPack8Bits => 251,
            FieldOp::PushTwoLeftDeltaZero => 0,
            FieldOp::PushTwoPack5LeftDeltaZero => 0,
            FieldOp::PushThreeLeftDeltaZero => 0,
            FieldOp::PushThreePack5LeftDeltaZero => 0,
            FieldOp::PushTwoLeftDeltaOne => 0,
            FieldOp::PushTwoPack5LeftDeltaOne => 0,
            FieldOp::PushThreeLeftDeltaOne => 0,
            FieldOp::PushThreePack5LeftDeltaOne => 0,
            FieldOp::PushTwoLeftDeltaN => 0,
            FieldOp::PushTwoPack5LeftDeltaN => 0,
            FieldOp::PushThreeLeftDeltaN => 0,
            FieldOp::PushThreePack5LeftDeltaN => 0,
            FieldOp::PushN => 0,
            FieldOp::PushNAndNonTopological => 310,
            FieldOp::PopOnePlusOne => 2,
            FieldOp::PopOnePlusN => 0,
            FieldOp::PopAllButOnePlusOne => 1837,
            FieldOp::PopAllButOnePlusN => 149,
            FieldOp::PopAllButOnePlusNPack3Bits => 300,
            FieldOp::PopAllButOnePlusNPack6Bits => 634,
            FieldOp::PopNPlusOne => 0,
            FieldOp::PopNPlusN => 0,
            FieldOp::PopNAndNonTopographical => 1,
            FieldOp::NonTopoComplex => 76,
            FieldOp::NonTopoPenultimatePlusOne => 271,
            FieldOp::NonTopoComplexPack4Bits => 99,
            FieldOp::FieldPathEncodeFinish => 25474,
        }
    }

    pub fn from_position(position: i32) -> Self {
        match position {
            0 => FieldOp::PlusOne,
            1 => FieldOp::PlusTwo,
            2 => FieldOp::PlusThree,
            3 => FieldOp::PlusFour,
            4 => FieldOp::PlusN,
            5 => FieldOp::PushOneLeftDeltaZeroRightZero,
            6 => FieldOp::PushOneLeftDeltaZeroRightNonZero,
            7 => FieldOp::PushOneLeftDeltaOneRightZero,
            8 => FieldOp::PushOneLeftDeltaOneRightNonZero,
            9 => FieldOp::PushOneLeftDeltaNRightZero,
            10 => FieldOp::PushOneLeftDeltaNRightNonZero,
            11 => FieldOp::PushOneLeftDeltaNRightNonZeroPack6Bits,
            12 => FieldOp::PushOneLeftDeltaNRightNonZeroPack8Bits,
            13 => FieldOp::PushTwoLeftDeltaZero,
            14 => FieldOp::PushTwoPack5LeftDeltaZero,
            15 => FieldOp::PushThreeLeftDeltaZero,
            16 => FieldOp::PushThreePack5LeftDeltaZero,
            17 => FieldOp::PushTwoLeftDeltaOne,
            18 => FieldOp::PushTwoPack5LeftDeltaOne,
            19 => FieldOp::PushThreeLeftDeltaOne,
            20 => FieldOp::PushThreePack5LeftDeltaOne,
            21 => FieldOp::PushTwoLeftDeltaN,
            22 => FieldOp::PushTwoPack5LeftDeltaN,
            23 => FieldOp::PushThreeLeftDeltaN,
            24 => FieldOp::PushThreePack5LeftDeltaN,
            25 => FieldOp::PushN,
            26 => FieldOp::PushNAndNonTopological,
            27 => FieldOp::PopOnePlusOne,
            28 => FieldOp::PopOnePlusN,
            29 => FieldOp::PopAllButOnePlusOne,
            30 => FieldOp::PopAllButOnePlusN,
            31 => FieldOp::PopAllButOnePlusNPack3Bits,
            32 => FieldOp::PopAllButOnePlusNPack6Bits,
            33 => FieldOp::PopNPlusOne,
            34 => FieldOp::PopNPlusN,
            35 => FieldOp::PopNAndNonTopographical,
            36 => FieldOp::NonTopoComplex,
            37 => FieldOp::NonTopoPenultimatePlusOne,
            38 => FieldOp::NonTopoComplexPack4Bits,
            39 => FieldOp::FieldPathEncodeFinish,
            _ => unreachable!(),
        }
    }
}
