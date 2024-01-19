use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct FieldType {
    pub base: String,
    pub generic: Option<Box<FieldType>>,
    pub pointer: bool,
    pub count: Option<i32>,
}

// I've seen implementation of field types by dumping all of them as store as static file (butterfly)
// I see no reason of doing that because time overhead of regex is about 4ms for all entries.

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
        let generic = captures.get(3).map(|v| Box::new(FieldType::new(v.as_str())));
        // let generic = match &captures.get(3) {
        //     None => None,
        //     Some(v) => Some(Box::new(FieldType::new(v.as_str()))),
        // };

        let mut item_counts: FxHashMap<&str, i32> = FxHashMap::default();
        item_counts.insert("MAX_ITEM_STOCKS", 8);
        item_counts.insert("MAX_ABILITY_DRAFT_ABILITIES", 48);

        // had fun with this one
        let count = captures
            .get(6)
            .and_then(|x| item_counts
                .get(x.as_str())
                .copied()
                .or_else(|| x
                    .as_str()
                    .parse()
                    .ok()));
        // let count = if let Some(x) = captures.get(6) {
        //     if let Some(&y) = item_counts.get(x.as_str()) {
        //         Some(y as u32)
        //     } else {
        //         Some(x.as_str().parse::<u32>().unwrap())
        //     }
        // } else {
        //     None
        // };
        
        
        // println!("{} {}", name, FieldType {
        //     base: base.clone(),
        //     generic: generic.clone(),
        //     pointer: pointer.clone(),
        //     count: count.clone(),
        // }.as_str());
        
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
            s += &format!("< {} >", &generic_type.as_str());
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