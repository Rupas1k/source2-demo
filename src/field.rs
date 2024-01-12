use std::cell::RefCell;
use std::cmp::max;
use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;
use protobuf::Message;
use protogen::netmessages::{CSVCMsg_FlattenedSerializer, ProtoFlattenedSerializerField_t};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use enum_as_inner::EnumAsInner;

use crate::huffman_tree::{build_huffman_tree, EHTree};
use crate::qfloat::QFloatDecoder;
use crate::reader::{Reader, ReaderMethods};
use crate::serializer::Serializer;

#[derive(Clone, Debug)]
pub struct FieldType {
    pub base: String,
    pub generic: Option<Box<FieldType>>,
    pub pointer: bool,
    pub count: Option<u32>,
}

impl FieldType {
    pub fn new(name: &str) -> Self {
        let re = Regex::new(r"([^<\[*]+)(<\s(.*)\s>)?(\*)?(\[(.*)])?").unwrap();
        let captures = re.captures(name).unwrap();

        // println!("{:?}", captures);
        if captures.len() != 7 {
            panic!("Regex error")
        }

        let base = captures[1].to_string();
        let pointer = captures.get(4).is_some() && captures.get(4).unwrap().as_str() == "*";
        let generic = match &captures.get(3) {
            None => None,
            Some(v) => Some(Box::new(FieldType::new(v.as_str()))),
        };

        let mut item_counts: HashMap<&str, i32> = HashMap::new();
        item_counts.insert("MAX_ITEM_STOCKS", 8);
        item_counts.insert("MAX_ABILITY_DRAFT_ABILITIES", 48);


        let count = match captures.get(6) {
            Some(x) => {
                if let Some(&y) = item_counts.get(x.as_str()) {
                    Some(y as u32)
                } else {
                    Some(x.as_str().parse::<u32>().unwrap())
                }

            }
            None => None
        };
        // let count = captures
        //     .get(6)
        //     .map_or(1024, |count_str| count_str.as_str().parse().unwrap_or(1024))
        //     as u32;


        FieldType {
            base,
            generic,
            pointer,
            count,
        }
    }

    pub fn as_str(&self) -> String {
        let mut s = self.base.clone();

        if let Some(generic_type) = &self.generic {
            s += &format!("<{}>", &generic_type.as_str());
        }

        if self.pointer {
            s += "*";
        }

        if self.count.is_some() && self.count.unwrap() > 0 {
            s += &format!("[{}]", self.count.unwrap());
        }

        s
    }
}

#[derive(Debug, Clone)]
pub struct FieldPath {
    pub path: [i32; 7],
    pub last: usize,
    pub done: bool,
}

impl FieldPath {
    pub fn new() -> Self {
        FieldPath {
            path: [-1, 0, 0, 0, 0, 0, 0],
            last: 0,
            done: false,
        }
    }

    pub fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.path[self.last] = 0;
            self.last -= 1;
        }
    }
}

#[derive(Debug, EnumIter)]
pub enum FieldOp {
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
    NonTopoPenultimatePluseOne,
    NonTopoComplexPack4Bits,
    FieldPathEncodeFinish,
}

