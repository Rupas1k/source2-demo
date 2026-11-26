use prost_reflect::{DynamicMessage, Kind, Value};
use serde::Serialize;

#[derive(Serialize)]
pub struct Field {
    name: String,
    kind: String,
    value: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<Field>,
}

impl Field {
    fn new(name: String, kind: String) -> Self {
        Self {
            name,
            kind,
            value: String::new(),
            children: Vec::new(),
        }
    }

    pub fn text(name: &str, kind: &str, value: String) -> Self {
        Self {
            value,
            ..Self::new(name.into(), kind.into())
        }
    }

    pub fn group(name: &str, children: Vec<Self>) -> Self {
        Self {
            children,
            ..Self::new(name.into(), "message".into())
        }
    }

    pub fn bytes(name: &str, len: usize) -> Self {
        Self::text(name, "bytes", format!("{len} bytes"))
    }

    fn from_value(name: String, kind: &Kind, value: &Value) -> Self {
        let label = match kind {
            Kind::Message(msg) => msg.name().to_owned(),
            Kind::Enum(en) => en.name().to_owned(),
            primitive => format!("{primitive:?}").to_lowercase(),
        };
        let mut row = Self::new(name, label);
        row.value = match value {
            Value::Bool(v) => v.to_string(),
            Value::I32(v) => v.to_string(),
            Value::I64(v) => v.to_string(),
            Value::U32(v) => v.to_string(),
            Value::U64(v) => v.to_string(),
            Value::F32(v) => v.to_string(),
            Value::F64(v) => v.to_string(),
            Value::String(v) => v.clone(),
            Value::EnumNumber(v) => {
                if let Kind::Enum(en) = kind {
                    if let Some(item) = en.get_value(*v) {
                        return Self {
                            value: format!("{} ({v})", item.name()),
                            ..row
                        };
                    }
                }
                v.to_string()
            }
            Value::Bytes(v) => format!("{} bytes", v.len()),
            Value::Message(v) => {
                row.children = fields(v);
                String::new()
            }
            Value::List(list) => {
                row.children = list
                    .iter()
                    .enumerate()
                    .map(|(i, v)| Self::from_value(i.to_string(), kind, v))
                    .collect();
                format!("{} items", list.len())
            }
            Value::Map(map) => {
                if let Kind::Message(entry) = kind {
                    let item = entry.map_entry_value_field().kind();
                    row.children = map
                        .iter()
                        .map(|(k, v)| Self::from_value(format!("{k:?}"), &item, v))
                        .collect();
                    row.children.sort_by(|a, b| a.name.cmp(&b.name));
                }
                format!("{} items", map.len())
            }
        };
        row
    }
}

pub fn fields(message: &DynamicMessage) -> Vec<Field> {
    let mut rows: Vec<_> = message
        .fields()
        .map(|(f, v)| Field::from_value(f.name().into(), &f.kind(), v))
        .collect();
    for unknown in message.unknown_fields() {
        rows.push(Field::bytes(
            &format!("Unknown field {}", unknown.number()),
            unknown.encoded_len(),
        ));
    }
    rows
}
