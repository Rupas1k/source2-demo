pub use crate::common::*;

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CBroadcastPostGameDataFrameRequest {
    #[prost(uint32, optional, tag = "1")]
    pub appid: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamid: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub broadcast_id: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub frame_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelEntityMsgBreakablePropSpawnDebris {
    #[prost(message, optional, tag = "1")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
    #[prost(message, optional, tag = "2")]
    pub damage_pos: ::core::option::Option<CMsgVector>,
    #[prost(float, optional, tag = "3")]
    pub damage: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMessageAbilityNotify {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_victim: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entindex_attacker: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub ability_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMessageAuraModifierApplied {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_caster: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entindex_target: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub modifier_type_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "4")]
    pub modifier_serial_number: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "5")]
    pub aura_start_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "6")]
    pub aura_end_time: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMessageBulletHit {
    #[prost(int32, optional, tag = "1")]
    pub shotid: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub pellet: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub hit_entindex: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMessageCurrencyChanged {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_hero_pawn: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub currency_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub currency_source: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub delta: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub notification: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "6", default = "-1")]
    pub entindex_victim: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "7")]
    pub victim_pos: ::core::option::Option<CMsgVector>,
    #[prost(int32, optional, tag = "8")]
    pub playsound: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "9")]
    pub ability_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMessageDamage {
    #[prost(int32, optional, tag = "1")]
    pub damage: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub pre_damage: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub citadel_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "5")]
    pub origin: ::core::option::Option<CMsgVector>,
    #[prost(int32, optional, tag = "6", default = "-1")]
    pub entindex_victim: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7", default = "-1")]
    pub entindex_inflictor: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8", default = "-1")]
    pub entindex_attacker: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9", default = "-1")]
    pub entindex_ability: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub damage_absorbed: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub victim_health_max: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub victim_health_new: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "13")]
    pub flags: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "14")]
    pub ability_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "15")]
    pub attacker_class: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "16")]
    pub victim_class: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "17")]
    pub victim_shield_max: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "18")]
    pub victim_shield_new: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "19")]
    pub hits: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "20")]
    pub health_lost: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMessageGameOver {
    #[prost(int32, optional, tag = "1")]
    pub winning_team: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub just_a_test: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMessageModifierApplied {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_caster: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entindex_parent: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub serial_number: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMessageObjectiveMask {
    #[prost(uint64, optional, tag = "2")]
    pub objective_mask_team0: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub objective_mask_team1: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgAbilitiesChanged {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_purchaser: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entindex_ability: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub ability_id: ::core::option::Option<u32>,
    #[prost(enumeration = "c_citadel_user_msg_abilities_changed::Change", optional, tag = "4", default = "EInvalid")]
    pub change: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgAbilityInterrupted {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_victim: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entindex_interrupter: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub ability_id_interrupted: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub ability_id_interrupter: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub hero_id_interrupter: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgAbilityLateFailure {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_caster: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entindex_ability: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub failure_type: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgAbilityPing {
    #[prost(message, optional, tag = "1")]
    pub ping_data: ::core::option::Option<PingCommonData>,
    #[prost(uint32, optional, tag = "2")]
    pub ability_id: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "3")]
    pub ability_cooldown: ::core::option::Option<f32>,
    #[prost(enumeration = "ChatMsgPingMarkerInfo", optional, tag = "4", default = "KEPingMarkerInfoShowMarkerAndSound")]
    pub ping_marker_and_sound_info: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgBossKilled {
    #[prost(int32, optional, tag = "1")]
    pub objective_team: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub objective_mask_change: ::core::option::Option<i32>,
    #[prost(uint32, required, tag = "3", default = "16777215")]
    pub entity_killed: u32,
    #[prost(int32, required, tag = "4")]
    pub entity_killed_class: i32,
    #[prost(uint32, required, tag = "5", default = "16777215")]
    pub entity_killer: u32,
    #[prost(float, required, tag = "6")]
    pub gametime: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgCameraController {
    #[prost(enumeration = "CameraAction", required, tag = "1", default = "KEActionAddOp")]
    pub action: i32,
    #[prost(enumeration = "CameraOperation", optional, tag = "2", default = "KECameraOpMaintain")]
    pub operation: ::core::option::Option<i32>,
    #[prost(enumeration = "CameraParam", optional, tag = "3", default = "KEParamClearAllOps")]
    pub param: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "4")]
    pub delay: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "11")]
    pub relative_values: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "5")]
    pub context_symbol_id: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "6")]
    pub maintain: ::core::option::Option<c_citadel_user_msg_camera_controller::Maintain>,
    #[prost(message, optional, tag = "7")]
    pub approach: ::core::option::Option<c_citadel_user_msg_camera_controller::Approach>,
    #[prost(message, optional, tag = "8")]
    pub spring: ::core::option::Option<c_citadel_user_msg_camera_controller::Spring>,
    #[prost(message, optional, tag = "9")]
    pub lerp: ::core::option::Option<c_citadel_user_msg_camera_controller::Lerp>,
    #[prost(message, optional, tag = "10")]
    pub lag: ::core::option::Option<c_citadel_user_msg_camera_controller::Lag>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgChatEvent {
    #[prost(enumeration = "ECitadelChatMessage", optional, tag = "1", default = "CitadelChatMessageUnpauseCountdown")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub values: ::prost::alloc::vec::Vec<u32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgChatMsg {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_slot: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub all_chat: ::core::option::Option<bool>,
    #[prost(enumeration = "CMsgLaneColor", optional, tag = "4", default = "KELaneColorInvalid")]
    pub lane_color: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgChatWheel {
    #[prost(uint32, optional, tag = "1")]
    pub chat_message_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_slot: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub pawn_entindex: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "4")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub hero_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "6")]
    pub param_1: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "CMsgLaneColor", optional, tag = "7", default = "KELaneColorInvalid")]
    pub lane_color: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgDeathReplayData {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub killer_scorer: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub killer_inflictor: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub damage_summary: ::core::option::Option<CCitadelUserMsgRecentDamageSummary>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgForceShopClosed {}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgGetDamageStatsResponse {
    #[prost(uint32, optional, tag = "1")]
    pub player_slot: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub ability_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub damage: ::core::option::Option<c_citadel_user_msg_get_damage_stats_response::StatType>,
    #[prost(message, optional, tag = "4")]
    pub healing: ::core::option::Option<c_citadel_user_msg_get_damage_stats_response::StatType>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgGoldHistory {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_player: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub minute_records: ::prost::alloc::vec::Vec<c_citadel_user_msg_gold_history::MinuteRecord>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgHeroKilled {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_victim: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entindex_inflictor: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub entindex_attacker: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub entindex_assisters: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "5", default = "-1")]
    pub entindex_scorer: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub respawn_reason: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgMapLine {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub sender_player_slot: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub mapline: ::core::option::Option<CMsgMapLine>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgMapPing {
    #[prost(message, required, tag = "1")]
    pub ping_data: PingCommonData,
    #[prost(uint32, optional, tag = "2")]
    pub event_type: ::core::option::Option<u32>,
    #[prost(enumeration = "ChatMsgPingMarkerInfo", optional, tag = "3", default = "KEPingMarkerInfoShowMarkerAndSound")]
    pub ping_marker_and_sound_info: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub pinged_enemy_entity: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "5")]
    pub pinged_entity_class: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "6")]
    pub is_minimap_ping: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "7")]
    pub pinged_hero_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "8")]
    pub is_blind_ping: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgObstructedShotFired {}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgParticipantSetLibraryStackFields {
    #[prost(message, required, tag = "1")]
    pub event: CMsgSosSetLibraryStackFields,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgParticipantSetSoundEventParams {
    #[prost(message, required, tag = "1")]
    pub event: CMsgSosSetSoundEventParams,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgParticipantStartSoundEvent {
    #[prost(message, required, tag = "1")]
    pub event: CMsgSosStartSoundEvent,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgParticipantStopSoundEvent {
    #[prost(message, required, tag = "1")]
    pub event: CMsgSosStopSoundEvent,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgParticipantStopSoundEventHash {
    #[prost(message, required, tag = "1")]
    pub event: CMsgSosStopSoundEventHash,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgPingWheel {
    #[prost(message, required, tag = "1")]
    pub ping_data: PingCommonData,
    #[prost(uint32, optional, tag = "2")]
    pub ping_wheel_option_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgPlayerLifetimeStatInfo {
    #[prost(message, repeated, tag = "1")]
    pub stats: ::prost::alloc::vec::Vec<c_citadel_user_msg_player_lifetime_stat_info::Stat>,
    #[prost(uint64, optional, tag = "2")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "3")]
    pub end_of_match: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub is_official_match: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgPostMatchDetails {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub match_details: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgPostProcessingAnim {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_owner: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub clear_all_states: ::core::option::Option<bool>,
    #[prost(enumeration = "PostProcessingGameStates", optional, tag = "3", default = "PostProcStateKilled")]
    pub state: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "4")]
    pub start_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "5")]
    pub fade_in_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "6")]
    pub hold_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "7")]
    pub fade_out_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "8")]
    pub scale: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgQuickResponse {
    #[prost(message, required, tag = "1")]
    pub ping_data: PingCommonData,
    #[prost(uint32, optional, tag = "2")]
    pub responding_to_ping_message_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub responding_to_player_slot: ::core::option::Option<i32>,
    #[prost(enumeration = "CMsgLaneColor", optional, tag = "4", default = "KELaneColorInvalid")]
    pub lane_color: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgRecentDamageSummary {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_slot: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub damage_records: ::prost::alloc::vec::Vec<c_citadel_user_msg_recent_damage_summary::DamageRecord>,
    #[prost(float, optional, tag = "3")]
    pub start_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub end_time: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "5")]
    pub total_damage: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub lost_gold: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "7")]
    pub modifier_records: ::prost::alloc::vec::Vec<c_citadel_user_msg_recent_damage_summary::ModifierRecord>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgReturnIdol {
    #[prost(int32, optional, tag = "1")]
    pub location_index: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub return_location: ::core::option::Option<CMsgVector>,
    #[prost(bool, optional, tag = "3")]
    pub location_enabled: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgSetClientCameraAngles {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_slot: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub camera_angles: ::core::option::Option<CMsgQAngle>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgSpectatorTeamChanged {
    #[prost(int32, optional, tag = "1")]
    pub teamnumber: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgStaminaDrained {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_victim: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub stamina_drained: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgTeamRewards {
    #[prost(uint32, optional, tag = "1")]
    pub xp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub gold: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub winner: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgTriggerDamageFlash {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_flash_victim: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entindex_flash_attacker: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub entindex_flash_hitgroup: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "4")]
    pub flash_value: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub flash_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub flash_flags: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "7")]
    pub flash_position: ::core::option::Option<CMsgVector>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCommunityClanAnnouncementInfo {
    #[prost(uint64, optional, tag = "1")]
    pub gid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub clanid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub posterid: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "4")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "5")]
    pub posttime: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub updatetime: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "7")]
    pub body: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "8")]
    pub commentcount: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "9")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "10")]
    pub language: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "11")]
    pub hidden: ::core::option::Option<bool>,
    #[prost(fixed64, optional, tag = "12")]
    pub forum_topic_id: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCommunityGetClanAnnouncementsRequest {
    #[prost(uint64, optional, tag = "1")]
    pub steamid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub offset: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub maxchars: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "5")]
    pub strip_html: ::core::option::Option<bool>,
    #[prost(string, repeated, tag = "6")]
    pub required_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "7")]
    pub require_no_tags: ::core::option::Option<bool>,
    #[prost(uint32, repeated, packed = "false", tag = "8")]
    pub language_preference: ::prost::alloc::vec::Vec<u32>,
    #[prost(bool, optional, tag = "9")]
    pub hidden_only: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "10")]
    pub only_gid: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "11")]
    pub rtime_oldest_date: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "12")]
    pub include_hidden: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "13")]
    pub include_partner_events: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CCommunityGetClanAnnouncementsResponse {
    #[prost(uint32, optional, tag = "1")]
    pub maxchars: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "2")]
    pub strip_html: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "3")]
    pub announcements: ::prost::alloc::vec::Vec<CCommunityClanAnnouncementInfo>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CExtraMsgBlock {
    #[prost(uint32, optional, tag = "1")]
    pub msg_type: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub contents: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "3")]
    pub msg_key: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "4")]
    pub is_compressed: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CLobbyDataPostMatchSurvey {
    #[prost(message, repeated, tag = "1")]
    pub surveys: ::prost::alloc::vec::Vec<c_lobby_data_post_match_survey::PlayerSurvey>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CModifierTableEntry {
    #[prost(enumeration = "ModifierEntryType", required, tag = "1", default = "Active")]
    pub entry_type: i32,
    #[prost(uint32, required, tag = "2", default = "16777215")]
    pub parent: u32,
    #[prost(uint32, required, tag = "3")]
    pub serial_number: u32,
    #[prost(uint32, optional, tag = "4")]
    pub modifier_subclass: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "5")]
    pub stack_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub max_stack_count: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "7")]
    pub last_applied_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "8", default = "-1")]
    pub duration: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "9", default = "16777215")]
    pub caster: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10", default = "16777215")]
    pub ability: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "11")]
    pub aura_provider_serial_number: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "12", default = "16777215")]
    pub aura_provider_ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "13")]
    pub ability_subclass: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "20")]
    pub bool1: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "21")]
    pub bool2: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "22")]
    pub bool3: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "23")]
    pub bool4: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "25")]
    pub int1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "26")]
    pub int2: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "27")]
    pub int3: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "28")]
    pub int4: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "30")]
    pub float1: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "31")]
    pub float2: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "32")]
    pub float3: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "33")]
    pub float4: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "49")]
    pub float5: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "50")]
    pub float6: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "51")]
    pub float7: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "52")]
    pub float8: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "53")]
    pub float9: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "54")]
    pub float10: ::core::option::Option<f32>,
    #[prost(uint64, optional, tag = "35")]
    pub uint1: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "36")]
    pub uint2: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "37")]
    pub uint3: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "38")]
    pub uint4: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "40")]
    pub vec1: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "41")]
    pub vec2: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "42")]
    pub vec3: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "43")]
    pub vec4: ::core::option::Option<CMsgVector>,
    #[prost(string, optional, tag = "45")]
    pub string1: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "46")]
    pub string2: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "47")]
    pub string3: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "48")]
    pub string4: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgAccountBookStats {
    #[prost(uint32, optional, tag = "1")]
    pub book_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub book_xp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub book_max_xp: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgAccountHeroStats {
    #[prost(uint32, optional, tag = "1")]
    pub hero_id: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub stat_id: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub total_value: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub medals_bronze: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub medals_silver: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub medals_gold: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgAccountStats {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub stats: ::prost::alloc::vec::Vec<CMsgAccountHeroStats>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgAnyToGcReportAsserts {
    #[prost(uint32, optional, tag = "1")]
    pub version: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub asserts: ::prost::alloc::vec::Vec<c_msg_any_to_gc_report_asserts::TrackedAssert>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgAnyToGcReportAssertsResponse {
    #[prost(bool, optional, tag = "1")]
    pub success: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgBulletImpact {
    #[prost(message, optional, tag = "1")]
    pub trace_start: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "2")]
    pub impact_origin: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "3")]
    pub surface_normal: ::core::option::Option<CMsgVector>,
    #[prost(uint32, optional, tag = "4")]
    pub damage: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub surface_type: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "7", default = "-1")]
    pub ability_entindex: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8", default = "-1")]
    pub impacted_entindex: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "9")]
    pub impacted_hitbox: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub weapon_subclass_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "11", default = "-1")]
    pub shooter_entindex: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgClientHello {
    #[prost(uint32, optional, tag = "1")]
    pub version: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub socache_have_versions: ::prost::alloc::vec::Vec<CMsgSoCacheHaveVersion>,
    #[prost(uint32, optional, tag = "3")]
    pub client_session_need: ::core::option::Option<u32>,
    #[prost(enumeration = "PartnerAccountType", optional, tag = "4", default = "PartnerNone")]
    pub client_launcher: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub secret_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "6")]
    pub client_language: ::core::option::Option<u32>,
    #[prost(enumeration = "ESourceEngine", optional, tag = "7", default = "KEseSource1")]
    pub engine: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub steamdatagram_login: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "9")]
    pub platform_id: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "10")]
    pub game_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "11")]
    pub os_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "12")]
    pub render_system: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "13")]
    pub render_system_req: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "14")]
    pub screen_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "15")]
    pub screen_height: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "16")]
    pub screen_refresh: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "17")]
    pub render_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "18")]
    pub render_height: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "19")]
    pub swap_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "20")]
    pub swap_height: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "22")]
    pub is_steam_china: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "24")]
    pub is_steam_china_client: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "23")]
    pub platform_name: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgClientWelcome {
    #[prost(uint32, optional, tag = "1")]
    pub version: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub game_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "3")]
    pub outofdate_subscribed_caches: ::prost::alloc::vec::Vec<CMsgSoCacheSubscribed>,
    #[prost(message, repeated, tag = "4")]
    pub uptodate_subscribed_caches: ::prost::alloc::vec::Vec<CMsgSoCacheSubscriptionCheck>,
    #[prost(message, optional, tag = "5")]
    pub location: ::core::option::Option<c_msg_client_welcome::Location>,
    #[prost(uint32, optional, tag = "9")]
    pub gc_socache_file_version: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "10")]
    pub txn_country_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "11")]
    pub game_data2: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "12")]
    pub rtime32_gc_welcome_timestamp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "13")]
    pub currency: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "14")]
    pub balance: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "15")]
    pub balance_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "16")]
    pub has_accepted_china_ssa: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "17")]
    pub is_banned_steam_china: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "18")]
    pub additional_welcome_msgs: ::core::option::Option<CExtraMsgBlock>,
    #[prost(message, optional, tag = "20")]
    pub steam_learn_server_info: ::core::option::Option<CMsgSteamLearnServerInfo>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgConnectionStatus {
    #[prost(enumeration = "GcConnectionStatus", optional, tag = "1", default = "HaveSession")]
    pub status: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub client_session_need: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub queue_position: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub queue_size: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub wait_seconds: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub estimated_wait_seconds_remaining: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgDisableSatVolumesEvent {}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgEnableSatVolumesEvent {
    #[prost(uint32, optional, tag = "1")]
    pub mode: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub desat_amount: ::core::option::Option<f32>,
    #[prost(fixed32, optional, tag = "3")]
    pub sat_tint: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "4")]
    pub desat_tint: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "5")]
    pub outline_color: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgFireBullets {
    #[prost(message, optional, tag = "1")]
    pub origin: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "2")]
    pub angles: ::core::option::Option<CMsgQAngle>,
    #[prost(uint32, optional, tag = "4")]
    pub seed: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "5", default = "-1")]
    pub shooter_entity: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7", default = "-1")]
    pub ability: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "8")]
    pub penetration_percent: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "9")]
    pub spread: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "10", default = "true")]
    pub fired_from_gun: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "11")]
    pub bullets_override: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "12")]
    pub tracer_replacement: ::core::option::Option<c_msg_fire_bullets::TracerAssignment>,
    #[prost(message, repeated, tag = "13")]
    pub tracer_additional: ::prost::alloc::vec::Vec<c_msg_fire_bullets::TracerAssignment>,
    #[prost(message, optional, tag = "14")]
    pub angles_original: ::core::option::Option<CMsgQAngle>,
    #[prost(uint32, optional, tag = "15")]
    pub weapon_subclass_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "16")]
    pub shot_number: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "17", default = "-1")]
    pub ignore_entity: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "18")]
    pub max_range: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcAssertJobData {
    #[prost(string, optional, tag = "1")]
    pub message_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub message_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcClientPing {}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcConCommand {
    #[prost(string, optional, tag = "1")]
    pub command: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcMultiplexMessage {
    #[prost(uint32, optional, tag = "1")]
    pub msgtype: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub payload: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(fixed64, repeated, packed = "false", tag = "3")]
    pub steamids: ::prost::alloc::vec::Vec<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcRequestSubGcSessionInfo {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcRequestSubGcSessionInfoResponse {
    #[prost(fixed32, optional, tag = "1")]
    pub ip: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "2")]
    pub trusted: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "3")]
    pub port: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "4")]
    pub success: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcToClientPollConvarRequest {
    #[prost(string, optional, tag = "1")]
    pub convar_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "2")]
    pub poll_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcToClientPollConvarResponse {
    #[prost(uint32, optional, tag = "1")]
    pub poll_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub convar_value: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcToClientRequestDropped {}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcForwardAccountDetails {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "2")]
    pub account_details: ::core::option::Option<CgcSystemMsgGetAccountDetailsResponse>,
    #[prost(uint32, optional, tag = "3")]
    pub age_seconds: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcLoadSessionSoCache {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub forward_account_details: ::core::option::Option<CMsgGcToGcForwardAccountDetails>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcLoadSessionSoCacheResponse {}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcMasterBroadcastMessage {
    #[prost(uint32, optional, tag = "1")]
    pub users_per_second: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "2")]
    pub send_to_users: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub send_to_servers: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "4")]
    pub msg_id: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub msg_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcMasterDestroyCache {
    #[prost(uint32, optional, tag = "1")]
    pub soid_type: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub soid_id: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcMasterSubscribeToCache {
    #[prost(uint32, optional, tag = "1")]
    pub soid_type: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub soid_id: ::core::option::Option<u64>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub account_ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(fixed64, repeated, packed = "false", tag = "4")]
    pub steam_ids: ::prost::alloc::vec::Vec<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcMasterSubscribeToCacheAsync {
    #[prost(message, optional, tag = "1")]
    pub subscribe_msg: ::core::option::Option<CMsgGcToGcMasterSubscribeToCache>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcMasterSubscribeToCacheResponse {}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcMasterUnsubscribeFromCache {
    #[prost(uint32, optional, tag = "1")]
    pub soid_type: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub soid_id: ::core::option::Option<u64>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub account_ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(fixed64, repeated, packed = "false", tag = "4")]
    pub steam_ids: ::prost::alloc::vec::Vec<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcSubGcStarting {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub dir_index: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcUniverseStartup {
    #[prost(bool, optional, tag = "1")]
    pub is_initial_startup: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcUniverseStartupResponse {
    #[prost(int32, optional, tag = "1")]
    pub eresult: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcUpdateSessionStats {
    #[prost(uint32, optional, tag = "1")]
    pub user_sessions: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub server_sessions: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub in_logon_surge: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcsoCacheSubscribe {
    #[prost(fixed64, optional, tag = "1")]
    pub subscriber: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub subscribe_to_id: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub sync_version: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "4")]
    pub have_versions: ::prost::alloc::vec::Vec<c_msg_gc_to_gcso_cache_subscribe::CMsgHaveVersions>,
    #[prost(uint32, optional, tag = "5")]
    pub subscribe_to_type: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcToGcsoCacheUnsubscribe {
    #[prost(fixed64, optional, tag = "1")]
    pub subscriber: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub unsubscribe_from_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub unsubscribe_from_type: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcUpdateSubGcSessionInfo {
    #[prost(message, repeated, tag = "1")]
    pub updates: ::prost::alloc::vec::Vec<c_msg_gc_update_sub_gc_session_info::CMsgUpdate>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgHeroSelectionMatchInfo {
    #[prost(message, repeated, tag = "1")]
    pub hero_selections: ::prost::alloc::vec::Vec<c_msg_hero_selection_match_info::Hero>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CMsgLaneColor {
    KELaneColorInvalid = 0,
    KELaneColorYellow = 1,
    KELaneColorOrange = 3,
    KELaneColorBlue = 4,
    KELaneColorPurple = 6,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgMapLine {
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
pub struct CMsgMatchMetaData {
    #[prost(uint32, optional, tag = "1")]
    pub version: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub match_details: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "3")]
    pub match_id: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgMatchMetaDataContents {
    #[prost(message, optional, tag = "2")]
    pub match_info: ::core::option::Option<c_msg_match_meta_data_contents::MatchInfo>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgMatchPlayerDamageMatrix {
    #[prost(message, repeated, tag = "1")]
    pub damage_dealers: ::prost::alloc::vec::Vec<c_msg_match_player_damage_matrix::DamageDealer>,
    #[prost(uint32, repeated, tag = "2")]
    pub sample_time_s: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, optional, tag = "3")]
    pub source_details: ::core::option::Option<c_msg_match_player_damage_matrix::SourceDetails>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgMatchPlayerPathsData {
    #[prost(uint32, optional, tag = "1")]
    pub version: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub interval_s: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "3")]
    pub x_resolution: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub y_resolution: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "5")]
    pub paths: ::prost::alloc::vec::Vec<c_msg_match_player_paths_data::Path>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgParticleSystemManager {
    #[prost(enumeration = "ParticleSystemManagerMessage", required, tag = "1", default = "ParticleSystemManagerEventCreate")]
    pub r#type: i32,
    #[prost(uint32, required, tag = "2")]
    pub index: u32,
    #[prost(message, optional, tag = "3")]
    pub create_particle: ::core::option::Option<c_msg_particle_system_manager::CreateParticle>,
    #[prost(message, optional, tag = "4")]
    pub destroy_particle: ::core::option::Option<c_msg_particle_system_manager::DestroyParticle>,
    #[prost(message, optional, tag = "5")]
    pub destroy_particle_involving: ::core::option::Option<c_msg_particle_system_manager::DestroyParticleInvolving>,
    #[prost(message, optional, tag = "6")]
    pub release_particle_index: ::core::option::Option<c_msg_particle_system_manager::ReleaseParticleIndex>,
    #[prost(message, optional, tag = "7")]
    pub update_particle: ::core::option::Option<c_msg_particle_system_manager::UpdateParticle>,
    #[prost(message, optional, tag = "8")]
    pub update_particle_fwd: ::core::option::Option<c_msg_particle_system_manager::UpdateParticleFwd>,
    #[prost(message, optional, tag = "9")]
    pub update_particle_orient: ::core::option::Option<c_msg_particle_system_manager::UpdateParticleOrient>,
    #[prost(message, optional, tag = "10")]
    pub update_particle_fallback: ::core::option::Option<c_msg_particle_system_manager::UpdateParticleFallback>,
    #[prost(message, optional, tag = "11")]
    pub update_particle_offset: ::core::option::Option<c_msg_particle_system_manager::UpdateParticleOffset>,
    #[prost(message, optional, tag = "12")]
    pub update_particle_ent: ::core::option::Option<c_msg_particle_system_manager::UpdateParticleEnt>,
    #[prost(message, optional, tag = "13")]
    pub update_particle_frozen: ::core::option::Option<c_msg_particle_system_manager::UpdateParticleFrozen>,
    #[prost(message, optional, tag = "14")]
    pub update_particle_should_draw: ::core::option::Option<c_msg_particle_system_manager::UpdateParticleShouldDraw>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgPlaceSatVolumeEvent {
    #[prost(message, optional, tag = "1")]
    pub position: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "2")]
    pub direction: ::core::option::Option<CMsgVector>,
    #[prost(float, optional, tag = "3")]
    pub radius: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub falloff_distance: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "5")]
    pub theta_dot: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "6")]
    pub phi_dot: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "7", default = "16777215")]
    pub entity_handle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub attachment_handle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "10")]
    pub volume_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgPlayerAnimEvent {
    #[prost(fixed32, optional, tag = "1", default = "16777215")]
    pub player: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub event: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub data: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgProtoBufHeader {
    #[prost(fixed64, optional, tag = "1")]
    pub client_steam_id: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "2")]
    pub client_session_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub source_app_id: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag = "10", default = "18446744073709551615")]
    pub job_id_source: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag = "11", default = "18446744073709551615")]
    pub job_id_target: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "12")]
    pub target_job_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "13", default = "2")]
    pub eresult: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "14")]
    pub error_message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "GcProtoBufMsgSrc", optional, tag = "200", default = "Unspecified")]
    pub gc_msg_src: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "201", default = "-1")]
    pub gc_dir_index_source: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgRegionPingTimesClient {
    #[prost(fixed32, repeated, tag = "1")]
    pub data_center_codes: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "2")]
    pub ping_times: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgRemoveSatVolumeEvent {
    #[prost(int32, optional, tag = "1")]
    pub volume_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgScreenTextPretty {
    #[prost(float, optional, tag = "1")]
    pub x_pos: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y_pos: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "3")]
    pub line: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "5")]
    pub r: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub g: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub b: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub a: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "9")]
    pub duration: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "10")]
    pub font_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "11")]
    pub font_size: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "12")]
    pub bold_font: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSdoAssert {
    #[prost(int32, optional, tag = "1")]
    pub sdo_type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<c_msg_sdo_assert::Request>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSerializedSoCache {
    #[prost(uint32, optional, tag = "1")]
    pub file_version: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub caches: ::prost::alloc::vec::Vec<c_msg_serialized_so_cache::Cache>,
    #[prost(uint32, optional, tag = "3")]
    pub gc_socache_file_version: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgServerRequestedTracer {
    #[prost(message, optional, tag = "1")]
    pub origin: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<CMsgVector>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub weaponid: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "4", default = "16777215")]
    pub entity_handle: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "5")]
    pub dps: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgSoCacheHaveVersion {
    #[prost(message, optional, tag = "1")]
    pub soid: ::core::option::Option<CMsgSoidOwner>,
    #[prost(fixed64, optional, tag = "2")]
    pub version: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub service_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub cached_file_version: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSoCacheSubscribed {
    #[prost(message, repeated, tag = "2")]
    pub objects: ::prost::alloc::vec::Vec<c_msg_so_cache_subscribed::SubscribedType>,
    #[prost(fixed64, optional, tag = "3")]
    pub version: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "4")]
    pub owner_soid: ::core::option::Option<CMsgSoidOwner>,
    #[prost(uint32, optional, tag = "5")]
    pub service_id: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub service_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(fixed64, optional, tag = "7")]
    pub sync_version: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSoCacheSubscribedUpToDate {
    #[prost(fixed64, optional, tag = "1")]
    pub version: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "2")]
    pub owner_soid: ::core::option::Option<CMsgSoidOwner>,
    #[prost(uint32, optional, tag = "3")]
    pub service_id: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub service_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(fixed64, optional, tag = "5")]
    pub sync_version: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSoCacheSubscriptionCheck {
    #[prost(fixed64, optional, tag = "2")]
    pub version: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "3")]
    pub owner_soid: ::core::option::Option<CMsgSoidOwner>,
    #[prost(uint32, optional, tag = "4")]
    pub service_id: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub service_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(fixed64, optional, tag = "6")]
    pub sync_version: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgSoCacheSubscriptionRefresh {
    #[prost(message, optional, tag = "2")]
    pub owner_soid: ::core::option::Option<CMsgSoidOwner>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgSoCacheUnsubscribed {
    #[prost(message, optional, tag = "2")]
    pub owner_soid: ::core::option::Option<CMsgSoidOwner>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgSoCacheVersion {
    #[prost(fixed64, optional, tag = "1")]
    pub version: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSoMultipleObjects {
    #[prost(message, repeated, tag = "2")]
    pub objects_modified: ::prost::alloc::vec::Vec<c_msg_so_multiple_objects::SingleObject>,
    #[prost(fixed64, optional, tag = "3")]
    pub version: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "4")]
    pub objects_added: ::prost::alloc::vec::Vec<c_msg_so_multiple_objects::SingleObject>,
    #[prost(message, repeated, tag = "5")]
    pub objects_removed: ::prost::alloc::vec::Vec<c_msg_so_multiple_objects::SingleObject>,
    #[prost(message, optional, tag = "6")]
    pub owner_soid: ::core::option::Option<CMsgSoidOwner>,
    #[prost(uint32, optional, tag = "7")]
    pub service_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSoSingleObject {
    #[prost(int32, optional, tag = "2")]
    pub type_id: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub object_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(fixed64, optional, tag = "4")]
    pub version: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "5")]
    pub owner_soid: ::core::option::Option<CMsgSoidOwner>,
    #[prost(uint32, optional, tag = "6")]
    pub service_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgSoidOwner {
    #[prost(uint32, optional, tag = "1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub id: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgStartFindingMatchInfo {
    #[prost(string, optional, tag = "1")]
    pub server_search_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub server_command_string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "ECitadelMatchMode", optional, tag = "3", default = "KECitadelMatchModeInvalid")]
    pub match_mode: ::core::option::Option<i32>,
    #[prost(enumeration = "ECitadelGameMode", optional, tag = "5", default = "KECitadelGameModeInvalid")]
    pub game_mode: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "6")]
    pub solo_match: ::core::option::Option<bool>,
    #[prost(enumeration = "ECitadelBotDifficulty", optional, tag = "7", default = "KECitadelBotDifficultyNone")]
    pub bot_difficulty: ::core::option::Option<i32>,
    #[prost(enumeration = "ECitadelRegionMode", optional, tag = "8", default = "KECitadelRegionModeRow")]
    pub region_mode: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnAccessTokens {
    #[prost(string, optional, tag = "1")]
    pub register_data_source_access_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub cache_data_access_tokens: ::prost::alloc::vec::Vec<c_msg_steam_learn_access_tokens::CacheDataAccessToken>,
    #[prost(message, repeated, tag = "3")]
    pub snapshot_project_access_tokens: ::prost::alloc::vec::Vec<c_msg_steam_learn_access_tokens::SnapshotProjectAccessToken>,
    #[prost(message, repeated, tag = "4")]
    pub inference_access_tokens: ::prost::alloc::vec::Vec<c_msg_steam_learn_access_tokens::InferenceAccessToken>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnBatchOperationRequest {
    #[prost(message, repeated, tag = "1")]
    pub cache_data_requests: ::prost::alloc::vec::Vec<CMsgSteamLearnCacheDataRequest>,
    #[prost(message, repeated, tag = "2")]
    pub snapshot_requests: ::prost::alloc::vec::Vec<CMsgSteamLearnSnapshotProjectRequest>,
    #[prost(message, repeated, tag = "3")]
    pub inference_requests: ::prost::alloc::vec::Vec<CMsgSteamLearnInferenceRequest>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnBatchOperationResponse {
    #[prost(message, repeated, tag = "1")]
    pub cache_data_responses: ::prost::alloc::vec::Vec<CMsgSteamLearnCacheDataResponse>,
    #[prost(message, repeated, tag = "2")]
    pub snapshot_responses: ::prost::alloc::vec::Vec<CMsgSteamLearnSnapshotProjectResponse>,
    #[prost(message, repeated, tag = "3")]
    pub inference_responses: ::prost::alloc::vec::Vec<CMsgSteamLearnInferenceResponse>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnCacheDataRequest {
    #[prost(string, optional, tag = "1")]
    pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<CMsgSteamLearnData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnCacheDataResponse {
    #[prost(enumeration = "ESteamLearnCacheDataResult", optional, tag = "1", default = "SteamlearnCacheDataError")]
    pub cache_data_result: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnData {
    #[prost(uint32, optional, tag = "1")]
    pub data_source_id: ::core::option::Option<u32>,
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "3")]
    pub data_object: ::core::option::Option<CMsgSteamLearnDataObject>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnDataElement {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, repeated, packed = "false", tag = "20")]
    pub data_int32s: ::prost::alloc::vec::Vec<i32>,
    #[prost(float, repeated, packed = "false", tag = "21")]
    pub data_floats: ::prost::alloc::vec::Vec<f32>,
    #[prost(bool, repeated, packed = "false", tag = "22")]
    pub data_bools: ::prost::alloc::vec::Vec<bool>,
    #[prost(string, repeated, tag = "23")]
    pub data_strings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "24")]
    pub data_objects: ::prost::alloc::vec::Vec<CMsgSteamLearnDataObject>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnDataList {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<CMsgSteamLearnData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnDataObject {
    #[prost(message, repeated, tag = "1")]
    pub elements: ::prost::alloc::vec::Vec<CMsgSteamLearnDataElement>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnDataSource {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub version: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "4")]
    pub source_description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub structure: ::core::option::Option<CMsgSteamLearnDataSourceDescObject>,
    #[prost(uint32, optional, tag = "6")]
    pub structure_crc: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub cache_duration_seconds: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnDataSourceDescElement {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "ESteamLearnDataType", optional, tag = "2", default = "SteamlearnDatatypeInvalid")]
    pub data_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub object: ::core::option::Option<CMsgSteamLearnDataSourceDescObject>,
    #[prost(uint32, optional, tag = "4")]
    pub count: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnDataSourceDescObject {
    #[prost(message, repeated, tag = "1")]
    pub elements: ::prost::alloc::vec::Vec<CMsgSteamLearnDataSourceDescElement>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnGetAccessTokensRequest {
    #[prost(uint32, optional, tag = "1")]
    pub appid: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnGetAccessTokensResponse {
    #[prost(enumeration = "ESteamLearnGetAccessTokensResult", optional, tag = "1", default = "SteamlearnGetAccessTokensError")]
    pub result: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub access_tokens: ::core::option::Option<CMsgSteamLearnAccessTokens>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnInferenceBackendResponse {
    #[prost(message, repeated, tag = "1")]
    pub outputs: ::prost::alloc::vec::Vec<c_msg_steam_learn_inference_backend_response::Output>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnInferenceMetadataBackendRequest {
    #[prost(uint32, optional, tag = "1")]
    pub project_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub fetch_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnInferenceMetadataRequest {
    #[prost(string, optional, tag = "1")]
    pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub project_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub published_version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub override_train_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnInferenceMetadataResponse {
    #[prost(enumeration = "ESteamLearnInferenceMetadataResult", optional, tag = "1", default = "SteamlearnInferenceMetadataError")]
    pub inference_metadata_result: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub row_range: ::core::option::Option<c_msg_steam_learn_inference_metadata_response::RowRange>,
    #[prost(message, repeated, tag = "3")]
    pub ranges: ::prost::alloc::vec::Vec<c_msg_steam_learn_inference_metadata_response::Range>,
    #[prost(message, repeated, tag = "4")]
    pub std_devs: ::prost::alloc::vec::Vec<c_msg_steam_learn_inference_metadata_response::StdDev>,
    #[prost(message, repeated, tag = "5")]
    pub compact_tables: ::prost::alloc::vec::Vec<c_msg_steam_learn_inference_metadata_response::CompactTable>,
    #[prost(message, repeated, tag = "9")]
    pub sequence_tables: ::prost::alloc::vec::Vec<c_msg_steam_learn_inference_metadata_response::SequenceTable>,
    #[prost(message, repeated, tag = "6")]
    pub kmeans: ::prost::alloc::vec::Vec<c_msg_steam_learn_inference_metadata_response::KMeans>,
    #[prost(message, repeated, tag = "8")]
    pub app_info: ::prost::alloc::vec::Vec<c_msg_steam_learn_inference_metadata_response::AppInfoEntry>,
    #[prost(message, optional, tag = "7")]
    pub snapshot_histogram: ::core::option::Option<c_msg_steam_learn_inference_metadata_response::SnapshotHistogram>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnInferenceRequest {
    #[prost(string, optional, tag = "1")]
    pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub project_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub published_version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub override_train_id: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "6")]
    pub data: ::core::option::Option<CMsgSteamLearnDataList>,
    #[prost(float, repeated, packed = "false", tag = "7")]
    pub additional_data: ::prost::alloc::vec::Vec<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnInferenceResponse {
    #[prost(enumeration = "ESteamLearnInferenceResult", optional, tag = "1", default = "SteamlearnInferenceError")]
    pub inference_result: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub backend_response: ::core::option::Option<CMsgSteamLearnInferenceBackendResponse>,
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub keys: ::prost::alloc::vec::Vec<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnRegisterDataSourceRequest {
    #[prost(string, optional, tag = "1")]
    pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub data_source: ::core::option::Option<CMsgSteamLearnDataSource>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnRegisterDataSourceResponse {
    #[prost(enumeration = "ESteammLearnRegisterDataSourceResult", optional, tag = "1", default = "SteamlearnRegisterDataSourceResultError")]
    pub result: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub data_source: ::core::option::Option<CMsgSteamLearnDataSource>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnServerInfo {
    #[prost(message, optional, tag = "4")]
    pub access_tokens: ::core::option::Option<CMsgSteamLearnAccessTokens>,
    #[prost(message, repeated, tag = "5")]
    pub project_infos: ::prost::alloc::vec::Vec<c_msg_steam_learn_server_info::ProjectInfo>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnSnapshotProjectRequest {
    #[prost(string, optional, tag = "1")]
    pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub project_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub published_version: ::core::option::Option<u32>,
    #[prost(uint64, repeated, packed = "false", tag = "4")]
    pub keys: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag = "5")]
    pub data: ::prost::alloc::vec::Vec<CMsgSteamLearnData>,
    #[prost(uint32, optional, tag = "6")]
    pub pending_data_limit_seconds: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnSnapshotProjectResponse {
    #[prost(enumeration = "ESteamLearnSnapshotProjectResult", optional, tag = "1", default = "SteamlearnSnapshotProjectError")]
    pub snapshot_result: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageEmpty {
    #[prost(uint32, optional, tag = "1")]
    pub empty: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CWorkshopGetContributorsRequest {
    #[prost(uint32, optional, tag = "1")]
    pub appid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub gameitemid: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CWorkshopGetContributorsResponse {
    #[prost(fixed64, repeated, packed = "false", tag = "1")]
    pub contributors: ::prost::alloc::vec::Vec<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CWorkshopPopulateItemDescriptionsRequest {
    #[prost(uint32, optional, tag = "1")]
    pub appid: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub languages: ::prost::alloc::vec::Vec<c_workshop_populate_item_descriptions_request::ItemDescriptionsLanguageBlock>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CWorkshopSetItemPaymentRulesRequest {
    #[prost(uint32, optional, tag = "1")]
    pub appid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub gameitemid: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "3")]
    pub associated_workshop_files: ::prost::alloc::vec::Vec<c_workshop_set_item_payment_rules_request::WorkshopItemPaymentRule>,
    #[prost(message, repeated, tag = "4")]
    pub partner_accounts: ::prost::alloc::vec::Vec<c_workshop_set_item_payment_rules_request::PartnerItemPaymentRule>,
    #[prost(bool, optional, tag = "5")]
    pub validate_only: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub make_workshop_files_subscribable: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "7")]
    pub associated_workshop_file_for_direct_payments: ::core::option::Option<c_workshop_set_item_payment_rules_request::WorkshopDirectPaymentRule>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CWorkshopSetItemPaymentRulesResponse {
    #[prost(string, repeated, tag = "1")]
    pub validation_errors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CameraAction {
    KEActionAddOp = 0,
    KEActionClearAllOps = 1,
    KEActionClearOpsForContext = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CameraOperation {
    KECameraOpMaintain = 2,
    KECameraOpApproach = 3,
    KECameraOpSpring = 4,
    KECameraOpLerp = 5,
    KECameraOpLag = 6,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CameraParam {
    KEParamClearAllOps = 0,
    KEParamClearAllOpsForContext = 1,
    KEParamDistance = 2,
    KEParamFov = 3,
    KEParamTargetPosition = 4,
    KEParamVertOffset = 5,
    KEParamHorizOffset = 6,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CgcMsgCompressedMsgToClient {
    #[prost(uint32, optional, tag = "1")]
    pub msg_id: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub compressed_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CgcMsgGetIpLocationResponse {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<CipLocationInfo>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CgcSystemMsgGetAccountDetails {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub appid: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CgcSystemMsgGetAccountDetailsResponse {
    #[prost(uint32, optional, tag = "1", default = "2")]
    pub eresult_deprecated: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub account_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub persona_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "26")]
    pub is_profile_created: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub is_profile_public: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub is_inventory_public: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub is_vac_banned: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub is_cyber_cafe: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "9")]
    pub is_school_account: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "10")]
    pub is_limited: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "11")]
    pub is_subscribed: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "12")]
    pub package: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "13")]
    pub is_free_trial_account: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "14")]
    pub free_trial_expiration: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "15")]
    pub is_low_violence: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "16")]
    pub is_account_locked_down: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "17")]
    pub is_community_banned: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "18")]
    pub is_trade_banned: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "19")]
    pub trade_ban_expiration: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "20")]
    pub accountid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "21")]
    pub suspension_end_time: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "22")]
    pub currency: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "23")]
    pub steam_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "24")]
    pub friend_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "25")]
    pub account_creation_time: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "27")]
    pub is_steamguard_enabled: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "28")]
    pub is_phone_verified: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "29")]
    pub is_two_factor_auth_enabled: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "30")]
    pub two_factor_enabled_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "31")]
    pub phone_verification_time: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "33")]
    pub phone_id: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "34")]
    pub is_phone_identifying: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "35")]
    pub rt_identity_linked: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "36")]
    pub rt_birth_date: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "37")]
    pub txn_country_code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "38")]
    pub has_accepted_china_ssa: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "39")]
    pub is_banned_steam_china: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CgcToGcMsgMasterAck {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub dir_index: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub machine_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub process_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub directory: ::prost::alloc::vec::Vec<cgc_to_gc_msg_master_ack::Process>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CgcToGcMsgMasterAckResponse {
    #[prost(int32, optional, tag = "1", default = "2")]
    pub eresult: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CgcToGcMsgMasterStartupComplete {
    #[prost(message, repeated, tag = "1")]
    pub gc_info: ::prost::alloc::vec::Vec<cgc_to_gc_msg_master_startup_complete::GcInfo>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CgcToGcMsgRouted {
    #[prost(uint32, optional, tag = "1")]
    pub msg_type: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub sender_id: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub net_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CgcToGcMsgRoutedReply {
    #[prost(uint32, optional, tag = "1")]
    pub msg_type: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub net_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChatMsgPingMarkerInfo {
    KEPingMarkerInfoShowMarkerAndSound = 0,
    KEPingMarkerInfoHideMarkerAndSound = 1,
    KEPingMarkerInfoShowMarkerOnSender = 2,
    KEPingMarkerInfoOnlyShowMarker = 3,
    KEPingMarkerInfoOnlyPlaySound = 4,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CipLocationInfo {
    #[prost(uint32, optional, tag = "1")]
    pub ip: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub latitude: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub longitude: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "4")]
    pub country: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub state: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CitadelEntityMessageIds {
    KEEntityMsgBreakablePropSpawnDebris = 500,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CitadelUserMessageIds {
    KEUserMsgDamage = 300,
    KEUserMsgMapPing = 303,
    KEUserMsgTeamRewards = 304,
    KEUserMsgAbilityFailed = 306,
    KEUserMsgTriggerDamageFlash = 308,
    KEUserMsgAbilitiesChanged = 309,
    KEUserMsgRecentDamageSummary = 310,
    KEUserMsgSpectatorTeamChanged = 311,
    KEUserMsgChatWheel = 312,
    KEUserMsgGoldHistory = 313,
    KEUserMsgChatMsg = 314,
    KEUserMsgQuickResponse = 315,
    KEUserMsgPostMatchDetails = 316,
    KEUserMsgChatEvent = 317,
    KEUserMsgAbilityInterrupted = 318,
    KEUserMsgHeroKilled = 319,
    KEUserMsgReturnIdol = 320,
    KEUserMsgSetClientCameraAngles = 321,
    KEUserMsgMapLine = 322,
    KEUserMsgBulletHit = 323,
    KEUserMsgObjectiveMask = 324,
    KEUserMsgModifierApplied = 325,
    KEUserMsgCameraController = 326,
    KEUserMsgAuraModifierApplied = 327,
    KEUserMsgObstructedShotFired = 329,
    KEUserMsgAbilityLateFailure = 330,
    KEUserMsgAbilityPing = 331,
    KEUserMsgPostProcessingAnim = 332,
    KEUserMsgDeathReplayData = 333,
    KEUserMsgPlayerLifetimeStatInfo = 334,
    KEUserMsgForceShopClosed = 336,
    KEUserMsgStaminaDrained = 337,
    KEUserMsgAbilityNotify = 338,
    KEUserMsgGetDamageStatsResponse = 339,
    KEUserMsgParticipantStartSoundEvent = 340,
    KEUserMsgParticipantStopSoundEvent = 341,
    KEUserMsgParticipantStopSoundEventHash = 342,
    KEUserMsgParticipantSetSoundEventParams = 343,
    KEUserMsgParticipantSetLibraryStackFields = 344,
    KEUserMsgCurrencyChanged = 345,
    KEUserMsgGameOver = 346,
    KEUserMsgBossKilled = 347,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsoCitadelLobby {
    #[prost(uint64, optional, tag = "1")]
    pub lobby_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(enumeration = "ECitadelMatchMode", optional, tag = "3", default = "KECitadelMatchModeInvalid")]
    pub match_mode: ::core::option::Option<i32>,
    #[prost(enumeration = "ECitadelGameMode", optional, tag = "4", default = "KECitadelGameModeInvalid")]
    pub game_mode: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "5")]
    pub compatibility_version: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "6")]
    pub extra_messages: ::prost::alloc::vec::Vec<CExtraMsgBlock>,
    #[prost(fixed64, optional, tag = "7")]
    pub server_steam_id: ::core::option::Option<u64>,
    #[prost(enumeration = "ELobbyServerState", optional, tag = "8", default = "KELobbyServerStateAssign")]
    pub server_state: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "9")]
    pub udp_connect_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub udp_connect_port: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "12")]
    pub sdr_address: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "13")]
    pub server_version: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "14")]
    pub safe_to_abandon: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsoCitadelParty {
    #[prost(uint64, optional, tag = "1")]
    pub party_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<cso_citadel_party::Member>,
    #[prost(message, repeated, tag = "3")]
    pub invites: ::prost::alloc::vec::Vec<cso_citadel_party::Invite>,
    #[prost(string, optional, tag = "4")]
    pub dev_server_command: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub left_members: ::prost::alloc::vec::Vec<cso_citadel_party::LeftMember>,
    #[prost(uint64, optional, tag = "6")]
    pub join_code: ::core::option::Option<u64>,
    #[prost(enumeration = "ECitadelBotDifficulty", optional, tag = "7", default = "KECitadelBotDifficultyNone")]
    pub bot_difficulty: ::core::option::Option<i32>,
    #[prost(enumeration = "ECitadelMatchMode", optional, tag = "9", default = "KECitadelMatchModeInvalid")]
    pub match_mode: ::core::option::Option<i32>,
    #[prost(enumeration = "ECitadelGameMode", optional, tag = "10", default = "KECitadelGameModeInvalid")]
    pub game_mode: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "11")]
    pub match_making_start_time: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "12")]
    pub server_search_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "13")]
    pub is_high_skill_range_party: ::core::option::Option<bool>,
    #[prost(enumeration = "cso_citadel_party::EChatMode", optional, tag = "14", default = "KENone")]
    pub chat_mode: ::core::option::Option<i32>,
    #[prost(enumeration = "ECitadelRegionMode", optional, tag = "15", default = "KECitadelRegionModeRow")]
    pub region_mode: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelAccountStatMedal {
    KENone = 0,
    KEBronze = 1,
    KESilver = 2,
    KEGold = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelBotDifficulty {
    KECitadelBotDifficultyNone = 0,
    KECitadelBotDifficultyEasy = 1,
    KECitadelBotDifficultyMedium = 2,
    KECitadelBotDifficultyHard = 3,
    KECitadelBotDifficultyNightmare = 4,
    KECitadelBotDifficultyGuided = 5,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelChatMessage {
    CitadelChatMessageUnpauseCountdown = 1,
    CitadelChatMessageUnpaused = 2,
    CitadelChatMessageAutoUnpaused = 3,
    CitadelChatMessagePauseCountdown = 4,
    CitadelChatMessagePaused = 5,
    CitadelChatMessageYoupaused = 6,
    CitadelChatMessageCantpause = 7,
    CitadelChatMessageCantunpauseteam = 8,
    CitadelChatMessageNopausesleft = 9,
    CitadelChatMessageCantpauseyet = 10,
    CitadelChatMessagePregameCountdown = 11,
    CitadelChatMessageNoteampausesleft = 12,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelGameEvents {
    GeFireBullets = 450,
    GePlayerAnimEvent = 451,
    GeParticleSystemManager = 458,
    GeScreenTextPretty = 459,
    GeServerRequestedTracer = 460,
    GeBulletImpact = 461,
    GeEnableSatVolumesEvent = 462,
    GePlaceSatVolumeEvent = 463,
    GeDisableSatVolumesEvent = 464,
    GeRemoveSatVolumeEvent = 465,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelGameMode {
    KECitadelGameModeInvalid = 0,
    KECitadelGameModeNormal = 1,
    KECitadelGameMode1v1Test = 2,
    KECitadelGameModeSandbox = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelLobbyTeam {
    KECitadelLobbyTeamTeam0 = 0,
    KECitadelLobbyTeamTeam1 = 1,
    KECitadelLobbyTeamSpectator = 16,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelMatchMode {
    KECitadelMatchModeInvalid = 0,
    KECitadelMatchModeUnranked = 1,
    KECitadelMatchModePrivateLobby = 2,
    KECitadelMatchModeCoopBot = 3,
    KECitadelMatchModeRanked = 4,
    KECitadelMatchModeServerTest = 5,
    KECitadelMatchModeTutorial = 6,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelObjective {
    KECitadelObjectiveTeam0Core = 0,
    KECitadelObjectiveTeam0Tier1Lane1 = 1,
    KECitadelObjectiveTeam0Tier1Lane2 = 2,
    KECitadelObjectiveTeam0Tier1Lane3 = 3,
    KECitadelObjectiveTeam0Tier1Lane4 = 4,
    KECitadelObjectiveTeam0Tier2Lane1 = 5,
    KECitadelObjectiveTeam0Tier2Lane2 = 6,
    KECitadelObjectiveTeam0Tier2Lane3 = 7,
    KECitadelObjectiveTeam0Tier2Lane4 = 8,
    KECitadelObjectiveTeam0Titan = 9,
    KECitadelObjectiveTeam0TitanShieldGenerator1 = 10,
    KECitadelObjectiveTeam0TitanShieldGenerator2 = 11,
    KECitadelObjectiveTeam0BarrackBossLane1 = 12,
    KECitadelObjectiveTeam0BarrackBossLane2 = 13,
    KECitadelObjectiveTeam0BarrackBossLane3 = 14,
    KECitadelObjectiveTeam0BarrackBossLane4 = 15,
    KECitadelObjectiveTeam1Core = 16,
    KECitadelObjectiveTeam1Tier1Lane1 = 17,
    KECitadelObjectiveTeam1Tier1Lane2 = 18,
    KECitadelObjectiveTeam1Tier1Lane3 = 19,
    KECitadelObjectiveTeam1Tier1Lane4 = 20,
    KECitadelObjectiveTeam1Tier2Lane1 = 21,
    KECitadelObjectiveTeam1Tier2Lane2 = 22,
    KECitadelObjectiveTeam1Tier2Lane3 = 23,
    KECitadelObjectiveTeam1Tier2Lane4 = 24,
    KECitadelObjectiveTeam1Titan = 25,
    KECitadelObjectiveTeam1TitanShieldGenerator1 = 26,
    KECitadelObjectiveTeam1TitanShieldGenerator2 = 27,
    KECitadelObjectiveTeam1BarrackBossLane1 = 28,
    KECitadelObjectiveTeam1BarrackBossLane2 = 29,
    KECitadelObjectiveTeam1BarrackBossLane3 = 30,
    KECitadelObjectiveTeam1BarrackBossLane4 = 31,
    KECitadelObjectiveNeutralMid = 32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelRegionMode {
    KECitadelRegionModeRow = 0,
    KECitadelRegionModeEurope = 1,
    KECitadelRegionModeSeAsia = 2,
    KECitadelRegionModeSAmerica = 3,
    KECitadelRegionModeRussia = 4,
    KECitadelRegionModeOceania = 5,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelTeamObjective {
    KECitadelTeamObjectiveCore = 0,
    KECitadelTeamObjectiveTier1Lane1 = 1,
    KECitadelTeamObjectiveTier1Lane2 = 2,
    KECitadelTeamObjectiveTier1Lane3 = 3,
    KECitadelTeamObjectiveTier1Lane4 = 4,
    KECitadelTeamObjectiveTier2Lane1 = 5,
    KECitadelTeamObjectiveTier2Lane2 = 6,
    KECitadelTeamObjectiveTier2Lane3 = 7,
    KECitadelTeamObjectiveTier2Lane4 = 8,
    KECitadelTeamObjectiveTitan = 9,
    KECitadelTeamObjectiveTitanShieldGenerator1 = 10,
    KECitadelTeamObjectiveTitanShieldGenerator2 = 11,
    KECitadelTeamObjectiveBarrackBossLane1 = 12,
    KECitadelTeamObjectiveBarrackBossLane2 = 13,
    KECitadelTeamObjectiveBarrackBossLane3 = 14,
    KECitadelTeamObjectiveBarrackBossLane4 = 15,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELobbyServerState {
    KELobbyServerStateAssign = 0,
    KELobbyServerStateInGame = 1,
    KELobbyServerStatePostMatch = 2,
    KELobbyServerStateSignedOut = 3,
    KELobbyServerStateAbandoned = 4,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EProtoExecutionSite {
    KEProtoExecutionSiteUnknown = 0,
    KEProtoExecutionSiteSteamClient = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ESourceEngine {
    KEseSource1 = 0,
    KEseSource2 = 1,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ESteamLearnCacheDataResult {
    SteamlearnCacheDataError = 0,
    SteamlearnCacheDataSuccess = 1,
    SteamlearnCacheDataErrorUnknownDataSource = 2,
    SteamlearnCacheDataErrorUncachedDataSource = 3,
    SteamlearnCacheDataErrorInvalidKeys = 4,
    SteamlearnCacheDataErrorForbidden = 5,
    SteamlearnCacheDataErrorInvalidTimestamp = 6,
    SteamlearnCacheDataDisabled = 7,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ESteamLearnDataType {
    SteamlearnDatatypeInvalid = 0,
    SteamlearnDatatypeInt32 = 1,
    SteamlearnDatatypeFloat32 = 2,
    SteamlearnDatatypeBool = 3,
    SteamlearnDatatypeString = 4,
    SteamlearnDatatypeObject = 5,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ESteamLearnGetAccessTokensResult {
    SteamlearnGetAccessTokensError = 0,
    SteamlearnGetAccessTokensSuccess = 1,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ESteamLearnInferenceMetadataResult {
    SteamlearnInferenceMetadataError = 0,
    SteamlearnInferenceMetadataSuccess = 1,
    SteamlearnInferenceMetadataErrorInvalidProjectId = 2,
    SteamlearnInferenceMetadataErrorNoPublishedConfig = 3,
    SteamlearnInferenceMetadataErrorForbidden = 4,
    SteamlearnInferenceMetadataErrorInvalidTimestamp = 5,
    SteamlearnInferenceMetadataErrorInvalidPublishedVersion = 6,
    SteamlearnInferenceMetadataErrorNoFetchIdFound = 7,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ESteamLearnInferenceResult {
    SteamlearnInferenceError = 0,
    SteamlearnInferenceSuccess = 1,
    SteamlearnInferenceErrorInvalidProjectId = 2,
    SteamlearnInferenceErrorMissingCachedSchemaData = 3,
    SteamlearnInferenceErrorNoPublishedConfig = 4,
    SteamlearnInferenceErrorForbidden = 5,
    SteamlearnInferenceErrorInvalidTimestamp = 6,
    SteamlearnInferenceErrorInvalidPublishedVersion = 7,
    SteamlearnInferenceErrorNoFetchIdFound = 8,
    SteamlearnInferenceErrorTooBusy = 9,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ESteamLearnSnapshotProjectResult {
    SteamlearnSnapshotProjectError = 0,
    SteamlearnSnapshotProjectSuccessStored = 1,
    SteamlearnSnapshotProjectSuccessQueued = 2,
    SteamlearnSnapshotProjectErrorInvalidProjectId = 3,
    SteamlearnSnapshotProjectErrorUnknownDataSource = 4,
    SteamlearnSnapshotProjectErrorInvalidDataSourceKey = 5,
    SteamlearnSnapshotProjectErrorMissingCacheDuration = 6,
    SteamlearnSnapshotProjectErrorNoPublishedConfig = 7,
    SteamlearnSnapshotProjectErrorForbidden = 8,
    SteamlearnSnapshotProjectErrorInvalidTimestamp = 9,
    SteamlearnSnapshotProjectErrorInternalDataSourceError = 10,
    SteamlearnSnapshotProjectDisabled = 11,
    SteamlearnSnapshotProjectErrorInvalidPublishedVersion = 12,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ESteammLearnRegisterDataSourceResult {
    SteamlearnRegisterDataSourceResultError = 0,
    SteamlearnRegisterDataSourceResultSuccessCreated = 1,
    SteamlearnRegisterDataSourceResultSuccessFound = 2,
    SteamlearnRegisterDataSourceResultErrorGeneric = 3,
    SteamlearnRegisterDataSourceResultErrorInvalidName = 4,
    SteamlearnRegisterDataSourceResultErrorInvalidVersion = 5,
    SteamlearnRegisterDataSourceResultErrorDataChanged = 6,
    SteamlearnRegisterDataSourceResultErrorDataInvalid = 7,
    SteamlearnRegisterDataSourceResultErrorForbidden = 8,
    SteamlearnRegisterDataSourceResultErrorInvalidTimestamp = 9,
    SteamlearnRegisterDataSourceResultDisabled = 10,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EgcCitadelCommonMessages {
    KEMsgAnyToGcReportAsserts = 7000,
    KEMsgAnyToGcReportAssertsResponse = 7001,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EgcPlatform {
    KEGcPlatformNone = 0,
    KEGcPlatformPc = 1,
    KEGcPlatformMac = 2,
    KEGcPlatformLinux = 3,
    KEGcPlatformAndroid = 4,
    KEGcPlatformIOs = 5,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GcConnectionStatus {
    HaveSession = 0,
    GcGoingDown = 1,
    NoSession = 2,
    NoSessionInLogonQueue = 3,
    NoSteam = 4,
    Suspended = 5,
    SteamGoingDown = 6,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GcProtoBufMsgSrc {
    Unspecified = 0,
    FromSystem = 1,
    FromSteamId = 2,
    FromGc = 3,
    ReplySystem = 4,
    SpoofedSteamId = 5,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModifierEntryType {
    Active = 1,
    Removed = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ParticleSystemManagerMessage {
    ParticleSystemManagerEventCreate = 0,
    ParticleSystemManagerEventDestroy = 1,
    ParticleSystemManagerEventDestroyInvolving = 2,
    ParticleSystemManagerEventRelease = 3,
    ParticleSystemManagerEventUpdate = 4,
    ParticleSystemManagerEventUpdateForward = 5,
    ParticleSystemManagerEventUpdateOrientation = 6,
    ParticleSystemManagerEventUpdateFallback = 7,
    ParticleSystemManagerEventUpdateEnt = 8,
    ParticleSystemManagerEventUpdateOffset = 9,
    ParticleSystemManagerEventUpdateFrozen = 10,
    ParticleSystemManagerEventUpdateShouldDraw = 11,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PartnerAccountType {
    PartnerNone = 0,
    PartnerPerfectWorld = 1,
    PartnerInvalid = 3,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingCommonData {
    #[prost(uint32, optional, tag = "1")]
    pub ping_message_id: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub ping_location: ::core::option::Option<CMsgVector>,
    #[prost(uint32, optional, tag = "3", default = "16777215")]
    pub entity_index: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub sender_player_slot: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub speech_concept: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub response_chosen: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, optional, tag = "7")]
    pub cooldown_time: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PostProcessingGameStates {
    PostProcStateKilled = 0,
    PostProcStateBlack = 1,
    PostProcStateBlinded = 2,
    PostProcStateShivPossessed = 3,
}

pub mod c_citadel_user_msg_abilities_changed {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Change {
        EInvalid = -1,
        EPurchased = 0,
        EUpgraded = 1,
        ESold = 2,
        ESwappedActivatedAbility = 3,
    }
}

pub mod c_citadel_user_msg_camera_controller {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Maintain {
        #[prost(float, optional, tag = "1", default = "1")]
        pub duration: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Approach {
        #[prost(float, optional, tag = "1", default = "600")]
        pub speed: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "2", default = "600")]
        pub default_speed: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "3", default = "1000")]
        pub acceleration: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "4", default = "0")]
        pub min_duration: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "5")]
        pub approach_float: ::core::option::Option<f32>,
        #[prost(message, optional, tag = "6")]
        pub approach_vector: ::core::option::Option<super::CMsgVector>,
        #[prost(bool, optional, tag = "7")]
        pub chase_default: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Spring {
        #[prost(float, optional, tag = "1", default = "10")]
        pub spring_strength: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "4", default = "0")]
        pub min_speed: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "5", default = "0")]
        pub max_duration: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "6")]
        pub target_float: ::core::option::Option<f32>,
        #[prost(message, optional, tag = "7")]
        pub target_vector: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Lerp {
        #[prost(float, optional, tag = "1")]
        pub start_float: ::core::option::Option<f32>,
        #[prost(message, optional, tag = "2")]
        pub start_vector: ::core::option::Option<super::CMsgVector>,
        #[prost(float, optional, tag = "3")]
        pub end_float: ::core::option::Option<f32>,
        #[prost(message, optional, tag = "4")]
        pub end_vector: ::core::option::Option<super::CMsgVector>,
        #[prost(float, optional, tag = "5")]
        pub bias: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "6")]
        pub gain: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "7", default = "1")]
        pub duration: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Lag {
        #[prost(float, optional, tag = "1")]
        pub min_duration: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "2")]
        pub lag_time: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "3")]
        pub max_speed: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "4")]
        pub spring_strength: ::core::option::Option<f32>,
    }
}

pub mod c_citadel_user_msg_get_damage_stats_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StatType {
        #[prost(uint32, repeated, tag = "1")]
        pub target_player_slot: ::prost::alloc::vec::Vec<u32>,
        #[prost(uint32, repeated, tag = "2")]
        pub value: ::prost::alloc::vec::Vec<u32>,
    }
}

pub mod c_citadel_user_msg_gold_history {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct GoldRecord {
        #[prost(int32, optional, tag = "1")]
        pub currency_source: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub gold: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub events: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MinuteRecord {
        #[prost(int32, optional, tag = "1")]
        pub match_minute: ::core::option::Option<i32>,
        #[prost(message, repeated, tag = "2")]
        pub gold_records: ::prost::alloc::vec::Vec<GoldRecord>,
    }
}

pub mod c_citadel_user_msg_player_lifetime_stat_info {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Stat {
        #[prost(string, optional, tag = "1")]
        pub stat_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "2")]
        pub match_total: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub lifetime_value: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub priority: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "5")]
        pub prev_lifetime_max: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub stat_type: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "7")]
        pub stat_type_id: ::core::option::Option<u32>,
    }
}