impl FieldOp {
    pub fn execute(&self, r: &mut Reader, fp: &mut FieldPath) {
        match &self {
            FieldOp::PlusOne => {
                fp.path[fp.last] += 1;
            }
            FieldOp::PlusTwo => {
                fp.path[fp.last] += 2;
            }
            FieldOp::PlusThree => {
                fp.path[fp.last] += 3;
            }
            FieldOp::PlusFour => {
                fp.path[fp.last] += 4;
            }
            FieldOp::PlusN => {
                fp.path[fp.last] += r.read_ubit_var_fieldpath() + 5;
            }
            FieldOp::PushOneLeftDeltaZeroRightZero => {
                fp.last += 1;
                fp.path[fp.last] = 0;
            }
            FieldOp::PushOneLeftDeltaZeroRightNonZero => {
                fp.last += 1;
                fp.path[fp.last] = r.read_ubit_var_fieldpath();
            }
            FieldOp::PushOneLeftDeltaOneRightZero => {
                fp.path[fp.last] += 1;
                fp.last += 1;
                fp.path[fp.last] = 0;
            }
            FieldOp::PushOneLeftDeltaOneRightNonZero => {
                fp.path[fp.last] += 1;
                fp.last += 1;
                fp.path[fp.last] = r.read_ubit_var_fieldpath();
            }
            FieldOp::PushOneLeftDeltaNRightZero => {
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] = 0;
            }
            FieldOp::PushOneLeftDeltaNRightNonZero => {
                fp.path[fp.last] += r.read_ubit_var_fieldpath() + 2;
                fp.last += 1;
                fp.path[fp.last] = r.read_ubit_var_fieldpath() + 1;
            }
            FieldOp::PushOneLeftDeltaNRightNonZeroPack6Bits => {
                fp.path[fp.last] += r.read_bits(3) as i32 + 2;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(3) as i32 + 1;
            }
            FieldOp::PushOneLeftDeltaNRightNonZeroPack8Bits => {
                fp.path[fp.last] += r.read_bits(4) as i32 + 2;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(4) as i32 + 1;
            }
            FieldOp::PushTwoLeftDeltaZero => {
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
            }
            FieldOp::PushTwoPack5LeftDeltaZero => {
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
            }
            FieldOp::PushThreeLeftDeltaZero => {
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
            }
            FieldOp::PushThreePack5LeftDeltaZero => {
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
            }
            FieldOp::PushTwoLeftDeltaOne => {
                fp.path[fp.last] += 1;
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
            }
            FieldOp::PushTwoPack5LeftDeltaOne => {
                fp.path[fp.last] += 1;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
            }
            FieldOp::PushThreeLeftDeltaOne => {
                fp.path[fp.last] += 1;
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
            }
            FieldOp::PushThreePack5LeftDeltaOne => {
                fp.path[fp.last] += 1;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
            }
            FieldOp::PushTwoLeftDeltaN => {
                fp.path[fp.last] += r.read_ubit_var() as i32 + 2;
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
            }
            FieldOp::PushTwoPack5LeftDeltaN => {
                fp.path[fp.last] += r.read_ubit_var() as i32 + 2;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
            }
            FieldOp::PushThreeLeftDeltaN => {
                fp.path[fp.last] += r.read_ubit_var() as i32 + 2;
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
                fp.last += 1;
                fp.path[fp.last] += r.read_ubit_var_fieldpath();
            }
            FieldOp::PushThreePack5LeftDeltaN => {
                fp.path[fp.last] += r.read_ubit_var() as i32 + 2;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
                fp.last += 1;
                fp.path[fp.last] = r.read_bits(5) as i32;
            }
            FieldOp::PushN => {
                let n = r.read_ubit_var() as i32;
                fp.path[fp.last] += r.read_ubit_var() as i32;
                for _ in 0..n {
                    fp.last += 1;
                    fp.path[fp.last] += r.read_ubit_var_fieldpath();
                }
            }
            FieldOp::PushNAndNonTopological => {
                for i in 0..=fp.last {
                    if r.read_bool() {
                        fp.path[i] += r.read_var_i32() + 1;
                    }
                }
                let count = r.read_ubit_var() as usize;
                for _ in 0..count {
                    fp.last += 1;
                    fp.path[fp.last] = r.read_ubit_var_fieldpath();
                }
            }
            FieldOp::PopOnePlusOne => {
                fp.pop(1);
                fp.path[fp.last] += 1;
            }

            FieldOp::PopOnePlusN => {
                fp.pop(1);
                fp.path[fp.last] += r.read_ubit_var_fieldpath() + 1;
            }
            FieldOp::PopAllButOnePlusOne => {
                fp.pop(fp.last);
                fp.path[0] += 1;
            }
            FieldOp::PopAllButOnePlusN => {
                fp.pop(fp.last);
                fp.path[0] += r.read_ubit_var_fieldpath() + 1;
            }
            FieldOp::PopAllButOnePlusNPack3Bits => {
                fp.pop(fp.last);
                fp.path[0] += r.read_bits(3) as i32 + 1;
            }
            FieldOp::PopAllButOnePlusNPack6Bits => {
                fp.pop(fp.last);
                fp.path[0] += r.read_bits(6) as i32 + 1;
            }
            FieldOp::PopNPlusOne => {
                fp.pop(r.read_ubit_var_fieldpath() as usize);
                fp.path[fp.last] += 1;
            }
            FieldOp::PopNPlusN => {
                fp.pop(r.read_ubit_var_fieldpath() as usize);
                fp.path[fp.last] += r.read_var_i32();
            }
            FieldOp::PopNAndNonTopographical => {
                fp.pop(r.read_ubit_var_fieldpath() as usize);
                for i in 0..=fp.last {
                    if r.read_bool() {
                        fp.path[i] += r.read_var_i32();
                    }
                }
            }
            FieldOp::NonTopoComplex => {
                for i in 0..=fp.last {
                    if r.read_bool() {
                        fp.path[i] += r.read_var_i32();
                    }
                }
            }
            FieldOp::NonTopoPenultimatePluseOne => {
                fp.path[fp.last - 1] += 1;
            }
            FieldOp::NonTopoComplexPack4Bits => {
                for i in 0..=fp.last {
                    if r.read_bool() {
                        fp.path[i] += r.read_bits(4) as i32 - 7;
                    }
                }
            }
            FieldOp::FieldPathEncodeFinish => {
                fp.done = true;
            }
        }
    }
    pub fn weight(&self) -> u32 {
        return match self {
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
            FieldOp::NonTopoPenultimatePluseOne => 271,
            FieldOp::NonTopoComplexPack4Bits => 99,
            FieldOp::FieldPathEncodeFinish => 25474,
        };
    }
}

pub struct FieldReader {
    // reader: Reader,
    table: HashMap<usize, FieldOp>,
    tree: EHTree,
}

