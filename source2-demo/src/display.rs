use crate::entity::field::*;
use crate::entity::*;
use crate::event::*;
use crate::parser::Context;
use crate::string_table::*;
use prettytable::{row, Table};
use std::fmt::{Display, Formatter};

impl Display for Classes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["id", "name"]);
        for class in self.classes_vec.iter() {
            table.add_row(row![class.id().to_string(), class.name]);
        }
        write!(f, "{}", table)
    }
}

impl Display for FieldPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..=self.last {
            write!(f, "{}", self.path[i])?;
            if i != self.last {
                write!(f, "/")?;
            }
        }
        Ok(())
    }
}

impl Display for StringTables {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["id", "name", "rows"]);
        for string_table in self.iter() {
            table.add_row(row![
                string_table.index.to_string(),
                string_table.name,
                string_table.items.len()
            ]);
        }
        write!(f, "{}", table)
    }
}

impl Display for StringTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["idx", "key", "value"]);
        for entry in self.items.iter() {
            table.add_row(row![
                entry.index,
                entry.key,
                format!(
                    "{:?}...",
                    entry
                        .value
                        .as_ref()
                        .map(|x| if x.len() > 10 { &x[..10] } else { &x[..] })
                )
            ]);
        }
        write!(f, "{}", table)
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["Classes", self.classes.classes_vec.len()]);
        table.add_row(row![
            "Entities",
            self.entities
                .entities_vec
                .iter()
                .flatten()
                .collect::<Vec<_>>()
                .len()
        ]);
        table.add_row(row!["String Tables", self.string_tables.tables.len()]);
        table.add_row(row!["Tick", self.tick]);
        table.add_row(row!["Net Tick", self.net_tick]);
        table.add_row(row!["Game Build", format!("{:?}", self.game_build)]);
        write!(f, "{}", table)
    }
}

impl Display for Entities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["idx", "serial", "handle", "class"]);
        for e in self.entities_vec.iter().flatten() {
            table.add_row(row![
                e.index().to_string(),
                e.serial().to_string(),
                e.handle().to_string(),
                e.class().name(),
            ]);
        }
        write!(f, "{}", table)
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();

        table.add_row(row!["#", "Field", "Type", "Value"]);

        for fp in self
            .class
            .serializer
            .get_field_paths(&mut FieldPath::default(), &self.state)
        {
            let field_type = self.class.serializer.get_type_for_field_path(&fp);
            let name = self.class.serializer.get_name_for_field_path(&fp);
            let value = self.state.get_value(&fp);
            if let Some(v) = value {
                table.add_row(row![fp, name, field_type.as_string(), format!("{:?}", v)]);
            } else {
                table.add_row(row![fp, name, field_type.as_string(), "None"]);
            }
        }

        write!(f, "{}", table)
    }
}

impl Display for FieldValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldValue::Boolean(val) => write!(f, "{}", val),
            FieldValue::String(val) => write!(f, "\"{}\"", val),
            FieldValue::Float(val) => write!(f, "{}", val),
            FieldValue::Vector2D(val) => write!(f, "[{}, {}]", val[0], val[1]),
            FieldValue::Vector3D(val) => write!(f, "[{}, {}, {}]", val[0], val[1], val[2]),
            FieldValue::Vector4D(val) => {
                write!(f, "[{}, {}, {}, {}]", val[0], val[1], val[2], val[3])
            }
            FieldValue::Signed8(val) => write!(f, "{}", val),
            FieldValue::Signed16(val) => write!(f, "{}", val),
            FieldValue::Signed32(val) => write!(f, "{}", val),
            FieldValue::Signed64(val) => write!(f, "{}", val),
            FieldValue::Unsigned8(val) => write!(f, "{}", val),
            FieldValue::Unsigned16(val) => write!(f, "{}", val),
            FieldValue::Unsigned32(val) => write!(f, "{}", val),
            FieldValue::Unsigned64(val) => write!(f, "{}", val),
        }
    }
}

impl Display for GameEvent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();

        table.add_row(row!["Key", "Value"]);

        for (key, value) in self.iter() {
            table.add_row(row![key, format!("{:?}", value)]);
        }

        write!(f, "{}", table)
    }
}

impl Display for GameEventList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();

        for (id, definition) in self
            .list
            .iter()
            .collect::<std::collections::BTreeMap<_, _>>()
        {
            table.add_row(row![id, definition.name]);
        }

        write!(f, "{}", table)
    }
}