pub mod c_citadel_user_msg_recent_damage_summary {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct DamageRecord {
        #[prost(int32, optional, tag = "1")]
        pub damage: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub hits: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "3")]
        pub damage_type: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub hero_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "5")]
        pub ability_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub attacker_class: ::core::option::Option<u32>,
        #[prost(int32, optional, tag = "7")]
        pub damage_absorbed: ::core::option::Option<i32>,
        #[prost(bool, optional, tag = "8")]
        pub is_killing_blow: ::core::option::Option<bool>,
        #[prost(uint32, optional, tag = "9")]
        pub victim_hero_id: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ModifierRecord {
        #[prost(uint32, optional, tag = "1")]
        pub ability_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub modifier_type_id: ::core::option::Option<u32>,
        #[prost(int32, optional, tag = "3", default = "-1")]
        pub entindex_caster: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "4")]
        pub start_time: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "5")]
        pub end_time: ::core::option::Option<f32>,
        #[prost(bool, optional, tag = "6")]
        pub debuff: ::core::option::Option<bool>,
    }
}

pub mod c_lobby_data_post_match_survey {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct PlayerSurvey {
        #[prost(uint32, optional, tag = "1")]
        pub account_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub question_id: ::core::option::Option<u32>,
    }
}