impl FieldReader {
    pub fn new() -> Self {
        let tree = build_huffman_tree(FieldOp::iter().map(|op| op.weight() as i32).collect()).unwrap();
        let mut table = HashMap::new();
        table.extend(FieldOp::iter().enumerate());
        FieldReader {
            // reader,
            table,
            tree,
        }
    }

    pub fn read_field_paths(&self, reader: &mut Reader) -> Vec<FieldPath> {
        let mut fp = FieldPath::new();
        let mut paths: Vec<FieldPath> = Vec::new();
        let mut node = &self.tree;
        let mut next = &self.tree;
        // println!("{:?}", self.tree);
        // println!("{:?}", self.table.len());
        while !fp.done {
            // println!("{:?}", fp);
            next = match reader.read_bool() {
                true => node.right(),
                false => node.left(),
            };
            // println!("{:?}", next);
            match next {
                EHTree::Leaf { value, .. } => {
                    node = &self.tree;
                    self.table
                        .get(&(*value as usize))
                        .unwrap()
                        .execute(reader, &mut fp);
                    if !fp.done {
                        paths.push(fp.clone());
                        // fp = FieldPath::new();
                    }
                }
                EHTree::Node { .. } => {
                    node = next;
                }
            }
        }
        // println!("{:?}", paths);
        paths
    }

    pub fn read_fields(&self, reader: &mut Reader, s: &Serializer, st: &mut FieldState) {
        let fps = self.read_field_paths(reader);
        for fp in fps {
            let decoder = s.get_decoder_for_field_path(&fp, 0);
            let val = decoder.decode(reader);
            st.set(&fp, val);
            // fp.release()
        }
    }
}

#[derive(Debug, Clone, EnumAsInner)]
pub enum States {
    // Value(DecodeResults),
    Value(EntityFieldTypes),
    FieldState(FieldState),
}

#[derive(Debug, Clone)]
pub struct FieldState {
    state: Vec<Option<States>>
}

impl FieldState {
    pub fn new(len: usize) -> Self {
        FieldState {
            state: vec![None; len]
        }
    }

    pub fn get(&self, fp: &FieldPath) -> Option<States> {
        let mut current_state = self.clone();
        let mut z = 0;
        for i in 0..=fp.last {
            z = fp.path[i];
            if (current_state.state.len() as i32) < z + 2 {
                return None
            }
            if i == fp.last {
                return current_state.state[z as usize].clone();
            }
            current_state = match &current_state.state[z as usize].as_ref().unwrap() {
                States::FieldState(state) => { state.clone() },
                _ => {
                    return None
                }
            };
        }
        return None
    }

    // pub fn set(&mut self, fp: &FieldPath, v: DecodeResults) {
    pub fn set(&mut self, fp: &FieldPath, v: EntityFieldTypes) {
        let mut x = self;
        for i in 0..=fp.last {
            let z = fp.path[i];
            let y = x.state.len() as i32;
            if y < z + 2 {
                let m = max(z + 2, y * 2);
                x.state.resize_with(m as usize, || None);
            }
            if i == fp.last {
                if x.state[z as usize].as_ref().is_none() {
                    x.state[z as usize] = Some(States::Value(v.clone()));
                    return
                }
                match x.state[z as usize].as_ref().unwrap(){
                    States::FieldState(_) => {},
                    _ => {
                        x.state[z as usize] = Some(States::Value(v.clone()));
                    }
                }
                return
            }


            if x.state[z as usize].is_none() {
                x.state[z as usize] = Some(States::FieldState(FieldState::new(8)));
            } else {
                match x.state[z as usize].as_ref().unwrap() {
                    States::FieldState(_) => {},
                    _ => {
                        x.state[z as usize] = Some(States::FieldState(FieldState::new(8)));
                    }
                }
            }


            x = match x.state[z as usize].as_mut().unwrap() {
                States::FieldState(state) => state,
                _ => panic!()
            }
            // self.state = &self.state[z as usize].clone();
        }
    }

    // pub fn set(&mut self, fp: &FieldPath, v: DecodeResults) {
    //     let mut current_state = self;
    //     let mut z = 0;
    //     for i in 0..=fp.last {
    //         z = fp.path[i];
    //         if (current_state.state.len() as i32) < z + 2 {
    //             // self.state = vec![States::None; max(z + 2, self.state.len() as i32) as usize];
    //             current_state.state.extend((0..max(z + 2, (current_state.state.len() * 2) as i32)).map(|_| States::None))
    //         }
    //         if i == fp.last {
    //             match &mut current_state.state[z as usize] {
    //                 States::FieldState(_) => { },
    //                 States::Value(x) => {
    //                     current_state.state[z as usize] = States::Value(v);
    //                     // *x = v;
    //                 }
    //                 _ => {}
    //             }
    //             return;
    //         }
    //         current_state = match &mut current_state.state[z as usize] {
    //             States::FieldState(state) => {
    //                 state
    //             },
    //             others => {
    //                 *others = States::FieldState(FieldState::new(8));
    //                 match others {
    //                     States::FieldState(s) => {
    //                         s
    //                     }
    //                     _ => { panic!("") }
    //                 }
    //             }
    //         };
    //     }
    // }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub parent: Option<String>,
    pub var_name: String,
    pub var_type: String,
    pub send_node: String,
    pub serializer_name: String,
    pub serializer_ver: i32,
    pub encoder: String,
    pub encoder_flags: Option<i32>,
    pub bit_count: Option<i32>,
    pub low_value: Option<f32>,
    pub high_value: Option<f32>,
    pub field_type: Option<FieldType>,
    pub serializer: Option<Rc<RefCell<Serializer>>>,
    pub value: Option<i32>,
    pub model: FieldModels,

