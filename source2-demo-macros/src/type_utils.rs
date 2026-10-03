use quote::ToTokens;
use syn::Type;

pub(crate) fn stringify_type(ty: &Type) -> String {
    ty.to_token_stream().to_string()
}

pub(crate) fn referenced_or_owned_type(ty: &Type) -> (Type, bool) {
    if let Type::Reference(reference) = ty {
        (*reference.elem.clone(), true)
    } else {
        (ty.clone(), false)
    }
}

pub(crate) fn is_context_type(value: &str) -> bool {
    matches!(value, "& :: source2_demo :: Context" | "& source2_demo :: Context" | "& Context")
}

pub(crate) fn is_entity_type(value: &str) -> bool {
    matches!(value, "& :: source2_demo :: Entity" | "& source2_demo :: Entity" | "& Entity")
}

pub(crate) fn is_entity_events_type(value: &str) -> bool {
    matches!(value, ":: source2_demo :: EntityEvents" | "source2_demo :: EntityEvents" | "EntityEvents")
}

pub(crate) fn is_entity_events_ref_type(value: &str) -> bool {
    matches!(
        value,
        "& :: source2_demo :: EntityEvents" | "& source2_demo :: EntityEvents" | "& EntityEvents"
    )
}

pub(crate) fn is_game_event_type(value: &str) -> bool {
    matches!(value, "& :: source2_demo :: GameEvent" | "& source2_demo :: GameEvent" | "& GameEvent")
}

pub(crate) fn is_string_table_type(value: &str) -> bool {
    matches!(
        value,
        "& :: source2_demo :: StringTable" | "& source2_demo :: StringTable" | "& StringTable"
    )
}

pub(crate) fn is_modified_indices_type(value: &str) -> bool {
    value == "& [i32]"
}

#[cfg(any(feature = "dota", feature = "citadel"))]
pub(crate) fn is_combat_log_type(value: &str) -> bool {
    matches!(
        value,
        "& :: source2_demo :: CombatLogEntry" | "& source2_demo :: CombatLogEntry" | "& CombatLogEntry"
    )
}

pub(crate) fn is_field_value_ref_type(value: &str) -> bool {
    matches!(value, "& :: source2_demo :: FieldValue" | "& source2_demo :: FieldValue" | "& FieldValue")
}

pub(crate) fn is_field_value_type(value: &str) -> bool {
    matches!(value, ":: source2_demo :: FieldValue" | "source2_demo :: FieldValue" | "FieldValue")
}

pub(crate) fn is_raw_field_value_type(value: &str) -> bool {
    is_field_value_ref_type(value) || is_field_value_type(value)
}

pub(crate) fn is_demo_command_type(value: &str) -> bool {
    matches!(
        value,
        ":: source2_demo :: proto :: EDemoCommands" | "source2_demo :: proto :: EDemoCommands" | "EDemoCommands"
    )
}

pub(crate) fn is_message_type(value: &str) -> bool {
    matches!(
        value,
        ":: source2_demo :: proto :: EDemoCommands"
            | "source2_demo :: proto :: EDemoCommands"
            | "EDemoCommands"
            | ":: source2_demo :: proto :: EBaseUserMessages"
            | "source2_demo :: proto :: EBaseUserMessages"
            | "EBaseUserMessages"
            | ":: source2_demo :: proto :: EBaseGameEvents"
            | "source2_demo :: proto :: EBaseGameEvents"
            | "EBaseGameEvents"
            | ":: source2_demo :: proto :: SvcMessages"
            | "source2_demo :: proto :: SvcMessages"
            | "SvcMessages"
            | ":: source2_demo :: proto :: NetMessages"
            | "source2_demo :: proto :: NetMessages"
            | "NetMessages"
            | ":: source2_demo :: proto :: EDotaUserMessages"
            | "source2_demo :: proto :: EDotaUserMessages"
            | "EDotaUserMessages"
            | ":: source2_demo :: proto :: CitadelUserMessageIds"
            | "source2_demo :: proto :: CitadelUserMessageIds"
            | "CitadelUserMessageIds"
            | ":: source2_demo :: proto :: ECitadelGameEvents"
            | "source2_demo :: proto :: ECitadelGameEvents"
            | "ECitadelGameEvents"
            | ":: source2_demo :: proto :: ECstrike15UserMessages"
            | "source2_demo :: proto :: ECstrike15UserMessages"
            | "ECstrike15UserMessages"
            | ":: source2_demo :: proto :: ECsgoGameEvents"
            | "source2_demo :: proto :: ECsgoGameEvents"
            | "ECsgoGameEvents"
    )
}