pub mod c_msg_any_to_gc_report_asserts {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrackedAssert {
        #[prost(string, optional, tag = "1")]
        pub filename: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "2")]
        pub line_number: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "3")]
        pub sample_msg: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "4")]
        pub sample_stack: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "5")]
        pub times_fired: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "6")]
        pub function_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "7")]
        pub condition: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "8")]
        pub total_times_fired: ::core::option::Option<u32>,
    }
}

pub mod c_msg_client_welcome {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Location {
        #[prost(float, optional, tag = "1")]
        pub latitude: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "2")]
        pub longitude: ::core::option::Option<f32>,
        #[prost(string, optional, tag = "3")]
        pub country: ::core::option::Option<::prost::alloc::string::String>,
    }
}

pub mod c_msg_fire_bullets {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct TracerAssignment {
        #[prost(uint64, optional, tag = "1")]
        pub tracer_resource_id: ::core::option::Option<u64>,
        #[prost(uint32, optional, tag = "2")]
        pub bullet_indicies: ::core::option::Option<u32>,
    }
}

pub mod c_msg_gc_to_gcso_cache_subscribe {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct CMsgHaveVersions {
        #[prost(uint32, optional, tag = "1")]
        pub service_id: ::core::option::Option<u32>,
        #[prost(uint64, optional, tag = "2")]
        pub version: ::core::option::Option<u64>,
    }
}

