pub use crate::common::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelEntityMsgBreakablePropSpawnDebris {
    #[prost(message, optional, tag = "1")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
    #[prost(message, optional, tag = "2")]
    pub damage_pos: ::core::option::Option<CMsgVector>,
    #[prost(float, optional, tag = "3")]
    pub damage: ::core::option::Option<f32>,
    #[prost(message, optional, tag = "4")]
    pub damage_force: ::core::option::Option<CMsgVector>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMessageAbilityNotify {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_victim: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entindex_attacker: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub ability_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub status_impact: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMessageBulletHit {
    #[prost(int32, optional, tag = "1")]
    pub shotid: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub pellet: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub hit_entindex: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub weapon_entindex: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub is_predicted: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMessageCurrencyChanged {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub userid: ::core::option::Option<i32>,
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
    #[prost(uint32, optional, tag = "10")]
    pub new_value: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMessageDamage {
    #[prost(int32, optional, tag = "1")]
    pub damage: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub pre_damage_deprecated: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "27")]
    pub pre_damage: ::core::option::Option<f32>,
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
    pub damage_absorbed_deprecated: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "28")]
    pub damage_absorbed: ::core::option::Option<f32>,
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
    #[prost(int32, optional, tag = "21")]
    pub hitgroup_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "22", default = "-1")]
    pub entindex_attacking_object: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "23")]
    pub damage_direction: ::core::option::Option<CMsgVector>,
    #[prost(bool, optional, tag = "24")]
    pub is_secondary_stat: ::core::option::Option<bool>,
    #[prost(float, optional, tag = "25")]
    pub effectiveness: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "26")]
    pub crit_damage: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "29")]
    pub server_tick: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMessageGameOver {
    #[prost(int32, optional, tag = "1")]
    pub winning_team: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub just_a_test: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMessageImportantAbilityUsed {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub player: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2", default = "16777215")]
    pub caster: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub ability_name: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMessageItemPurchaseNotification {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub userid: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub ability_id: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub sell: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub quickbuy: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMessageMeleeHit {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub hit_entindex: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub heavy: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMessageModifierApplied {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_caster: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entindex_parent: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub serial_number: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMessageObjectiveMask {
    #[prost(uint64, optional, tag = "2")]
    pub objective_mask_team0: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub objective_mask_team1: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgAbilitiesChanged {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub purchaser_player_slot: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub ability_id: ::core::option::Option<u32>,
    #[prost(enumeration = "c_citadel_user_msg_abilities_changed::Change", optional, tag = "3", default = "EInvalid")]
    pub change: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgAbilityLateFailure {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_caster: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entindex_ability: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub failure_type: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgAbilityPing {
    #[prost(message, optional, tag = "1")]
    pub ping_data: ::core::option::Option<PingCommonData>,
    #[prost(uint32, optional, tag = "2")]
    pub ability_id: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "3")]
    pub ability_cooldown: ::core::option::Option<f32>,
    #[prost(
        enumeration = "ChatMsgPingMarkerInfo",
        optional,
        tag = "4",
        default = "KEPingMarkerInfoShowMarkerAndSound"
    )]
    pub ping_marker_and_sound_info: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgAg2ParamTrigger {
    #[prost(string, optional, tag = "1")]
    pub param_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub param_value: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgBannedHeroes {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub banned_hero_ids: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgBossDamaged {
    #[prost(int32, optional, tag = "1")]
    pub objective_team: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub objective_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3", default = "16777215")]
    pub entity_damaged: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgBossKilled {
    #[prost(int32, optional, tag = "1")]
    pub objective_team: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub objective_mask_change: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3", default = "16777215")]
    pub entity_killed: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "4")]
    pub entity_killed_class: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "5", default = "16777215")]
    pub entity_killer: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "6")]
    pub gametime: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "7")]
    pub bosses_remaining: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "8")]
    pub entity_position: ::core::option::Option<CMsgVector>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgCallCheaterVote {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_slot: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgCameraController {
    #[prost(enumeration = "CameraAction", optional, tag = "1", default = "KEActionAddOp")]
    pub action: ::core::option::Option<i32>,
    #[prost(enumeration = "CameraOperation", optional, tag = "2", default = "KECameraOpMaintain")]
    pub operation: ::core::option::Option<i32>,
    #[prost(enumeration = "CameraParam", optional, tag = "3", default = "KEParamClearAllOps")]
    pub param: ::core::option::Option<i32>,
    #[prost(enumeration = "CameraParamMode", optional, tag = "12", default = "KEParamModeAllowInOneContext")]
    pub param_mode: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "4")]
    pub delay: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "11")]
    pub relative_values: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "5")]
    pub context_symbol_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "13", default = "1")]
    pub priority: ::core::option::Option<u32>,
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

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgChatEvent {
    #[prost(enumeration = "ECitadelChatMessage", optional, tag = "1", default = "CitadelChatMessageUnpauseCountdown")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub values: ::prost::alloc::vec::Vec<u32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgDeathReplayData {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub killer_scorer: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub killer_inflictor: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub damage_summary: ::core::option::Option<CCitadelUserMsgRecentDamageSummary>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgEntityPortalled {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub entity_portalled: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub portal_transform: ::core::option::Option<CMsgTransform>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgFlexSlotUnlocked {
    #[prost(int32, optional, tag = "1")]
    pub team_number: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub flexslot_unlocked: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgForceShopClosed {}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgGoldHistory {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_player: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub minute_records: ::prost::alloc::vec::Vec<c_citadel_user_msg_gold_history::MinuteRecord>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
    #[prost(int32, optional, tag = "7")]
    pub victim_team_number: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgHudGameAnnouncement {
    #[prost(string, optional, tag = "1")]
    pub title_locstring: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub description_locstring: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub classname: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub dialog_variable_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub dialog_variable_locstring: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgItemDraftReaction {
    #[prost(message, optional, tag = "1")]
    pub ping_data: ::core::option::Option<PingCommonData>,
    #[prost(bool, optional, tag = "2")]
    pub rare: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub legendary: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgKillStreak {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub player_pawn: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub num_kills: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub is_first_blood: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub streak_ended: ::core::option::Option<bool>,
    #[prost(float, optional, tag = "5", default = "5")]
    pub duration: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgMapLine {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub sender_player_slot: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub mapline: ::core::option::Option<CMsgMapLine>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgMapPing {
    #[prost(message, optional, tag = "1")]
    pub ping_data: ::core::option::Option<PingCommonData>,
    #[prost(uint32, optional, tag = "2")]
    pub event_type: ::core::option::Option<u32>,
    #[prost(
        enumeration = "ChatMsgPingMarkerInfo",
        optional,
        tag = "3",
        default = "KEPingMarkerInfoShowMarkerAndSound"
    )]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgMidBossSpawned {}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgObstructedShotFired {}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgParticipantSetLibraryStackFields {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<CMsgSosSetLibraryStackFields>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgParticipantSetSoundEventParams {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<CMsgSosSetSoundEventParams>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgParticipantStartSoundEvent {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<CMsgSosStartSoundEvent>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgParticipantStopSoundEvent {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<CMsgSosStopSoundEvent>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgParticipantStopSoundEventHash {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<CMsgSosStopSoundEventHash>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgPingWheel {
    #[prost(message, optional, tag = "1")]
    pub ping_data: ::core::option::Option<PingCommonData>,
    #[prost(uint32, optional, tag = "2")]
    pub ping_wheel_option_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgPlayerRespawned {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub player_pawn: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub facing_yaw: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgPostMatchDetails {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub match_details: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgPostProcessingAnim {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_owner: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub clear_all_states: ::core::option::Option<bool>,
    #[prost(enumeration = "PostProcessingGameStates", optional, tag = "3", default = "PostProcStateKilled")]
    pub state: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "4")]
    pub delay: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "5")]
    pub fade_in_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "6")]
    pub hold_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "7")]
    pub fade_out_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "8")]
    pub scale: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgQuickResponse {
    #[prost(message, optional, tag = "1")]
    pub ping_data: ::core::option::Option<PingCommonData>,
    #[prost(uint32, optional, tag = "2")]
    pub responding_to_ping_message_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub responding_to_player_slot: ::core::option::Option<i32>,
    #[prost(enumeration = "CMsgLaneColor", optional, tag = "4", default = "KELaneColorInvalid")]
    pub lane_color: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgRejuvStatus {
    #[prost(int32, optional, tag = "1")]
    pub killing_team: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2", default = "16777215")]
    pub player_pawn: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub user_team: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub event_type: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgReturnIdol {
    #[prost(int32, optional, tag = "1")]
    pub location_index: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub return_location: ::core::option::Option<CMsgVector>,
    #[prost(bool, optional, tag = "3")]
    pub location_enabled: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgSeasonalKill {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub killer: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2", default = "16777215")]
    pub victim: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgSetClientCameraAngles {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_slot: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub camera_angles: ::core::option::Option<CMsgQAngle>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgSpectatorTeamChanged {
    #[prost(int32, optional, tag = "1")]
    pub teamnumber: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCitadelUserMsgStaminaConsumed {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex_target: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "3")]
    pub stamina_before: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub stamina_after: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "5")]
    pub drained: ::core::option::Option<bool>,
    #[prost(float, optional, tag = "6")]
    pub stamina_max: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "7")]
    pub gametime: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgStreetBrawlScoring {
    #[prost(int32, optional, tag = "1")]
    pub scoring_team: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub just_a_test: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "3")]
    pub sapphire_score: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub amber_score: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgTeamMsg {
    #[prost(int32, optional, tag = "1")]
    pub event_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub team_number: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub lane_color: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "4", default = "16777215")]
    pub player_controller: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCitadelUserMsgTeamRewards {
    #[prost(uint32, optional, tag = "1")]
    pub xp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub gold: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub winner: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CLobbyDataPostMatchSurvey {
    #[prost(message, repeated, tag = "1")]
    pub surveys: ::prost::alloc::vec::Vec<c_lobby_data_post_match_survey::PlayerSurvey>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CModifierTableEntry {
    #[prost(enumeration = "ModifierEntryType", optional, tag = "1", default = "Active")]
    pub entry_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2", default = "16777215")]
    pub parent: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub serial_number: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4", default = "0")]
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
    #[prost(uint32, optional, tag = "13", default = "0")]
    pub ability_subclass: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "14")]
    pub in_aura_range: ::core::option::Option<bool>,
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
    #[prost(float, optional, tag = "55")]
    pub float11: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "56")]
    pub float12: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "57")]
    pub float13: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "58")]
    pub float14: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "59")]
    pub float15: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "60")]
    pub float16: ::core::option::Option<f32>,
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgAccountBookStats {
    #[prost(uint32, optional, tag = "1")]
    pub book_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub book_xp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub book_max_xp: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgAccountStats {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub stats: ::prost::alloc::vec::Vec<CMsgAccountHeroStats>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgAnyToGcReportAsserts {
    #[prost(uint32, optional, tag = "1")]
    pub version: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub asserts: ::prost::alloc::vec::Vec<c_msg_any_to_gc_report_asserts::TrackedAssert>,
    #[prost(uint64, optional, tag = "3")]
    pub match_id: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgAnyToGcReportAssertsResponse {
    #[prost(bool, optional, tag = "1")]
    pub success: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
    #[prost(uint32, optional, tag = "7", default = "16777215")]
    pub ability_ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8", default = "16777215")]
    pub impacted_ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub impacted_bone_index: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub weapon_subclass_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11", default = "16777215")]
    pub shooter_ehandle: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "12")]
    pub bullet_radius_override: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgDisableSatVolumesEvent {}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgEquippedItemList {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<CSoEconItem>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[prost(uint32, optional, tag = "19")]
    pub shot_id: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "20", default = "true")]
    pub predict_hits_against_units: ::core::option::Option<bool>,
    #[prost(float, optional, tag = "21", default = "0")]
    pub bullet_radius_override: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "22", default = "0")]
    pub bullet_speed_override: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "23", default = "0")]
    pub bullet_gravity_override: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "24")]
    pub muzzle_number: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "25")]
    pub ability_as_bullet: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcAccountData {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub cheater_report_score: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgHeroBuild {
    #[prost(uint32, optional, tag = "1")]
    pub hero_build_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub hero_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub author_account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub last_updated_timestamp: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "7")]
    pub language: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub origin_build_id: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "10")]
    pub details: ::core::option::Option<c_msg_hero_build::DetailsV0>,
    #[prost(uint32, repeated, packed = "false", tag = "11")]
    pub tags: ::prost::alloc::vec::Vec<u32>,
    #[prost(bool, optional, tag = "12")]
    pub development_build: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "13")]
    pub publish_timestamp: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgHeroBuildPreference {
    #[prost(bool, optional, tag = "1")]
    pub favorited: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub ignored: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub reported: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgHeroReleaseVoteTally {
    #[prost(uint32, optional, tag = "1")]
    pub remaining_votes: ::core::option::Option<u32>,
    #[prost(uint32, repeated, tag = "2")]
    pub votes_cast: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub daily_reward_time_stamp: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgHeroSelectionMatchInfo {
    #[prost(message, repeated, tag = "1")]
    pub hero_selections: ::prost::alloc::vec::Vec<c_msg_hero_selection_match_info::Hero>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub banned_heroes: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CMsgLaneColor {
    KELaneColorInvalid = 0,
    KELaneColorYellow = 1,
    KELaneColorGreen = 3,
    KELaneColorBlue = 4,
    KELaneColorPurple = 6,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgMapLine {
    #[prost(int32, optional, tag = "1")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub y: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub initial: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgMatchMetaData {
    #[prost(uint32, optional, tag = "1")]
    pub version: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub match_details: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "3")]
    pub match_id: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgMatchMetaDataContents {
    #[prost(message, optional, tag = "2")]
    pub match_info: ::core::option::Option<c_msg_match_meta_data_contents::MatchInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgMatchPlayerDamageMatrix {
    #[prost(message, repeated, tag = "1")]
    pub damage_dealers: ::prost::alloc::vec::Vec<c_msg_match_player_damage_matrix::DamageDealer>,
    #[prost(uint32, repeated, tag = "2")]
    pub sample_time_s: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, optional, tag = "3")]
    pub source_details: ::core::option::Option<c_msg_match_player_damage_matrix::SourceDetails>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgParticleSystemManager {
    #[prost(
        enumeration = "ParticleSystemManagerMessage",
        optional,
        tag = "1",
        default = "ParticleSystemManagerEventCreate"
    )]
    pub r#type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub index: ::core::option::Option<u32>,
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgPlayerAnimEvent {
    #[prost(fixed32, optional, tag = "1", default = "16777215")]
    pub player: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub event: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub data: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgPlayerHeroData {
    #[prost(uint32, optional, tag = "1")]
    pub hero_xp: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub hero_equips: ::core::option::Option<CMsgEquippedItemList>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgRegionPingTimesClient {
    #[prost(fixed32, repeated, tag = "1")]
    pub data_center_codes: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "2")]
    pub ping_times: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgRemoveBullet {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub shooter_entindex: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub shot_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub bullet_index: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgRemoveSatVolumeEvent {
    #[prost(int32, optional, tag = "1")]
    pub volume_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgStartFindingMatchInfo {
    #[prost(string, optional, tag = "1")]
    pub server_search_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub server_command_string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "ECitadelMatchMode", optional, tag = "3", default = "KECitadelMatchModeInvalid")]
    pub match_mode: ::core::option::Option<i32>,
    #[prost(enumeration = "ECitadelGameMode", optional, tag = "5", default = "KECitadelGameModeInvalid")]
    pub game_mode: ::core::option::Option<i32>,
    #[prost(enumeration = "ECitadelBotDifficulty", optional, tag = "7", default = "KECitadelBotDifficultyNone")]
    pub bot_difficulty: ::core::option::Option<i32>,
    #[prost(enumeration = "ECitadelRegionMode", optional, tag = "8", default = "KECitadelRegionModeRow")]
    pub region_mode: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "9")]
    pub prefer_solo_only: ::core::option::Option<bool>,
    #[prost(enumeration = "ECitadelMmPreference", optional, tag = "10", default = "KECitadelMmPreferenceInvalid")]
    pub mm_preference: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgTrackedStat {
    #[prost(uint32, optional, tag = "1")]
    pub tracked_stat_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub tracked_stat_value: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSoCitadelHideoutLobby {
    #[prost(uint64, optional, tag = "1")]
    pub hideout_lobby_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub party_id: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub server_steam_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "4")]
    pub udp_connect_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub udp_connect_port: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub sdr_address: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "7")]
    pub server_version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub compat_version: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "9")]
    pub members: ::prost::alloc::vec::Vec<cso_citadel_hideout_lobby::Member>,
    #[prost(uint32, optional, tag = "10")]
    pub active_account_hideout: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "11")]
    pub extra_messages: ::prost::alloc::vec::Vec<CExtraMsgBlock>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSoCitadelLobby {
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
    #[prost(bool, optional, tag = "15")]
    pub match_punishes_abandons: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "16")]
    pub game_mode_version: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSoCitadelParty {
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
    #[prost(bool, optional, tag = "16")]
    pub is_private_lobby: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "17")]
    pub private_lobby_settings: ::core::option::Option<cso_citadel_party::PrivateLobbySettings>,
    #[prost(bool, optional, tag = "18")]
    pub desires_laning_together: ::core::option::Option<bool>,
    #[prost(enumeration = "ECitadelMmPreference", optional, tag = "19", default = "KECitadelMmPreferenceInvalid")]
    pub mm_preference: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "21")]
    pub hideout_search_key: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageEmpty {
    #[prost(uint32, optional, tag = "1")]
    pub empty: ::core::option::Option<u32>,
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

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CameraParamMode {
    KEParamModeAllowInOneContext = 0,
    KEParamModeAllowInMultipleContexts = 1,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChatMsgPingMarkerInfo {
    KEPingMarkerInfoShowMarkerAndSound = 0,
    KEPingMarkerInfoShowMarkerOnSender = 1,
    KEPingMarkerInfoHideMarkerAndSound = 2,
    KEPingMarkerInfoOnlyShowMarker = 3,
    KEPingMarkerInfoOnlyPlaySound = 4,
    KEPingMarkerInfoOnlyMiniMap = 5,
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
    KEUserMsgStaminaConsumed = 337,
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
    KEUserMsgBossDamaged = 348,
    KEUserMsgMidBossSpawned = 349,
    KEUserMsgRejuvStatus = 350,
    KEUserMsgKillStreak = 351,
    KEUserMsgTeamMsg = 352,
    KEUserMsgPlayerRespawned = 353,
    KEUserMsgCallCheaterVote = 354,
    KEUserMsgMeleeHit = 355,
    KEUserMsgFlexSlotUnlocked = 356,
    KEUserMsgSeasonalKill = 357,
    KEUserMsgAg2ParamTrigger = 359,
    KEUserMsgItemPurchaseNotification = 360,
    KEUserMsgEntityPortalled = 361,
    KEUserMsgStreetBrawlScoring = 362,
    KEUserMsgHudGameAnnouncement = 363,
    KEUserMsgItemDraftReaction = 364,
    KEUserMsgImportantAbilityUsed = 365,
    KEUserMsgBannedHeroes = 366,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EBannedFeature {
    KEBannedFeatureInvalid = 0,
    KEBannedFeatureLowPriorityMatchmaking = 1,
    KEBannedFeatureCommsRestricted = 2,
    KEBannedFeatureReportingDisabled = 3,
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
    CitadelChatMessageCommsRestricted = 13,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelGameEvents {
    GeFireBullets = 450,
    GePlayerAnimEvent = 451,
    GeParticleSystemManager = 458,
    GeScreenTextPretty = 459,
    GeBulletImpact = 461,
    GeEnableSatVolumesEvent = 462,
    GePlaceSatVolumeEvent = 463,
    GeDisableSatVolumesEvent = 464,
    GeRemoveSatVolumeEvent = 465,
    GeRemoveBullet = 466,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelGameMode {
    KECitadelGameModeInvalid = 0,
    KECitadelGameModeNormal = 1,
    KECitadelGameMode1v1Test = 2,
    KECitadelGameModeSandbox = 3,
    KECitadelGameModeStreetBrawl = 4,
    KECitadelGameModeExploreNyc = 5,
    KECitadelGameModeInternal = 6,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelLeaderboardRegion {
    KECitadelLeaderboardRegionNone = 0,
    KECitadelLeaderboardRegionEurope = 1,
    KECitadelLeaderboardRegionAsia = 2,
    KECitadelLeaderboardRegionNAmerica = 3,
    KECitadelLeaderboardRegionSAmerica = 4,
    KECitadelLeaderboardRegionOceania = 5,
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
    KECitadelMatchModeHeroLabs = 7,
    KECitadelMatchModeCalibration = 8,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECitadelMmPreference {
    KECitadelMmPreferenceInvalid = 0,
    KECitadelMmPreferenceCasual = 1,
    KECitadelMmPreferenceSerious = 2,
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
pub enum EFeatureBanReason {
    KEFeatureBanReasonInvalid = 0,
    KEFeatureBanReasonDevCommand = 1,
    KEFeatureBanReasonReportedByOtherPlayers = 2,
    KEFeatureBanReasonMatchAbandons = 3,
    KEFeatureBanReasonTooManyReportsSubmitted = 4,
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
pub enum EgcCitadelCommonMessages {
    KEMsgAnyToGcReportAsserts = 7000,
    KEMsgAnyToGcReportAssertsResponse = 7001,
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

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    PostProcStateDoormanHotelVictim = 2,
    PostProcStateBlinded = 3,
    PostProcStateDrifterDarknessCaster = 4,
    PostProcStateMatchIntro = 5,
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
        EFailure = 4,
    }
}

pub mod c_citadel_user_msg_camera_controller {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Maintain {
        #[prost(float, optional, tag = "1", default = "0")]
        pub duration: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Lag {
        #[prost(float, optional, tag = "1")]
        pub min_duration: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "2")]
        pub lag_time: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "3")]
        pub max_speed: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "4")]
        pub spring_strength: ::core::option::Option<f32>,
        #[prost(bool, optional, tag = "5", default = "true")]
        pub increase_spring_strength_to_keep_target_on_screen: ::core::option::Option<bool>,
    }
}

pub mod c_citadel_user_msg_get_damage_stats_response {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct StatType {
        #[prost(uint32, repeated, tag = "1")]
        pub target_player_slot: ::prost::alloc::vec::Vec<u32>,
        #[prost(uint32, repeated, tag = "2")]
        pub value: ::prost::alloc::vec::Vec<u32>,
    }
}

pub mod c_citadel_user_msg_gold_history {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct GoldRecord {
        #[prost(int32, optional, tag = "1")]
        pub currency_source: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub gold: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub events: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct MinuteRecord {
        #[prost(int32, optional, tag = "1")]
        pub match_minute: ::core::option::Option<i32>,
        #[prost(message, repeated, tag = "2")]
        pub gold_records: ::prost::alloc::vec::Vec<GoldRecord>,
    }
}

pub mod c_citadel_user_msg_player_lifetime_stat_info {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
        #[prost(float, optional, tag = "7")]
        pub damage_absorbed: ::core::option::Option<f32>,
        #[prost(bool, optional, tag = "8")]
        pub is_killing_blow: ::core::option::Option<bool>,
        #[prost(uint32, optional, tag = "9")]
        pub victim_hero_id: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "10")]
        pub is_secondary_stat: ::core::option::Option<bool>,
        #[prost(float, optional, tag = "11")]
        pub pre_damage: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "12")]
        pub crit_damage: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct PlayerSurvey {
        #[prost(uint32, optional, tag = "1")]
        pub account_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub question_id: ::core::option::Option<u32>,
    }
}

