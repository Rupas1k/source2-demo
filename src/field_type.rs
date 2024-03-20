use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct FieldType {
    pub base: String,
    pub generic: Option<Box<FieldType>>,
    pub pointer: bool,
    pub count: Option<i32>,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"([^<\[*]+)(<\s(.*)\s>)?(\*)?(\[(.*)])?").unwrap();
}

impl FieldType {
    pub fn new(name: &str) -> Self {
        let captures = RE.captures(name).unwrap();

        if captures.len() != 7 {
            panic!("Regex error")
        }

        let base = captures[1].to_string();
        let pointer = captures.get(4).is_some() && &captures[4] == "*";
        let generic = captures
            .get(3)
            .map(|v| Box::new(FieldType::new(v.as_str())));

        let mut item_counts: FxHashMap<&str, i32> = FxHashMap::default();
        item_counts.insert("MAX_ITEM_STOCKS", 8);
        item_counts.insert("MAX_ABILITY_DRAFT_ABILITIES", 48);

        let count = captures.get(6).and_then(|x| {
            item_counts
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

impl Display for FieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = self.base.clone();

        if let Some(generic_type) = &self.generic {
            s += &format!("< {} >", &generic_type.as_ref());
        }

        if self.pointer {
            s += "*";
        }

        if self.count.is_some() && self.count.unwrap() > 0 {
            s += &format!("[{}]", self.count.unwrap());
        }

        write!(f, "{}", s)
    }
}