pub mod c_msg_gc_update_sub_gc_session_info {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct CMsgUpdate {
        #[prost(fixed64, optional, tag = "1")]
        pub steamid: ::core::option::Option<u64>,
        #[prost(fixed32, optional, tag = "2")]
        pub ip: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "3")]
        pub trusted: ::core::option::Option<bool>,
    }
}

pub mod c_msg_hero_selection_match_info {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Hero {
        #[prost(uint32, optional, tag = "1")]
        pub hero_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub priority: ::core::option::Option<u32>,
    }
}

pub mod c_msg_match_meta_data_contents {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Position {
        #[prost(float, optional, tag = "1")]
        pub x: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "2")]
        pub y: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "3")]
        pub z: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Deaths {
        #[prost(uint32, optional, tag = "1")]
        pub game_time_s: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "9")]
        pub killer_player_slot: ::core::option::Option<u32>,
        #[prost(message, optional, tag = "10")]
        pub death_pos: ::core::option::Option<Position>,
        #[prost(message, optional, tag = "11")]
        pub killer_pos: ::core::option::Option<Position>,
        #[prost(uint32, optional, tag = "12")]
        pub death_duration_s: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Items {
        #[prost(uint32, optional, tag = "1")]
        pub game_time_s: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub item_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub upgrade_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub sold_time_s: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "5")]
        pub flags: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub imbued_ability_id: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Ping {
        #[prost(uint32, optional, tag = "1")]
        pub ping_type: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub ping_data: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub game_time_s: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct GoldSource {
        #[prost(enumeration = "EGoldSource", optional, tag = "1", default = "KEPlayers")]
        pub source: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "2")]
        pub kills: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub damage: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub gold: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "5")]
        pub gold_orbs: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomUserStatInfo {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "2")]
        pub id: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct CustomUserStat {
        #[prost(uint32, optional, tag = "2")]
        pub value: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub id: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlayerStats {
        #[prost(uint32, optional, tag = "1")]
        pub time_stamp_s: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub net_worth: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub gold_player: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub gold_player_orbs: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "5")]
        pub gold_lane_creep_orbs: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub gold_neutral_creep_orbs: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "7")]
        pub gold_boss: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "8")]
        pub gold_boss_orb: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "9")]
        pub gold_treasure: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "10")]
        pub gold_denied: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "11")]
        pub gold_death_loss: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "12")]
        pub gold_lane_creep: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "13")]
        pub gold_neutral_creep: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "14")]
        pub kills: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "15")]
        pub deaths: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "16")]
        pub assists: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "17")]
        pub creep_kills: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "18")]
        pub neutral_kills: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "19")]
        pub possible_creeps: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "20")]
        pub creep_damage: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "21")]
        pub player_damage: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "22")]
        pub neutral_damage: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "23")]
        pub boss_damage: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "24")]
        pub denies: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "25")]
        pub player_healing: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "26")]
        pub ability_points: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "27")]
        pub self_healing: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "28")]
        pub player_damage_taken: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "29")]
        pub max_health: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "30")]
        pub weapon_power: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "31")]
        pub tech_power: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "32")]
        pub shots_hit: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "33")]
        pub shots_missed: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "34")]
        pub damage_absorbed: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "35")]
        pub absorption_provided: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "36")]
        pub hero_bullets_hit: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "37")]
        pub hero_bullets_hit_crit: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "38")]
        pub heal_prevented: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "39")]
        pub heal_lost: ::core::option::Option<u32>,
        #[prost(message, repeated, tag = "40")]
        pub gold_sources: ::prost::alloc::vec::Vec<GoldSource>,
        #[prost(message, repeated, tag = "41")]
        pub custom_user_stats: ::prost::alloc::vec::Vec<CustomUserStat>,
        #[prost(uint32, optional, tag = "42")]
        pub damage_mitigated: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "43")]
        pub level: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct AbilityStat {
        #[prost(uint32, optional, tag = "1")]
        pub ability_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub ability_value: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct BookReward {
        #[prost(uint32, optional, tag = "1")]
        pub book_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub xp_amount: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub starting_xp: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Players {
        #[prost(uint32, optional, tag = "1")]
        pub account_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub player_slot: ::core::option::Option<u32>,
        #[prost(message, repeated, tag = "3")]
        pub death_details: ::prost::alloc::vec::Vec<Deaths>,
        #[prost(message, repeated, tag = "4")]
        pub items: ::prost::alloc::vec::Vec<Items>,
        #[prost(message, repeated, tag = "5")]
        pub stats: ::prost::alloc::vec::Vec<PlayerStats>,
        #[prost(enumeration = "super::ECitadelLobbyTeam", optional, tag = "6", default = "KECitadelLobbyTeamTeam0")]
        pub team: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "8")]
        pub kills: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "9")]
        pub deaths: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "10")]
        pub assists: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "11")]
        pub net_worth: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "12")]
        pub hero_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "13")]
        pub last_hits: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "14")]
        pub denies: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "15")]
        pub ability_points: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "16")]
        pub party: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "17")]
        pub assigned_lane: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "18")]
        pub level: ::core::option::Option<u32>,
        #[prost(message, repeated, tag = "19")]
        pub pings: ::prost::alloc::vec::Vec<Ping>,
        #[prost(message, repeated, tag = "20")]
        pub ability_stats: ::prost::alloc::vec::Vec<AbilityStat>,
        #[prost(float, repeated, tag = "21")]
        pub stats_type_stat: ::prost::alloc::vec::Vec<f32>,
        #[prost(message, repeated, tag = "22")]
        pub book_rewards: ::prost::alloc::vec::Vec<BookReward>,
        #[prost(uint32, optional, tag = "23")]
        pub abandon_match_time_s: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Objective {
        #[prost(enumeration = "super::ECitadelObjective", optional, tag = "1", default = "KECitadelObjectiveTeam0Core")]
        pub legacy_objective_id: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "2")]
        pub destroyed_time_s: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub creep_damage: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "5")]
        pub creep_damage_mitigated: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub player_damage: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "7")]
        pub player_damage_mitigated: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "8")]
        pub first_damage_time_s: ::core::option::Option<u32>,
        #[prost(enumeration = "super::ECitadelTeamObjective", optional, tag = "9", default = "KECitadelTeamObjectiveCore")]
        pub team_objective_id: ::core::option::Option<i32>,
        #[prost(enumeration = "super::ECitadelLobbyTeam", optional, tag = "10", default = "KECitadelLobbyTeamTeam0")]
        pub team: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct MidBoss {
        #[prost(enumeration = "super::ECitadelLobbyTeam", optional, tag = "1", default = "KECitadelLobbyTeamTeam0")]
        pub team_killed: ::core::option::Option<i32>,
        #[prost(enumeration = "super::ECitadelLobbyTeam", optional, tag = "2", default = "KECitadelLobbyTeamTeam0")]
        pub team_claimed: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "3")]
        pub destroyed_time_s: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Pause {
        #[prost(uint32, optional, tag = "1")]
        pub game_time_s: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub pause_duration_s: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub player_slot: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct WatchedDeathReplay {
        #[prost(uint32, optional, tag = "1")]
        pub game_time_s: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub player_slot: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchInfo {
        #[prost(uint32, optional, tag = "1")]
        pub duration_s: ::core::option::Option<u32>,
        #[prost(enumeration = "EMatchOutcome", optional, tag = "2", default = "KEOutcomeTeamWin")]
        pub match_outcome: ::core::option::Option<i32>,
        #[prost(enumeration = "super::ECitadelLobbyTeam", optional, tag = "3", default = "KECitadelLobbyTeamTeam0")]
        pub winning_team: ::core::option::Option<i32>,
        #[prost(message, repeated, tag = "4")]
        pub players: ::prost::alloc::vec::Vec<Players>,
        #[prost(uint32, optional, tag = "5")]
        pub start_time: ::core::option::Option<u32>,
        #[prost(uint64, optional, tag = "6")]
        pub match_id: ::core::option::Option<u64>,
        #[prost(uint32, optional, tag = "8")]
        pub legacy_objectives_mask: ::core::option::Option<u32>,
        #[prost(enumeration = "super::ECitadelGameMode", optional, tag = "9", default = "KECitadelGameModeInvalid")]
        pub game_mode: ::core::option::Option<i32>,
        #[prost(enumeration = "super::ECitadelMatchMode", optional, tag = "10", default = "KECitadelMatchModeInvalid")]
        pub match_mode: ::core::option::Option<i32>,
        #[prost(message, repeated, tag = "11")]
        pub objectives: ::prost::alloc::vec::Vec<Objective>,
        #[prost(message, optional, tag = "12")]
        pub match_paths: ::core::option::Option<super::CMsgMatchPlayerPathsData>,
        #[prost(message, optional, tag = "13")]
        pub damage_matrix: ::core::option::Option<super::CMsgMatchPlayerDamageMatrix>,
        #[prost(message, repeated, tag = "14")]
        pub match_pauses: ::prost::alloc::vec::Vec<Pause>,
        #[prost(message, repeated, tag = "15")]
        pub custom_user_stats: ::prost::alloc::vec::Vec<CustomUserStatInfo>,
        #[prost(message, repeated, tag = "16")]
        pub watched_death_replays: ::prost::alloc::vec::Vec<WatchedDeathReplay>,
        #[prost(uint64, optional, tag = "17")]
        pub objectives_mask_team0: ::core::option::Option<u64>,
        #[prost(uint64, optional, tag = "18")]
        pub objectives_mask_team1: ::core::option::Option<u64>,
        #[prost(message, repeated, tag = "19")]
        pub mid_boss: ::prost::alloc::vec::Vec<MidBoss>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EMatchOutcome {
        KEOutcomeTeamWin = 0,
        KEOutcomeError = 1,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EGoldSource {
        KEPlayers = 1,
        KELaneCreeps = 2,
        KENeutrals = 3,
        KEBosses = 4,
        KETreasure = 5,
        KEAssists = 6,
        KEDenies = 7,
    }
}

pub mod c_msg_match_player_damage_matrix {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DamageToPlayer {
        #[prost(uint32, optional, tag = "1")]
        pub target_player_slot: ::core::option::Option<u32>,
        #[prost(uint32, repeated, tag = "2")]
        pub damage: ::prost::alloc::vec::Vec<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DamageSource {
        #[prost(message, repeated, tag = "2")]
        pub damage_to_players: ::prost::alloc::vec::Vec<DamageToPlayer>,
        #[prost(uint32, optional, tag = "4")]
        pub source_details_index: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DamageDealer {
        #[prost(uint32, optional, tag = "1")]
        pub dealer_player_slot: ::core::option::Option<u32>,
        #[prost(message, repeated, tag = "2")]
        pub damage_sources: ::prost::alloc::vec::Vec<DamageSource>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourceDetails {
        #[prost(enumeration = "EStatType", repeated, tag = "1")]
        pub stat_type: ::prost::alloc::vec::Vec<i32>,
        #[prost(string, repeated, tag = "2")]
        pub source_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EStatType {
        KETypeDamage = 0,
        KETypeHealing = 1,
        KETypeHealPrevented = 2,
        KETypeMitigated = 3,
        KETypeLethalDamage = 4,
    }
}

pub mod c_msg_match_player_paths_data {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Path {
        #[prost(uint32, optional, tag = "1")]
        pub player_slot: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "2")]
        pub x_min: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "3")]
        pub y_min: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "4")]
        pub x_max: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "5")]
        pub y_max: ::core::option::Option<f32>,
        #[prost(uint32, repeated, tag = "6")]
        pub x_pos: ::prost::alloc::vec::Vec<u32>,
        #[prost(uint32, repeated, tag = "7")]
        pub y_pos: ::prost::alloc::vec::Vec<u32>,
        #[prost(bool, repeated, tag = "8")]
        pub alive: ::prost::alloc::vec::Vec<bool>,
        #[prost(uint32, repeated, tag = "9")]
        pub health: ::prost::alloc::vec::Vec<u32>,
    }
}