pub mod c_msg_any_to_gc_report_asserts {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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

pub mod c_msg_fire_bullets {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct TracerAssignment {
        #[prost(uint64, optional, tag = "1")]
        pub tracer_resource_id: ::core::option::Option<u64>,
        #[prost(uint32, optional, tag = "2")]
        pub bullet_indicies: ::core::option::Option<u32>,
    }
}

pub mod c_msg_hero_build {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct BuildModEntry {
        #[prost(uint32, optional, tag = "1")]
        pub ability_id: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub annotation: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "3")]
        pub required_flex_slots: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub sell_priority: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "5")]
        pub imbue_target_ability_id: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct BuildModCategory {
        #[prost(message, repeated, tag = "1")]
        pub mods: ::prost::alloc::vec::Vec<BuildModEntry>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "3")]
        pub description: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(float, optional, tag = "4")]
        pub width: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "5")]
        pub height: ::core::option::Option<f32>,
        #[prost(bool, optional, tag = "6")]
        pub optional: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CurrencyChange {
        #[prost(uint32, optional, tag = "1")]
        pub ability_id: ::core::option::Option<u32>,
        #[prost(int32, optional, tag = "2")]
        pub currency_type: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub delta: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "4")]
        pub annotation: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct AbilityOrder {
        #[prost(message, repeated, tag = "1")]
        pub currency_changes: ::prost::alloc::vec::Vec<CurrencyChange>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct DetailsV0 {
        #[prost(message, repeated, tag = "1")]
        pub mod_categories: ::prost::alloc::vec::Vec<BuildModCategory>,
        #[prost(message, optional, tag = "2")]
        pub ability_order: ::core::option::Option<AbilityOrder>,
    }
}

