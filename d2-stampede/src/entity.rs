use crate::class::Class;
use crate::field::{FieldPath, FieldState};
use crate::field_value::FieldValue;
use anyhow::{anyhow, Context, Result};
use nohash_hasher::IntMap;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EntityEvent {
    Created = 1 << 0,
    Updated = 1 << 1,
    Deleted = 1 << 2,
    Entered = 1 << 3,
    Left = 1 << 4,
}

pub struct Entities {
    pub(crate) entity_full_packets: u32,
    pub(crate) undone_entities: VecDeque<(i32, isize)>,
    pub(crate) index_to_entity: IntMap<i32, Entity>,
}

impl Entities {
    pub(crate) fn new() -> Self {
        Entities {
            index_to_entity: IntMap::default(),
            undone_entities: VecDeque::new(),
            entity_full_packets: 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.index_to_entity.values()
    }

    pub fn get_by_index(&self, index: &i32) -> Result<&Entity> {
        self.index_to_entity
            .get(index)
            .with_context(|| anyhow!("No entities for index \"{index}\""))
    }
    pub fn get_by_handle(&self, handle: &i32) -> Result<&Entity> {
        self.get_by_index(&(handle & 0x3fff))
            .with_context(|| anyhow!("No entities for handle \"{handle}\""))
    }
    pub fn get_by_class_id(&self, id: &i32) -> Result<&Entity> {
        self.index_to_entity
            .values()
            .find(|&entity| &entity.class().id() == id)
            .with_context(|| anyhow!("No entities for class with id {id}"))
    }
    pub fn get_by_class_name(&self, name: &str) -> Result<&Entity> {
        self.index_to_entity
            .values()
            .find(|&entity| entity.class().name() == name)
            .with_context(|| anyhow!("No entities for class with name {name}"))
    }

    pub fn get_all_by_class_id<'a>(&'a self, id: &'a i32) -> impl Iterator<Item = &Entity> {
        self.index_to_entity
            .values()
            .filter(|&entity| entity.class().id() == *id)
    }

    pub fn get_all_by_class_name<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &Entity> {
        self.index_to_entity
            .values()
            .filter(move |&entity| entity.class().name() == name)
    }
}

#[derive(Clone)]
pub struct Entity {
    pub(crate) index: i32,
    pub(crate) serial: i32,
    pub(crate) class: Rc<Class>,
    pub(crate) active: bool,
    pub(crate) state: FieldState,
}

impl Entity {
    pub(crate) fn new(index: i32, serial: i32, class: Rc<Class>) -> Self {
        Entity {
            index,
            serial,
            class,
            active: true,
            state: FieldState::new(16),
        }
    }

    pub fn index(&self) -> i32 {
        self.index
    }
    pub fn serial(&self) -> i32 {
        self.serial
    }
    pub fn handle(&self) -> i32 {
        self.serial << 14 | self.index
    }
    pub fn class(&self) -> &Class {
        &self.class
    }
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_property_by_name(&self, name: &str) -> Result<&FieldValue> {
        self.get_property_by_field_path(
            &self.class.serializer.get_field_path_for_name(name)?, // .with_contextwith_context(|| anyhow!("No property for given name \"{}\"", name))?,
        )
    }

    pub(crate) fn get_property_by_field_path(&self, fp: &FieldPath) -> Result<&FieldValue> {
        self.state
            .get_value(fp)
            .ok_or_else(|| anyhow!("No property for given field path {:?}", fp))
    }
}

// ChatGPT
impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Function to generate a horizontal line
        fn horizontal_line(width1: usize, width2: usize, width3: usize) -> String {
            format!(
                "+{:<width1$}+{:<width2$}+{:<width3$}+\n",
                "-".repeat(width1 + 2),
                "-".repeat(width2 + 2),
                "-".repeat(width3 + 2),
                width1 = width1 + 2,
                width2 = width2 + 2,
                width3 = width3 + 2
            )
        }

        // Function to format a table row
        fn format_row(
            name: &str,
            t: &str,
            value: &str,
            width1: usize,
            width2: usize,
            width3: usize,
        ) -> String {
            format!(
                "| {:<width1$} | {:<width2$} | {:<width3$} |\n",
                name,
                t,
                value,
                width1 = width1,
                width2 = width2,
                width3 = width3
            )
        }

        let mut table = String::new();

        // Calculate column widths based on the longest field name
        let mut name_width = 5; // Minimum width for the "Field" column
        for fp in self
            .class
            .serializer
            .get_field_paths(&mut FieldPath::new(), &self.state)
        {
            let name = self.class.serializer.get_name_for_field_path(&fp);
            name_width = name.len().max(name_width);
        }

        let type_width = 35; // Fixed width for the "Type" column
        let value_width = 35; // Fixed width for the "Value" column

        // Add header row
        table += &horizontal_line(name_width, type_width, value_width);
        table += &format_row(
            "Field",
            "Type",
            "Value",
            name_width,
            type_width,
            value_width,
        );
        table += &horizontal_line(name_width, type_width, value_width);

        // Add rows for each field
        for fp in self
            .class
            .serializer
            .get_field_paths(&mut FieldPath::new(), &self.state)
        {
            let t = self
                .class
                .serializer
                .get_type_for_field_path(&fp)
                .base
                .clone();
            let name = self.class.serializer.get_name_for_field_path(&fp);
            let value = match self.state.get_value(&fp) {
                Some(v) => match t.as_ref() {
                    "bool" => format!("{}", v.as_bool()),
                    "char" | "CUtlString" | "CUtlSymbolLarge" => format!("\"{}\"", v.to_owned()),
                    "int8" | "int16" | "int32" | "int64" => format!("{}", v),
                    "uint8"
                    | "uint16"
                    | "uint32"
                    | "uint64"
                    | "CStrongHandle"
                    | "color32"
                    | "CGameSceneNodeHandle"
                    | "Color"
                    | "CUtlStringToken"
                    | "CHandle"
                    | "CEntityHandle"
                    | "CBodyComponent"
                    | "CPhysicsComponent"
                    | "CRenderComponent" => format!("{}", v),
                    "float32" | "GameTime_t" | "CNetworkedQuantizedFloat" => {
                        format!("{}", v.as_float())
                    }
                    "Vector2D" => format!(
                        "[{}]",
                        v.as_vector2d()
                            .iter()
                            .map(|&x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    ),
                    "Vector3D" | "Vector" | "QAngle" => format!(
                        "[{}]",
                        v.as_vector3d()
                            .iter()
                            .map(|&x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    ),
                    "Vector4D" => format!(
                        "[{}]",
                        v.as_vector4d()
                            .iter()
                            .map(|&x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    ),
                    _ => format!("{}", v),
                },
                _ => "None".to_string(),
            };
            table += &format_row(&name, &t, &value, name_width, type_width, value_width);
        }

        // Add bottom border
        table += &horizontal_line(name_width, type_width, value_width);

        write!(f, "{}", table)?;
        Ok(())
    }
}