pub mod c_msg_particle_system_manager {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct CreateParticle {
        #[prost(fixed64, optional, tag = "1")]
        pub particle_name_index: ::core::option::Option<u64>,
        #[prost(int32, optional, tag = "2")]
        pub attach_type: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "3", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
        #[prost(message, optional, tag = "4")]
        pub position: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "5")]
        pub angles: ::core::option::Option<super::CMsgQAngle>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct DestroyParticle {
        #[prost(bool, optional, tag = "1")]
        pub destroy_immediately: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct DestroyParticleInvolving {
        #[prost(bool, optional, tag = "1")]
        pub destroy_immediately: ::core::option::Option<bool>,
        #[prost(uint32, optional, tag = "3", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ReleaseParticleIndex {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticle {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleFwd {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub forward: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleOrient {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub forward: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "3")]
        pub left: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "4")]
        pub up: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleFallback {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleEnt {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "2", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
        #[prost(int32, optional, tag = "3")]
        pub attach_type: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub attachment: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "5")]
        pub fallback_position: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleOffset {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub origin_offset: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleFrozen {
        #[prost(bool, optional, tag = "1")]
        pub set_frozen: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleShouldDraw {
        #[prost(bool, optional, tag = "1")]
        pub should_draw: ::core::option::Option<bool>,
    }
}

pub mod c_msg_sdo_assert {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(uint64, repeated, packed = "false", tag = "1")]
        pub key: ::prost::alloc::vec::Vec<u64>,
        #[prost(string, optional, tag = "2")]
        pub requesting_job: ::core::option::Option<::prost::alloc::string::String>,
    }
}

