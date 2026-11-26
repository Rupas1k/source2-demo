use crate::entity::field::{FieldPath, FieldState, FieldValue, Serializer};

/// A read-only entity field entry for inspection UIs and debugging tools.
pub struct EntityField<'a> {
    /// Numeric field path used by the demo stream.
    pub path: Vec<u16>,
    /// Human-readable dotted field path.
    pub name: String,
    /// Source 2 field type reported by the serializer.
    pub field_type: String,
    /// Decoded value type used by `source2-demo`.
    pub decoded_type: Option<&'static str>,
    /// Current field value, if the field is present in the state.
    pub value: Option<&'a FieldValue>,
}

/// One field write from an entity creation or update, in wire order.
#[derive(Debug)]
pub struct EntityFieldUpdate {
    /// Numeric serializer path.
    pub path: Vec<u16>,
    /// Dotted serializer path.
    pub name: String,
    /// Source 2 serializer type.
    pub field_type: String,
    /// Value immediately before this write, including inherited baseline values.
    pub before: Option<FieldValue>,
    /// Value decoded from the packet.
    pub after: FieldValue,
}

pub(crate) fn inspect_fields<'a>(
    serializer: &Serializer,
    state: &'a FieldState,
) -> Vec<EntityField<'a>> {
    serializer
        .get_paths(&mut FieldPath::default(), state)
        .into_iter()
        .map(|fp| {
            let value = state.get_value(&fp);
            EntityField {
                path: (0..=fp.last).map(|idx| fp.path[idx]).collect(),
                name: serializer.get_name(&fp).to_string(),
                field_type: serializer.get_type(&fp).to_string(),
                decoded_type: value.map(FieldValue::type_name),
                value,
            }
        })
        .collect()
}

/// A baseline entity entry keyed by class id.
pub struct BaselineEntity {
    /// Entity class id used to look up the baseline.
    pub class_id: i32,
    /// Human-readable entity class name.
    pub class_name: String,
}