pub mod c_msg_hero_selection_match_info {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Hero {
        #[prost(uint32, optional, tag = "1")]
        pub hero_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub priority: ::core::option::Option<u32>,
    }
}

pub mod c_msg_match_meta_data_contents {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Position {
        #[prost(float, optional, tag = "1")]
        pub x: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "2")]
        pub y: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "3")]
        pub z: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Deaths {
        #[prost(uint32, optional, tag = "1")]
        pub game_time_s: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "2")]
        pub time_to_kill_s: ::core::option::Option<f32>,
        #[prost(uint32, optional, tag = "9")]
        pub killer_player_slot: ::core::option::Option<u32>,
        #[prost(message, optional, tag = "10")]
        pub death_pos: ::core::option::Option<Position>,
        #[prost(message, optional, tag = "11")]
        pub killer_pos: ::core::option::Option<Position>,
        #[prost(uint32, optional, tag = "12")]
        pub death_duration_s: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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
        #[prost(uint32, optional, tag = "7")]
        pub upgrade_info: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Ping {
        #[prost(uint32, optional, tag = "1")]
        pub ping_type: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub ping_data: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub game_time_s: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CustomUserStatInfo {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "2")]
        pub id: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CustomUserStat {
        #[prost(uint32, optional, tag = "2")]
        pub value: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub id: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct PowerUpBuff {
        #[prost(string, optional, tag = "1")]
        pub r#type: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "2")]
        pub value: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "3")]
        pub is_permanent: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
        #[prost(uint32, optional, tag = "44")]
        pub player_barriering: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "45")]
        pub teammate_healing: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "46")]
        pub teammate_barriering: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "47")]
        pub self_damage: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "48")]
        pub bullet_kills: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "49")]
        pub melee_kills: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "50")]
        pub ability_kills: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "51")]
        pub headshot_kills: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct AbilityStat {
        #[prost(uint32, optional, tag = "1")]
        pub ability_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub ability_value: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct BookReward {
        #[prost(uint32, optional, tag = "1")]
        pub book_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub xp_amount: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub starting_xp: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct PlayerAccolade {
        #[prost(uint32, optional, tag = "1")]
        pub accolade_id: ::core::option::Option<u32>,
        #[prost(int32, optional, tag = "2")]
        pub accolade_stat_value: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub accolade_threshold_achieved: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
        #[prost(message, optional, tag = "25")]
        pub hero_data: ::core::option::Option<super::CMsgPlayerHeroData>,
        #[prost(bool, optional, tag = "26")]
        pub rewards_eligible: ::core::option::Option<bool>,
        #[prost(message, repeated, tag = "48")]
        pub player_tracked_stats: ::prost::alloc::vec::Vec<super::CMsgTrackedStat>,
        #[prost(message, repeated, tag = "27")]
        pub accolades: ::prost::alloc::vec::Vec<PlayerAccolade>,
        #[prost(uint32, optional, tag = "28")]
        pub mvp_rank: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "29")]
        pub earned_holiday_award_2025: ::core::option::Option<bool>,
        #[prost(message, repeated, tag = "30")]
        pub power_up_buffs: ::prost::alloc::vec::Vec<PowerUpBuff>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct Teams {
        #[prost(enumeration = "super::ECitadelLobbyTeam", optional, tag = "1", default = "KECitadelLobbyTeamTeam0")]
        pub team: ::core::option::Option<i32>,
        #[prost(message, repeated, tag = "2")]
        pub team_tracked_stats: ::prost::alloc::vec::Vec<super::CMsgTrackedStat>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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
        #[prost(uint32, optional, tag = "11")]
        pub player_spirit_damage: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct MidBoss {
        #[prost(enumeration = "super::ECitadelLobbyTeam", optional, tag = "1", default = "KECitadelLobbyTeamTeam0")]
        pub team_killed: ::core::option::Option<i32>,
        #[prost(enumeration = "super::ECitadelLobbyTeam", optional, tag = "2", default = "KECitadelLobbyTeamTeam0")]
        pub team_claimed: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "3")]
        pub destroyed_time_s: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Pause {
        #[prost(uint32, optional, tag = "1")]
        pub game_time_s: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub pause_duration_s: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub player_slot: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct WatchedDeathReplay {
        #[prost(uint32, optional, tag = "1")]
        pub game_time_s: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub player_slot: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct StreetBrawlRound {
        #[prost(uint32, optional, tag = "1")]
        pub round_duration_s: ::core::option::Option<u32>,
        #[prost(enumeration = "super::ECitadelLobbyTeam", optional, tag = "2", default = "KECitadelLobbyTeamTeam0")]
        pub winning_team: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
        #[prost(bool, optional, tag = "20")]
        pub is_high_skill_range_parties: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "21")]
        pub low_pri_pool: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "22")]
        pub new_player_pool: ::core::option::Option<bool>,
        #[prost(uint32, optional, tag = "23")]
        pub average_badge_team0: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "24")]
        pub average_badge_team1: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "25")]
        pub game_mode_version: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "26")]
        pub rewards_eligible: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "27")]
        pub not_scored: ::core::option::Option<bool>,
        #[prost(uint32, repeated, packed = "false", tag = "28")]
        pub team_score: ::prost::alloc::vec::Vec<u32>,
        #[prost(message, repeated, tag = "29")]
        pub match_tracked_stats: ::prost::alloc::vec::Vec<super::CMsgTrackedStat>,
        #[prost(message, repeated, tag = "30")]
        pub teams: ::prost::alloc::vec::Vec<Teams>,
        #[prost(
            enumeration = "super::ECitadelBotDifficulty",
            optional,
            tag = "32",
            default = "KECitadelBotDifficultyNone"
        )]
        pub bot_difficulty: ::core::option::Option<i32>,
        #[prost(message, repeated, tag = "33")]
        pub street_brawl_rounds: ::prost::alloc::vec::Vec<StreetBrawlRound>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EMatchOutcome {
        KEOutcomeTeamWin = 0,
        KEOutcomeError = 1,
        KEOutcomeMatchDraw = 2,
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
        KETeamBonus = 8,
        KEAbilityAssassinate = 9,
        KEItemTrophyCollector = 10,
        KEItemCultistSacrifice = 11,
        KEBreakable = 12,
        KEItemGooseEgg = 13,
    }
}