pub mod c_msg_serialized_so_cache {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TypeCache {
        #[prost(uint32, optional, tag = "1")]
        pub r#type: ::core::option::Option<u32>,
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub objects: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        #[prost(uint32, optional, tag = "3")]
        pub service_id: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Cache {
        #[prost(uint32, optional, tag = "1")]
        pub r#type: ::core::option::Option<u32>,
        #[prost(uint64, optional, tag = "2")]
        pub id: ::core::option::Option<u64>,
        #[prost(message, repeated, tag = "3")]
        pub versions: ::prost::alloc::vec::Vec<cache::Version>,
        #[prost(message, repeated, tag = "4")]
        pub type_caches: ::prost::alloc::vec::Vec<TypeCache>,
    }
    pub mod cache {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Version {
            #[prost(uint32, optional, tag = "1")]
            pub service: ::core::option::Option<u32>,
            #[prost(uint64, optional, tag = "2")]
            pub version: ::core::option::Option<u64>,
        }
    }
}

pub mod c_msg_so_cache_subscribed {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubscribedType {
        #[prost(int32, optional, tag = "1")]
        pub type_id: ::core::option::Option<i32>,
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub object_data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
}

pub mod c_msg_so_multiple_objects {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SingleObject {
        #[prost(int32, optional, tag = "1")]
        pub type_id: ::core::option::Option<i32>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub object_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
}

pub mod c_msg_steam_learn_access_tokens {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CacheDataAccessToken {
        #[prost(uint32, optional, tag = "1")]
        pub data_source_id: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SnapshotProjectAccessToken {
        #[prost(uint32, optional, tag = "1")]
        pub project_id: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InferenceAccessToken {
        #[prost(uint32, optional, tag = "1")]
        pub project_id: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    }
}

pub mod c_msg_steam_learn_inference_backend_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Sequence {
        #[prost(float, repeated, packed = "false", tag = "1")]
        pub value: ::prost::alloc::vec::Vec<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RegressionOutput {
        #[prost(float, optional, tag = "1")]
        pub value: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct BinaryCrossEntropyOutput {
        #[prost(float, optional, tag = "1")]
        pub value: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MutliBinaryCrossEntropyOutput {
        #[prost(float, repeated, packed = "false", tag = "1")]
        pub weight: ::prost::alloc::vec::Vec<f32>,
        #[prost(float, repeated, packed = "false", tag = "2")]
        pub value: ::prost::alloc::vec::Vec<f32>,
        #[prost(message, repeated, tag = "3")]
        pub value_sequence: ::prost::alloc::vec::Vec<Sequence>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CategoricalCrossEntropyOutput {
        #[prost(float, repeated, packed = "false", tag = "1")]
        pub weight: ::prost::alloc::vec::Vec<f32>,
        #[prost(float, repeated, packed = "false", tag = "2")]
        pub value: ::prost::alloc::vec::Vec<f32>,
        #[prost(message, repeated, tag = "3")]
        pub value_sequence: ::prost::alloc::vec::Vec<Sequence>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Output {
        #[prost(oneof = "output::ResponseType", tags = "1, 2, 3, 4")]
        pub response_type: ::core::option::Option<output::ResponseType>,
    }
    pub mod output {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ResponseType {
            #[prost(message, tag = "1")]
            BinaryCrossentropy(super::BinaryCrossEntropyOutput),
            #[prost(message, tag = "2")]
            CategoricalCrossentropy(super::CategoricalCrossEntropyOutput),
            #[prost(message, tag = "3")]
            MultiBinaryCrossentropy(super::MutliBinaryCrossEntropyOutput),
            #[prost(message, tag = "4")]
            Regression(super::RegressionOutput),
        }
    }
}

pub mod c_msg_steam_learn_inference_metadata_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RowRange {
        #[prost(uint64, optional, tag = "1")]
        pub min_row: ::core::option::Option<u64>,
        #[prost(uint64, optional, tag = "2")]
        pub max_row: ::core::option::Option<u64>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Range {
        #[prost(string, optional, tag = "1")]
        pub data_element_path: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(float, optional, tag = "2")]
        pub min_value: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "3")]
        pub max_value: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StdDev {
        #[prost(string, optional, tag = "1")]
        pub data_element_path: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(float, optional, tag = "2")]
        pub mean: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "3")]
        pub std_dev: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompactTable {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub map_values: ::prost::alloc::vec::Vec<compact_table::MapValuesEntry>,
        #[prost(message, repeated, tag = "3")]
        pub map_mappings: ::prost::alloc::vec::Vec<compact_table::MapMappingsEntry>,
    }
    pub mod compact_table {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Entry {
            #[prost(uint32, optional, tag = "1")]
            pub value: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "2")]
            pub mapping: ::core::option::Option<u32>,
            #[prost(uint64, optional, tag = "3")]
            pub count: ::core::option::Option<u64>,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct MapValuesEntry {
            #[prost(uint32, optional, tag = "1")]
            pub key: ::core::option::Option<u32>,
            #[prost(message, optional, tag = "2")]
            pub value: ::core::option::Option<Entry>,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct MapMappingsEntry {
            #[prost(uint32, optional, tag = "1")]
            pub key: ::core::option::Option<u32>,
            #[prost(message, optional, tag = "2")]
            pub value: ::core::option::Option<Entry>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SequenceTable {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub map_values: ::prost::alloc::vec::Vec<sequence_table::MapValuesEntry>,
        #[prost(message, repeated, tag = "3")]
        pub map_mappings: ::prost::alloc::vec::Vec<sequence_table::MapMappingsEntry>,
        #[prost(uint64, optional, tag = "4")]
        pub total_count: ::core::option::Option<u64>,
    }
    pub mod sequence_table {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Entry {
            #[prost(uint32, repeated, packed = "false", tag = "1")]
            pub values: ::prost::alloc::vec::Vec<u32>,
            #[prost(uint32, optional, tag = "2")]
            pub crc: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "3")]
            pub count: ::core::option::Option<u32>,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MapValuesEntry {
            #[prost(uint32, optional, tag = "1")]
            pub key: ::core::option::Option<u32>,
            #[prost(message, optional, tag = "2")]
            pub value: ::core::option::Option<Entry>,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MapMappingsEntry {
            #[prost(string, optional, tag = "1")]
            pub key: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(message, optional, tag = "2")]
            pub value: ::core::option::Option<Entry>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KMeans {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub clusters: ::prost::alloc::vec::Vec<k_means::Cluster>,
    }
    pub mod k_means {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Cluster {
            #[prost(float, optional, tag = "1")]
            pub x: ::core::option::Option<f32>,
            #[prost(float, optional, tag = "2")]
            pub y: ::core::option::Option<f32>,
            #[prost(float, optional, tag = "3")]
            pub radius: ::core::option::Option<f32>,
            #[prost(float, optional, tag = "4")]
            pub radius_75pct: ::core::option::Option<f32>,
            #[prost(float, optional, tag = "5")]
            pub radius_50pct: ::core::option::Option<f32>,
            #[prost(float, optional, tag = "6")]
            pub radius_25pct: ::core::option::Option<f32>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SnapshotHistogram {
        #[prost(float, optional, tag = "1")]
        pub min_value: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "2")]
        pub max_value: ::core::option::Option<f32>,
        #[prost(uint32, optional, tag = "3")]
        pub num_buckets: ::core::option::Option<u32>,
        #[prost(uint32, repeated, packed = "false", tag = "4")]
        pub bucket_counts: ::prost::alloc::vec::Vec<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppInfo {
        #[prost(string, optional, tag = "1")]
        pub country_allow: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub country_deny: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bool, optional, tag = "3")]
        pub platform_win: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "4")]
        pub platform_mac: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "5")]
        pub platform_linux: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "6")]
        pub adult_violence: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "7")]
        pub adult_sex: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppInfoEntry {
        #[prost(uint32, optional, tag = "1")]
        pub key: ::core::option::Option<u32>,
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<AppInfo>,
    }
}