    pub decoder: Option<Decoders>,
    pub base_decoder: Option<Decoders>,
    pub child_decoder: Option<Decoders>
}

impl Field {
    pub fn new(ser: CSVCMsg_FlattenedSerializer, f: ProtoFlattenedSerializerField_t) -> Self {
        let resolve = |p: Option<i32>| -> String {
            if p.is_none() {
                return "".to_string()
            }
            ser.symbols.get(p.unwrap() as usize).cloned().unwrap_or(String::new())
        };
        let mut send_node = resolve(f.send_node_sym);
        if send_node == "(root)" {
            send_node = String::new();
        }
        // if resolve(f.var_name_sym()) == "m_flAnimTime" {
        //     println!("{}", resolve(f.field_serializer_name_sym()));
        //     panic!();
        // }

        Field {
            parent: None,
            var_name: resolve(f.var_name_sym),
            var_type: resolve(f.var_type_sym),
            send_node,
            serializer_name: resolve(f.field_serializer_name_sym),
            serializer_ver: f.field_serializer_version(),
            encoder: resolve(f.var_encoder_sym),
            encoder_flags: f.encode_flags,
            bit_count: f.bit_count,
            low_value: f.low_value,
            high_value: f.high_value,
            field_type: None,
            serializer: None,
            value: None,
            model: FieldModels::Simple,

            decoder: Some(Decoders::Default),
            base_decoder: Some(Decoders::Default),
            child_decoder: Some(Decoders::Default),
        }
    }

    pub fn get_name_for_field_path(&self, fp: &FieldPath, pos: i32) -> Vec<String> {
        let mut x = vec![self.var_name.clone()];
        match self.model {
            FieldModels::Simple => {}
            FieldModels::FixedArray => {
                if fp.last == pos as usize {
                    x.push(format!("{:04}", fp.path[pos as usize]));
                }
            }
            FieldModels::FixedTable => {
                if fp.last >= pos as usize {
                    x.extend_from_slice(&self.serializer.as_ref().unwrap().borrow().get_name_for_field_path(fp, pos));
                }
            }
            FieldModels::VariableArray => {
                if fp.last == pos as usize {
                    x.push(format!("{:04}", fp.path[pos as usize]));
                }
            }
            FieldModels::VariableTable => {
                if fp.last != (pos - 1) as usize {
                    x.push(format!("{:04}", fp.path[pos as usize]));
                    if fp.last != pos as usize {
                        x.extend_from_slice(&self.serializer.as_ref().unwrap().borrow().get_name_for_field_path(fp, pos + 1))
                    }
                }
            }
        };
        x
    }

    pub fn get_field_for_field_path(&self, fp: &FieldPath, pos: i32) -> Field {
        match self.model {
            FieldModels::FixedTable => {
                if fp.last as i32 != pos - 1 {
                    return self.serializer.as_ref().unwrap().borrow().get_field_for_field_path(fp, pos);
                }
                self.clone()
            }
            FieldModels::VariableTable => {
                if fp.last as i32 >= pos + 1 {
                    return self.serializer.as_ref().unwrap().borrow().get_field_for_field_path(fp, pos+1);
                }
                return self.clone()
            }
            _ => return self.clone()
        }
    }

    pub fn get_type_for_field_path(&self, fp: &FieldPath, pos: i32) -> FieldType {
        match self.model {
            FieldModels::Simple => {}
            FieldModels::FixedArray => {
                return self.field_type.as_ref().unwrap().clone()
            }
            FieldModels::FixedTable => {
                if fp.last as i32 != pos - 1 {
                    return self.serializer.as_ref().unwrap().borrow().get_type_for_field_path(fp, pos);
                }
            }
            FieldModels::VariableArray => {
                if fp.last as i32 == pos {
                    return *self.field_type.as_ref().unwrap().generic.as_ref().unwrap().clone();
                }
            }
            FieldModels::VariableTable => {
                if fp.last as i32 >= pos + 1 {
                    return self.serializer.as_ref().unwrap().borrow().get_type_for_field_path(fp, pos + 1);
                }
            }
        };
        self.field_type.as_ref().unwrap().clone()
    }

