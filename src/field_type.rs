use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::fmt::{Display, Formatter};

lazy_static! {
    static ref RE: Regex = Regex::new(r"([^<\[*]+)(<\s(.*)\s>)?(\*)?(\[(.*)])?").unwrap();
    static ref COUNT_TYPES: FxHashMap<&'static str, i32> =
        [("MAX_ITEM_STOCKS", 8), ("MAX_ABILITY_DRAFT_ABILITIES", 48)]
            .iter()
            .copied()
            .collect();
}

#[derive(Clone, Debug)]
pub struct FieldType {
    pub base: Box<str>,
    pub generic: Option<Box<FieldType>>,
    pub pointer: bool,
    pub count: Option<i32>,
}


impl FieldType {
    pub fn new(name: &str) -> Self {
        let captures = RE.captures(name).unwrap();

        let base = captures[1].to_string().into_boxed_str();
        let pointer = captures.get(4).is_some();
        let generic = captures
            .get(3)
            .map(|v| Box::new(FieldType::new(v.as_str())));

        let count = captures.get(6).and_then(|x| {
            COUNT_TYPES
                .get(x.as_str())
                .copied()
                .or_else(|| x.as_str().parse().ok())
        });

        FieldType {
            base,
            generic,
            pointer,
            count,
        }
    }
}

// impl Display for FieldType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut s = self.base.clone();
//
//         if let Some(generic_type) = &self.generic {
//             s += &format!("< {} >", &generic_type.as_ref());
//         }
//
//         if self.pointer {
//             s += "*";
//         }
//
//         if self.count.is_some() && self.count.unwrap() > 0 {
//             s += &format!("[{}]", self.count.unwrap());
//         }
//
//         write!(f, "{}", s)
//     }
// }