pub mod c_msg_steam_learn_server_info {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ProjectInfo {
        #[prost(uint32, optional, tag = "1")]
        pub project_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub snapshot_published_version: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub inference_published_version: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub snapshot_percentage: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "7")]
        pub snapshot_enabled: ::core::option::Option<bool>,
    }
}

pub mod c_workshop_populate_item_descriptions_request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SingleItemDescription {
        #[prost(uint32, optional, tag = "1")]
        pub gameitemid: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub item_description: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ItemDescriptionsLanguageBlock {
        #[prost(string, optional, tag = "1")]
        pub language: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub descriptions: ::prost::alloc::vec::Vec<SingleItemDescription>,
    }
}

pub mod c_workshop_set_item_payment_rules_request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WorkshopItemPaymentRule {
        #[prost(uint64, optional, tag = "1")]
        pub workshop_file_id: ::core::option::Option<u64>,
        #[prost(float, optional, tag = "2")]
        pub revenue_percentage: ::core::option::Option<f32>,
        #[prost(string, optional, tag = "3")]
        pub rule_description: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "4", default = "1")]
        pub rule_type: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WorkshopDirectPaymentRule {
        #[prost(uint64, optional, tag = "1")]
        pub workshop_file_id: ::core::option::Option<u64>,
        #[prost(string, optional, tag = "2")]
        pub rule_description: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PartnerItemPaymentRule {
        #[prost(uint32, optional, tag = "1")]
        pub account_id: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "2")]
        pub revenue_percentage: ::core::option::Option<f32>,
        #[prost(string, optional, tag = "3")]
        pub rule_description: ::core::option::Option<::prost::alloc::string::String>,
    }
}

