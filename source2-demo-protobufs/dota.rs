pub use crate::common::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaClientHardwareSpecs {
    #[prost(uint32, optional, tag = "1")]
    pub logical_processors: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub cpu_cycles_per_second: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub total_physical_memory: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "4")]
    pub is_64_bit_os: ::core::option::Option<bool>,
    #[prost(uint64, optional, tag = "5")]
    pub upload_measurement: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "6")]
    pub prefer_not_host: ::core::option::Option<bool>,
    #[prost(uint32, repeated, packed = "false", tag = "7")]
    pub crc: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaEntityMsgInvokerSpellCast {
    #[prost(message, optional, tag = "1")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
    #[prost(int32, optional, tag = "2")]
    pub cast_activity: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaLuaModifierEntry {
    #[prost(int32, required, tag = "1")]
    pub modifier_type: i32,
    #[prost(string, required, tag = "2")]
    pub modifier_filename: ::prost::alloc::string::String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaModifierBuffTableEntry {
    #[prost(enumeration = "DotaModifierEntryType", required, tag = "1", default = "Active")]
    pub entry_type: i32,
    #[prost(uint32, required, tag = "2", default = "16777215")]
    pub parent: u32,
    #[prost(int32, required, tag = "3")]
    pub index: i32,
    #[prost(int32, required, tag = "4")]
    pub serial_num: i32,
    #[prost(int32, optional, tag = "5")]
    pub modifier_class: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub ability_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub stack_count: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "8")]
    pub creation_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "9", default = "-1")]
    pub duration: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "10", default = "16777215")]
    pub caster: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11", default = "16777215")]
    pub ability: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "12")]
    pub armor: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "13")]
    pub fade_time: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "14")]
    pub subtle: ::core::option::Option<bool>,
    #[prost(float, optional, tag = "15")]
    pub channel_time: ::core::option::Option<f32>,
    #[prost(message, optional, tag = "16")]
    pub v_start: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "17")]
    pub v_end: ::core::option::Option<CMsgVector>,
    #[prost(string, optional, tag = "18")]
    pub portal_loop_appear: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "19")]
    pub portal_loop_disappear: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "20")]
    pub hero_loop_appear: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "21")]
    pub hero_loop_disappear: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "22")]
    pub movement_speed: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "23")]
    pub aura: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "24")]
    pub activity: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "25")]
    pub damage: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "26")]
    pub range: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "27")]
    pub dd_modifier_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "28", default = "-1")]
    pub dd_ability_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "29")]
    pub illusion_label: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "30")]
    pub active: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "31")]
    pub player_ids: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "32")]
    pub lua_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "33")]
    pub attack_speed: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "34", default = "16777215")]
    pub aura_owner: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "35")]
    pub bonus_all_stats: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "36")]
    pub bonus_health: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "37")]
    pub bonus_mana: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "38", default = "16777215")]
    pub custom_entity: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "39")]
    pub aura_within_range: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaMsgCoachHudPing {
    #[prost(uint32, optional, tag = "1")]
    pub x: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub y: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub tgtpath: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaMsgDismissAllStatPopups {
    #[prost(float, optional, tag = "1")]
    pub time_delay: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaMsgItemAlert {
    #[prost(int32, optional, tag = "1")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub y: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaMsgLocationPing {
    #[prost(int32, optional, tag = "1")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub y: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub target: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub direct_ping: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "5", default = "4294967295")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(enumeration = "EPingSource", optional, tag = "6", default = "KEPingSourceDefault")]
    pub ping_source: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaMsgMapLine {
    #[prost(int32, optional, tag = "1")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub y: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub initial: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaMsgSendStatPopup {
    #[prost(enumeration = "EdotaStatPopupTypes", optional, tag = "1", default = "KEdotaSptTextline")]
    pub style: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "2")]
    pub stat_strings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub stat_images: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub stat_image_types: ::prost::alloc::vec::Vec<i32>,
    #[prost(float, optional, tag = "5")]
    pub duration: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "6")]
    pub use_html: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "7")]
    pub movie_name: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaMsgUnitOrder {
    #[prost(enumeration = "DotaunitorderT", optional, tag = "2", default = "DotaUnitOrderNone")]
    pub order_type: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub units: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "4", default = "0")]
    pub target_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5", default = "-1")]
    pub ability_index: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "6")]
    pub position: ::core::option::Option<CMsgVector>,
    #[prost(int32, optional, tag = "8")]
    pub sequence_number: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "9")]
    pub flags: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaMsgWorldLine {
    #[prost(int32, optional, tag = "1")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub y: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub z: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub initial: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub end: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaResponseQuerySerialized {
    #[prost(message, repeated, tag = "1")]
    pub facts: ::prost::alloc::vec::Vec<cdota_response_query_serialized::Fact>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaSaveGame {
    #[prost(uint64, optional, tag = "5")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub save_time: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "3")]
    pub players: ::prost::alloc::vec::Vec<cdota_save_game::Player>,
    #[prost(message, repeated, tag = "4")]
    pub save_instances: ::prost::alloc::vec::Vec<cdota_save_game::SaveInstance>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaSpeechMatchOnClient {
    #[prost(int32, optional, tag = "1")]
    pub speech_concept: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub recipient_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub responsequery: ::core::option::Option<CDotaResponseQuerySerialized>,
    #[prost(sfixed32, optional, tag = "4", default = "0")]
    pub randomseed: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMessageRequestItemSuggestions {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMessageTeamCaptainChanged {
    #[prost(uint32, optional, tag = "1")]
    pub team: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub captain_player_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgAbilityDraftRequestAbility {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub requested_ability_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub ctrl_is_down: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgAbilityPing {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub ability_id: ::core::option::Option<i32>,
    #[prost(enumeration = "DotaAbilityPingType", optional, tag = "3", default = "AbilityPingReady")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "4")]
    pub cooldown_seconds: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub level: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "6")]
    pub passive: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "7")]
    pub mana_needed: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub entity_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "9")]
    pub primary_charges: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub secondary_charges: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "12")]
    pub ctrl_held: ::core::option::Option<bool>,
    #[prost(float, optional, tag = "13")]
    pub reclaim_time: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "14", default = "-1")]
    pub owner_entity: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgAbilitySteal {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub ability_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub ability_level: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgAddQuestLogEntry {
    #[prost(string, optional, tag = "1")]
    pub npc_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub npc_dialog: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgAghsStatusAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub target_entindex: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "4")]
    pub alert_type: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "5")]
    pub has_scepter: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub has_shard: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgAiDebugLine {
    #[prost(string, optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgAllStarEvent {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub point_amount: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub event_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "5")]
    pub player_scores: ::prost::alloc::vec::Vec<cdota_user_msg_all_star_event::PlayerScore>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgBeastChat {
    #[prost(uint32, optional, tag = "1")]
    pub team: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub format: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub target: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgBoosterState {
    #[prost(message, repeated, tag = "1")]
    pub boosted_players: ::prost::alloc::vec::Vec<CDotaUserMsgBoosterStatePlayer>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgBoosterStatePlayer {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub bonus: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub event_bonus: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "4")]
    pub bonus_item_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub event_bonus_item_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgBotChat {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub target: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "5")]
    pub team_only: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgBuyBackStateAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgChatEvent {
    #[prost(enumeration = "DotaChatMessage", required, tag = "1", default = "ChatMessageInvalid")]
    pub r#type: i32,
    #[prost(uint32, optional, tag = "2")]
    pub value: ::core::option::Option<u32>,
    #[prost(sint32, optional, tag = "3", default = "-1")]
    pub playerid_1: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag = "4", default = "-1")]
    pub playerid_2: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag = "5", default = "-1")]
    pub playerid_3: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag = "6", default = "-1")]
    pub playerid_4: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag = "7", default = "-1")]
    pub playerid_5: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag = "8", default = "-1")]
    pub playerid_6: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "9")]
    pub value2: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub value3: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgChatMessage {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub channel_type: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub message_text: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgChatWheel {
    #[prost(uint32, optional, tag = "1", default = "4294967295")]
    pub chat_message_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "4")]
    pub param_hero_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "5")]
    pub emoticon_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgChatWheelCooldown {
    #[prost(uint32, optional, tag = "1", default = "4294967295")]
    pub message_id: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub cooldown_remaining: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgClientLoadGridNav {}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgCoachHudPing {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub hud_ping: ::core::option::Option<CDotaMsgCoachHudPing>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgCombatHeroPositions {
    #[prost(uint32, optional, tag = "1")]
    pub index: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub time: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub world_pos: ::core::option::Option<CMsgVector2D>,
    #[prost(int32, optional, tag = "4")]
    pub health: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgCombatLogBulkData {
    #[prost(message, repeated, tag = "1")]
    pub combat_entries: ::prost::alloc::vec::Vec<CMsgDotaCombatLogEntry>,
    #[prost(float, optional, tag = "2")]
    pub timestamp: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub duration: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "5")]
    pub request_time: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgCompendiumState {
    #[prost(message, repeated, tag = "1")]
    pub compendium_players: ::prost::alloc::vec::Vec<CDotaUserMsgCompendiumStatePlayer>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgCompendiumStatePlayer {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub level: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgContextualTip {
    #[prost(int32, optional, tag = "1")]
    pub tip_id: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "2")]
    pub referenced_abilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub referenced_units: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub panorama_classes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "5")]
    pub force_annotation: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "6")]
    pub variant: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub int_param: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub int_param2: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "9")]
    pub float_param: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "10")]
    pub float_param2: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "11")]
    pub string_param: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub string_param2: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub tip_text_override: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub tip_annotation_override: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub panorama_snippet: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgCourierKilledAlert {
    #[prost(uint32, optional, tag = "1")]
    pub team: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub gold_value: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3", default = "16777215")]
    pub entity_handle: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "4")]
    pub timestamp: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub lost_items: ::prost::alloc::vec::Vec<cdota_user_msg_courier_killed_alert::LostItem>,
    #[prost(int32, optional, tag = "6", default = "-1")]
    pub killer_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7", default = "-1")]
    pub owning_player_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgCreateLinearProjectile {
    #[prost(message, optional, tag = "1")]
    pub origin: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "2")]
    pub velocity: ::core::option::Option<CMsgVector2D>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub entindex: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "5")]
    pub particle_index: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "6")]
    pub handle: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "7")]
    pub acceleration: ::core::option::Option<CMsgVector2D>,
    #[prost(float, optional, tag = "8")]
    pub max_speed: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "9")]
    pub fow_radius: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "10")]
    pub sticky_fow_reveal: ::core::option::Option<bool>,
    #[prost(float, optional, tag = "11")]
    pub distance: ::core::option::Option<f32>,
    #[prost(fixed32, optional, tag = "12")]
    pub colorgemcolor: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "13")]
    pub particle_cp_data: ::prost::alloc::vec::Vec<CDotaUserMsgProjectileParticleCpData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgCustomHeaderMessage {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub value: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgCustomHudElementCreate {
    #[prost(string, optional, tag = "1")]
    pub element_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub layout_filename: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgCustomHudElementDestroy {
    #[prost(string, optional, tag = "1")]
    pub element_id: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgCustomHudElementModify {
    #[prost(string, optional, tag = "1")]
    pub element_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub modify_visible: ::core::option::Option<bool>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgCustomMsg {
    #[prost(string, optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub value: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgDamageReport {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub target_hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub source_hero_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub damage_amount: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub broadcast: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgDebugChallenge {
    #[prost(uint32, required, tag = "1")]
    pub challenge_type: u32,
    #[prost(uint32, required, tag = "2")]
    pub challenge_query_id: u32,
    #[prost(uint32, required, tag = "3")]
    pub event_id: u32,
    #[prost(uint32, optional, tag = "4")]
    pub instance_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub challenge_var_0: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub challenge_var_1: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub challenge_max_rank: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgDestroyLinearProjectile {
    #[prost(int32, optional, tag = "1")]
    pub handle: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgDismissAllStatPopups {
    #[prost(message, optional, tag = "1")]
    pub dismissallmsg: ::core::option::Option<CDotaMsgDismissAllStatPopups>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgDodgeTrackingProjectiles {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub entindex: i32,
    #[prost(bool, optional, tag = "2")]
    pub attacks_only: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgDuelAccepted {
    #[prost(int32, optional, tag = "1")]
    pub player_id_1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub player_id_2: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgDuelOpponentKilled {
    #[prost(int32, optional, tag = "1")]
    pub player_id_winner: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub player_id_loser: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgDuelRequested {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_requestor: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgEmptyItemSlotAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub slot_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub cooldown_seconds: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgEmptyTeleportAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub cooldown_seconds: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgEnemyItemAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub rune_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub entity_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6", default = "-1")]
    pub item_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7", default = "-1")]
    pub primary_charges: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8", default = "-1")]
    pub secondary_charges: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgEsArcanaCombo {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub combo_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub arcana_level: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgEsArcanaComboSummary {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub combo_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub damage_amount: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgFacetPing {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub facet_strhash: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub entity_id: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "4")]
    pub all_chat: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgFlipCoinResult {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub channel_type: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub result: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgFoundNeutralItem {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub item_tier: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub tier_item_count: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgGamerulesStateChanged {
    #[prost(uint32, optional, tag = "1")]
    pub state: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgGiftPlayer {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub gift_item_def_index: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgGlobalLightColor {
    #[prost(uint32, optional, tag = "1")]
    pub color: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgGlobalLightDirection {
    #[prost(message, optional, tag = "1")]
    pub direction: ::core::option::Option<CMsgVector>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgGlyphAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub negative: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgGuildChallengeProgress {
    #[prost(message, repeated, tag = "1")]
    pub player_progress: ::prost::alloc::vec::Vec<cdota_user_msg_guild_challenge_progress::PlayerProgress>,
    #[prost(uint32, optional, tag = "2")]
    pub guild_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub challenge_instance_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub challenge_parameter: ::core::option::Option<u32>,
    #[prost(enumeration = "cdota_user_msg_guild_challenge_progress::EChallengeType", optional, tag = "5", default = "KEChallengeTypeInvalid")]
    pub challenge_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "7")]
    pub challenge_progress_at_start: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "8")]
    pub complete: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgHalloweenDrops {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub item_defs: ::prost::alloc::vec::Vec<u32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub prize_list: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgHeroRelicProgress {
    #[prost(uint32, optional, tag = "1")]
    pub hero_relic_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub value: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub event_id: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "5")]
    pub value_display: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgHighFiveCompleted {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id_2: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub special_high_five: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub special_entindex: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgHighFiveLeftHanging {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgHotPotatoCreated {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id_2: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgHotPotatoExploded {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgHpManaAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_entindex: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub show_raw_values: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgHudError {
    #[prost(int32, optional, tag = "1")]
    pub order_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub sequence_number: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgInnatePing {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub entity_id: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub all_chat: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgInvalidCommand {
    #[prost(string, optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub sequence_number: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgItemAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub item_alert: ::core::option::Option<CDotaMsgItemAlert>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgItemFound {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub quality: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub rarity: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub method: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "5")]
    pub itemdef: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgItemPurchased {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgItemSold {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgKillcamDamageTaken {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub player_id: i32,
    #[prost(uint32, optional, tag = "2")]
    pub damage_taken: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub item_type: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub hero_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub damage_color: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgLocationPing {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub location_ping: ::core::option::Option<CDotaMsgLocationPing>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgMapLine {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub mapline: ::core::option::Option<CDotaMsgMapLine>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgMarsArenaOfBloodAttack {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub source_ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2", default = "16777215")]
    pub target_ehandle: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub warrior_index: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgMiniKillCamInfo {
    #[prost(message, repeated, tag = "1")]
    pub attackers: ::prost::alloc::vec::Vec<cdota_user_msg_mini_kill_cam_info::Attacker>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgMiniTaunt {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub taunting_player_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgMinimapDebugPoint {
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<CMsgVector>,
    #[prost(uint32, optional, tag = "2")]
    pub color: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub size: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "4")]
    pub duration: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "5")]
    pub index: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgMinimapEvent {
    #[prost(int32, optional, tag = "1")]
    pub event_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2", default = "16777215")]
    pub entity_handle: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub y: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub duration: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "6", default = "16777215")]
    pub target_entity_handle: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgModifierAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub class_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub stack_count: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "4")]
    pub is_debuff: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "5", default = "-1")]
    pub target_entindex: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "6")]
    pub seconds_remaining: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgMoveCameraToUnit {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub unit_ehandle: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgMuertaReleaseEventAssignedTargetKilled {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub player_id_killer: i32,
    #[prost(int32, required, tag = "2", default = "-1")]
    pub player_id_target: i32,
    #[prost(int32, required, tag = "3")]
    pub points: i32,
    #[prost(int32, required, tag = "4")]
    pub points_total: i32,
    #[prost(bool, required, tag = "5")]
    pub last_hit: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgMutedPlayers {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub text_muted_player_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub voice_muted_player_ids: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgNeutralCampAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub spawner_entindex: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub unit_entindex: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub stack_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub camp_type: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "6")]
    pub stack_request: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub stack_intention: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgNevermoreRequiem {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub entity_handle: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub lines: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub origin: ::core::option::Option<CMsgVector>,
    #[prost(bool, optional, tag = "4")]
    pub reverse: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgOmArcanaCombo {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub multicast_amount: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub arcana_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub multicast_chance: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgOutpostCaptured {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub outpost_entindex: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub team_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgOutpostGrantedXp {
    #[prost(uint32, optional, tag = "1")]
    pub team_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub xp_amount: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgOverheadEvent {
    #[prost(enumeration = "DotaOverheadAlert", required, tag = "1", default = "OverheadAlertGold")]
    pub message_type: i32,
    #[prost(int32, optional, tag = "2")]
    pub value: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub target_player_entindex: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub target_entindex: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5", default = "-1")]
    pub source_player_entindex: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgPauseMinigameData {
    #[prost(message, repeated, tag = "1")]
    pub data_bits: ::prost::alloc::vec::Vec<cdota_user_msg_pause_minigame_data::DataBit>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgPing {
    #[prost(uint32, optional, tag = "2")]
    pub ping: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub loss: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgPingConfirmation {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_of_original_pinger: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub entity_index: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub icon_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "4")]
    pub location: ::core::option::Option<CMsgVector>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgPlayerDraftPick {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_captain: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id_target: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub team: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgPlayerDraftSuggestPick {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub suggestion_player_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgProjectileParticleCpData {
    #[prost(int32, optional, tag = "1")]
    pub control_point: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub vector: ::core::option::Option<CMsgVector>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgProjectionAbility {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub ability_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub caster_ent_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub caster_team: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub channel_end: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "5")]
    pub origin: ::core::option::Option<CMsgVector>,
    #[prost(bool, optional, tag = "6")]
    pub track_caster_only: ::core::option::Option<bool>,
    #[prost(float, optional, tag = "7")]
    pub end_time: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "8", default = "-1")]
    pub victim_ent_index: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgProjectionEvent {
    #[prost(enumeration = "EProjectionEvent", optional, tag = "1", default = "EPeFirstBlood")]
    pub event_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub team: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgQoPArcanaSummary {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub arcana_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub players_hit: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub players_killed: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgQuestStatus {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub player_id: i32,
    #[prost(uint32, optional, tag = "2")]
    pub quest_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub challenge_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub progress: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub goal: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub query: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "7")]
    pub fail_gametime: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "8", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgQueuedOrderRemoved {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub unit_order_sequence: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgQuickBuyAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub gold_cost: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub item_cooldown_seconds: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub show_buyback: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgRadarAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub negative: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgReceivedXmasGift {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub item_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub inventory_slot: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgReplaceQueryUnit {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub player_id: i32,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub source_entindex: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub target_entindex: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgRockPaperScissorsFinished {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id_2: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub player_1_choice: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub player_2_choice: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgRockPaperScissorsStarted {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_source: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id_target: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgRollDiceResult {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub channel_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub roll_min: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub roll_max: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub result: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgRoshanTimer {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub negative: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSalutePlayer {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub tip_amount: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub event_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "5")]
    pub custom_tip_style: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "6")]
    pub num_recent_tips: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSelectPenaltyGold {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub player_id: i32,
    #[prost(sint32, optional, tag = "2")]
    pub cost: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSendFinalGold {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub reliable_gold: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub unreliable_gold: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSendGenericToolTip {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub entindex: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub close: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSendRoshanPopup {
    #[prost(bool, optional, tag = "1")]
    pub reclaimed: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "2")]
    pub gametime: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSendRoshanSpectatorPhase {
    #[prost(enumeration = "DotaRoshanPhase", optional, tag = "1", default = "KSrspRoshanAlive")]
    pub phase: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub phase_start_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub phase_length: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSendStatPopup {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub statpopup: ::core::option::Option<CDotaMsgSendStatPopup>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSetNextAutobuyItem {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSharedCooldown {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, optional, tag = "3")]
    pub cooldown: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "4")]
    pub name_index: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgShovelUnearth {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub all_chat: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "3")]
    pub locstring: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub quantity: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgShowGenericPopup {
    #[prost(string, required, tag = "1")]
    pub header: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub body: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub param1: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub param2: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "5")]
    pub tint_screen: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub show_no_other_dialogs: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgShowSurvey {
    #[prost(int32, optional, tag = "1")]
    pub survey_id: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "2")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub response_style: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub teammate_hero_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub teammate_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "6")]
    pub teammate_account_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSpectatorPlayerClick {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub entindex: i32,
    #[prost(int32, optional, tag = "2")]
    pub order_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "0")]
    pub target_index: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSpectatorPlayerUnitOrders {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub order_type: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub units: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "4", default = "0")]
    pub target_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5", default = "-1")]
    pub ability_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "6")]
    pub position: ::core::option::Option<CMsgVector>,
    #[prost(bool, optional, tag = "7")]
    pub queue: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "8")]
    pub sequence_number: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "9")]
    pub flags: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSpeechBubble {
    #[prost(bool, optional, tag = "1")]
    pub destroy_all: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgStatsHeroLookup {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub hero_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub persona: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgStatsHeroMinuteDetails {
    #[prost(uint32, optional, tag = "1")]
    pub last_hits: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub hero_kills: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub hero_damage: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub tower_damage: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "5")]
    pub position_info: ::core::option::Option<CDotaUserMsgStatsHeroPositionInfo>,
    #[prost(uint32, optional, tag = "6")]
    pub total_xp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub net_worth: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub harvested_creep_gold: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub claimed_farm: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub wards_placed: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub runes_collected: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub tps_used: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "13")]
    pub mana_spent: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "14")]
    pub damage_absorbed: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "15")]
    pub damage_done: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgStatsHeroPositionInfo {
    #[prost(float, optional, tag = "1")]
    pub average_position: ::core::option::Option<f32>,
    #[prost(message, repeated, tag = "2")]
    pub position_details: ::prost::alloc::vec::Vec<cdota_user_msg_stats_hero_position_info::PositionPair>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgStatsKillDetails {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub victim_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub kill_shares: ::prost::alloc::vec::Vec<CDotaUserMsgStatsPlayerKillShare>,
    #[prost(uint32, optional, tag = "3")]
    pub damage_to_kill: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub effective_health: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "5")]
    pub death_time: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "6", default = "-1")]
    pub killer_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgStatsMatchDetails {
    #[prost(message, repeated, tag = "1")]
    pub hero_lookup: ::prost::alloc::vec::Vec<CDotaUserMsgStatsHeroLookup>,
    #[prost(message, repeated, tag = "2")]
    pub radiant_stats: ::prost::alloc::vec::Vec<CDotaUserMsgStatsTeamMinuteDetails>,
    #[prost(message, repeated, tag = "3")]
    pub dire_stats: ::prost::alloc::vec::Vec<CDotaUserMsgStatsTeamMinuteDetails>,
    #[prost(message, repeated, tag = "4")]
    pub radiant_kills: ::prost::alloc::vec::Vec<CDotaUserMsgStatsKillDetails>,
    #[prost(message, repeated, tag = "5")]
    pub dire_kills: ::prost::alloc::vec::Vec<CDotaUserMsgStatsKillDetails>,
    #[prost(message, repeated, tag = "6")]
    pub fight_details: ::prost::alloc::vec::Vec<cdota_user_msg_stats_match_details::CDotaUserMsgStatsFightDetails>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgStatsPlayerKillShare {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub kill_share_percent: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub player_loc_x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub player_loc_y: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "5")]
    pub health_percent: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "6")]
    pub mana_percent: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgStatsTeamMinuteDetails {
    #[prost(message, repeated, tag = "1")]
    pub player_stats: ::prost::alloc::vec::Vec<CDotaUserMsgStatsHeroMinuteDetails>,
    #[prost(uint32, optional, tag = "2")]
    pub tower_kills: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub barrack_kills: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub available_lane_creep_gold: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub balance_kill_value: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub balance_tower_value: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub balance_barracks_value: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub balance_gold_value: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub balance_xp_value: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "10")]
    pub lane_performance: ::prost::alloc::vec::Vec<cdota_user_msg_stats_team_minute_details::LocationPerformance>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSuggestHeroPick {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub player_id: i32,
    #[prost(int32, optional, tag = "2")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub ban: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "4")]
    pub facet_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSuggestHeroRole {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub player_id: i32,
    #[prost(string, optional, tag = "2")]
    pub hero_role: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgSwapVerify {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTalentTreeAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_entindex: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub ability_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub slot: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub learned: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTeDestroyProjectile {
    #[prost(int32, optional, tag = "1")]
    pub handle: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTeDotaBloodImpact {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub entity: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub scale: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub xnormal: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub ynormal: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTeProjectile {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub source: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2", default = "16777215")]
    pub target: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub move_speed: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub source_attachment: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "5")]
    pub particle_system_handle: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "6")]
    pub dodgeable: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub is_attack: ::core::option::Option<bool>,
    #[prost(float, optional, tag = "9")]
    pub expire_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "10")]
    pub maximpacttime: ::core::option::Option<f32>,
    #[prost(fixed32, optional, tag = "11")]
    pub colorgemcolor: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "12")]
    pub launch_tick: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub handle: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "14")]
    pub target_loc: ::core::option::Option<CMsgVector>,
    #[prost(message, repeated, tag = "15")]
    pub particle_cp_data: ::prost::alloc::vec::Vec<CDotaUserMsgProjectileParticleCpData>,
    #[prost(int64, optional, tag = "16")]
    pub additional_particle_system_handle: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "17")]
    pub original_move_speed: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "18", default = "16777215")]
    pub ability: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTeProjectileLoc {
    #[prost(message, optional, tag = "1")]
    pub source_loc: ::core::option::Option<CMsgVector>,
    #[prost(uint32, optional, tag = "2", default = "16777215")]
    pub target: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub move_speed: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "4")]
    pub particle_system_handle: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "5")]
    pub dodgeable: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub is_attack: ::core::option::Option<bool>,
    #[prost(float, optional, tag = "9")]
    pub expire_time: ::core::option::Option<f32>,
    #[prost(message, optional, tag = "10")]
    pub target_loc: ::core::option::Option<CMsgVector>,
    #[prost(fixed32, optional, tag = "11")]
    pub colorgemcolor: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "12")]
    pub launch_tick: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub handle: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "14", default = "16777215")]
    pub source: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "15")]
    pub source_attachment: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "16")]
    pub particle_cp_data: ::prost::alloc::vec::Vec<CDotaUserMsgProjectileParticleCpData>,
    #[prost(int64, optional, tag = "17")]
    pub additional_particle_system_handle: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "18")]
    pub original_move_speed: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTeUnitAnimation {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub entity: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub sequence_variant: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "3")]
    pub playbackrate: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub castpoint: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "5")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub activity: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "7")]
    pub lag_compensation_time: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTeUnitAnimationEnd {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub entity: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "2")]
    pub snap: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTipAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub tip_text: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTutorialFade {
    #[prost(int32, optional, tag = "1")]
    pub tgt_alpha: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTutorialFinish {
    #[prost(string, optional, tag = "1")]
    pub heading: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub emblem: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub body: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "4")]
    pub success: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTutorialMinimapPosition {}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTutorialPingMinimap {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub pos_x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub pos_y: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub pos_z: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "5")]
    pub entity_index: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTutorialRequestExp {}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgTutorialTipInfo {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub progress: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgUnitEvent {
    #[prost(enumeration = "EDotaEntityMessages", required, tag = "1", default = "DotaUnitSpeech")]
    pub msg_type: i32,
    #[prost(int32, required, tag = "2")]
    pub entity_index: i32,
    #[prost(message, optional, tag = "3")]
    pub speech: ::core::option::Option<cdota_user_msg_unit_event::Speech>,
    #[prost(message, optional, tag = "4")]
    pub speech_mute: ::core::option::Option<cdota_user_msg_unit_event::SpeechMute>,
    #[prost(message, optional, tag = "5")]
    pub add_gesture: ::core::option::Option<cdota_user_msg_unit_event::AddGesture>,
    #[prost(message, optional, tag = "6")]
    pub remove_gesture: ::core::option::Option<cdota_user_msg_unit_event::RemoveGesture>,
    #[prost(message, optional, tag = "7")]
    pub blood_impact: ::core::option::Option<cdota_user_msg_unit_event::BloodImpact>,
    #[prost(message, optional, tag = "8")]
    pub fade_gesture: ::core::option::Option<cdota_user_msg_unit_event::FadeGesture>,
    #[prost(message, optional, tag = "9")]
    pub speech_match_on_client: ::core::option::Option<CDotaSpeechMatchOnClient>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgUpdateLinearProjectileCpData {
    #[prost(int32, optional, tag = "1")]
    pub handle: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub control_point: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub vector: ::core::option::Option<CMsgVector>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgUpdateQuestProgress {}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgUpdateSharedContent {
    #[prost(int32, optional, tag = "1")]
    pub slot_type: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgVersusScenePlayerBehavior {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(enumeration = "EdotaVersusScenePlayerBehavior", optional, tag = "2", default = "VsPlayerBehaviorPlayActivity")]
    pub behavior: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub play_activity: ::core::option::Option<VersusScenePlayActivity>,
    #[prost(message, optional, tag = "4")]
    pub chat_wheel: ::core::option::Option<VersusSceneChatWheel>,
    #[prost(message, optional, tag = "5")]
    pub playback_rate: ::core::option::Option<VersusScenePlaybackRate>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgVoteEnd {
    #[prost(int32, optional, tag = "1")]
    pub selected_choice: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgVoteStart {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "3")]
    pub choice_count: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "4")]
    pub choices: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgVoteUpdate {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub choice_counts: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgWillPurchaseAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub gold_remaining: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub suggestion_player_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgWkArcanaProgress {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub arcana_level: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub hero_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgWorldLine {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub worldline: ::core::option::Option<CDotaMsgWorldLine>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgWrArcanaProgress {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2", default = "16777215")]
    pub target_ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub arrows_landed: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub damage_dealt: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub target_hp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub target_max_hp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub arcana_level: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgWrArcanaSummary {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2", default = "16777215")]
    pub target_ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub arrows_landed: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub damage_dealt: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub target_hp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub target_max_hp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub arcana_level: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "8")]
    pub success: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDotaUserMsgXpAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_entindex: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgCombatAnalyzerPlayerStat {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub hero_ability_stats: ::prost::alloc::vec::Vec<CMsgHeroAbilityStat>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgCombatAnalyzerStats {
    #[prost(uint64, optional, tag = "1")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "2")]
    pub player_stats: ::prost::alloc::vec::Vec<CMsgCombatAnalyzerPlayerStat>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgDotaCombatLogEntry {
    #[prost(enumeration = "DotaCombatlogTypes", optional, tag = "1", default = "DotaCombatlogInvalid")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub target_name: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub target_source_name: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub attacker_name: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub damage_source_name: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub inflictor_name: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "7")]
    pub is_attacker_illusion: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub is_attacker_hero: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "9")]
    pub is_target_illusion: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "10")]
    pub is_target_hero: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "11")]
    pub is_visible_radiant: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "12")]
    pub is_visible_dire: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "13")]
    pub value: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "14")]
    pub health: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "15")]
    pub timestamp: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "16")]
    pub stun_duration: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "17")]
    pub slow_duration: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "18")]
    pub is_ability_toggle_on: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "19")]
    pub is_ability_toggle_off: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "20")]
    pub ability_level: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "21")]
    pub location_x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "22")]
    pub location_y: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "23")]
    pub gold_reason: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "24")]
    pub timestamp_raw: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "25")]
    pub modifier_duration: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "26")]
    pub xp_reason: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "27")]
    pub last_hits: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "28")]
    pub attacker_team: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "29")]
    pub target_team: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "30")]
    pub obs_wards_placed: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "31")]
    pub assist_player0: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "32")]
    pub assist_player1: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "33")]
    pub assist_player2: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "34")]
    pub assist_player3: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "35")]
    pub stack_count: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "36")]
    pub hidden_modifier: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "37")]
    pub is_target_building: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "38")]
    pub neutral_camp_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "39")]
    pub rune_type: ::core::option::Option<u32>,
    #[prost(int32, repeated, packed = "false", tag = "40")]
    pub assist_players: ::prost::alloc::vec::Vec<i32>,
    #[prost(bool, optional, tag = "41")]
    pub is_heal_save: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "42")]
    pub is_ultimate_ability: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "43")]
    pub attacker_hero_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "44")]
    pub target_hero_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "45")]
    pub xpm: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "46")]
    pub gpm: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "47")]
    pub event_location: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "48")]
    pub target_is_self: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "49")]
    pub damage_type: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "50")]
    pub invisibility_modifier: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "51")]
    pub damage_category: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "52")]
    pub networth: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "53")]
    pub building_type: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "54")]
    pub modifier_elapsed_duration: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "55")]
    pub silence_modifier: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "56")]
    pub heal_from_lifesteal: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "57")]
    pub modifier_purged: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "58")]
    pub spell_evaded: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "59")]
    pub motion_controller_modifier: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "60")]
    pub long_range_kill: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "61")]
    pub modifier_purge_ability: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "62")]
    pub modifier_purge_npc: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "63")]
    pub root_modifier: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "64")]
    pub total_unit_death_count: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "65")]
    pub aura_modifier: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "66")]
    pub armor_debuff_modifier: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "67")]
    pub no_physical_damage_modifier: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "68")]
    pub modifier_ability: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "69")]
    pub modifier_hidden: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "70")]
    pub inflictor_is_stolen_ability: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "71")]
    pub kill_eater_event: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "72")]
    pub unit_status_label: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "73")]
    pub spell_generated_attack: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "74")]
    pub at_night_time: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "75")]
    pub attacker_has_scepter: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "76")]
    pub neutral_camp_team: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "77")]
    pub regenerated_health: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "78")]
    pub will_reincarnate: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "79")]
    pub uses_charges: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgHeroAbilityStat {
    #[prost(enumeration = "EHeroStatType", optional, tag = "1", default = "KEHeroStatTypeNone")]
    pub stat_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub int_value: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "3")]
    pub float_value: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgPendingEventAward {
    #[prost(enumeration = "EEvent", optional, tag = "1", default = "EventIdNone")]
    pub event_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub action_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub num_to_grant: ::core::option::Option<u32>,
    #[prost(enumeration = "EEventActionScoreMode", optional, tag = "4", default = "KEEventActionScoreModeAdd")]
    pub score_mode: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "5")]
    pub audit_action: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "6")]
    pub audit_data: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaAbilityPingType {
    AbilityPingReady = 1,
    AbilityPingMana = 2,
    AbilityPingCooldown = 3,
    AbilityPingEnemy = 4,
    AbilityPingUnlearned = 5,
    AbilityPingInbackpack = 6,
    AbilityPingInstash = 7,
    AbilityPingOncourier = 8,
    AbilityPingAlly = 9,
    AbilityPingLearnReady = 10,
    AbilityPingWillLearn = 11,
    AbilityPingFutureLearn = 12,
    AbilityPingNeutralOffer = 13,
    AbilityPingNeutralRequest = 14,
    AbilityPingNeutralEquip = 15,
    AbilityPingIncourierbackpack = 16,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaAbilityTargetType {
    DotaAbilityTargetNone = 0,
    DotaAbilityTargetSelf = 1,
    DotaAbilityTargetAllyHero = 2,
    DotaAbilityTargetAllyCreep = 3,
    DotaAbilityTargetEnemyHero = 4,
    DotaAbilityTargetEnemyCreep = 5,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaBehaviorLevelT {
    DotaBehaviorLevelNone = 0,
    DotaBehaviorLevelRankedAllowed = 1,
    DotaBehaviorLevelPausing = 2,
    DotaBehaviorLevelDrops = 3,
    DotaBehaviorLevelCoaching = 4,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaBotDifficulty {
    BotDifficultyPassive = 0,
    BotDifficultyEasy = 1,
    BotDifficultyMedium = 2,
    BotDifficultyHard = 3,
    BotDifficultyUnfair = 4,
    BotDifficultyInvalid = 5,
    BotDifficultyExtra1 = 6,
    BotDifficultyExtra2 = 7,
    BotDifficultyExtra3 = 8,
    BotDifficultyNpx = 9,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaBotMode {
    None = 0,
    Laning = 1,
    Attack = 2,
    Roam = 3,
    Retreat = 4,
    SecretShop = 5,
    SideShop = 6,
    Rune = 7,
    PushTowerTop = 8,
    PushTowerMid = 9,
    PushTowerBot = 10,
    DefendTowerTop = 11,
    DefendTowerMid = 12,
    DefendTowerBot = 13,
    Assemble = 14,
    AssembleWithHumans = 15,
    TeamRoam = 16,
    Farm = 17,
    DefendAlly = 18,
    EvasiveManeuvers = 19,
    Roshan = 20,
    Item = 21,
    Ward = 22,
    Companion = 23,
    TutorialBoss = 24,
    Minion = 25,
    Outpost = 26,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaChatChannelTypeT {
    DotaChannelTypeRegional = 0,
    DotaChannelTypeCustom = 1,
    DotaChannelTypeParty = 2,
    DotaChannelTypeLobby = 3,
    DotaChannelTypeTeam = 4,
    DotaChannelTypeGuild = 5,
    DotaChannelTypeFantasy = 6,
    DotaChannelTypeWhisper = 7,
    DotaChannelTypeConsole = 8,
    DotaChannelTypeTab = 9,
    DotaChannelTypeInvalid = 10,
    DotaChannelTypeGameAll = 11,
    DotaChannelTypeGameAllies = 12,
    DotaChannelTypeGameSpectator = 13,
    DotaChannelTypeGameCoaching = 14,
    DotaChannelTypeCafe = 15,
    DotaChannelTypeCustomGame = 16,
    DotaChannelTypePrivate = 17,
    DotaChannelTypePostGame = 18,
    DotaChannelTypeBattleCup = 19,
    DotaChannelTypeHltvSpectator = 20,
    DotaChannelTypeGameEvents = 21,
    DotaChannelTypeTrivia = 22,
    DotaChannelTypeNewPlayer = 23,
    DotaChannelTypePrivateCoaching = 24,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaChatInformational {
    InfoCoopBattlePointsRules = 1,
    InfoFrostivusAbandonReminder = 2,
    InfoRankedReminder = 3,
    InfoCoopLowPriorityPassiveReminder = 4,
    InfoCustomGamePenaltyReminder = 5,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaChatMessage {
    ChatMessageInvalid = -1,
    ChatMessageHeroKill = 0,
    ChatMessageHeroDeny = 1,
    ChatMessageBarracksKill = 2,
    ChatMessageTowerKill = 3,
    ChatMessageTowerDeny = 4,
    ChatMessageFirstblood = 5,
    ChatMessageStreakKill = 6,
    ChatMessageBuyback = 7,
    ChatMessageAegis = 8,
    ChatMessageRoshanKill = 9,
    ChatMessageCourierLost = 10,
    ChatMessageCourierRespawned = 11,
    ChatMessageGlyphUsed = 12,
    ChatMessageItemPurchase = 13,
    ChatMessageConnect = 14,
    ChatMessageDisconnect = 15,
    ChatMessageDisconnectWaitForReconnect = 16,
    ChatMessageDisconnectTimeRemaining = 17,
    ChatMessageDisconnectTimeRemainingPlural = 18,
    ChatMessageReconnect = 19,
    ChatMessagePlayerLeft = 20,
    ChatMessageSafeToLeave = 21,
    ChatMessageRunePickup = 22,
    ChatMessageRuneBottle = 23,
    ChatMessageRuneDeny = 114,
    ChatMessageInthebag = 24,
    ChatMessageSecretshop = 25,
    ChatMessageItemAutopurchased = 26,
    ChatMessageItemsCombined = 27,
    ChatMessageSuperCreeps = 28,
    ChatMessageCantUseActionItem = 29,
    ChatMessageCantpause = 31,
    ChatMessageNopausesleft = 32,
    ChatMessageCantpauseyet = 33,
    ChatMessagePaused = 34,
    ChatMessageUnpauseCountdown = 35,
    ChatMessageUnpaused = 36,
    ChatMessageAutoUnpaused = 37,
    ChatMessageYoupaused = 38,
    ChatMessageCantunpauseteam = 39,
    ChatMessageVoiceTextBanned = 41,
    ChatMessageSpectatorsWatchingThisGame = 42,
    ChatMessageReportReminder = 43,
    ChatMessageEconItem = 44,
    ChatMessageTaunt = 45,
    ChatMessageRandom = 46,
    ChatMessageRdTurn = 47,
    ChatMessageDropRateBonus = 49,
    ChatMessageNoBattlePoints = 50,
    ChatMessageDeniedAegis = 51,
    ChatMessageInformational = 52,
    ChatMessageAegisStolen = 53,
    ChatMessageRoshanCandy = 54,
    ChatMessageItemGifted = 55,
    ChatMessageHeroKillWithGreevil = 56,
    ChatMessageHoldoutTowerDestroyed = 57,
    ChatMessageHoldoutWallDestroyed = 58,
    ChatMessageHoldoutWallFinished = 59,
    ChatMessagePlayerLeftLimitedHero = 62,
    ChatMessageAbandonLimitedHeroExplanation = 63,
    ChatMessageDisconnectLimitedHero = 64,
    ChatMessageLowPriorityCompletedExplanation = 65,
    ChatMessageRecruitmentDropRateBonus = 66,
    ChatMessageFrostivusShiningBoosterActive = 67,
    ChatMessagePlayerLeftAfk = 73,
    ChatMessagePlayerLeftDisconnectedTooLong = 74,
    ChatMessagePlayerAbandoned = 75,
    ChatMessagePlayerAbandonedAfk = 76,
    ChatMessagePlayerAbandonedDisconnectedTooLong = 77,
    ChatMessageWillNotBeScored = 78,
    ChatMessageWillNotBeScoredRanked = 79,
    ChatMessageWillNotBeScoredNetwork = 80,
    ChatMessageWillNotBeScoredNetworkRanked = 81,
    ChatMessageCanQuitWithoutAbandon = 82,
    ChatMessageRankedGameStillScoredLeaversGetLoss = 83,
    ChatMessageAbandonRankedBeforeFirstBloodParty = 84,
    ChatMessageCompendiumLevel = 85,
    ChatMessageVictoryPredictionStreak = 86,
    ChatMessageAssassinAnnounce = 87,
    ChatMessageAssassinSuccess = 88,
    ChatMessageAssassinDenied = 89,
    ChatMessageVictoryPredictionSingleUserConfirm = 90,
    ChatMessageEffigyKill = 91,
    ChatMessageVoiceTextBannedOverflow = 92,
    ChatMessageYearBeastKilled = 93,
    ChatMessagePauseCountdown = 94,
    ChatMessageCoinsWagered = 95,
    ChatMessageHeroNominatedBan = 96,
    ChatMessageHeroBanned = 97,
    ChatMessageHeroBanCount = 98,
    ChatMessageRiverPainted = 99,
    ChatMessageScanUsed = 100,
    ChatMessageShrineKilled = 101,
    ChatMessageWagerTokenSpent = 102,
    ChatMessageRankWager = 103,
    ChatMessageNewPlayerReminder = 104,
    ChatMessageObserverWardKilled = 105,
    ChatMessageSentryWardKilled = 106,
    ChatMessageItemPlacedInNeutralStash = 107,
    ChatMessageHeroChoiceInvalid = 108,
    ChatMessageBounty = 109,
    ChatMessageAbilityDraftStart = 110,
    ChatMessageHeroFoundCandy = 111,
    ChatMessageAbilityDraftRandomed = 112,
    ChatMessagePrivateCoachConnected = 113,
    ChatMessageCantPauseTooEarly = 115,
    ChatMessageHeroKillWithPenguin = 116,
    ChatMessageMinibossKill = 117,
    ChatMessagePlayerInGameBanText = 118,
    ChatMessageBannerPlanted = 119,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaCmPick {
    DotaCmRandom = 0,
    DotaCmGoodGuys = 1,
    DotaCmBadGuys = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaCombatlogTypes {
    DotaCombatlogInvalid = -1,
    DotaCombatlogDamage = 0,
    DotaCombatlogHeal = 1,
    DotaCombatlogModifierAdd = 2,
    DotaCombatlogModifierRemove = 3,
    DotaCombatlogDeath = 4,
    DotaCombatlogAbility = 5,
    DotaCombatlogItem = 6,
    DotaCombatlogLocation = 7,
    DotaCombatlogGold = 8,
    DotaCombatlogGameState = 9,
    DotaCombatlogXp = 10,
    DotaCombatlogPurchase = 11,
    DotaCombatlogBuyback = 12,
    DotaCombatlogAbilityTrigger = 13,
    DotaCombatlogPlayerstats = 14,
    DotaCombatlogMultikill = 15,
    DotaCombatlogKillstreak = 16,
    DotaCombatlogTeamBuildingKill = 17,
    DotaCombatlogFirstBlood = 18,
    DotaCombatlogModifierStackEvent = 19,
    DotaCombatlogNeutralCampStack = 20,
    DotaCombatlogPickupRune = 21,
    DotaCombatlogRevealedInvisible = 22,
    DotaCombatlogHeroSaved = 23,
    DotaCombatlogManaRestored = 24,
    DotaCombatlogHeroLevelup = 25,
    DotaCombatlogBottleHealAlly = 26,
    DotaCombatlogEndgameStats = 27,
    DotaCombatlogInterruptChannel = 28,
    DotaCombatlogAlliedGold = 29,
    DotaCombatlogAegisTaken = 30,
    DotaCombatlogManaDamage = 31,
    DotaCombatlogPhysicalDamagePrevented = 32,
    DotaCombatlogUnitSummoned = 33,
    DotaCombatlogAttackEvade = 34,
    DotaCombatlogTreeCut = 35,
    DotaCombatlogSuccessfulScan = 36,
    DotaCombatlogEndKillstreak = 37,
    DotaCombatlogBloodstoneCharge = 38,
    DotaCombatlogCriticalDamage = 39,
    DotaCombatlogSpellAbsorb = 40,
    DotaCombatlogUnitTeleported = 41,
    DotaCombatlogKillEaterEvent = 42,
    DotaCombatlogNeutralItemEarned = 43,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaCommLevelT {
    DotaCommLevelNone = 0,
    DotaCommLevelCooldown = 1,
    DotaCommLevelPings = 2,
    DotaCommLevelMapdrawing = 3,
    DotaCommLevelChat = 4,
    DotaCommLevelTipping = 5,
    DotaCommLevelVoice = 6,
    DotaCommLevelAlliedAbility = 7,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaCommTypeT {
    DotaCommTypeNone = 0,
    DotaCommTypePing = 1,
    DotaCommTypeChatwheel = 2,
    DotaCommTypeTip = 3,
    DotaCommTypeText = 4,
    DotaCommTypeShowcase = 5,
    DotaCommTypeVoice = 6,
    DotaCommTypeAllyAbility = 7,
    DotaCommTypePause = 8,
    DotaCommTypeCoaching = 9,
    DotaCommTypeNocooldown = 10,
    DotaCommTypeRankedmatchmake = 11,
    DotaCommTypeDrops = 12,
    DotaCommTypeNewplayerExpert = 13,
    DotaCommTypeCoached = 14,
    DotaCommTypeMapdrawing = 15,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaConnectionStateT {
    DotaConnectionStateUnknown = 0,
    DotaConnectionStateNotYetConnected = 1,
    DotaConnectionStateConnected = 2,
    DotaConnectionStateDisconnected = 3,
    DotaConnectionStateAbandoned = 4,
    DotaConnectionStateLoading = 5,
    DotaConnectionStateFailed = 6,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaGameMode {
    DotaGamemodeNone = 0,
    DotaGamemodeAp = 1,
    DotaGamemodeCm = 2,
    DotaGamemodeRd = 3,
    DotaGamemodeSd = 4,
    DotaGamemodeAr = 5,
    DotaGamemodeIntro = 6,
    DotaGamemodeHw = 7,
    DotaGamemodeReverseCm = 8,
    DotaGamemodeXmas = 9,
    DotaGamemodeTutorial = 10,
    DotaGamemodeMo = 11,
    DotaGamemodeLp = 12,
    DotaGamemodePool1 = 13,
    DotaGamemodeFh = 14,
    DotaGamemodeCustom = 15,
    DotaGamemodeCd = 16,
    DotaGamemodeBd = 17,
    DotaGamemodeAbilityDraft = 18,
    DotaGamemodeEvent = 19,
    DotaGamemodeArdm = 20,
    DotaGamemode1v1mid = 21,
    DotaGamemodeAllDraft = 22,
    DotaGamemodeTurbo = 23,
    DotaGamemodeMutation = 24,
    DotaGamemodeCoachesChallenge = 25,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaGameState {
    DotaGamerulesStateInit = 0,
    DotaGamerulesStateWaitForPlayersToLoad = 1,
    DotaGamerulesStateHeroSelection = 2,
    DotaGamerulesStateStrategyTime = 3,
    DotaGamerulesStatePreGame = 4,
    DotaGamerulesStateGameInProgress = 5,
    DotaGamerulesStatePostGame = 6,
    DotaGamerulesStateDisconnect = 7,
    DotaGamerulesStateTeamShowcase = 8,
    DotaGamerulesStateCustomGameSetup = 9,
    DotaGamerulesStateWaitForMapToLoad = 10,
    DotaGamerulesStateScenarioSetup = 11,
    DotaGamerulesStatePlayerDraft = 12,
    DotaGamerulesStateLast = 13,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaGcTeam {
    GoodGuys = 0,
    BadGuys = 1,
    Broadcaster = 2,
    Spectator = 3,
    PlayerPool = 4,
    Noteam = 5,
    Custom1 = 6,
    Custom2 = 7,
    Custom3 = 8,
    Custom4 = 9,
    Custom5 = 10,
    Custom6 = 11,
    Custom7 = 12,
    Custom8 = 13,
    Neutrals = 14,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaJoinLobbyResult {
    DotaJoinResultSuccess = 0,
    DotaJoinResultAlreadyInGame = 1,
    DotaJoinResultInvalidLobby = 2,
    DotaJoinResultIncorrectPassword = 3,
    DotaJoinResultAccessDenied = 4,
    DotaJoinResultGenericError = 5,
    DotaJoinResultIncorrectVersion = 6,
    DotaJoinResultInTeamParty = 7,
    DotaJoinResultNoLobbyFound = 8,
    DotaJoinResultLobbyFull = 9,
    DotaJoinResultCustomGameIncorrectVersion = 10,
    DotaJoinResultTimeout = 11,
    DotaJoinResultCustomGameCooldown = 12,
    DotaJoinResultBusy = 13,
    DotaJoinResultNoPlaytime = 14,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaLeaverStatusT {
    DotaLeaverNone = 0,
    DotaLeaverDisconnected = 1,
    DotaLeaverDisconnectedTooLong = 2,
    DotaLeaverAbandoned = 3,
    DotaLeaverAfk = 4,
    DotaLeaverNeverConnected = 5,
    DotaLeaverNeverConnectedTooLong = 6,
    DotaLeaverFailedToReadyUp = 7,
    DotaLeaverDeclined = 8,
    DotaLeaverDeclinedRequeue = 9,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaLobbyReadyState {
    Undeclared = 0,
    Accepted = 1,
    Declined = 2,
    DeclinedRequeue = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaLobbyVisibility {
    Public = 0,
    Friends = 1,
    Unlisted = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaLowPriorityBanType {
    DotaLowPriorityBanAbandon = 0,
    DotaLowPriorityBanReports = 1,
    DotaLowPriorityBanSecondaryAbandon = 2,
    DotaLowPriorityBanPreGameRole = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaMatchVote {
    Invalid = 0,
    Positive = 1,
    Negative = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaModifierEntryType {
    Active = 1,
    Removed = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaNoBattlePointsReasons {
    NoBattlePointsWrongLobbyType = 1,
    NoBattlePointsPracticeBots = 2,
    NoBattlePointsCheatsEnabled = 3,
    NoBattlePointsLowPriority = 4,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaOverheadAlert {
    OverheadAlertGold = 0,
    OverheadAlertDeny = 1,
    OverheadAlertCritical = 2,
    OverheadAlertXp = 3,
    OverheadAlertBonusSpellDamage = 4,
    OverheadAlertMiss = 5,
    OverheadAlertDamage = 6,
    OverheadAlertEvade = 7,
    OverheadAlertBlock = 8,
    OverheadAlertBonusPoisonDamage = 9,
    OverheadAlertHeal = 10,
    OverheadAlertManaAdd = 11,
    OverheadAlertManaLoss = 12,
    OverheadAlertLastHitEarly = 13,
    OverheadAlertLastHitClose = 14,
    OverheadAlertLastHitMiss = 15,
    OverheadAlertMagicalBlock = 16,
    OverheadAlertIncomingDamage = 17,
    OverheadAlertOutgoingDamage = 18,
    OverheadAlertDisableResist = 19,
    OverheadAlertDeath = 20,
    OverheadAlertBlocked = 21,
    OverheadAlertItemReceived = 22,
    OverheadAlertShard = 23,
    OverheadAlertDeadlyBlow = 24,
    OverheadAlertForceMiss = 25,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaPositionCategory {
    DotaPositionNone = 0,
    DotaPositionBottomLane = 1,
    DotaPositionMidLane = 2,
    DotaPositionTopLane = 3,
    DotaPositionRadiantJungle = 4,
    DotaPositionDireJungle = 5,
    DotaPositionRadiantAncients = 6,
    DotaPositionDireAncients = 7,
    DotaPositionRadiantSecretShop = 8,
    DotaPositionDireSecretShop = 9,
    DotaPositionRiver = 10,
    DotaPositionRoshanPit = 11,
    DotaPositionRadiantBase = 12,
    DotaPositionDireBase = 13,
    DotaPositionFountain = 14,
    DotaPositionOther = 15,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaReplayStateEvent {
    GameStart = 1,
    StartingHorn = 2,
    FirstBlood = 3,
    Showcase = 4,
    PostGame = 5,
    WaitForMap = 6,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaRoshanPhase {
    KSrspRoshanAlive = 0,
    KSrspRoshanBaseTimer = 1,
    KSrspRoshanVisibleTimer = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaSelectionPriorityChoice {
    KDotaSelectionPriorityChoiceInvalid = 0,
    KDotaSelectionPriorityChoiceFirstPick = 1,
    KDotaSelectionPriorityChoiceSecondPick = 2,
    KDotaSelectionPriorityChoiceRadiant = 3,
    KDotaSelectionPriorityChoiceDire = 4,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaSelectionPriorityRules {
    KDotaSelectionPriorityRulesManual = 0,
    KDotaSelectionPriorityRulesAutomatic = 1,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaunitorderT {
    DotaUnitOrderNone = 0,
    DotaUnitOrderMoveToPosition = 1,
    DotaUnitOrderMoveToTarget = 2,
    DotaUnitOrderAttackMove = 3,
    DotaUnitOrderAttackTarget = 4,
    DotaUnitOrderCastPosition = 5,
    DotaUnitOrderCastTarget = 6,
    DotaUnitOrderCastTargetTree = 7,
    DotaUnitOrderCastNoTarget = 8,
    DotaUnitOrderCastToggle = 9,
    DotaUnitOrderHoldPosition = 10,
    DotaUnitOrderTrainAbility = 11,
    DotaUnitOrderDropItem = 12,
    DotaUnitOrderGiveItem = 13,
    DotaUnitOrderPickupItem = 14,
    DotaUnitOrderPickupRune = 15,
    DotaUnitOrderPurchaseItem = 16,
    DotaUnitOrderSellItem = 17,
    DotaUnitOrderDisassembleItem = 18,
    DotaUnitOrderMoveItem = 19,
    DotaUnitOrderCastToggleAuto = 20,
    DotaUnitOrderStop = 21,
    DotaUnitOrderTaunt = 22,
    DotaUnitOrderBuyback = 23,
    DotaUnitOrderGlyph = 24,
    DotaUnitOrderEjectItemFromStash = 25,
    DotaUnitOrderCastRune = 26,
    DotaUnitOrderPingAbility = 27,
    DotaUnitOrderMoveToDirection = 28,
    DotaUnitOrderPatrol = 29,
    DotaUnitOrderVectorTargetPosition = 30,
    DotaUnitOrderRadar = 31,
    DotaUnitOrderSetItemCombineLock = 32,
    DotaUnitOrderContinue = 33,
    DotaUnitOrderVectorTargetCanceled = 34,
    DotaUnitOrderCastRiverPaint = 35,
    DotaUnitOrderPregameAdjustItemAssignment = 36,
    DotaUnitOrderDropItemAtFountain = 37,
    DotaUnitOrderTakeItemFromNeutralItemStash = 38,
    DotaUnitOrderMoveRelative = 39,
    DotaUnitOrderCastToggleAlt = 40,
    DotaUnitOrderConsumeItem = 41,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EBadgeType {
    KEBadgeTypeInvalid = 0,
    KEBadgeTypeTi7Midweek = 1,
    KEBadgeTypeTi7Finals = 2,
    KEBadgeTypeTi7AllEvent = 3,
    KEBadgeTypeTi8Midweek = 4,
    KEBadgeTypeTi8Finals = 5,
    KEBadgeTypeTi8AllEvent = 6,
    KEBadgeTypeTi10 = 7,
    KEBadgeTypeTi11PlayoffsDay1 = 8,
    KEBadgeTypeTi11PlayoffsDay2 = 9,
    KEBadgeTypeTi11PlayoffsDay3 = 10,
    KEBadgeTypeTi11PlayoffsDay4 = 11,
    KEBadgeTypeTi11FinalsWeekend = 12,
    KEBadgeTypeTi12PlayoffsDay1 = 13,
    KEBadgeTypeTi12PlayoffsDay2 = 14,
    KEBadgeTypeTi12PlayoffsDay3 = 15,
    KEBadgeTypeTi12FinalsWeekend = 16,
    KEBadgeTypeTi12Special = 17,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECandyShopUpgrade {
    KECandyShopUpgradeInvalid = -1,
    KECandyShopUpgradeInventorySize = 0,
    KECandyShopUpgradeRewardShelf = 1,
    KECandyShopUpgradeExtraExchangeRecipe = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EChatSpecialPrivileges {
    KEChatSpecialPrivilegesNone = 0,
    KEChatSpecialPrivilegesModerator = 1,
    KEChatSpecialPrivilegesSuperModerator = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EDotaEntityMessages {
    DotaUnitSpeech = 0,
    DotaUnitSpeechMute = 1,
    DotaUnitAddGesture = 2,
    DotaUnitRemoveGesture = 3,
    DotaUnitRemoveAllGestures = 4,
    DotaUnitFadeGesture = 6,
    DotaUnitSpeechClientsideRules = 7,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EDotaUserMessages {
    DotaUmAddUnitToSelection = 464,
    DotaUmAiDebugLine = 465,
    DotaUmChatEvent = 466,
    DotaUmCombatHeroPositions = 467,
    DotaUmCombatLogData = 468,
    DotaUmCombatLogBulkData = 470,
    DotaUmCreateLinearProjectile = 471,
    DotaUmDestroyLinearProjectile = 472,
    DotaUmDodgeTrackingProjectiles = 473,
    DotaUmGlobalLightColor = 474,
    DotaUmGlobalLightDirection = 475,
    DotaUmInvalidCommand = 476,
    DotaUmLocationPing = 477,
    DotaUmMapLine = 478,
    DotaUmMiniKillCamInfo = 479,
    DotaUmMinimapDebugPoint = 480,
    DotaUmMinimapEvent = 481,
    DotaUmNevermoreRequiem = 482,
    DotaUmOverheadEvent = 483,
    DotaUmSetNextAutobuyItem = 484,
    DotaUmSharedCooldown = 485,
    DotaUmSpectatorPlayerClick = 486,
    DotaUmTutorialTipInfo = 487,
    DotaUmUnitEvent = 488,
    DotaUmParticleManager = 489,
    DotaUmBotChat = 490,
    DotaUmHudError = 491,
    DotaUmItemPurchased = 492,
    DotaUmPing = 493,
    DotaUmItemFound = 494,
    DotaUmCharacterSpeakConcept = 495,
    DotaUmSwapVerify = 496,
    DotaUmWorldLine = 497,
    DotaUmTournamentDrop = 498,
    DotaUmItemAlert = 499,
    DotaUmHalloweenDrops = 500,
    DotaUmChatWheel = 501,
    DotaUmReceivedXmasGift = 502,
    DotaUmUpdateSharedContent = 503,
    DotaUmTutorialRequestExp = 504,
    DotaUmTutorialPingMinimap = 505,
    DotaUmGamerulesStateChanged = 506,
    DotaUmShowSurvey = 507,
    DotaUmTutorialFade = 508,
    DotaUmAddQuestLogEntry = 509,
    DotaUmSendStatPopup = 510,
    DotaUmTutorialFinish = 511,
    DotaUmSendRoshanPopup = 512,
    DotaUmSendGenericToolTip = 513,
    DotaUmSendFinalGold = 514,
    DotaUmCustomMsg = 515,
    DotaUmCoachHudPing = 516,
    DotaUmClientLoadGridNav = 517,
    DotaUmTeProjectile = 518,
    DotaUmTeProjectileLoc = 519,
    DotaUmTeDotaBloodImpact = 520,
    DotaUmTeUnitAnimation = 521,
    DotaUmTeUnitAnimationEnd = 522,
    DotaUmAbilityPing = 523,
    DotaUmShowGenericPopup = 524,
    DotaUmVoteStart = 525,
    DotaUmVoteUpdate = 526,
    DotaUmVoteEnd = 527,
    DotaUmBoosterState = 528,
    DotaUmWillPurchaseAlert = 529,
    DotaUmTutorialMinimapPosition = 530,
    DotaUmAbilitySteal = 532,
    DotaUmCourierKilledAlert = 533,
    DotaUmEnemyItemAlert = 534,
    DotaUmStatsMatchDetails = 535,
    DotaUmMiniTaunt = 536,
    DotaUmBuyBackStateAlert = 537,
    DotaUmSpeechBubble = 538,
    DotaUmCustomHeaderMessage = 539,
    DotaUmQuickBuyAlert = 540,
    DotaUmStatsHeroDetails = 541,
    DotaUmPredictionResult = 542,
    DotaUmModifierAlert = 543,
    DotaUmHpManaAlert = 544,
    DotaUmGlyphAlert = 545,
    DotaUmBeastChat = 546,
    DotaUmSpectatorPlayerUnitOrders = 547,
    DotaUmCustomHudElementCreate = 548,
    DotaUmCustomHudElementModify = 549,
    DotaUmCustomHudElementDestroy = 550,
    DotaUmCompendiumState = 551,
    DotaUmProjectionAbility = 552,
    DotaUmProjectionEvent = 553,
    DotaUmCombatLogDataHltv = 554,
    DotaUmXpAlert = 555,
    DotaUmUpdateQuestProgress = 556,
    DotaUmMatchMetadata = 557,
    DotaUmMatchDetails = 558,
    DotaUmQuestStatus = 559,
    DotaUmSuggestHeroPick = 560,
    DotaUmSuggestHeroRole = 561,
    DotaUmKillcamDamageTaken = 562,
    DotaUmSelectPenaltyGold = 563,
    DotaUmRollDiceResult = 564,
    DotaUmFlipCoinResult = 565,
    DotaUmRequestItemSuggestions = 566,
    DotaUmTeamCaptainChanged = 567,
    DotaUmSendRoshanSpectatorPhase = 568,
    DotaUmChatWheelCooldown = 569,
    DotaUmDismissAllStatPopups = 570,
    DotaUmTeDestroyProjectile = 571,
    DotaUmHeroRelicProgress = 572,
    DotaUmAbilityDraftRequestAbility = 573,
    DotaUmItemSold = 574,
    DotaUmDamageReport = 575,
    DotaUmSalutePlayer = 576,
    DotaUmTipAlert = 577,
    DotaUmReplaceQueryUnit = 578,
    DotaUmEmptyTeleportAlert = 579,
    DotaUmMarsArenaOfBloodAttack = 580,
    DotaUmEsArcanaCombo = 581,
    DotaUmEsArcanaComboSummary = 582,
    DotaUmHighFiveLeftHanging = 583,
    DotaUmHighFiveCompleted = 584,
    DotaUmShovelUnearth = 585,
    DotaEmInvokerSpellCast = 586,
    DotaUmRadarAlert = 587,
    DotaUmAllStarEvent = 588,
    DotaUmTalentTreeAlert = 589,
    DotaUmQueuedOrderRemoved = 590,
    DotaUmDebugChallenge = 591,
    DotaUmOmArcanaCombo = 592,
    DotaUmFoundNeutralItem = 593,
    DotaUmOutpostCaptured = 594,
    DotaUmOutpostGrantedXp = 595,
    DotaUmMoveCameraToUnit = 596,
    DotaUmPauseMinigameData = 597,
    DotaUmVersusScenePlayerBehavior = 598,
    DotaUmQoPArcanaSummary = 600,
    DotaUmHotPotatoCreated = 601,
    DotaUmHotPotatoExploded = 602,
    DotaUmWkArcanaProgress = 603,
    DotaUmGuildChallengeProgress = 604,
    DotaUmWrArcanaProgress = 605,
    DotaUmWrArcanaSummary = 606,
    DotaUmEmptyItemSlotAlert = 607,
    DotaUmAghsStatusAlert = 608,
    DotaUmPingConfirmation = 609,
    DotaUmMutedPlayers = 610,
    DotaUmContextualTip = 611,
    DotaUmChatMessage = 612,
    DotaUmNeutralCampAlert = 613,
    DotaUmRockPaperScissorsStarted = 614,
    DotaUmRockPaperScissorsFinished = 615,
    DotaUmDuelOpponentKilled = 616,
    DotaUmDuelAccepted = 617,
    DotaUmDuelRequested = 618,
    DotaUmMuertaReleaseEventAssignedTargetKilled = 619,
    DotaUmPlayerDraftSuggestPick = 620,
    DotaUmPlayerDraftPick = 621,
    DotaUmUpdateLinearProjectileCpData = 622,
    DotaUmGiftPlayer = 623,
    DotaUmFacetPing = 624,
    DotaUmInnatePing = 625,
    DotaUmRoshanTimer = 626,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EEvent {
    EventIdNone = 0,
    EventIdDiretide = 1,
    EventIdSpringFestival = 2,
    EventIdFrostivus2013 = 3,
    EventIdCompendium2014 = 4,
    EventIdNexonPcBang = 5,
    EventIdPwrdDac2015 = 6,
    EventIdNewBloom2015 = 7,
    EventIdInternational2015 = 8,
    EventIdFallMajor2015 = 9,
    EventIdOraclePa = 10,
    EventIdNewBloom2015Prebeast = 11,
    EventIdFrostivus = 12,
    EventIdWinterMajor2016 = 13,
    EventIdInternational2016 = 14,
    EventIdFallMajor2016 = 15,
    EventIdWinterMajor2017 = 16,
    EventIdNewBloom2017 = 17,
    EventIdInternational2017 = 18,
    EventIdPlusSubscription = 19,
    EventIdSinglesDay2017 = 20,
    EventIdFrostivus2017 = 21,
    EventIdInternational2018 = 22,
    EventIdFrostivus2018 = 23,
    EventIdNewBloom2019 = 24,
    EventIdInternational2019 = 25,
    EventIdNewPlayerExperience = 26,
    EventIdFrostivus2019 = 27,
    EventIdNewBloom2020 = 28,
    EventIdInternational2020 = 29,
    EventIdTeamFandom = 30,
    EventIdDiretide2020 = 31,
    EventIdSpring2021 = 32,
    EventIdFall2021 = 33,
    EventIdTeamFandomFall2021 = 34,
    EventIdTeam20212022Tour2 = 35,
    EventIdInternational2022 = 36,
    EventIdTeam20212022Tour3 = 37,
    EventIdTeamInternational2022 = 38,
    EventIdPermanentGrants = 39,
    EventIdMuertaReleaseSpring2023 = 40,
    EventIdTeam2023Tour1 = 41,
    EventIdTeam2023Tour2 = 42,
    EventIdTeam2023Tour3 = 43,
    EventIdInternational2023 = 45,
    EventId10thAnniversary = 46,
    EventIdCrownfall = 47,
    EventIdFrostivus2023 = 48,
    EventIdInternational2024 = 49,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EEventActionScoreMode {
    KEEventActionScoreModeAdd = 0,
    KEEventActionScoreModeMin = 1,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EHeroStatType {
    KEHeroStatTypeNone = 0,
    KEHeroStatTypeAxeTotalDamage = 2000,
    KEHeroStatTypeBattleHungerDamage = 2001,
    KEHeroStatTypeCounterHelixDamage = 2002,
    KEHeroStatTypeCullingBladeDamage = 2003,
    KEHeroStatTypeBerserkersCallCastCount = 2004,
    KEHeroStatTypeBerserkersCallHeroesHitAverage = 2005,
    KEHeroStatTypeBerserkersCallOtherUnitsHit = 2006,
    KEHeroStatTypeBerserkersCallHeroAttacksTaken = 2007,
    KEHeroStatTypeBerserkersCallOtherAttacksTaken = 2008,
    KEHeroStatTypeBattleHungerCastCount = 2009,
    KEHeroStatTypeBattleHungerPotentialDuration = 2010,
    KEHeroStatTypeBattleHungerAverageDuration = 2011,
    KEHeroStatTypeCounterHelixProcCount = 2012,
    KEHeroStatTypeCounterHelixHeroProcCount = 2013,
    KEHeroStatTypeCounterHelixHeroesHitAverage = 2014,
    KEHeroStatTypeCounterHelixOtherUnitsHitCount = 2015,
    KEHeroStatTypeCullingBladeCastCount = 2016,
    KEHeroStatTypeCullingBladeKillCount = 2017,
    KEHeroStatTypeCullingBladeAverageHealthCulled = 2018,
    KEHeroStatTypeCullingBladeAverageDamageAvailable = 2019,
    KEHeroStatTypeCullingBladeHeroBuffAverage = 2020,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELaneType {
    LaneTypeUnknown = 0,
    LaneTypeSafe = 1,
    LaneTypeOff = 2,
    LaneTypeMid = 3,
    LaneTypeJungle = 4,
    LaneTypeRoam = 5,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELeagueAuditAction {
    LeagueAuditActionInvalid = 0,
    LeagueAuditActionLeagueCreate = 1,
    LeagueAuditActionLeagueEdit = 2,
    LeagueAuditActionLeagueDelete = 3,
    LeagueAuditActionLeagueAdminAdd = 4,
    LeagueAuditActionLeagueAdminRevoke = 5,
    LeagueAuditActionLeagueAdminPromote = 6,
    LeagueAuditActionLeagueStreamAdd = 7,
    LeagueAuditActionLeagueStreamRemove = 8,
    LeagueAuditActionLeagueImageUpdated = 9,
    LeagueAuditActionLeagueMessageAdded = 10,
    LeagueAuditActionLeagueSubmitted = 11,
    LeagueAuditActionLeagueSetPrizePool = 12,
    LeagueAuditActionLeagueAddPrizePoolItem = 13,
    LeagueAuditActionLeagueRemovePrizePoolItem = 14,
    LeagueAuditActionLeagueMatchStart = 15,
    LeagueAuditActionLeagueMatchEnd = 16,
    LeagueAuditActionLeagueAddInvitedTeam = 17,
    LeagueAuditActionLeagueRemoveInvitedTeam = 18,
    LeagueAuditActionLeagueStatusChanged = 19,
    LeagueAuditActionLeagueStreamEdit = 20,
    LeagueAuditActionLeagueTeamSwap = 21,
    LeagueAuditActionNodegroupCreate = 100,
    LeagueAuditActionNodegroupDestroy = 101,
    LeagueAuditActionNodegroupAddTeam = 102,
    LeagueAuditActionNodegroupRemoveTeam = 103,
    LeagueAuditActionNodegroupSetAdvancing = 104,
    LeagueAuditActionNodegroupEdit = 105,
    LeagueAuditActionNodegroupPopulate = 106,
    LeagueAuditActionNodegroupCompleted = 107,
    LeagueAuditActionNodegroupSetSecondaryAdvancing = 108,
    LeagueAuditActionNodegroupSetTertiaryAdvancing = 109,
    LeagueAuditActionNodeCreate = 200,
    LeagueAuditActionNodeDestroy = 201,
    LeagueAuditActionNodeAutocreate = 202,
    LeagueAuditActionNodeSetTeam = 203,
    LeagueAuditActionNodeSetSeriesId = 204,
    LeagueAuditActionNodeSetAdvancing = 205,
    LeagueAuditActionNodeSetTime = 206,
    LeagueAuditActionNodeMatchCompleted = 207,
    LeagueAuditActionNodeCompleted = 208,
    LeagueAuditActionNodeEdit = 209,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELeagueBroadcastProvider {
    LeagueBroadcastUnknown = 0,
    LeagueBroadcastSteam = 1,
    LeagueBroadcastTwitch = 2,
    LeagueBroadcastYoutube = 3,
    LeagueBroadcastOther = 100,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELeagueDivision {
    LeagueDivisionUnset = 0,
    LeagueDivisionI = 1,
    LeagueDivisionIi = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELeaguePhase {
    LeaguePhaseUnset = 0,
    LeaguePhaseRegionalQualifier = 1,
    LeaguePhaseGroupStage = 2,
    LeaguePhaseMainEvent = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELeagueRegion {
    LeagueRegionUnset = 0,
    LeagueRegionNa = 1,
    LeagueRegionSa = 2,
    LeagueRegionWeu = 3,
    LeagueRegionEeu = 4,
    LeagueRegionChina = 5,
    LeagueRegionSea = 6,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELeagueStatus {
    LeagueStatusUnset = 0,
    LeagueStatusUnsubmitted = 1,
    LeagueStatusSubmitted = 2,
    LeagueStatusAccepted = 3,
    LeagueStatusRejected = 4,
    LeagueStatusConcluded = 5,
    LeagueStatusDeleted = 6,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELeagueTier {
    LeagueTierUnset = 0,
    LeagueTierAmateur = 1,
    LeagueTierProfessional = 2,
    LeagueTierMinor = 3,
    LeagueTierMajor = 4,
    LeagueTierInternational = 5,
    LeagueTierDpcQualifier = 6,
    LeagueTierDpcLeagueQualifier = 7,
    LeagueTierDpcLeague = 8,
    LeagueTierDpcLeagueFinals = 9,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELeagueTierCategory {
    LeagueTierCategoryAmateur = 1,
    LeagueTierCategoryProfessional = 2,
    LeagueTierCategoryDpc = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EMatchGroupServerStatus {
    KEMatchGroupServerStatusOk = 0,
    KEMatchGroupServerStatusLimitedAvailability = 1,
    KEMatchGroupServerStatusOffline = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EMatchOutcome {
    KEMatchOutcomeUnknown = 0,
    KEMatchOutcomeRadVictory = 2,
    KEMatchOutcomeDireVictory = 3,
    KEMatchOutcomeNeutralVictory = 4,
    KEMatchOutcomeNoTeamWinner = 5,
    KEMatchOutcomeCustom1Victory = 6,
    KEMatchOutcomeCustom2Victory = 7,
    KEMatchOutcomeCustom3Victory = 8,
    KEMatchOutcomeCustom4Victory = 9,
    KEMatchOutcomeCustom5Victory = 10,
    KEMatchOutcomeCustom6Victory = 11,
    KEMatchOutcomeCustom7Victory = 12,
    KEMatchOutcomeCustom8Victory = 13,
    KEMatchOutcomeNotScoredPoorNetworkConditions = 64,
    KEMatchOutcomeNotScoredLeaver = 65,
    KEMatchOutcomeNotScoredServerCrash = 66,
    KEMatchOutcomeNotScoredNeverStarted = 67,
    KEMatchOutcomeNotScoredCanceled = 68,
    KEMatchOutcomeNotScoredSuspicious = 69,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EOverwatchReportReason {
    KEOverwatchReportReasonUnknown = 0,
    KEOverwatchReportReasonCheating = 1,
    KEOverwatchReportReasonFeeding = 2,
    KEOverwatchReportReasonGriefing = 3,
    KEOverwatchReportReasonSuspicious = 4,
    KEOverwatchReportReasonAbilityAbuse = 5,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EPingSource {
    KEPingSourceDefault = 0,
    KEPingSourceWarning = 1,
    KEPingSourceWheel = 2,
    KEPingSourceSystem = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EPlayerChallengeHistoryType {
    KEPlayerChallengeHistoryTypeInvalid = 0,
    KEPlayerChallengeHistoryTypeKillEater = 1,
    KEPlayerChallengeHistoryTypeDotaPlusRelic = 2,
    KEPlayerChallengeHistoryTypeDotaPlusHeroPlayerChallenge = 3,
    KEPlayerChallengeHistoryTypeInGameEventChallenge = 4,
    KEPlayerChallengeHistoryTypeGuildContract = 5,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EPlayerVoiceListenState {
    KPvlsNone = 0,
    KPvlsDeniedChatBanned = 1,
    KPvlsDeniedPartner = 2,
    KPvlsDeniedHltvTalkerNotSpectator = 3,
    KPvlsDeniedHltvNoTalkerPlayerId = 4,
    KPvlsDeniedHltvTalkerNotBroadcaster = 5,
    KPvlsDeniedTeamSpectator = 6,
    KPvlsDeniedStudent = 8,
    KPvlsDeniedPrivateCoach = 9,
    KPvlsDenied = 64,
    KPvlsAllowHltvTalkerIsBroadcaster = 65,
    KPvlsAllowCoBroadcaster = 66,
    KPvlsAllowAllChat = 67,
    KPvlsAllowStudentToCoach = 68,
    KPvlsAllowFellowStudent = 69,
    KPvlsAllowTalkerIsCoach = 70,
    KPvlsAllowCoachHearTeam = 71,
    KPvlsAllowSameTeam = 72,
    KPvlsAllowShowcase = 73,
    KPvlsAllowPrivateCoach = 74,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EProfileCardSlotType {
    KEProfileCardSlotTypeEmpty = 0,
    KEProfileCardSlotTypeStat = 1,
    KEProfileCardSlotTypeTrophy = 2,
    KEProfileCardSlotTypeItem = 3,
    KEProfileCardSlotTypeHero = 4,
    KEProfileCardSlotTypeEmoticon = 5,
    KEProfileCardSlotTypeTeam = 6,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EProjectionEvent {
    EPeFirstBlood = 0,
    EPeKillstreakGodlike = 1,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ERankType {
    KERankTypeInvalid = 0,
    KERankTypeCasual = 1,
    KERankTypeRanked = 2,
    KERankTypeCasualLegacy = 3,
    KERankTypeRankedLegacy = 4,
    KERankTypeCasualGlicko = 5,
    KERankTypeRankedGlicko = 6,
    KERankTypeRankMax = 7,
    KERankTypeBehaviorPrivate = 100,
    KERankTypeBehaviorPublic = 101,
    KERankTypeMax = 102,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ETourneyQueueDeadlineState {
    KETourneyQueueDeadlineStateNormal = 0,
    KETourneyQueueDeadlineStateMissed = 1,
    KETourneyQueueDeadlineStateExpiredOk = 2,
    KETourneyQueueDeadlineStateSeekingBye = 3,
    KETourneyQueueDeadlineStateEligibleForRefund = 4,
    KETourneyQueueDeadlineStateNa = -1,
    KETourneyQueueDeadlineStateExpiringSoon = 101,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EdotaPlayerMmrType {
    KEdotaPlayerMmrTypeInvalid = 0,
    KEdotaPlayerMmrTypeGeneralHidden = 1,
    KEdotaPlayerMmrTypeGeneralCompetitive = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EdotaStatPopupTypes {
    KEdotaSptTextline = 0,
    KEdotaSptBasic = 1,
    KEdotaSptPoll = 2,
    KEdotaSptGrid = 3,
    KEdotaSptDualImage = 4,
    KEdotaSptMovie = 5,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EdotaVersusScenePlayerBehavior {
    VsPlayerBehaviorPlayActivity = 1,
    VsPlayerBehaviorChatWheel = 2,
    VsPlayerBehaviorPlaybackRate = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EdotammrBoostType {
    KEdotammrBoostTypeNone = 0,
    KEdotammrBoostTypeLeader = 1,
    KEdotammrBoostTypeFollower = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EdpcFavoriteType {
    FavoriteTypeAll = 0,
    FavoriteTypePlayer = 1,
    FavoriteTypeTeam = 2,
    FavoriteTypeLeague = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EdpcPushNotification {
    DpcPushNotificationMatchStarting = 1,
    DpcPushNotificationPlayerLeftTeam = 10,
    DpcPushNotificationPlayerJoinedTeam = 11,
    DpcPushNotificationPlayerJoinedTeamAsCoach = 12,
    DpcPushNotificationPlayerLeftTeamAsCoach = 13,
    DpcPushNotificationLeagueResult = 20,
    DpcPushNotificationPredictionMatchesAvailable = 30,
    DpcPushNotificationPredictionResult = 31,
    DpcPushNotificationFantasyPlayerCleared = 40,
    DpcPushNotificationFantasyDailySummary = 41,
    DpcPushNotificationFantasyFinalResults = 42,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FantasyGemType {
    Ruby = 0,
    Sapphire = 1,
    Emerald = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FantasyRoles {
    FantasyRoleUndefined = 0,
    FantasyRoleCore = 1,
    FantasyRoleSupport = 2,
    FantasyRoleOfflane = 3,
    FantasyRoleMid = 4,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FantasyScoring {
    Kills = 0,
    Deaths = 1,
    Cs = 2,
    Gpm = 3,
    TowerKills = 4,
    RoshanKills = 5,
    TeamfightParticipation = 6,
    WardsPlanted = 7,
    CampsStacked = 8,
    RunesGrabbed = 9,
    FirstBlood = 10,
    Stuns = 11,
    SmokesUsed = 12,
    NeutralTokensFound = 13,
    WatchersTaken = 14,
    LotusesGained = 15,
    TormentorKills = 16,
    CourierKills = 17,
    Types = 18,
    Invalid = 19,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FantasySelectionMode {
    FantasySelectionInvalid = 0,
    FantasySelectionLocked = 1,
    FantasySelectionShuffle = 2,
    FantasySelectionFreePick = 3,
    FantasySelectionEnded = 4,
    FantasySelectionPreSeason = 5,
    FantasySelectionPreDraft = 6,
    FantasySelectionDrafting = 7,
    FantasySelectionRegularSeason = 8,
    FantasySelectionCardBased = 9,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FantasyTeamSlots {
    FantasySlotNone = 0,
    FantasySlotCore = 1,
    FantasySlotSupport = 2,
    FantasySlotAny = 3,
    FantasySlotBench = 4,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MatchLanguages {
    MatchLanguageInvalid = 0,
    MatchLanguageEnglish = 1,
    MatchLanguageRussian = 2,
    MatchLanguageChinese = 3,
    MatchLanguageKorean = 4,
    MatchLanguageSpanish = 5,
    MatchLanguagePortuguese = 6,
    MatchLanguageEnglish2 = 7,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MatchType {
    Casual = 0,
    CoopBots = 1,
    Competitive = 4,
    WeekendTourney = 5,
    Event = 7,
    CoachesChallenge = 12,
    NewPlayerPool = 14,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct VersusSceneChatWheel {
    #[prost(uint32, optional, tag = "1", default = "4294967295")]
    pub chat_message_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub emoticon_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersusScenePlayActivity {
    #[prost(message, repeated, tag = "1")]
    pub activities: ::prost::alloc::vec::Vec<versus_scene_play_activity::ActivityInfo>,
    #[prost(float, optional, tag = "2")]
    pub playback_rate: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct VersusScenePlaybackRate {
    #[prost(float, optional, tag = "1")]
    pub rate: ::core::option::Option<f32>,
}

pub mod cdota_response_query_serialized {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Fact {
        #[prost(int32, required, tag = "1")]
        pub key: i32,
        #[prost(enumeration = "fact::ValueType", required, tag = "2", default = "Numeric")]
        pub valtype: i32,
        #[prost(float, optional, tag = "3")]
        pub val_numeric: ::core::option::Option<f32>,
        #[prost(string, optional, tag = "4")]
        pub val_string: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "5")]
        pub val_stringtable_index: ::core::option::Option<i32>,
        #[prost(sint32, optional, tag = "6")]
        pub val_int_numeric: ::core::option::Option<i32>,
    }
    pub mod fact {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ValueType {
            Numeric = 1,
            String = 2,
            StringtableIndex = 3,
            IntNumeric = 4,
        }
    }
}

pub mod cdota_save_game {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Player {
        #[prost(enumeration = "super::DotaGcTeam", optional, tag = "1", default = "GoodGuys")]
        pub team: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "3")]
        pub hero: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SaveInstance {
        #[prost(uint32, optional, tag = "2")]
        pub game_time: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub team1_score: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub team2_score: ::core::option::Option<u32>,
        #[prost(message, repeated, tag = "5")]
        pub player_positions: ::prost::alloc::vec::Vec<save_instance::PlayerPositions>,
        #[prost(uint32, optional, tag = "6")]
        pub save_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "7")]
        pub save_time: ::core::option::Option<u32>,
    }
    pub mod save_instance {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct PlayerPositions {
            #[prost(float, optional, tag = "1")]
            pub x: ::core::option::Option<f32>,
            #[prost(float, optional, tag = "2")]
            pub y: ::core::option::Option<f32>,
        }
    }
}

pub mod cdota_user_msg_all_star_event {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct PlayerScore {
        #[prost(int32, optional, tag = "1", default = "-1")]
        pub player_id: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "2")]
        pub score_sans_kda: ::core::option::Option<u32>,
    }
}

pub mod cdota_user_msg_courier_killed_alert {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct LostItem {
        #[prost(int32, optional, tag = "1", default = "-1")]
        pub item_ability_id: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "2")]
        pub quantity: ::core::option::Option<u32>,
    }
}

pub mod cdota_user_msg_guild_challenge_progress {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct PlayerProgress {
        #[prost(int32, optional, tag = "1", default = "-1")]
        pub player_id: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "6")]
        pub progress: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EChallengeType {
        KEChallengeTypeInvalid = 0,
        KEChallengeTypeCooperative = 1,
        KEChallengeTypeContract = 2,
    }
}

pub mod cdota_user_msg_mini_kill_cam_info {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Attacker {
        #[prost(uint32, optional, tag = "1", default = "16777215")]
        pub attacker: ::core::option::Option<u32>,
        #[prost(int32, optional, tag = "2")]
        pub total_damage: ::core::option::Option<i32>,
        #[prost(message, repeated, tag = "3")]
        pub abilities: ::prost::alloc::vec::Vec<attacker::Ability>,
        #[prost(string, optional, tag = "4")]
        pub attacker_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    pub mod attacker {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Ability {
            #[prost(int32, optional, tag = "1", default = "-1")]
            pub ability_id: ::core::option::Option<i32>,
            #[prost(int32, optional, tag = "2")]
            pub damage: ::core::option::Option<i32>,
        }
    }
}

pub mod cdota_user_msg_pause_minigame_data {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct DataBit {
        #[prost(uint32, optional, tag = "1")]
        pub index: ::core::option::Option<u32>,
        #[prost(int32, optional, tag = "2")]
        pub data: ::core::option::Option<i32>,
        #[prost(int64, optional, tag = "3")]
        pub data_extra: ::core::option::Option<i64>,
    }
}

pub mod cdota_user_msg_stats_hero_position_info {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct PositionPair {
        #[prost(enumeration = "super::DotaPositionCategory", optional, tag = "1", default = "DotaPositionNone")]
        pub position_category: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "2")]
        pub position_count: ::core::option::Option<u32>,
    }
}

pub mod cdota_user_msg_stats_match_details {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CDotaUserMsgStatsFightTeamDetails {
        #[prost(int32, repeated, packed = "false", tag = "1")]
        pub participants: ::prost::alloc::vec::Vec<i32>,
        #[prost(int32, repeated, packed = "false", tag = "2")]
        pub deaths: ::prost::alloc::vec::Vec<i32>,
        #[prost(uint32, optional, tag = "3")]
        pub gold_delta: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub xp_delta: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CDotaUserMsgStatsFightDetails {
        #[prost(float, optional, tag = "1")]
        pub start_time: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "2")]
        pub end_time: ::core::option::Option<f32>,
        #[prost(message, optional, tag = "3")]
        pub radiant_fight_details: ::core::option::Option<CDotaUserMsgStatsFightTeamDetails>,
        #[prost(message, optional, tag = "4")]
        pub dire_fight_details: ::core::option::Option<CDotaUserMsgStatsFightTeamDetails>,
    }
}

pub mod cdota_user_msg_stats_team_minute_details {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct LocationPerformance {
        #[prost(uint32, optional, tag = "1")]
        pub location_category: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub stat_type: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub value: ::core::option::Option<u32>,
    }
}

pub mod cdota_user_msg_unit_event {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Interval {
        #[prost(float, optional, tag = "1")]
        pub start: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "2")]
        pub range: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Speech {
        #[prost(int32, optional, tag = "1")]
        pub speech_concept: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub response: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "3")]
        pub recipient_type: ::core::option::Option<i32>,
        #[prost(bool, optional, tag = "5", default = "false")]
        pub muteable: ::core::option::Option<bool>,
        #[prost(message, optional, tag = "6")]
        pub predelay: ::core::option::Option<Interval>,
        #[prost(uint32, optional, tag = "7")]
        pub flags: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SpeechMute {
        #[prost(float, optional, tag = "1", default = "0.5")]
        pub delay: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct AddGesture {
        #[prost(int32, optional, tag = "1")]
        pub activity: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub slot: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "3", default = "0")]
        pub fade_in: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "4", default = "0.1")]
        pub fade_out: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "5", default = "1")]
        pub playback_rate: ::core::option::Option<f32>,
        #[prost(int32, optional, tag = "6")]
        pub sequence_variant: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RemoveGesture {
        #[prost(int32, optional, tag = "1")]
        pub activity: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct BloodImpact {
        #[prost(int32, optional, tag = "1")]
        pub scale: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub x_normal: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub y_normal: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct FadeGesture {
        #[prost(int32, optional, tag = "1")]
        pub activity: ::core::option::Option<i32>,
    }
}

pub mod versus_scene_play_activity {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActivityInfo {
        #[prost(string, optional, tag = "1")]
        pub activity: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bool, optional, tag = "2")]
        pub disable_auto_kill: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "3")]
        pub force_looping: ::core::option::Option<bool>,
    }
}
