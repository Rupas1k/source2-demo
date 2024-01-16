use std::collections::HashMap;
use regex::Regex;

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