pub mod cgc_to_gc_msg_master_ack {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Process {
        #[prost(int32, optional, tag = "1", default = "-1")]
        pub dir_index: ::core::option::Option<i32>,
        #[prost(uint32, repeated, packed = "false", tag = "2")]
        pub type_instances: ::prost::alloc::vec::Vec<u32>,
    }
}

pub mod cgc_to_gc_msg_master_startup_complete {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcInfo {
        #[prost(int32, optional, tag = "1", default = "-1")]
        pub dir_index: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub machine_name: ::core::option::Option<::prost::alloc::string::String>,
    }
}

pub mod cso_citadel_party {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Member {
        #[prost(uint32, optional, tag = "1")]
        pub account_id: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub persona_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "3")]
        pub rights_flags: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "4")]
        pub is_ready: ::core::option::Option<bool>,
        #[prost(enumeration = "EPlayerType", optional, tag = "5", default = "KEPlayerTypePlayer")]
        pub player_type: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "6")]
        pub compatibility_version: ::core::option::Option<u32>,
        #[prost(enumeration = "super::EgcPlatform", optional, tag = "7", default = "KEGcPlatformNone")]
        pub platform: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "8")]
        pub team: ::core::option::Option<u32>,
        #[prost(message, optional, tag = "9")]
        pub hero_roster: ::core::option::Option<super::CMsgHeroSelectionMatchInfo>,
        #[prost(uint64, optional, tag = "10")]
        pub permissions: ::core::option::Option<u64>,
        #[prost(uint64, optional, tag = "11")]
        pub new_player_progress: ::core::option::Option<u64>,
        #[prost(uint32, repeated, tag = "12")]
        pub owned_heroes: ::prost::alloc::vec::Vec<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct LeftMember {
        #[prost(uint32, optional, tag = "1")]
        pub account_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub rights_flags: ::core::option::Option<u32>,
        #[prost(enumeration = "EPlayerType", optional, tag = "3", default = "KEPlayerTypePlayer")]
        pub player_type: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Invite {
        #[prost(uint32, optional, tag = "1")]
        pub account_id: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub persona_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "3")]
        pub invited_by: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EMemberRights {
        KEMemberRightsAdmin = 1,
        KEMemberRightsCreator = 2,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EPlayerType {
        KEPlayerTypePlayer = 0,
        KEPlayerTypeSpectator = 1,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EChatMode {
        KENone = 0,
        KEPartyChat = 1,
        KETeamChat = 2,
    }
}