    pub fn get_decoder_for_field_path(&self, fp: &FieldPath, pos: i32) -> Decoders {
        match self.model {
            FieldModels::Simple => {}
            FieldModels::FixedArray => {
                return self.decoder.as_ref().unwrap().clone();
            }
            FieldModels::FixedTable => {
                if fp.last as i32 == pos - 1{
                    return self.base_decoder.as_ref().unwrap().clone();
                }
                return self.serializer.as_ref().unwrap().borrow().get_decoder_for_field_path(fp, pos)
            }
            FieldModels::VariableArray => {
                if fp.last as i32 == pos {
                    return self.child_decoder.as_ref().unwrap().clone();
                }
                return self.base_decoder.as_ref().unwrap().clone();
            }
            FieldModels::VariableTable => {
                if fp.last as i32 >= pos + 1 {
                    return self.serializer.as_ref().unwrap().borrow().get_decoder_for_field_path(fp, pos+1);
                }
                return self.base_decoder.as_ref().unwrap().clone();
            }
        }
        // println!("{:?}", self.decoder);
        // println!("{:?}", self);
        self.decoder.as_ref().unwrap().clone()
    }

    pub fn get_field_path_for_name(&self, fp: &mut FieldPath, name: String) -> bool {
        match self.model {
            FieldModels::Simple => {
                panic!("not supported")
            }
            FieldModels::FixedArray => {
                if name.len() != 4 { panic!("wrong size") }
                fp.path[fp.last] = name.parse::<i32>().unwrap();
                return true;
            }
            FieldModels::FixedTable => {
                return self.serializer.as_ref().unwrap().borrow().get_field_path_for_name(fp, &name);
            }
            FieldModels::VariableArray => {
                if name.len() != 4 { panic!("wrong size") }
                fp.path[fp.last] = name.parse::<i32>().unwrap();
            }
            FieldModels::VariableTable => {
                if name.len() != 6 { panic!("wrong size") }
                fp.path[fp.last] = name[0..4].parse::<i32>().unwrap();
                fp.last += 1;
                return self.serializer.as_ref().unwrap().borrow().get_field_path_for_name(fp, &name[5..].to_string())
            }
        }
        false
    }