pub mod c_msg_match_player_damage_matrix {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct DamageToPlayer {
        #[prost(uint32, optional, tag = "1")]
        pub target_player_slot: ::core::option::Option<u32>,
        #[prost(uint32, repeated, tag = "2")]
        pub damage: ::prost::alloc::vec::Vec<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct DamageSource {
        #[prost(message, repeated, tag = "2")]
        pub damage_to_players: ::prost::alloc::vec::Vec<DamageToPlayer>,
        #[prost(uint32, optional, tag = "4")]
        pub source_details_index: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct DamageDealer {
        #[prost(uint32, optional, tag = "1")]
        pub dealer_player_slot: ::core::option::Option<u32>,
        #[prost(message, repeated, tag = "2")]
        pub damage_sources: ::prost::alloc::vec::Vec<DamageSource>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
        KETypeRegen = 5,
    }
}

pub mod c_msg_match_player_paths_data {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
        #[prost(uint32, repeated, tag = "9")]
        pub health: ::prost::alloc::vec::Vec<u32>,
        #[prost(enumeration = "ECombatType", repeated, tag = "10")]
        pub combat_type: ::prost::alloc::vec::Vec<i32>,
        #[prost(enumeration = "EMoveType", repeated, tag = "11")]
        pub move_type: ::prost::alloc::vec::Vec<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ECombatType {
        KECombatTypeOut = 0,
        KECombatTypePlayer = 1,
        KECombatTypeEnemyNpc = 2,
        KECombatTypeNeutral = 3,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EMoveType {
        KEMoveTypeNormal = 0,
        KEMoveTypeAbility = 1,
        KEMoveTypeAbilityDebuff = 2,
        KEMoveTypeGroundDash = 3,
        KEMoveTypeSlide = 4,
        KEMoveTypeRopeClimbing = 5,
        KEMoveTypeZiplining = 6,
        KEMoveTypeInAir = 7,
        KEMoveTypeAirDash = 8,
    }
}

pub mod c_msg_particle_system_manager {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct DestroyParticle {
        #[prost(bool, optional, tag = "1")]
        pub destroy_immediately: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct DestroyParticleInvolving {
        #[prost(bool, optional, tag = "1")]
        pub destroy_immediately: ::core::option::Option<bool>,
        #[prost(uint32, optional, tag = "3", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ReleaseParticleIndex {}
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticle {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleFwd {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub forward: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleFallback {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleOffset {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub origin_offset: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct UpdateParticleFrozen {
        #[prost(bool, optional, tag = "1")]
        pub set_frozen: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct UpdateParticleShouldDraw {
        #[prost(bool, optional, tag = "1")]
        pub should_draw: ::core::option::Option<bool>,
    }
}

pub mod cso_citadel_hideout_lobby {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Member {
        #[prost(uint32, optional, tag = "1")]
        pub account_id: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "30")]
        pub hideout_holiday_award_2024: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "32")]
        pub hideout_holiday_award_2025: ::core::option::Option<bool>,
    }
}

pub mod cso_citadel_party {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct PrivateLobbySlot {
        #[prost(uint32, optional, tag = "1")]
        pub slot_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub player_account_id: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ServerRegion {
        #[prost(uint32, optional, tag = "1")]
        pub region_id: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct PrivateLobbySettings {
        #[prost(uint32, optional, tag = "1")]
        pub min_roster_size: ::core::option::Option<u32>,
        #[prost(message, repeated, tag = "2")]
        pub match_slots: ::prost::alloc::vec::Vec<PrivateLobbySlot>,
        #[prost(bool, optional, tag = "3")]
        pub randomize_lanes: ::core::option::Option<bool>,
        #[prost(uint32, optional, tag = "4")]
        pub server_region: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "6")]
        pub is_publicly_visible: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "7")]
        pub cheats_enabled: ::core::option::Option<bool>,
        #[prost(message, repeated, tag = "8")]
        pub available_regions: ::prost::alloc::vec::Vec<ServerRegion>,
        #[prost(bool, optional, tag = "9")]
        pub duplicate_heroes_enabled: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
        #[prost(uint32, optional, tag = "13")]
        pub low_priority_games_remaining: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct LeftMember {
        #[prost(uint32, optional, tag = "1")]
        pub account_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub rights_flags: ::core::option::Option<u32>,
        #[prost(enumeration = "EPlayerType", optional, tag = "3", default = "KEPlayerTypePlayer")]
        pub player_type: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