pub(crate) fn is_message_type_ref(value: &str) -> bool {
    matches!(
        value,
        "& :: source2_demo :: proto :: EDemoCommands"
            | "& source2_demo :: proto :: EDemoCommands"
            | "& EDemoCommands"
            | "& :: source2_demo :: proto :: EBaseUserMessages"
            | "& source2_demo :: proto :: EBaseUserMessages"
            | "& EBaseUserMessages"
            | "& :: source2_demo :: proto :: EBaseGameEvents"
            | "& source2_demo :: proto :: EBaseGameEvents"
            | "& EBaseGameEvents"
            | "& :: source2_demo :: proto :: SvcMessages"
            | "& source2_demo :: proto :: SvcMessages"
            | "& SvcMessages"
            | "& :: source2_demo :: proto :: NetMessages"
            | "& source2_demo :: proto :: NetMessages"
            | "& NetMessages"
            | "& :: source2_demo :: proto :: EDotaUserMessages"
            | "& source2_demo :: proto :: EDotaUserMessages"
            | "& EDotaUserMessages"
            | "& :: source2_demo :: proto :: CitadelUserMessageIds"
            | "& source2_demo :: proto :: CitadelUserMessageIds"
            | "& CitadelUserMessageIds"
            | "& :: source2_demo :: proto :: ECitadelGameEvents"
            | "& source2_demo :: proto :: ECitadelGameEvents"
            | "& ECitadelGameEvents"
            | "& :: source2_demo :: proto :: ECstrike15UserMessages"
            | "& source2_demo :: proto :: ECstrike15UserMessages"
            | "& ECstrike15UserMessages"
            | "& :: source2_demo :: proto :: ECsgoGameEvents"
            | "& source2_demo :: proto :: ECsgoGameEvents"
            | "& ECsgoGameEvents"
    )
}

pub(crate) fn canonical_message_type(value: &str) -> Option<String> {
    if matches!(
        value,
        ":: source2_demo :: proto :: EDemoCommands" | "source2_demo :: proto :: EDemoCommands" | "EDemoCommands"
    ) {
        Some("EDemoCommands".to_string())
    } else if matches!(
        value,
        ":: source2_demo :: proto :: EBaseUserMessages" | "source2_demo :: proto :: EBaseUserMessages" | "EBaseUserMessages"
    ) {
        Some("EBaseUserMessages".to_string())
    } else if matches!(
        value,
        ":: source2_demo :: proto :: EBaseGameEvents" | "source2_demo :: proto :: EBaseGameEvents" | "EBaseGameEvents"
    ) {
        Some("EBaseGameEvents".to_string())
    } else if matches!(
        value,
        ":: source2_demo :: proto :: SvcMessages" | "source2_demo :: proto :: SvcMessages" | "SvcMessages"
    ) {
        Some("SvcMessages".to_string())
    } else if matches!(
        value,
        ":: source2_demo :: proto :: NetMessages" | "source2_demo :: proto :: NetMessages" | "NetMessages"
    ) {
        Some("NetMessages".to_string())
    } else if matches!(
        value,
        ":: source2_demo :: proto :: EDotaUserMessages" | "source2_demo :: proto :: EDotaUserMessages" | "EDotaUserMessages"
    ) {
        Some("EDotaUserMessages".to_string())
    } else if matches!(
        value,
        ":: source2_demo :: proto :: CitadelUserMessageIds" | "source2_demo :: proto :: CitadelUserMessageIds" | "CitadelUserMessageIds"
    ) {
        Some("CitadelUserMessageIds".to_string())
    } else if matches!(
        value,
        ":: source2_demo :: proto :: ECitadelGameEvents" | "source2_demo :: proto :: ECitadelGameEvents" | "ECitadelGameEvents"
    ) {
        Some("ECitadelGameEvents".to_string())
    } else if matches!(
        value,
        ":: source2_demo :: proto :: ECstrike15UserMessages" | "source2_demo :: proto :: ECstrike15UserMessages" | "ECstrike15UserMessages"
    ) {
        Some("ECstrike15UserMessages".to_string())
    } else if matches!(
        value,
        ":: source2_demo :: proto :: ECsgoGameEvents" | "source2_demo :: proto :: ECsgoGameEvents" | "ECsgoGameEvents"
    ) {
        Some("ECsgoGameEvents".to_string())
    } else {
        None
    }
}

pub(crate) fn is_payload_type(value: &str) -> bool {
    value == "& [u8]"
}

pub(crate) fn is_packet_messages_type(value: &str) -> bool {
    matches!(
        value,
        "& mut Vec < :: source2_demo :: writer :: PacketMessage >"
            | "& mut Vec < source2_demo :: writer :: PacketMessage >"
            | "& mut Vec < PacketMessage >"
    )
}

pub(crate) fn is_string_table_entry_type(value: &str) -> bool {
    matches!(
        value,
        "& mut :: source2_demo :: writer :: StringTableEntryUpdate"
            | "& mut source2_demo :: writer :: StringTableEntryUpdate"
            | "& mut StringTableEntryUpdate"
    )
}

pub(crate) fn is_rewrite_field_value_type(value: &str) -> bool {
    matches!(
        value,
        "& str"
            | "String"
            | "& String"
            | "bool"
            | "f32"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "[f32 ; 2]"
            | "[f32 ; 3]"
            | "[f32 ; 4]"
    ) || is_raw_field_value_type(value)
}