    pub fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldState) -> Vec<FieldPath> {
        let mut vec: Vec<FieldPath> = vec![];

        // println!("{:?}", st.get(fp));
        // println!("{:?}", self.model);

        match self.model {
            FieldModels::Simple => {
                vec.push(fp.clone());
            }
            FieldModels::FixedArray => {
                // println!("{:?}", fp);
                if let Some(x) = st.get(fp) {
                    match x {
                        States::FieldState(s) => {
                            fp.last += 1;
                            for (i, v) in s.state.iter().enumerate() {
                                if v.is_some() {
                                    fp.path[fp.last] = i as i32;
                                    vec.push(fp.clone());
                                }
                            }
                            fp.last -=1;
                        }
                        _ => {}
                    }
                }
            }
            FieldModels::FixedTable => {
                if let Some(x) = st.get(fp) {
                    match x {
                        States::FieldState(v) => {
                            // println!("{:?}", &v);
                            fp.last += 1;
                            vec.extend_from_slice(&self.serializer.as_ref().unwrap().borrow().get_field_paths(fp, &v));
                            // println!("{:?}", vec);
                            fp.last -= 1;
                        }
                        _ => {}
                    }
                }
                // vec.extend_from_slice(&self.serializer.as_ref().unwrap().borrow().get_field_paths(fp, st));
            }
            FieldModels::VariableArray => {
                match st.get(fp) {
                    Some(sub) => {
                        match sub {
                            States::FieldState(x) => {
                                fp.last += 1;
                                for (i, v) in x.state.iter().enumerate() {
                                    fp.path[fp.last] = i as i32;
                                    vec.push(fp.clone());
                                }
                                fp.last -= 1;
                            }
                            _ => {}
                        }
                    }
                    None => {}
                }
            }
            FieldModels::VariableTable => {
                if let Some(sub) = st.get(fp) {
                    match sub {
                        States::FieldState(x) => {
                            fp.last += 2;
                            for (i, v) in x.state.iter().enumerate() {
                                if v.is_some() {
                                    match v.as_ref().unwrap().clone() {
                                        States::FieldState(vv) => {
                                            fp.path[fp.last - 1] = i as i32;
                                            vec.extend_from_slice(&self.serializer.as_ref().unwrap().borrow().get_field_paths(fp, &vv));
                                            // println!("{:?}", self.model);
                                            // println!("{:?}", vec);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            fp.last -= 2;
                        }
                        _ => {}
                    }
                }
                // match st.get(fp) {
                //     Some(sub) => {
                //         match sub {
                //             States::FieldState(x) => {
                //                 fp.last += 2;
                //                 for (i, v) in x.state.iter().enumerate() {
                //                     if v.as_ref().is_some() {
                //                         match v.as_ref().unwrap() {
                //                             States::FieldState(vv) => {
                //                                 fp.path[fp.last - 1] = i as i32;
                //                                 vec.extend_from_slice(&self.serializer.as_ref().unwrap().borrow().get_field_paths(fp, &vv));
                //                             }
                //                             _ => {}
                //                         }
                //                     }
                //                 }
                //                 fp.last -= 2;
                //             }
                //             _ => {}
                //         }
                //     }
                //     None => {}
                // }
            }
        }
        vec
    }

    pub fn set_model(&mut self, model: FieldModels) {
        self.model = model.clone();
        let temp_decoder= Decoders::from_field(self, false);
        match model {
            FieldModels::FixedArray => {
                self.decoder = Some(temp_decoder);
            }
            FieldModels::FixedTable => {
                self.base_decoder = Some(Decoders::Boolean)
            }
            FieldModels::VariableArray => {
                if self.field_type.as_ref().unwrap().generic.is_none() {
                    panic!("No generic")
                }
                self.base_decoder = Some(Decoders::Unsigned);
                self.child_decoder = Some(Decoders::from_field(self, true))
            }
            FieldModels::VariableTable => {
                self.base_decoder = Some(Decoders::Unsigned);
            }
            FieldModels::Simple => {
                self.decoder = Some(temp_decoder);
            }
        }
    }

    pub fn get_name(&self) -> &str {
        &self.var_name
    }
}

#[derive(Clone, Debug)]
pub enum FieldModels {
    Simple = 0,
    FixedArray = 1,
    FixedTable = 2,
    VariableArray = 3,
    VariableTable = 4
}

impl FieldModels {
    pub fn as_string(&self) -> &str {
        match &self {
            FieldModels::Simple => "fixed-array",
            FieldModels::FixedArray => "fixed-table",
            FieldModels::FixedTable => "variable-array",
            FieldModels::VariableArray => "variable-table",
            FieldModels::VariableTable => "simple"
        }
    }
}


#[derive(Clone, Debug)]
pub enum Decoders {
    VectorNormal,
    Fixed64,
    Handle,
    Boolean,
    String,
    Default,
    Signed,
    FloatCoordinate,
    NoScale,
    RuneTime,
    SimulationTime,
    Unsigned,
    Component,

    Vector(FieldValues, u8),
    Unsigned64(FieldValues),
    Float32(FieldValues),
    QuantizedFloat(FieldValues),
    QAngle(FieldValues),
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum DecodeResults {
//     VectorNormal(Vec<f32>),
//     Fixed64(u64),
//     Handle(u64),
//     Boolean(bool),
//     String(String),
//     Default(u32),
//     Signed(i64),
//     FloatCoordinate(f32),
//     NoScale(f32),
//     RuneTime(f32),
//     SimulationTime(f32),
//     Unsigned(u64),
//     Component(u32),
//     Vector(Vec<f32>),
//     QAngle(Vec<f32>),
//     QuantizedFloat(f32)
// }

#[derive(Debug, Clone, PartialEq, EnumAsInner)]
pub enum EntityFieldTypes {
    Boolean(bool),
    String(String),
    Signed(i64),
    Unsigned(u64),
    Float(f32),
    Vector2D([f32; 2]),
    Vector3D([f32; 3]),
    Vector4D([f32; 4]),
}

#[derive(Clone, Debug)]
pub struct FieldValues {
    encoder: String,
    encoder_flags: Option<i32>,
    bit_count: Option<i32>,
    low_value: Option<f32>,
    high_value: Option<f32>
}

impl Decoders {
    pub fn from_field(field: &Field, by_base: bool) -> Self {
        let field_values = FieldValues {
            encoder: field.encoder.clone(),
            encoder_flags: field.encoder_flags,
            bit_count: field.bit_count,
            high_value: field.high_value,
            low_value: field.low_value,
        };

        let match_var = match  by_base {
            true => field.field_type.as_ref().unwrap().generic.as_ref().unwrap().base.as_str(),
            false => field.field_type.as_ref().unwrap().base.as_str()
        };

        match match_var {
            "bool" =>                                                       Decoders::Boolean,
            "char" | "CUtlString" | "CUtlSymbolLarge" =>                    Decoders::String,
            "int8" | "int16" | "int32" | "int64" =>                         Decoders::Signed,
            "uint8" | "uint16" | "uint32" | "color32" |
            "CGameSceneNodeHandle" | "Color" | "CUtlStringToken" |
            "CHandle" | "CEntityHandle" =>                                  Decoders::Unsigned,
            "GameTime_t" =>                                                 Decoders::NoScale,
            "CBodyComponent" | "CPhysicsComponent" | "CRenderComponent" =>  Decoders::Component,

            "CNetworkedQuantizedFloat" => Decoders::QuantizedFloat(field_values),

            "float32" => Decoders::Float32(field_values),

            "Vector" => Decoders::Vector(field_values, 3),
            "Vector2D" => Decoders::Vector(field_values, 2),
            "Vector4D" => Decoders::Vector(field_values, 4),

            "uint64" | "CStrongHandle" => Decoders::Unsigned64(field_values),

            "QAngle" => Decoders::QAngle(field_values),

            _ => { Decoders::Default }
        }
    }

    pub fn decode(&self, reader: &mut Reader) -> EntityFieldTypes {
        match self {
            Decoders::VectorNormal => {
                // DecodeResults::VectorNormal(reader.read_3bit_normal())
                // Rc::new(reader.read_3bit_normal())
                EntityFieldTypes::Vector3D(reader.read_3bit_normal())
            }
            Decoders::Fixed64 => {
                EntityFieldTypes::Unsigned(reader.read_le_u64())
                // Rc::new(reader.read_le_u64())
            }
            Decoders::Handle => {
                EntityFieldTypes::Unsigned(reader.read_var_u32() as u64)
                // Rc::new(reader.read_var_u32() as u64)
            }
            Decoders::Boolean => {
                EntityFieldTypes::Boolean(reader.read_bool())
                // Rc::new(reader.read_bool())
            }
            Decoders::String => {
                EntityFieldTypes::String(reader.read_string().unwrap())
                // Rc::new(reader.read_string().unwrap())
            }
            Decoders::Default => {
                EntityFieldTypes::Unsigned(reader.read_var_u32() as u64)
                // Rc::new(reader.read_var_u32())
            }
            Decoders::Signed => {
                EntityFieldTypes::Signed(reader.read_var_i32() as i64)
                // Rc::new(reader.read_var_i32())
            }
            Decoders::FloatCoordinate => {
                EntityFieldTypes::Float(reader.read_coordinate())
                // Rc::new(reader.read_coordinate())
            }
            Decoders::NoScale => {
                EntityFieldTypes::Float(f32::from_bits(reader.read_bits(32)))
                // Rc::new(f32::from_bits(reader.read_bits(32)))
            }
            Decoders::RuneTime => {
                EntityFieldTypes::Float(f32::from_bits(reader.read_bits(4)))
                // Rc::new(f32::from_bits(reader.read_bits(4)))
            }
            Decoders::SimulationTime => {
                EntityFieldTypes::Float(reader.read_var_u32() as f32 * (1.0 / 30.0))
                // Rc::new(reader.read_var_u32() as f32 * (1.0 / 30.0))
            }
            Decoders::Unsigned => {
                EntityFieldTypes::Unsigned(reader.read_var_u32() as u64)
                // Rc::new(reader.read_var_u32() as u64)
            }
            Decoders::Component => {
                EntityFieldTypes::Unsigned(reader.read_bits(1) as u64)
                // Rc::new(reader.read_bits(1))
            }
            Decoders::Float32(fv) => {
                match fv.encoder.as_str() {
                    "coord" => { return Decoders::FloatCoordinate.decode(reader); }
                    "simtime" => { return Decoders::SimulationTime.decode(reader); }
                    "runetime" => { return Decoders::RuneTime.decode(reader); }
                    _ => {}
                };
                if fv.bit_count.is_none() || (fv.bit_count.unwrap() <= 0 || fv.bit_count.unwrap() >= 32) {
                    return Decoders::NoScale.decode(reader);
                }
                return Decoders::QuantizedFloat(fv.clone()).decode(reader);
            }
            Decoders::Vector(fv, n) => {
                // if *n == 3 && fv.encoder.as_str() == "normal" {
                //     return Decoders::VectorNormal.decode(reader)
                // }
                if *n == 2 {
                    let mut r = [0.0f32; 2];
                    if let EntityFieldTypes::Float(f) = Decoders::Float32(fv.clone()).decode(reader) { r[0] = f }
                    if let EntityFieldTypes::Float(f) = Decoders::Float32(fv.clone()).decode(reader) { r[1] = f }
                    return EntityFieldTypes::Vector2D(r)
                }
                if *n == 3 {
                    if fv.encoder.as_str() == "normal" {
                        return Decoders::VectorNormal.decode(reader)
                    }
                    let mut r = [0.0f32; 3];
                    if let EntityFieldTypes::Float(f) = Decoders::Float32(fv.clone()).decode(reader) { r[0] = f }
                    if let EntityFieldTypes::Float(f) = Decoders::Float32(fv.clone()).decode(reader) { r[1] = f }
                    if let EntityFieldTypes::Float(f) = Decoders::Float32(fv.clone()).decode(reader) { r[2] = f }
                    return EntityFieldTypes::Vector3D(r)
                }
                if *n == 4 {
                    let mut r = [0.0f32; 4];
                    if let EntityFieldTypes::Float(f) = Decoders::Float32(fv.clone()).decode(reader) { r[0] = f }
                    if let EntityFieldTypes::Float(f) = Decoders::Float32(fv.clone()).decode(reader) { r[1] = f }
                    if let EntityFieldTypes::Float(f) = Decoders::Float32(fv.clone()).decode(reader) { r[2] = f }
                    if let EntityFieldTypes::Float(f) = Decoders::Float32(fv.clone()).decode(reader) { r[2] = f }
                    return EntityFieldTypes::Vector4D(r)
                }
                panic!();
                // let mut r: Vec<f32> = vec![];
                // for _ in 0..*n {
                //     // r.push(*Decoders::Float32(fv.clone()).decode(reader).downcast_ref::<f32>().unwrap())
                //     if let EntityFieldTypes::Float(f) = Decoders::Float32(fv.clone()).decode(reader) {
                //         r.push(f)
                //     }
                // }
                // EntityFieldTypes::Vector(r)
                // // Rc::new(r)
            }
            Decoders::Unsigned64(fv) => {
                if fv.encoder.as_str() == "fixed64" {
                    return Decoders::Fixed64.decode(reader);
                }
                return EntityFieldTypes::Unsigned(reader.read_var_u64());
                // return Rc::new(reader.read_var_u64());
                // Decoders::Unsigned64(fv.clone()).decode(reader)
            }
            Decoders::QuantizedFloat(fv) => {
                let qd = QFloatDecoder::new(fv.bit_count.unwrap(), fv.encoder_flags, fv.low_value, fv.high_value);
                EntityFieldTypes::Float(qd.decode(reader))
                // Rc::new(qd.decode(reader))
            }
            Decoders::QAngle(fv) => {
                if fv.encoder == "qangle_pitch_yaw" {
                    let n = fv.bit_count.unwrap() as u32;
                    return EntityFieldTypes::Vector3D([reader.read_angle(n), reader.read_angle(n), 0.0]);
                    // return Rc::new(vec![reader.read_angle(n), reader.read_angle(n), 0.0]);
                }

                if fv.bit_count.is_some() && fv.bit_count.unwrap() != 0 {
                    let n = fv.bit_count.unwrap() as u32;
                    return EntityFieldTypes::Vector3D([reader.read_angle(n), reader.read_angle(n), reader.read_angle(n)]);
                    // return Rc::new(vec![reader.read_angle(n), reader.read_angle(n), reader.read_angle(n)]);
                }

                let mut v = [0f32; 3];
                let x = reader.read_bool();
                let y = reader.read_bool();
                let z = reader.read_bool();
                if x{
                    v[0] = reader.read_coordinate();
                }
                if y {
                    v[1] = reader.read_coordinate();
                }
                if z {
                    v[2] = reader.read_coordinate();
                }
                EntityFieldTypes::Vector3D(v)
                // Rc::new(v)
            }
        }
    }
}


pub struct FieldPatch {
    min_build: u32,
    max_build: u32,
    pub patch: fn(&mut Field),
}

impl FieldPatch {
    pub fn should_apply(&self, build: u32) -> bool {
        if self.min_build == 0 && self.max_build == 0 {
            true
        } else {
            build >= self.min_build && build <= self.max_build
        }
    }
}

pub static FIELD_PATCHES: [FieldPatch; 4] = [
    FieldPatch {
        min_build: 0,
        max_build: 990,
        patch: |f: &mut Field| match f.var_name.as_str() {
            "angExtraLocalAngles"
            | "angLocalAngles"
            | "m_angInitialAngles"
            | "m_angRotation"
            | "m_ragAngles"
            | "m_vLightDirection" => {
                f.encoder = if f.parent.as_ref().unwrap().as_str() == "CBodyComponentBaseAnimatingOverlay" {
                    "qangle_pitch_yaw".to_string()
                } else {
                    "QAngle".to_string()
                };
            }
            "dirPrimary"
            | "localSound"
            | "m_flElasticity"
            | "m_location"
            | "m_poolOrigin"
            | "m_ragPos"
            | "m_vecEndPos"
            | "m_vecLadderDir"
            | "m_vecPlayerMountPositionBottom"
            | "m_vecPlayerMountPositionTop"
            | "m_viewtarget"
            | "m_WorldMaxs"
            | "m_WorldMins"
            | "origin"
            | "vecLocalOrigin" => {
                f.encoder = "coord".to_string();
            }
            "m_vecLadderNormal" => {
                f.encoder = "normal".to_string();
            }
            _ => {}
        },
    },
    FieldPatch {
        min_build: 0,
        max_build: 954,
        patch: |f: &mut Field| match f.var_name.as_str() {
            "m_flMana" | "m_flMaxMana" => {
                f.low_value = Some(0.0);
                f.high_value = Some(8192.0f32);
            }
            _ => {}
        },
    },
    FieldPatch {
        min_build: 1016,
        max_build: 1027,
        patch: |f: &mut Field| match f.var_name.as_str() {
            "m_bItemWhiteList"
            | "m_bWorldTreeState"
            | "m_iPlayerIDsInControl"
            | "m_iPlayerSteamID"
            | "m_ulTeamBannerLogo"
            | "m_ulTeamBaseLogo"
            | "m_ulTeamLogo" => {
                f.encoder = "fixed64".to_string();
            }
            _ => {}
        },
    },
    FieldPatch {
        min_build: 0,
        max_build: 0,
        patch: |f: &mut Field| match f.var_name.as_str() {
            "m_flSimulationTime" | "m_flAnimTime" => {
                f.encoder = "simtime".to_string();
            }
            "m_flRuneTime" => {
                f.encoder = "runetime".to_string();
            }
            _ => {}
        },
    },
];