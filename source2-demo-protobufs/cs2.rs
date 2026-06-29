pub use crate::common::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct AccountActivity {
    #[prost(uint32, optional, tag = "1")]
    pub activity: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub mode: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub map: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "4")]
    pub matchid: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CAttributeString {
    #[prost(string, optional, tag = "1")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CClientHeaderOverwatchEvidence {
    #[prost(uint32, optional, tag = "1")]
    pub accountid: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub caseid: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgAchievementEvent {
    #[prost(int32, optional, tag = "1")]
    pub achievement: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub user_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgAdjustMoney {
    #[prost(int32, optional, tag = "1")]
    pub amount: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgAmmoDenied {
    #[prost(int32, optional, tag = "1")]
    pub ammoidx: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgBarTime {
    #[prost(string, optional, tag = "1")]
    pub time: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgCallVoteFailed {
    #[prost(int32, optional, tag = "1")]
    pub reason: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub time: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgClientInfo {
    #[prost(int32, optional, tag = "1")]
    pub dummy: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgCloseCaption {
    #[prost(uint32, optional, tag = "1")]
    pub hash: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub duration: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub from_player: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub cctoken: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgCloseCaptionDirect {
    #[prost(uint32, optional, tag = "1")]
    pub hash: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub duration: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub from_player: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgCounterStrafe {
    #[prost(int32, optional, tag = "1")]
    pub press_to_release_ns: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub total_keys_down: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgCurrentRoundOdds {
    #[prost(int32, optional, tag = "1")]
    pub odds: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgCurrentTimescale {
    #[prost(float, optional, tag = "1")]
    pub cur_timescale: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgDamage {
    #[prost(int32, optional, tag = "1")]
    pub amount: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub inflictor_world_pos: ::core::option::Option<CMsgVector>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub victim_entindex: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgDamagePrediction {
    #[prost(int32, optional, tag = "1")]
    pub command_num: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub pellet_idx: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub victim_slot: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub victim_starting_health: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub victim_damage: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "6")]
    pub shoot_pos: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "7")]
    pub shoot_dir: ::core::option::Option<CMsgQAngle>,
    #[prost(message, optional, tag = "8")]
    pub aim_punch: ::core::option::Option<CMsgQAngle>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgDeepStats {
    #[prost(message, optional, tag = "1")]
    pub stats: ::core::option::Option<CMsgGccStrike15ClientDeepStats>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgDesiredTimescale {
    #[prost(float, optional, tag = "1")]
    pub desired_timescale: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub duration_realtime_sec: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "3")]
    pub interpolator_type: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "4")]
    pub start_blend_time: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgDisconnectToLobby {
    #[prost(int32, optional, tag = "1")]
    pub dummy: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgEndOfMatchAllPlayersData {
    #[prost(message, repeated, tag = "1")]
    pub allplayerdata: ::prost::alloc::vec::Vec<ccs_usr_msg_end_of_match_all_players_data::PlayerData>,
    #[prost(int32, optional, tag = "2")]
    pub scene: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgEntityOutlineHighlight {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entidx: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub removehighlight: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgFade {
    #[prost(int32, optional, tag = "1")]
    pub duration: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub hold_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub flags: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub clr: ::core::option::Option<CMsgRgba>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgGameTitle {
    #[prost(int32, optional, tag = "1")]
    pub dummy: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgGeiger {
    #[prost(int32, optional, tag = "1")]
    pub range: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgHintText {
    #[prost(string, optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgHudMsg {
    #[prost(int32, optional, tag = "1")]
    pub channel: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub pos: ::core::option::Option<CMsgVector2D>,
    #[prost(message, optional, tag = "3")]
    pub clr1: ::core::option::Option<CMsgRgba>,
    #[prost(message, optional, tag = "4")]
    pub clr2: ::core::option::Option<CMsgRgba>,
    #[prost(int32, optional, tag = "5")]
    pub effect: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "6")]
    pub fade_in_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "7")]
    pub fade_out_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "9")]
    pub hold_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "10")]
    pub fx_time: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "11")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgHudText {
    #[prost(string, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgItemDrop {
    #[prost(int64, optional, tag = "1")]
    pub itemid: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "2")]
    pub death: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgItemPickup {
    #[prost(string, optional, tag = "1")]
    pub item: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgKeyHintText {
    #[prost(string, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgKillCam {
    #[prost(int32, optional, tag = "1")]
    pub obs_mode: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub first_target: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub second_target: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgMarkAchievement {
    #[prost(string, optional, tag = "1")]
    pub achievement: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgMatchEndConditions {
    #[prost(int32, optional, tag = "1")]
    pub fraglimit: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub mp_maxrounds: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub mp_winlimit: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "4")]
    pub mp_timelimit: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgMatchStatsUpdate {
    #[prost(string, optional, tag = "1")]
    pub update: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgPlayerDecalDigitalSignature {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<PlayerDecalDigitalSignature>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgPlayerStatsUpdate {
    #[prost(int32, optional, tag = "1")]
    pub version: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub stats: ::prost::alloc::vec::Vec<ccs_usr_msg_player_stats_update::Stat>,
    #[prost(uint32, optional, tag = "5")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "6")]
    pub crc: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgPostRoundDamageReport {
    #[prost(uint64, optional, tag = "1")]
    pub other_xuid: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "2")]
    pub given_kill_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub given_health_removed: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub given_num_hits: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub taken_kill_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub taken_health_removed: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub taken_num_hits: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgPreMatchSayText {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub all_chat: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgProcessSpottedEntityUpdate {
    #[prost(bool, optional, tag = "1")]
    pub new_update: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "2")]
    pub entity_updates: ::prost::alloc::vec::Vec<ccs_usr_msg_process_spotted_entity_update::SpottedEntityUpdate>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgQuestProgress {
    #[prost(uint32, optional, tag = "1")]
    pub quest_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub normal_points: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub bonus_points: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "4")]
    pub is_event_quest: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgRadioText {
    #[prost(int32, optional, tag = "1")]
    pub msg_dst: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub client: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub msg_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub params: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgRawAudio {
    #[prost(int32, optional, tag = "1")]
    pub pitch: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entidx: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "3")]
    pub duration: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "4")]
    pub voice_filename: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgRecurringMissionSchema {
    #[prost(uint32, optional, tag = "1")]
    pub period: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub mission_schema: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgReloadEffect {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entidx: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub actanim: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "3")]
    pub origin_x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub origin_y: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "5")]
    pub origin_z: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgReportHit {
    #[prost(float, optional, tag = "1")]
    pub pos_x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub pos_y: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub timestamp: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub pos_z: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgRequestState {
    #[prost(int32, optional, tag = "1")]
    pub dummy: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgResetHud {
    #[prost(bool, optional, tag = "1")]
    pub reset: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgRoundBackupFilenames {
    #[prost(int32, optional, tag = "1")]
    pub count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub index: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub filename: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub nicename: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgRoundEndReportData {
    #[prost(message, optional, tag = "1")]
    pub init_conditions: ::core::option::Option<ccs_usr_msg_round_end_report_data::InitialConditions>,
    #[prost(message, repeated, tag = "2")]
    pub all_rer_event_data: ::prost::alloc::vec::Vec<ccs_usr_msg_round_end_report_data::RerEvent>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgRumble {
    #[prost(int32, optional, tag = "1")]
    pub index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub data: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub flags: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgScoreLeaderboardData {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<ScoreLeaderboardData>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgSendAudio {
    #[prost(string, optional, tag = "1")]
    pub radio_sound: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgSendLastKillerDamageToClient {
    #[prost(int32, optional, tag = "1")]
    pub num_hits_given: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub damage_given: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub num_hits_taken: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub damage_taken: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub actual_damage_given: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub actual_damage_taken: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgSendPlayerItemDrops {
    #[prost(message, repeated, tag = "1")]
    pub entity_updates: ::prost::alloc::vec::Vec<CEconItemPreviewDataBlock>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgSendPlayerItemFound {
    #[prost(message, optional, tag = "1")]
    pub iteminfo: ::core::option::Option<CEconItemPreviewDataBlock>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub playerslot: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgSendPlayerLoadout {
    #[prost(message, repeated, tag = "1")]
    pub loadout: ::prost::alloc::vec::Vec<ccs_usr_msg_send_player_loadout::LoadoutItem>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub playerslot: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgServerRankRevealAll {
    #[prost(int32, optional, tag = "1")]
    pub seconds_till_shutdown: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub reservation: ::core::option::Option<CMsgGccStrike15V2MatchmakingGc2ServerReserve>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgServerRankUpdate {
    #[prost(message, repeated, tag = "1")]
    pub rank_update: ::prost::alloc::vec::Vec<ccs_usr_msg_server_rank_update::RankUpdate>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgShake {
    #[prost(int32, optional, tag = "1")]
    pub command: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub local_amplitude: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub frequency: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub duration: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgShootInfo {
    #[prost(int32, optional, tag = "1")]
    pub frame_number: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub hitbox_transforms: ::prost::alloc::vec::Vec<CMsgTransform>,
    #[prost(message, optional, tag = "3")]
    pub shoot_pos: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "4")]
    pub shoot_dir: ::core::option::Option<CMsgQAngle>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgShowMenu {
    #[prost(int32, optional, tag = "1")]
    pub bits_valid_slots: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub display_time: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub menu_string: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgSsui {
    #[prost(bool, optional, tag = "1")]
    pub show: ::core::option::Option<bool>,
    #[prost(float, optional, tag = "2")]
    pub start_time: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub end_time: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgStopSpectatorMode {
    #[prost(int32, optional, tag = "1")]
    pub dummy: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgSurvivalStats {
    #[prost(uint64, optional, tag = "1")]
    pub xuid: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "2")]
    pub facts: ::prost::alloc::vec::Vec<ccs_usr_msg_survival_stats::Fact>,
    #[prost(message, repeated, tag = "3")]
    pub users: ::prost::alloc::vec::Vec<ccs_usr_msg_survival_stats::Placement>,
    #[prost(message, repeated, tag = "5")]
    pub damages: ::prost::alloc::vec::Vec<ccs_usr_msg_survival_stats::Damage>,
    #[prost(int32, optional, tag = "4")]
    pub ticknumber: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgTrain {
    #[prost(int32, optional, tag = "1")]
    pub train: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgUpdateScreenHealthBar {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entidx: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub healthratio_old: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub healthratio_new: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "4")]
    pub style: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgVguiMenu {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub show: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "3")]
    pub keys: ::prost::alloc::vec::Vec<ccs_usr_msg_vgui_menu::Keys>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgVoiceMask {
    #[prost(message, repeated, tag = "1")]
    pub player_masks: ::prost::alloc::vec::Vec<ccs_usr_msg_voice_mask::PlayerMask>,
    #[prost(bool, optional, tag = "2")]
    pub player_mod_enable: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgVoteFailed {
    #[prost(int32, optional, tag = "1")]
    pub team: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub reason: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgVotePass {
    #[prost(int32, optional, tag = "1")]
    pub team: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub vote_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub disp_str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub details_str: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgVoteSetup {
    #[prost(string, repeated, tag = "1")]
    pub potential_issues: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgVoteStart {
    #[prost(int32, optional, tag = "1")]
    pub team: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_slot: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub vote_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub disp_str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub details_str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub other_team_str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "7")]
    pub is_yes_no_vote: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "8", default = "-1")]
    pub player_slot_target: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgWeaponMagDrop {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entidx: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub secondary_data: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub server_event: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgWeaponSound {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entidx: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub origin_x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub origin_y: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub origin_z: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "5")]
    pub sound: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, optional, tag = "6")]
    pub game_timestamp: ::core::option::Option<f32>,
    #[prost(fixed32, optional, tag = "7")]
    pub source_soundscapeid: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgXRankGet {
    #[prost(int32, optional, tag = "1")]
    pub mode_idx: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub controller: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CCsUsrMsgXRankUpd {
    #[prost(int32, optional, tag = "1")]
    pub mode_idx: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub controller: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub ranking: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCsUsrMsgXpUpdate {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<CMsgGcCstrike15V2Gc2ServerNotifyXpRewarded>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CDataGccStrike15V2MatchInfo {
    #[prost(uint64, optional, tag = "1")]
    pub matchid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub matchtime: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "3")]
    pub watchablematchinfo: ::core::option::Option<WatchableMatchInfo>,
    #[prost(message, optional, tag = "4")]
    pub roundstats_legacy: ::core::option::Option<CMsgGccStrike15V2MatchmakingServerRoundStats>,
    #[prost(message, repeated, tag = "5")]
    pub roundstatsall: ::prost::alloc::vec::Vec<CMsgGccStrike15V2MatchmakingServerRoundStats>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CDataGccStrike15V2TournamentGroup {
    #[prost(uint32, optional, tag = "1")]
    pub groupid: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub desc: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub picks_deprecated: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "5")]
    pub teams: ::prost::alloc::vec::Vec<CDataGccStrike15V2TournamentGroupTeam>,
    #[prost(int32, repeated, packed = "false", tag = "6")]
    pub stage_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint32, optional, tag = "7")]
    pub picklockuntiltime: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub pickableteams: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub points_per_pick: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "10")]
    pub picks: ::prost::alloc::vec::Vec<c_data_gcc_strike15_v2_tournament_group::Picks>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDataGccStrike15V2TournamentGroupTeam {
    #[prost(int32, optional, tag = "1")]
    pub team_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub score: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub correctpick: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CDataGccStrike15V2TournamentInfo {
    #[prost(message, repeated, tag = "1")]
    pub sections: ::prost::alloc::vec::Vec<CDataGccStrike15V2TournamentSection>,
    #[prost(message, optional, tag = "2")]
    pub tournament_event: ::core::option::Option<TournamentEvent>,
    #[prost(message, repeated, tag = "3")]
    pub tournament_teams: ::prost::alloc::vec::Vec<TournamentTeam>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CDataGccStrike15V2TournamentMatchDraft {
    #[prost(int32, optional, tag = "1")]
    pub event_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub event_stage_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub team_id_0: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub team_id_1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub maps_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub maps_current: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub team_id_start: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub team_id_veto1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub team_id_pickn: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "10")]
    pub drafts: ::prost::alloc::vec::Vec<c_data_gcc_strike15_v2_tournament_match_draft::Entry>,
    #[prost(int32, repeated, packed = "false", tag = "11")]
    pub vote_mapid_0: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "12")]
    pub vote_mapid_1: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "13")]
    pub vote_mapid_2: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "14")]
    pub vote_mapid_3: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "15")]
    pub vote_mapid_4: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "16")]
    pub vote_mapid_5: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "17")]
    pub vote_starting_side: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "18")]
    pub vote_phase: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "19")]
    pub vote_phase_start: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "20")]
    pub vote_phase_length: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CDataGccStrike15V2TournamentSection {
    #[prost(uint32, optional, tag = "1")]
    pub sectionid: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub desc: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub groups: ::prost::alloc::vec::Vec<CDataGccStrike15V2TournamentGroup>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CEconItemPreviewDataBlock {
    #[prost(uint32, optional, tag = "1")]
    pub accountid: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub itemid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub defindex: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub paintindex: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub rarity: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub quality: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub paintwear: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub paintseed: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub killeaterscoretype: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub killeatervalue: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "11")]
    pub customname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "12")]
    pub stickers: ::prost::alloc::vec::Vec<c_econ_item_preview_data_block::Sticker>,
    #[prost(uint32, optional, tag = "13")]
    pub inventory: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "14")]
    pub origin: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "15")]
    pub questid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "16")]
    pub dropreason: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "17")]
    pub musicindex: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "18")]
    pub entindex: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "19")]
    pub petindex: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "20")]
    pub keychains: ::prost::alloc::vec::Vec<c_econ_item_preview_data_block::Sticker>,
    #[prost(uint32, optional, tag = "21")]
    pub style: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "22")]
    pub variations: ::prost::alloc::vec::Vec<c_econ_item_preview_data_block::Sticker>,
    #[prost(uint32, optional, tag = "23")]
    pub upgrade_level: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CEngineGotvSyncPacket {
    #[prost(uint64, optional, tag = "1")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub instance_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub signupfragment: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub currentfragment: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "5")]
    pub tickrate: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "6")]
    pub tick: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "8")]
    pub rtdelay: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "9")]
    pub rcvage: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "10")]
    pub keyframe_interval: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "11")]
    pub cdndelay: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgCStrike15Welcome {
    #[prost(uint32, optional, tag = "5")]
    pub store_item_hash: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub timeplayedconsecutively: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub time_first_played: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub last_time_played: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "13")]
    pub last_ip_address: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "18")]
    pub gscookieid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "19")]
    pub uniqueid: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgCsgoSteamUserStatChange {
    #[prost(int32, optional, tag = "1")]
    pub ecsgosteamuserstat: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub delta: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub absolute: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcCstrike15V2ClientRedeemFreeReward {
    #[prost(uint32, optional, tag = "1")]
    pub generation_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub redeemable_balance: ::core::option::Option<u32>,
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub items: ::prost::alloc::vec::Vec<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcCstrike15V2ClientRedeemMissionReward {
    #[prost(uint32, optional, tag = "1")]
    pub campaign_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub redeem_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub redeemable_balance: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub expected_cost: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "5")]
    pub bid_control: ::core::option::Option<i32>,
    #[prost(uint64, repeated, packed = "false", tag = "6")]
    pub volatile_shop: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, optional, tag = "7")]
    pub souvenir_matchid: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcCstrike15V2Gc2ServerNotifyXpRewarded {
    #[prost(message, repeated, tag = "1")]
    pub xp_progress_data: ::prost::alloc::vec::Vec<XpProgressData>,
    #[prost(uint32, optional, tag = "2")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub current_xp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub current_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub upgraded_defidx: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub operation_points_awarded: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub free_rewards: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub xp_trail_remaining: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "9")]
    pub xp_trail_xp_needed: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "10")]
    pub xp_trail_level: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcGlobalGamePlay {
    #[prost(uint64, optional, tag = "1")]
    pub ticket: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub gametimems: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub msperpoint: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcGlobalGameSubscribe {
    #[prost(uint64, optional, tag = "1")]
    pub ticket: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcGlobalGameUnsubscribe {
    #[prost(int32, optional, tag = "1")]
    pub timeleft: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcServerQuestUpdateData {
    #[prost(message, repeated, tag = "1")]
    pub player_quest_data: ::prost::alloc::vec::Vec<PlayerQuestData>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub binary_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "3")]
    pub mm_game_mode: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "4")]
    pub missionlbsdata: ::core::option::Option<ScoreLeaderboardData>,
    #[prost(uint32, optional, tag = "5")]
    pub flags: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToClientSteamDatagramTicket {
    #[prost(bytes = "vec", optional, tag = "16")]
    pub serialized_ticket: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15ClientDeepStats {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<c_msg_gcc_strike15_client_deep_stats::DeepStatsRange>,
    #[prost(message, repeated, tag = "3")]
    pub matches: ::prost::alloc::vec::Vec<c_msg_gcc_strike15_client_deep_stats::DeepStatsMatch>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15GotvSyncPacket {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<CEngineGotvSyncPacket>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2AccountPrivacySettings {
    #[prost(message, repeated, tag = "1")]
    pub settings: ::prost::alloc::vec::Vec<c_msg_gcc_strike15_v2_account_privacy_settings::Setting>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2AccountRequestCoPlays {
    #[prost(message, repeated, tag = "1")]
    pub players: ::prost::alloc::vec::Vec<c_msg_gcc_strike15_v2_account_request_co_plays::Player>,
    #[prost(uint32, optional, tag = "2")]
    pub servertime: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2AcknowledgePenalty {
    #[prost(int32, optional, tag = "1")]
    pub acknowledged: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2BetaEnrollment {
    #[prost(uint32, optional, tag = "1")]
    pub eresult: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Client2GcAckXpShopTracks {}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Client2GcEconPreviewDataBlockRequest {
    #[prost(uint64, optional, tag = "1")]
    pub param_s: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub param_a: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub param_d: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "4")]
    pub param_m: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2Client2GcEconPreviewDataBlockResponse {
    #[prost(message, optional, tag = "1")]
    pub iteminfo: ::core::option::Option<CEconItemPreviewDataBlock>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Client2GcRequestPrestigeCoin {
    #[prost(uint32, optional, tag = "1")]
    pub defindex: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub upgradeid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub hours: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub prestigetime: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Client2GcStreamUnlock {
    #[prost(uint64, optional, tag = "1")]
    pub ticket: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "2")]
    pub os: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Client2GcTextMsg {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientAccountBalance {
    #[prost(uint64, optional, tag = "1")]
    pub amount: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientAuthKeyCode {
    #[prost(uint32, optional, tag = "1")]
    pub eventid: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientCommendPlayer {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "8")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "9")]
    pub commendation: ::core::option::Option<PlayerCommendationInfo>,
    #[prost(uint32, optional, tag = "10")]
    pub tokens: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientGcRankUpdate {
    #[prost(message, repeated, tag = "1")]
    pub rankings: ::prost::alloc::vec::Vec<PlayerRankingInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientLogonFatalError {
    #[prost(uint32, optional, tag = "1")]
    pub errorcode: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub country: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientNetworkConfig {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientPartyJoinRelay {
    #[prost(uint32, optional, tag = "1")]
    pub accountid: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub lobbyid: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientPartyWarning {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<c_msg_gcc_strike15_v2_client_party_warning::Entry>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientPerfReport {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<c_msg_gcc_strike15_v2_client_perf_report::Entry>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientPlayerDecalSign {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<PlayerDecalDigitalSignature>,
    #[prost(uint64, optional, tag = "2")]
    pub itemid: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientPollState {
    #[prost(uint32, optional, tag = "1")]
    pub pollid: ::core::option::Option<u32>,
    #[prost(string, repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub values: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientReportPlayer {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub rpt_aimbot: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub rpt_wallhack: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub rpt_speedhack: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub rpt_teamharm: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub rpt_textabuse: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub rpt_voiceabuse: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "8")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "9")]
    pub report_from_demo: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientReportResponse {
    #[prost(uint64, optional, tag = "1")]
    pub confirmation_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub server_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub response_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub response_result: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub tokens: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientReportServer {
    #[prost(uint32, optional, tag = "1")]
    pub rpt_poorperf: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub rpt_abusivemodels: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub rpt_badmotd: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub rpt_listingabuse: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub rpt_inventoryabuse: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "8")]
    pub match_id: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientRequestJoinFriendData {
    #[prost(uint32, optional, tag = "1")]
    pub version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub join_token: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub join_ipp: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "5")]
    pub res: ::core::option::Option<CMsgGccStrike15V2MatchmakingGc2ClientReserve>,
    #[prost(string, optional, tag = "6")]
    pub errormsg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "7")]
    pub is_local_server: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientRequestJoinServerData {
    #[prost(uint32, optional, tag = "1")]
    pub version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "3")]
    pub serverid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "4")]
    pub server_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub server_port: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "6")]
    pub res: ::core::option::Option<CMsgGccStrike15V2MatchmakingGc2ClientReserve>,
    #[prost(string, optional, tag = "7")]
    pub errormsg: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientRequestOffers {}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientRequestPlayersProfile {
    #[prost(uint32, optional, tag = "1")]
    pub request_id_deprecated: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub account_ids_deprecated: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub request_level: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientRequestSouvenir {
    #[prost(uint64, optional, tag = "1")]
    pub itemid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub matchid: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "3")]
    pub eventid: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientRequestWatchInfoFriends {
    #[prost(uint32, optional, tag = "1")]
    pub request_id: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub account_ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint64, optional, tag = "3")]
    pub serverid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "4")]
    pub matchid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "5")]
    pub client_launcher: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "6")]
    pub data_center_pings: ::prost::alloc::vec::Vec<DataCenterPing>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientSubmitSurveyVote {
    #[prost(uint32, optional, tag = "1")]
    pub survey_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub vote: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientToGcChat {
    #[prost(uint64, optional, tag = "1")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientToGcRequestElevate {
    #[prost(uint32, optional, tag = "1")]
    pub stage: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientToGcRequestTicket {
    #[prost(fixed64, optional, tag = "1")]
    pub authorized_steam_id: ::core::option::Option<u64>,
    #[prost(fixed32, optional, tag = "2")]
    pub authorized_public_ip: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag = "3")]
    pub gameserver_steam_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "5")]
    pub gameserver_sdr_routing: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ClientVarValueNotificationInfo {
    #[prost(string, optional, tag = "1")]
    pub value_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub value_int: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub server_addr: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub server_port: ::core::option::Option<u32>,
    #[prost(string, repeated, tag = "5")]
    pub choked_blocks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2Fantasy {
    #[prost(uint32, optional, tag = "1")]
    pub event_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub teams: ::prost::alloc::vec::Vec<c_msg_gcc_strike15_v2_fantasy::FantasyTeam>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Gc2ClientInitSystem {
    #[prost(bool, optional, tag = "1")]
    pub load: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub outputname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub key_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub sha_hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "6")]
    pub cookie: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "7")]
    pub manifest: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub system_package: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "9")]
    pub load_system: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Gc2ClientInitSystemResponse {
    #[prost(bool, optional, tag = "1")]
    pub success: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub diagnostic: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub sha_hash: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "4")]
    pub response: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub error_code1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub error_code2: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "7")]
    pub handle: ::core::option::Option<i64>,
    #[prost(enumeration = "EInitSystemResult", optional, tag = "8", default = "KEInitSystemResultInvalid")]
    pub einit_result: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub aux_system1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub aux_system2: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Gc2ClientNotifyXpShop {
    #[prost(message, optional, tag = "1")]
    pub prematch: ::core::option::Option<CSoAccountXpShop>,
    #[prost(message, optional, tag = "2")]
    pub postmatch: ::core::option::Option<CSoAccountXpShop>,
    #[prost(uint32, optional, tag = "3")]
    pub current_xp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub current_level: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Gc2ClientRefuseSecureMode {
    #[prost(string, optional, tag = "1")]
    pub file_report: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub offer_insecure_mode: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub offer_secure_mode: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub show_unsigned_ui: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub kick_user: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub show_trusted_ui: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub show_warning_not_trusted: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub show_warning_not_trusted_2: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "9")]
    pub files_prevented_trusted: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Gc2ClientRequestValidation {
    #[prost(bool, optional, tag = "1")]
    pub full_report: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub module: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Gc2ClientTextMsg {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub payload: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Gc2ClientTournamentInfo {
    #[prost(uint32, optional, tag = "1")]
    pub eventid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub stageid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub game_type: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub teamids: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Gc2ServerReservationUpdate {
    #[prost(uint32, optional, tag = "1")]
    pub viewers_external_total: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub viewers_external_steam: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2GcToClientChat {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2GetEventFavoritesRequest {
    #[prost(bool, optional, tag = "1")]
    pub all_events: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2GetEventFavoritesResponse {
    #[prost(bool, optional, tag = "1")]
    pub all_events: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub json_favorites: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub json_featured: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2GiftsLeaderboardRequest {}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2GiftsLeaderboardResponse {
    #[prost(uint32, optional, tag = "1")]
    pub servertime: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub time_period_seconds: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub total_gifts_given: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub total_givers: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "5")]
    pub entries: ::prost::alloc::vec::Vec<c_msg_gcc_strike15_v2_gifts_leaderboard_response::GiftLeaderboardEntry>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchEndRewardDropsNotification {
    #[prost(message, optional, tag = "6")]
    pub iteminfo: ::core::option::Option<CEconItemPreviewDataBlock>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchEndRunRewardDrops {
    #[prost(message, optional, tag = "3")]
    pub serverinfo: ::core::option::Option<CMsgGccStrike15V2MatchmakingServerReservationResponse>,
    #[prost(message, optional, tag = "4")]
    pub match_end_quest_data: ::core::option::Option<CMsgGcServerQuestUpdateData>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchList {
    #[prost(uint32, optional, tag = "1")]
    pub msgrequestid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub accountid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub servertime: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "4")]
    pub matches: ::prost::alloc::vec::Vec<CDataGccStrike15V2MatchInfo>,
    #[prost(message, repeated, tag = "5")]
    pub streams: ::prost::alloc::vec::Vec<TournamentTeam>,
    #[prost(message, optional, tag = "6")]
    pub tournamentinfo: ::core::option::Option<CDataGccStrike15V2TournamentInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchListRequestCurrentLiveGames {}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchListRequestFullGameInfo {
    #[prost(uint64, optional, tag = "1")]
    pub matchid: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub outcomeid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub token: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchListRequestLiveGameForUser {
    #[prost(uint32, optional, tag = "1")]
    pub accountid: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchListRequestRecentUserGames {
    #[prost(uint32, optional, tag = "1")]
    pub accountid: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchListRequestTournamentGames {
    #[prost(int32, optional, tag = "1")]
    pub eventid: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchListTournamentOperatorMgmt {
    #[prost(int32, optional, tag = "1")]
    pub eventid: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub matches: ::prost::alloc::vec::Vec<CDataGccStrike15V2MatchInfo>,
    #[prost(uint32, optional, tag = "3")]
    pub accountid: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingClient2GcHello {}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingClient2ServerPing {
    #[prost(message, repeated, tag = "1")]
    pub gameserverpings: ::prost::alloc::vec::Vec<GameServerPing>,
    #[prost(int32, optional, tag = "2")]
    pub offset_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub final_batch: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub data_center_pings: ::prost::alloc::vec::Vec<DataCenterPing>,
    #[prost(uint32, optional, tag = "5")]
    pub max_ping: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "6")]
    pub test_token: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub search_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "8")]
    pub notes: ::prost::alloc::vec::Vec<CMsgGccStrike15V2MatchmakingGc2ClientUpdateNote>,
    #[prost(string, optional, tag = "9")]
    pub debug_message: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingGc2ClientAbandon {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub abandoned_match: ::core::option::Option<CMsgGccStrike15V2MatchmakingGc2ClientReserve>,
    #[prost(uint32, optional, tag = "3")]
    pub penalty_seconds: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub penalty_reason: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingGc2ClientHello {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub ongoingmatch: ::core::option::Option<CMsgGccStrike15V2MatchmakingGc2ClientReserve>,
    #[prost(message, optional, tag = "3")]
    pub global_stats: ::core::option::Option<GlobalStatistics>,
    #[prost(uint32, optional, tag = "4")]
    pub penalty_seconds: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub penalty_reason: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "6")]
    pub vac_banned: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "7")]
    pub ranking: ::core::option::Option<PlayerRankingInfo>,
    #[prost(message, optional, tag = "8")]
    pub commendation: ::core::option::Option<PlayerCommendationInfo>,
    #[prost(message, optional, tag = "9")]
    pub medals: ::core::option::Option<PlayerMedalsInfo>,
    #[prost(message, optional, tag = "10")]
    pub my_current_event: ::core::option::Option<TournamentEvent>,
    #[prost(message, repeated, tag = "11")]
    pub my_current_event_teams: ::prost::alloc::vec::Vec<TournamentTeam>,
    #[prost(message, optional, tag = "12")]
    pub my_current_team: ::core::option::Option<TournamentTeam>,
    #[prost(message, repeated, tag = "13")]
    pub my_current_event_stages: ::prost::alloc::vec::Vec<TournamentEvent>,
    #[prost(uint32, optional, tag = "14")]
    pub survey_vote: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "15")]
    pub activity: ::core::option::Option<AccountActivity>,
    #[prost(int32, optional, tag = "17")]
    pub player_level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "18")]
    pub player_cur_xp: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "19")]
    pub player_xp_bonus_flags: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "20")]
    pub rankings: ::prost::alloc::vec::Vec<PlayerRankingInfo>,
    #[prost(uint64, optional, tag = "21")]
    pub owcaseid: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingGc2ClientReserve {
    #[prost(uint64, optional, tag = "1")]
    pub serverid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub direct_udp_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub direct_udp_port: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "4")]
    pub reservationid: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "5")]
    pub reservation: ::core::option::Option<CMsgGccStrike15V2MatchmakingGc2ServerReserve>,
    #[prost(string, optional, tag = "6")]
    pub map: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub server_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "8")]
    pub gs_ping: ::core::option::Option<DataCenterPing>,
    #[prost(uint32, optional, tag = "9")]
    pub gs_location_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingGc2ClientSearchStats {
    #[prost(uint32, optional, tag = "1")]
    pub gs_location_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub data_center_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub num_locked_in: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub num_found_nearby: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub note_level: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingGc2ClientUpdate {
    #[prost(int32, optional, tag = "1")]
    pub matchmaking: ::core::option::Option<i32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub waiting_account_id_sessions: ::prost::alloc::vec::Vec<u32>,
    #[prost(string, optional, tag = "3")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub ongoingmatch_account_id_sessions: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, optional, tag = "7")]
    pub global_stats: ::core::option::Option<GlobalStatistics>,
    #[prost(uint32, repeated, packed = "false", tag = "8")]
    pub failping_account_id_sessions: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "9")]
    pub penalty_account_id_sessions: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "10")]
    pub failready_account_id_sessions: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "11")]
    pub vacbanned_account_id_sessions: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, optional, tag = "12")]
    pub server_ipaddress_mask: ::core::option::Option<IpAddressMask>,
    #[prost(message, repeated, tag = "13")]
    pub notes: ::prost::alloc::vec::Vec<CMsgGccStrike15V2MatchmakingGc2ClientUpdateNote>,
    #[prost(uint32, repeated, packed = "false", tag = "14")]
    pub penalty_account_id_sessions_green: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "15")]
    pub insufficientlevel_sessions: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "16")]
    pub vsncheck_account_id_sessions: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "17")]
    pub launcher_mismatch_sessions: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "18")]
    pub insecure_account_id_sessions: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingGc2ClientUpdateNote {
    #[prost(int32, optional, tag = "1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub region_id: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "3")]
    pub region_r: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub distance: ::core::option::Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingGc2ServerConfirm {
    #[prost(uint32, optional, tag = "1")]
    pub token: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub stamp: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "3")]
    pub exchange: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "4")]
    pub retry: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingGc2ServerReserve {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub account_ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub game_type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "3")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "4")]
    pub server_version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "18")]
    pub flags: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "5")]
    pub rankings: ::prost::alloc::vec::Vec<PlayerRankingInfo>,
    #[prost(uint64, optional, tag = "6")]
    pub encryption_key: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "7")]
    pub encryption_key_pub: ::core::option::Option<u64>,
    #[prost(uint32, repeated, packed = "false", tag = "8")]
    pub party_ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "9")]
    pub whitelist: ::prost::alloc::vec::Vec<IpAddressMask>,
    #[prost(uint64, optional, tag = "10")]
    pub tv_master_steamid: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "11")]
    pub tournament_event: ::core::option::Option<TournamentEvent>,
    #[prost(message, repeated, tag = "12")]
    pub tournament_teams: ::prost::alloc::vec::Vec<TournamentTeam>,
    #[prost(uint32, repeated, packed = "false", tag = "13")]
    pub tournament_casters_account_ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint64, optional, tag = "14")]
    pub tv_relay_steamid: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "15")]
    pub pre_match_data: ::core::option::Option<CPreMatchInfoData>,
    #[prost(uint32, optional, tag = "17")]
    pub tv_control: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "19")]
    pub op_var_values: ::prost::alloc::vec::Vec<OperationalVarValue>,
    #[prost(uint32, optional, tag = "20")]
    pub socache_control: ::core::option::Option<u32>,
    #[prost(int32, repeated, packed = "false", tag = "21")]
    pub teammate_colors: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint32, optional, tag = "22")]
    pub match_id_additional: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingOperator2GcBlogUpdate {
    #[prost(string, optional, tag = "1")]
    pub main_post_url: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingServerReservationResponse {
    #[prost(uint64, optional, tag = "1")]
    pub reservationid: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "2")]
    pub reservation: ::core::option::Option<CMsgGccStrike15V2MatchmakingGc2ServerReserve>,
    #[prost(string, optional, tag = "3")]
    pub map: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "4")]
    pub gc_reservation_sent: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "5")]
    pub server_version: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "6")]
    pub tv_info: ::core::option::Option<ServerHltvInfo>,
    #[prost(uint32, repeated, packed = "false", tag = "7")]
    pub reward_player_accounts: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "8")]
    pub idle_player_accounts: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub reward_item_attr_def_idx: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub reward_item_attr_value: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub reward_item_attr_reward_idx: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub reward_drop_list: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "13")]
    pub tournament_tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "14")]
    pub legacy_steamdatagram_port: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "17")]
    pub steamdatagram_routing: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "15")]
    pub test_token: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "16")]
    pub flags: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "18")]
    pub system_load: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "19")]
    pub cpus_online: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingServerRoundStats {
    #[prost(uint64, optional, tag = "1")]
    pub reservationid: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "2")]
    pub reservation: ::core::option::Option<CMsgGccStrike15V2MatchmakingGc2ServerReserve>,
    #[prost(string, optional, tag = "3")]
    pub map: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub round: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "5")]
    pub kills: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "6")]
    pub assists: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "7")]
    pub deaths: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "8")]
    pub scores: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "9")]
    pub pings: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "10")]
    pub round_result: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub match_result: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "12")]
    pub team_scores: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, optional, tag = "13")]
    pub confirm: ::core::option::Option<CMsgGccStrike15V2MatchmakingGc2ServerConfirm>,
    #[prost(int32, optional, tag = "14")]
    pub reservation_stage: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "15")]
    pub match_duration: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "16")]
    pub enemy_kills: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "17")]
    pub enemy_headshots: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "18")]
    pub enemy_3ks: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "19")]
    pub enemy_4ks: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "20")]
    pub enemy_5ks: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "21")]
    pub mvps: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint32, optional, tag = "22")]
    pub spectators_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "23")]
    pub spectators_count_tv: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "24")]
    pub spectators_count_lnk: ::core::option::Option<u32>,
    #[prost(int32, repeated, packed = "false", tag = "25")]
    pub enemy_kills_agg: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, optional, tag = "26")]
    pub drop_info: ::core::option::Option<c_msg_gcc_strike15_v2_matchmaking_server_round_stats::DropInfo>,
    #[prost(bool, optional, tag = "27")]
    pub b_switched_teams: ::core::option::Option<bool>,
    #[prost(int32, repeated, packed = "false", tag = "28")]
    pub enemy_2ks: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "29")]
    pub player_spawned: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "30")]
    pub team_spawn_count: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint32, optional, tag = "31")]
    pub max_rounds: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "32")]
    pub map_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingStart {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub account_ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub game_type: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub ticket_data: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub client_version: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "5")]
    pub tournament_match: ::core::option::Option<TournamentMatchSetup>,
    #[prost(bool, optional, tag = "6")]
    pub prime_only: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "7")]
    pub tv_control: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "8")]
    pub lobby_id: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2MatchmakingStop {
    #[prost(int32, optional, tag = "1")]
    pub abandon: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2PartyInvite {
    #[prost(uint32, optional, tag = "1")]
    pub accountid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub lobbyid: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2PartyRegister {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub ver: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub apr: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub ark: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub nby: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub grp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub slots: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub launcher: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub game_type: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2PartySearch {
    #[prost(uint32, optional, tag = "1")]
    pub ver: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub apr: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub ark: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub grps: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub launcher: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub game_type: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2PartySearchResults {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<c_msg_gcc_strike15_v2_party_search_results::Entry>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2PlayerOverwatchCaseAssignment {
    #[prost(uint64, optional, tag = "1")]
    pub caseid: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub caseurl: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub verdict: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub timestamp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub throttleseconds: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub suspectid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub fractionid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub numrounds: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub fractionrounds: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "10")]
    pub streakconvictions: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "11")]
    pub reason: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2PlayerOverwatchCaseStatus {
    #[prost(uint64, optional, tag = "1")]
    pub caseid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub statusid: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2PlayerOverwatchCaseUpdate {
    #[prost(uint64, optional, tag = "1")]
    pub caseid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub suspectid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub fractionid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub rpt_aimbot: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub rpt_wallhack: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub rpt_speedhack: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub rpt_teamharm: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub reason: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2PlayersProfile {
    #[prost(uint32, optional, tag = "1")]
    pub request_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub account_profiles: ::prost::alloc::vec::Vec<CMsgGccStrike15V2MatchmakingGc2ClientHello>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2Predictions {
    #[prost(uint32, optional, tag = "1")]
    pub event_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub group_match_team_picks: ::prost::alloc::vec::Vec<c_msg_gcc_strike15_v2_predictions::GroupMatchTeamPick>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2PremierSeasonSummary {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub season_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "3")]
    pub data_per_week: ::prost::alloc::vec::Vec<c_msg_gcc_strike15_v2_premier_season_summary::DataPerWeek>,
    #[prost(message, repeated, tag = "4")]
    pub data_per_map: ::prost::alloc::vec::Vec<c_msg_gcc_strike15_v2_premier_season_summary::DataPerMap>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2Server2GcClientValidate {
    #[prost(uint32, optional, tag = "1")]
    pub accountid: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ServerNotificationForUserPenalty {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub reason: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub seconds: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "4")]
    pub communication_cooldown: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2ServerVarValueNotificationInfo {
    #[prost(uint32, optional, tag = "1")]
    pub accountid: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub viewangles: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub userdata: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2SetEventFavorite {
    #[prost(uint64, optional, tag = "1")]
    pub eventid: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "2")]
    pub is_favorite: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2SetPlayerLeaderboardSafeName {
    #[prost(string, optional, tag = "1")]
    pub leaderboard_safe_name: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGccStrike15V2VolatileShopSubscribe {
    #[prost(uint32, optional, tag = "1")]
    pub defidx: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub psid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub upnext: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub gctime: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub payload: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGccStrike15V2WatchInfoUsers {
    #[prost(uint32, optional, tag = "1")]
    pub request_id: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub account_ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "3")]
    pub watchable_match_infos: ::prost::alloc::vec::Vec<WatchableMatchInfo>,
    #[prost(uint32, optional, tag = "5")]
    pub extended_timeout: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgLegacySource1ClientWelcome {
    #[prost(uint32, optional, tag = "1")]
    pub version: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub game_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "3")]
    pub outofdate_subscribed_caches: ::prost::alloc::vec::Vec<CMsgSoCacheSubscribed>,
    #[prost(message, repeated, tag = "4")]
    pub uptodate_subscribed_caches: ::prost::alloc::vec::Vec<CMsgSoCacheSubscriptionCheck>,
    #[prost(message, optional, tag = "5")]
    pub location: ::core::option::Option<c_msg_legacy_source1_client_welcome::Location>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub game_data2: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "7")]
    pub rtime32_gc_welcome_timestamp: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub currency: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub balance: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "10")]
    pub balance_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub txn_country_code: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgPlayerBulletHit {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub attacker_slot: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub victim_slot: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub victim_pos: ::core::option::Option<CMsgVector>,
    #[prost(int32, optional, tag = "4")]
    pub hit_group: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub damage: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub penetration_count: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "7")]
    pub is_kill: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub through_smoke: ::core::option::Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgRecurringMissionSchema {
    #[prost(message, repeated, tag = "1")]
    pub missions: ::prost::alloc::vec::Vec<c_msg_recurring_mission_schema::MissionTemplateList>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgRequestRecurringMissionSchedule {}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgTeFireBullets {
    #[prost(message, optional, tag = "1")]
    pub origin: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "2")]
    pub angles: ::core::option::Option<CMsgQAngle>,
    #[prost(uint32, optional, tag = "3", default = "16777215")]
    pub weapon_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub mode: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub seed: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "6", default = "16777215")]
    pub player: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "7")]
    pub inaccuracy: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "8")]
    pub recoil_index: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "9")]
    pub spread: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "10")]
    pub sound_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "11")]
    pub item_def_index: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "12")]
    pub sound_dsp_effect: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "13")]
    pub ent_origin: ::core::option::Option<CMsgVector>,
    #[prost(uint32, optional, tag = "14")]
    pub num_bullets_remaining: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "15")]
    pub attack_type: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "17")]
    pub player_inair: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "18")]
    pub player_scoped: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "19")]
    pub tick: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "16")]
    pub extra: ::core::option::Option<c_msg_te_fire_bullets::Extra>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgTePlayerAnimEvent {
    #[prost(fixed32, optional, tag = "1", default = "16777215")]
    pub player: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub event: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub data: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgTeRadioIcon {
    #[prost(fixed32, optional, tag = "1", default = "16777215")]
    pub player: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CPreMatchInfoData {
    #[prost(int32, optional, tag = "1")]
    pub predictions_pct: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub draft: ::core::option::Option<CDataGccStrike15V2TournamentMatchDraft>,
    #[prost(message, repeated, tag = "5")]
    pub stats: ::prost::alloc::vec::Vec<c_pre_match_info_data::TeamStats>,
    #[prost(int32, repeated, packed = "false", tag = "6")]
    pub wins: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoAccountItemPersonalStore {
    #[prost(uint32, optional, tag = "1")]
    pub generation_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub redeemable_balance: ::core::option::Option<u32>,
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub items: ::prost::alloc::vec::Vec<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoAccountKeychainRemoveToolCharges {
    #[prost(uint32, optional, tag = "1")]
    pub charges: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoAccountRecurringMission {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub mission_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub period: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub progress: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoAccountRecurringSubscription {
    #[prost(uint32, optional, tag = "1")]
    pub time_next_cycle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub time_initiated: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoAccountSeasonalOperation {
    #[prost(uint32, optional, tag = "1")]
    pub season_value: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub tier_unlocked: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub premium_tiers: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub mission_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub missions_completed: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub redeemable_balance: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub season_pass_time: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoAccountXpShop {
    #[prost(uint32, optional, tag = "1")]
    pub generation_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub redeemable_balance: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub xp_tracks: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoAccountXpShopBids {
    #[prost(uint32, optional, tag = "1")]
    pub campaign_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub redeem_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub expected_cost: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub generation_time: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoEconCoupon {
    #[prost(uint32, optional, tag = "1")]
    pub entryid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub defidx: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "3")]
    pub expiration_date: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoGameAccountSteamChina {
    #[prost(uint32, optional, tag = "1")]
    pub time_last_update: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub time_comms_ban: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub time_play_ban: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoPersonaDataPublic {
    #[prost(int32, optional, tag = "1")]
    pub player_level: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub commendation: ::core::option::Option<PlayerCommendationInfo>,
    #[prost(bool, optional, tag = "3")]
    pub elevated_state: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "4")]
    pub xp_trail_timestamp_refresh: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub xp_trail_level: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoQuestProgress {
    #[prost(uint32, optional, tag = "1")]
    pub questid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub points_remaining: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub bonus_points: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoVolatileItemClaimedRewards {
    #[prost(uint32, optional, tag = "1")]
    pub defidx: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub reward: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub generation_time: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoVolatileItemOffer {
    #[prost(uint32, optional, tag = "1")]
    pub defidx: ::core::option::Option<u32>,
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub faux_itemid: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub generation_time: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CvDiagnostic {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub extended: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "3")]
    pub value: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "4")]
    pub string_value: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DataCenterPing {
    #[prost(fixed32, optional, tag = "1")]
    pub data_center_id: ::core::option::Option<u32>,
    #[prost(sint32, optional, tag = "2")]
    pub ping: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DeepPlayerMatchEvent {
    #[prost(uint32, optional, tag = "1")]
    pub accountid: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub event_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub event_type: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "5")]
    pub b_playing_ct: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "6")]
    pub user_pos_x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub user_pos_y: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub user_pos_z: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "8")]
    pub user_defidx: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "9")]
    pub other_pos_x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub other_pos_y: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub other_pos_z: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "11")]
    pub other_defidx: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "14")]
    pub event_data: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DeepPlayerStatsEntry {
    #[prost(uint32, optional, tag = "1")]
    pub accountid: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub mm_game_mode: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub mapid: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "5")]
    pub b_starting_ct: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "6")]
    pub match_outcome: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub rounds_won: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub rounds_lost: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub stat_score: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub stat_deaths: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "13")]
    pub stat_mvps: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "14")]
    pub enemy_kills: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "15")]
    pub enemy_headshots: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "16")]
    pub enemy_2ks: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "17")]
    pub enemy_3ks: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "18")]
    pub enemy_4ks: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "19")]
    pub total_damage: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "23")]
    pub engagements_entry_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "24")]
    pub engagements_entry_wins: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "25")]
    pub engagements_1v1_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "26")]
    pub engagements_1v1_wins: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "27")]
    pub engagements_1v2_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "28")]
    pub engagements_1v2_wins: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "29")]
    pub utility_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "30")]
    pub utility_success: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "32")]
    pub flash_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "33")]
    pub flash_success: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "34")]
    pub mates: ::prost::alloc::vec::Vec<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DetailedSearchStatistic {
    #[prost(uint32, optional, tag = "1")]
    pub game_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub search_time_avg: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub players_searching: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EClientReportingVersion {
    KEClientReportingVersionOldVersion = 0,
    KEClientReportingVersionBetaVersion = 1,
    KEClientReportingVersionSupportsTrustedMode = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECsgoGameEvents {
    GePlayerAnimEventId = 450,
    GeRadioIconEventId = 451,
    GeFireBulletsId = 452,
    GePlayerBulletHitId = 453,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECsgoGcMsg {
    KEMsgGccStrike15V2Base = 9100,
    KEMsgGccStrike15V2MatchmakingStart = 9101,
    KEMsgGccStrike15V2MatchmakingStop = 9102,
    KEMsgGccStrike15V2MatchmakingClient2ServerPing = 9103,
    KEMsgGccStrike15V2MatchmakingGc2ClientUpdate = 9104,
    KEMsgGccStrike15V2MatchmakingServerReservationResponse = 9106,
    KEMsgGccStrike15V2MatchmakingGc2ClientReserve = 9107,
    KEMsgGccStrike15V2MatchmakingClient2GcHello = 9109,
    KEMsgGccStrike15V2MatchmakingGc2ClientHello = 9110,
    KEMsgGccStrike15V2MatchmakingGc2ClientAbandon = 9112,
    KEMsgGccStrike15V2MatchmakingOperator2GcBlogUpdate = 9117,
    KEMsgGccStrike15V2ServerNotificationForUserPenalty = 9118,
    KEMsgGccStrike15V2ClientReportPlayer = 9119,
    KEMsgGccStrike15V2ClientReportServer = 9120,
    KEMsgGccStrike15V2ClientCommendPlayer = 9121,
    KEMsgGccStrike15V2ClientReportResponse = 9122,
    KEMsgGccStrike15V2ClientCommendPlayerQuery = 9123,
    KEMsgGccStrike15V2ClientCommendPlayerQueryResponse = 9124,
    KEMsgGccStrike15V2WatchInfoUsers = 9126,
    KEMsgGccStrike15V2ClientRequestPlayersProfile = 9127,
    KEMsgGccStrike15V2PlayersProfile = 9128,
    KEMsgGccStrike15V2PlayerOverwatchCaseUpdate = 9131,
    KEMsgGccStrike15V2PlayerOverwatchCaseAssignment = 9132,
    KEMsgGccStrike15V2PlayerOverwatchCaseStatus = 9133,
    KEMsgGccStrike15V2Gc2ClientTextMsg = 9134,
    KEMsgGccStrike15V2Client2GcTextMsg = 9135,
    KEMsgGccStrike15V2MatchEndRunRewardDrops = 9136,
    KEMsgGccStrike15V2MatchEndRewardDropsNotification = 9137,
    KEMsgGccStrike15V2ClientRequestWatchInfoFriends2 = 9138,
    KEMsgGccStrike15V2MatchList = 9139,
    KEMsgGccStrike15V2MatchListRequestCurrentLiveGames = 9140,
    KEMsgGccStrike15V2MatchListRequestRecentUserGames = 9141,
    KEMsgGccStrike15V2Gc2ServerReservationUpdate = 9142,
    KEMsgGccStrike15V2ClientVarValueNotificationInfo = 9144,
    KEMsgGccStrike15V2MatchListRequestTournamentGames = 9146,
    KEMsgGccStrike15V2MatchListRequestFullGameInfo = 9147,
    KEMsgGccStrike15V2GiftsLeaderboardRequest = 9148,
    KEMsgGccStrike15V2GiftsLeaderboardResponse = 9149,
    KEMsgGccStrike15V2ServerVarValueNotificationInfo = 9150,
    KEMsgGccStrike15V2ClientSubmitSurveyVote = 9152,
    KEMsgGccStrike15V2Server2GcClientValidate = 9153,
    KEMsgGccStrike15V2MatchListRequestLiveGameForUser = 9154,
    KEMsgGccStrike15V2Client2GcEconPreviewDataBlockRequest = 9156,
    KEMsgGccStrike15V2Client2GcEconPreviewDataBlockResponse = 9157,
    KEMsgGccStrike15V2AccountPrivacySettings = 9158,
    KEMsgGccStrike15V2SetMyActivityInfo = 9159,
    KEMsgGccStrike15V2MatchListRequestTournamentPredictions = 9160,
    KEMsgGccStrike15V2MatchListUploadTournamentPredictions = 9161,
    KEMsgGccStrike15V2DraftSummary = 9162,
    KEMsgGccStrike15V2ClientRequestJoinFriendData = 9163,
    KEMsgGccStrike15V2ClientRequestJoinServerData = 9164,
    KEMsgGccStrike15V2Gc2ClientTournamentInfo = 9167,
    KEMsgGcGlobalGameSubscribe = 9168,
    KEMsgGcGlobalGameUnsubscribe = 9169,
    KEMsgGcGlobalGamePlay = 9170,
    KEMsgGccStrike15V2AcknowledgePenalty = 9171,
    KEMsgGccStrike15V2Client2GcRequestPrestigeCoin = 9172,
    KEMsgGccStrike15V2Gc2ClientGlobalStats = 9173,
    KEMsgGccStrike15V2Client2GcStreamUnlock = 9174,
    KEMsgGccStrike15V2FantasyRequestClientData = 9175,
    KEMsgGccStrike15V2FantasyUpdateClientData = 9176,
    KEMsgGccStrike15V2GcToClientSteamdatagramTicket = 9177,
    KEMsgGccStrike15V2ClientToGcRequestTicket = 9178,
    KEMsgGccStrike15V2ClientToGcRequestElevate = 9179,
    KEMsgGccStrike15V2GlobalChat = 9180,
    KEMsgGccStrike15V2GlobalChatSubscribe = 9181,
    KEMsgGccStrike15V2GlobalChatUnsubscribe = 9182,
    KEMsgGccStrike15V2ClientAuthKeyCode = 9183,
    KEMsgGccStrike15V2GotvSyncPacket = 9184,
    KEMsgGccStrike15V2ClientPlayerDecalSign = 9185,
    KEMsgGccStrike15V2ClientLogonFatalError = 9187,
    KEMsgGccStrike15V2ClientPollState = 9188,
    KEMsgGccStrike15V2PartyRegister = 9189,
    KEMsgGccStrike15V2PartyUnregister = 9190,
    KEMsgGccStrike15V2PartySearch = 9191,
    KEMsgGccStrike15V2PartyInvite = 9192,
    KEMsgGccStrike15V2AccountRequestCoPlays = 9193,
    KEMsgGccStrike15V2ClientGcRankUpdate = 9194,
    KEMsgGccStrike15V2ClientRequestOffers = 9195,
    KEMsgGccStrike15V2ClientAccountBalance = 9196,
    KEMsgGccStrike15V2ClientPartyJoinRelay = 9197,
    KEMsgGccStrike15V2ClientPartyWarning = 9198,
    KEMsgGccStrike15V2SetEventFavorite = 9200,
    KEMsgGccStrike15V2GetEventFavoritesRequest = 9201,
    KEMsgGccStrike15V2ClientPerfReport = 9202,
    KEMsgGccStrike15V2GetEventFavoritesResponse = 9203,
    KEMsgGccStrike15V2ClientRequestSouvenir = 9204,
    KEMsgGccStrike15V2Gc2ClientRefuseSecureMode = 9206,
    KEMsgGccStrike15V2Gc2ClientRequestValidation = 9207,
    KEMsgGccStrike15V2ClientRedeemMissionReward = 9209,
    KEMsgGccStrike15ClientDeepStats = 9210,
    KEMsgGccStrike15StartAgreementSessionInGame = 9211,
    KEMsgGccStrike15V2Gc2ClientInitSystem = 9212,
    KEMsgGccStrike15V2Gc2ClientInitSystemResponse = 9213,
    KEMsgGccStrike15V2PrivateQueues = 9214,
    KEMsgGccStrike15V2MatchListTournamentOperatorMgmt = 9215,
    KEMsgGccStrike15V2BetaEnrollment = 9217,
    KEMsgGccStrike15V2SetPlayerLeaderboardSafeName = 9218,
    KEMsgGccStrike15V2ClientRedeemFreeReward = 9219,
    KEMsgGccStrike15V2ClientNetworkConfig = 9220,
    KEMsgGccStrike15V2Gc2ClientNotifyXpShop = 9221,
    KEMsgGccStrike15V2Client2GcAckXpShopTracks = 9222,
    KEMsgGccStrike15V2MatchmakingGc2ClientSearchStats = 9223,
    KEMsgGccStrike15V2PremierSeasonSummary = 9224,
    KEMsgGccStrike15V2RequestRecurringMissionSchedule = 9225,
    KEMsgGccStrike15V2RecurringMissionSchema = 9226,
    KEMsgGccStrike15V2VolatileItemClaimReward = 9227,
    KEMsgGccStrike15V2VolatileShopSubscribe = 9228,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECsgoSteamUserStat {
    KECsgoSteamUserStatXpEarnedGames = 1,
    KECsgoSteamUserStatMatchWinsCompetitive = 2,
    KECsgoSteamUserStatSurvivedDangerZone = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECstrike15UserMessages {
    CsUmVguiMenu = 301,
    CsUmGeiger = 302,
    CsUmTrain = 303,
    CsUmHudText = 304,
    CsUmSayText = 305,
    CsUmSayText2 = 306,
    CsUmTextMsg = 307,
    CsUmHudMsg = 308,
    CsUmResetHud = 309,
    CsUmGameTitle = 310,
    CsUmShake = 312,
    CsUmFade = 313,
    CsUmRumble = 314,
    CsUmCloseCaption = 315,
    CsUmCloseCaptionDirect = 316,
    CsUmSendAudio = 317,
    CsUmRawAudio = 318,
    CsUmVoiceMask = 319,
    CsUmRequestState = 320,
    CsUmDamage = 321,
    CsUmRadioText = 322,
    CsUmHintText = 323,
    CsUmKeyHintText = 324,
    CsUmProcessSpottedEntityUpdate = 325,
    CsUmReloadEffect = 326,
    CsUmAdjustMoney = 327,
    CsUmUpdateTeamMoney = 328,
    CsUmStopSpectatorMode = 329,
    CsUmKillCam = 330,
    CsUmDesiredTimescale = 331,
    CsUmCurrentTimescale = 332,
    CsUmAchievementEvent = 333,
    CsUmMatchEndConditions = 334,
    CsUmDisconnectToLobby = 335,
    CsUmPlayerStatsUpdate = 336,
    CsUmClientInfo = 339,
    CsUmXRankGet = 340,
    CsUmXRankUpd = 341,
    CsUmCallVoteFailed = 345,
    CsUmVoteStart = 346,
    CsUmVotePass = 347,
    CsUmVoteFailed = 348,
    CsUmVoteSetup = 349,
    CsUmServerRankRevealAll = 350,
    CsUmSendLastKillerDamageToClient = 351,
    CsUmServerRankUpdate = 352,
    CsUmItemPickup = 353,
    CsUmShowMenu = 354,
    CsUmBarTime = 355,
    CsUmAmmoDenied = 356,
    CsUmMarkAchievement = 357,
    CsUmMatchStatsUpdate = 358,
    CsUmItemDrop = 359,
    CsUmSendPlayerItemDrops = 361,
    CsUmRoundBackupFilenames = 362,
    CsUmSendPlayerItemFound = 363,
    CsUmReportHit = 364,
    CsUmXpUpdate = 365,
    CsUmQuestProgress = 366,
    CsUmScoreLeaderboardData = 367,
    CsUmPlayerDecalDigitalSignature = 368,
    CsUmWeaponSound = 369,
    CsUmUpdateScreenHealthBar = 370,
    CsUmEntityOutlineHighlight = 371,
    CsUmSsui = 372,
    CsUmSurvivalStats = 373,
    CsUmDisconnectToLobby2 = 374,
    CsUmEndOfMatchAllPlayersData = 375,
    CsUmPostRoundDamageReport = 376,
    CsUmRoundEndReportData = 379,
    CsUmCurrentRoundOdds = 380,
    CsUmDeepStats = 381,
    CsUmShootInfo = 383,
    CsUmCounterStrafe = 385,
    CsUmDamagePrediction = 386,
    CsUmRecurringMissionSchema = 387,
    CsUmSendPlayerLoadout = 388,
    CsUmWeaponMagDrop = 389,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EInitSystemResult {
    KEInitSystemResultInvalid = 0,
    KEInitSystemResultSuccess = 1,
    KEInitSystemResultNone = 2,
    KEInitSystemResultNotFound = 3,
    KEInitSystemResultExisting = 4,
    KEInitSystemResultFailedOpen = 5,
    KEInitSystemResultMismatch = 6,
    KEInitSystemResultFailedInit = 7,
    KEInitSystemResultMax = 8,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EcsUsrMsgDisconnectToLobbyAction {
    KEcsUsrMsgDisconnectToLobbyActionDefault = 0,
    KEcsUsrMsgDisconnectToLobbyActionGoQueue = 1,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GameServerPing {
    #[prost(int32, optional, tag = "2")]
    pub ping: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub instances: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GlobalStatistics {
    #[prost(uint32, optional, tag = "1")]
    pub players_online: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub servers_online: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub players_searching: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub servers_available: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub ongoing_matches: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub search_time_avg: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "7")]
    pub search_statistics: ::prost::alloc::vec::Vec<DetailedSearchStatistic>,
    #[prost(string, optional, tag = "8")]
    pub main_post_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "9")]
    pub required_appid_version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub pricesheet_version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub twitch_streams_version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub active_tournament_eventid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "13")]
    pub active_survey_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "14")]
    pub rtime32_cur: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "16")]
    pub required_appid_version2: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct IpAddressMask {
    #[prost(uint32, optional, tag = "1")]
    pub a: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub b: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub c: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub d: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub bits: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub token: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct MatchEndItemUpdates {
    #[prost(uint64, optional, tag = "1")]
    pub item_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub item_attr_defidx: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub item_attr_delta_value: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct OperationalStatisticDescription {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "2")]
    pub idkey: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct OperationalStatisticElement {
    #[prost(uint32, optional, tag = "1")]
    pub idkey: ::core::option::Option<u32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub values: ::prost::alloc::vec::Vec<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OperationalStatisticsPacket {
    #[prost(int32, optional, tag = "1")]
    pub packetid: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub mstimestamp: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<OperationalStatisticElement>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct OperationalVarValue {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub ivalue: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "3")]
    pub fvalue: ::core::option::Option<f32>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub svalue: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct PlayerCommendationInfo {
    #[prost(uint32, optional, tag = "1")]
    pub cmd_friendly: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub cmd_teaching: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub cmd_leader: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PlayerDecalDigitalSignature {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub signature: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "2")]
    pub accountid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub rtime: ::core::option::Option<u32>,
    #[prost(float, repeated, packed = "false", tag = "4")]
    pub endpos: ::prost::alloc::vec::Vec<f32>,
    #[prost(float, repeated, packed = "false", tag = "5")]
    pub startpos: ::prost::alloc::vec::Vec<f32>,
    #[prost(float, repeated, packed = "false", tag = "6")]
    pub left: ::prost::alloc::vec::Vec<f32>,
    #[prost(uint32, optional, tag = "7")]
    pub tx_defidx: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "8")]
    pub entindex: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "9")]
    pub hitbox: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "10")]
    pub creationtime: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "11")]
    pub equipslot: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub trace_id: ::core::option::Option<u32>,
    #[prost(float, repeated, packed = "false", tag = "13")]
    pub normal: ::prost::alloc::vec::Vec<f32>,
    #[prost(uint32, optional, tag = "14")]
    pub tint_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct PlayerMedalsInfo {
    #[prost(uint32, repeated, packed = "false", tag = "7")]
    pub display_items_defidx: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub featured_display_item_defidx: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PlayerQuestData {
    #[prost(uint32, optional, tag = "1")]
    pub quester_account_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub quest_item_data: ::prost::alloc::vec::Vec<player_quest_data::QuestItemData>,
    #[prost(message, repeated, tag = "3")]
    pub xp_progress_data: ::prost::alloc::vec::Vec<XpProgressData>,
    #[prost(uint32, optional, tag = "4")]
    pub time_played: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub mm_game_mode: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "6")]
    pub item_updates: ::prost::alloc::vec::Vec<MatchEndItemUpdates>,
    #[prost(bool, optional, tag = "7")]
    pub operation_points_eligible: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "8")]
    pub userstatchanges: ::prost::alloc::vec::Vec<CMsgCsgoSteamUserStatChange>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PlayerRankingInfo {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub rank_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub wins: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "4")]
    pub rank_change: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "6")]
    pub rank_type_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub tv_control: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "8")]
    pub rank_window_stats: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "9")]
    pub leaderboard_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "10")]
    pub rank_if_win: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub rank_if_lose: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub rank_if_tie: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "13")]
    pub per_map_rank: ::prost::alloc::vec::Vec<player_ranking_info::PerMapRank>,
    #[prost(uint32, optional, tag = "14")]
    pub leaderboard_name_status: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "15")]
    pub highest_rank: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "16")]
    pub rank_expiry: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QuestType {
    KEQuestTypeOperation = 1,
    KEQuestTypeRecurringMission = 2,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ScoreLeaderboardData {
    #[prost(uint64, optional, tag = "1")]
    pub quest_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub score: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "3")]
    pub accountentries: ::prost::alloc::vec::Vec<score_leaderboard_data::AccountEntries>,
    #[prost(message, repeated, tag = "5")]
    pub matchentries: ::prost::alloc::vec::Vec<score_leaderboard_data::Entry>,
    #[prost(string, optional, tag = "6")]
    pub leaderboard_name: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ServerHltvInfo {
    #[prost(uint32, optional, tag = "1")]
    pub tv_udp_port: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub tv_watch_key: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub tv_slots: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub tv_clients: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub tv_proxies: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub tv_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub game_type: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "9")]
    pub game_mapgroup: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub game_map: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "11")]
    pub tv_master_steamid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "12")]
    pub tv_local_slots: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "13")]
    pub tv_local_clients: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "14")]
    pub tv_local_proxies: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "15")]
    pub tv_relay_slots: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "16")]
    pub tv_relay_clients: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "17")]
    pub tv_relay_proxies: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "18")]
    pub tv_relay_address: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "19")]
    pub tv_relay_port: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "20")]
    pub tv_relay_steamid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "21")]
    pub flags: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TournamentEvent {
    #[prost(int32, optional, tag = "1")]
    pub event_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub event_tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub event_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub event_time_start: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub event_time_end: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "6")]
    pub event_public: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub event_stage_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "8")]
    pub event_stage_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "9")]
    pub active_section_id: ::core::option::Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TournamentMatchSetup {
    #[prost(int32, optional, tag = "1")]
    pub event_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub team_id_ct: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub team_id_t: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub event_stage_id: ::core::option::Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TournamentPlayer {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub player_nick: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub player_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub player_dob: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "5")]
    pub player_flag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub player_location: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub player_desc: ::core::option::Option<::prost::alloc::string::String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TournamentTeam {
    #[prost(int32, optional, tag = "1")]
    pub team_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub team_tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub team_flag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub team_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub players: ::prost::alloc::vec::Vec<TournamentPlayer>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct WatchableMatchInfo {
    #[prost(uint32, optional, tag = "1")]
    pub server_ip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub tv_port: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub tv_spectators: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub tv_time: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub tv_watch_password: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag = "6")]
    pub cl_decryptdata_key: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "7")]
    pub cl_decryptdata_key_pub: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "8")]
    pub game_type: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "9")]
    pub game_mapgroup: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub game_map: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "11")]
    pub server_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "12")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "13")]
    pub reservation_id: ::core::option::Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct XpProgressData {
    #[prost(uint32, optional, tag = "1")]
    pub xp_points: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub xp_category: ::core::option::Option<i32>,
}

pub mod c_data_gcc_strike15_v2_tournament_group {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Picks {
        #[prost(int32, repeated, packed = "false", tag = "1")]
        pub pickids: ::prost::alloc::vec::Vec<i32>,
    }
}

pub mod c_data_gcc_strike15_v2_tournament_match_draft {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Entry {
        #[prost(int32, optional, tag = "1")]
        pub mapid: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub team_id_ct: ::core::option::Option<i32>,
    }
}

pub mod c_econ_item_preview_data_block {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Sticker {
        #[prost(uint32, optional, tag = "1")]
        pub slot: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub sticker_id: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "3")]
        pub wear: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "4")]
        pub scale: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "5")]
        pub rotation: ::core::option::Option<f32>,
        #[prost(uint32, optional, tag = "6")]
        pub tint_id: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "7")]
        pub offset_x: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "8")]
        pub offset_y: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "9")]
        pub offset_z: ::core::option::Option<f32>,
        #[prost(uint32, optional, tag = "10")]
        pub pattern: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "11")]
        pub highlight_reel: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "12")]
        pub wrapped_sticker: ::core::option::Option<u32>,
    }
}

pub mod c_msg_gcc_strike15_client_deep_stats {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct DeepStatsRange {
        #[prost(uint32, optional, tag = "1")]
        pub begin: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub end: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "3")]
        pub frozen: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct DeepStatsMatch {
        #[prost(message, optional, tag = "1")]
        pub player: ::core::option::Option<super::DeepPlayerStatsEntry>,
        #[prost(message, repeated, tag = "2")]
        pub events: ::prost::alloc::vec::Vec<super::DeepPlayerMatchEvent>,
    }
}

pub mod c_msg_gcc_strike15_v2_account_privacy_settings {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Setting {
        #[prost(uint32, optional, tag = "1")]
        pub setting_type: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub setting_value: ::core::option::Option<u32>,
    }
}

pub mod c_msg_gcc_strike15_v2_account_request_co_plays {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Player {
        #[prost(uint32, optional, tag = "1")]
        pub accountid: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub rtcoplay: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "3")]
        pub online: ::core::option::Option<bool>,
    }
}

pub mod c_msg_gcc_strike15_v2_client_party_warning {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Entry {
        #[prost(uint32, optional, tag = "1")]
        pub accountid: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub warntype: ::core::option::Option<u32>,
    }
}

pub mod c_msg_gcc_strike15_v2_client_perf_report {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Entry {
        #[prost(uint32, optional, tag = "1")]
        pub perfcounter: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub length: ::core::option::Option<u32>,
        #[prost(bytes = "vec", optional, tag = "3")]
        pub reference: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes = "vec", optional, tag = "4")]
        pub actual: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(uint32, optional, tag = "5")]
        pub sourceid: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub status: ::core::option::Option<u32>,
    }
}

pub mod c_msg_gcc_strike15_v2_fantasy {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct FantasySlot {
        #[prost(int32, optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub pick: ::core::option::Option<i32>,
        #[prost(uint64, optional, tag = "3")]
        pub itemid: ::core::option::Option<u64>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct FantasyTeam {
        #[prost(int32, optional, tag = "1")]
        pub sectionid: ::core::option::Option<i32>,
        #[prost(message, repeated, tag = "2")]
        pub slots: ::prost::alloc::vec::Vec<FantasySlot>,
    }
}

pub mod c_msg_gcc_strike15_v2_gifts_leaderboard_response {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct GiftLeaderboardEntry {
        #[prost(uint32, optional, tag = "1")]
        pub accountid: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub gifts: ::core::option::Option<u32>,
    }
}

pub mod c_msg_gcc_strike15_v2_matchmaking_server_round_stats {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct DropInfo {
        #[prost(uint32, optional, tag = "1")]
        pub account_mvp: ::core::option::Option<u32>,
    }
}

pub mod c_msg_gcc_strike15_v2_party_search_results {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Entry {
        #[prost(uint32, optional, tag = "1")]
        pub id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub grp: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub game_type: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub apr: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "5")]
        pub ark: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub loc: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "7")]
        pub accountid: ::core::option::Option<u32>,
    }
}

pub mod c_msg_gcc_strike15_v2_predictions {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct GroupMatchTeamPick {
        #[prost(int32, optional, tag = "1")]
        pub sectionid: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub groupid: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub index: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub teamid: ::core::option::Option<i32>,
        #[prost(uint64, optional, tag = "5")]
        pub itemid: ::core::option::Option<u64>,
    }
}

pub mod c_msg_gcc_strike15_v2_premier_season_summary {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct DataPerWeek {
        #[prost(uint64, optional, tag = "1")]
        pub week_id: ::core::option::Option<u64>,
        #[prost(uint32, optional, tag = "2")]
        pub rank_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub matches_played: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct DataPerMap {
        #[prost(uint32, optional, tag = "1")]
        pub map_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub wins: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub ties: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub losses: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "5")]
        pub rounds: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "6")]
        pub kills: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "7")]
        pub headshots: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "8")]
        pub assists: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "9")]
        pub deaths: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "10")]
        pub mvps: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "11")]
        pub rounds_3k: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "12")]
        pub rounds_4k: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "13")]
        pub rounds_5k: ::core::option::Option<u32>,
    }
}

pub mod c_msg_legacy_source1_client_welcome {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct Location {
        #[prost(float, optional, tag = "1")]
        pub latitude: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "2")]
        pub longitude: ::core::option::Option<f32>,
        #[prost(string, optional, tag = "3")]
        pub country: ::core::option::Option<::prost::alloc::string::String>,
    }
}

pub mod c_msg_recurring_mission_schema {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct MissionTemplateList {
        #[prost(uint32, optional, tag = "1")]
        pub period: ::core::option::Option<u32>,
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub mission_templates: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
}

pub mod c_msg_te_fire_bullets {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Extra {
        #[prost(message, optional, tag = "1")]
        pub aim_punch: ::core::option::Option<super::CMsgQAngle>,
        #[prost(int32, optional, tag = "2")]
        pub attack_tick_count: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "3")]
        pub attack_tick_frac: ::core::option::Option<f32>,
        #[prost(int32, optional, tag = "4")]
        pub render_tick_count: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "5")]
        pub render_tick_frac: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "6")]
        pub inaccuracy_move: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "7")]
        pub inaccuracy_air: ::core::option::Option<f32>,
        #[prost(int32, optional, tag = "8")]
        pub r#type: ::core::option::Option<i32>,
    }
}

pub mod c_pre_match_info_data {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct TeamStats {
        #[prost(int32, optional, tag = "1")]
        pub match_info_idxtxt: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub match_info_txt: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "3")]
        pub match_info_teams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}

pub mod ccs_usr_msg_end_of_match_all_players_data {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Accolade {
        #[prost(int32, optional, tag = "1")]
        pub eaccolade: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "2")]
        pub value: ::core::option::Option<f32>,
        #[prost(int32, optional, tag = "3")]
        pub position: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct PlayerData {
        #[prost(int32, optional, tag = "1", default = "-1")]
        pub slot: ::core::option::Option<i32>,
        #[prost(uint64, optional, tag = "2")]
        pub xuid: ::core::option::Option<u64>,
        #[prost(string, optional, tag = "3")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "4")]
        pub teamnumber: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "5")]
        pub nomination: ::core::option::Option<Accolade>,
        #[prost(message, repeated, tag = "6")]
        pub items: ::prost::alloc::vec::Vec<super::CEconItemPreviewDataBlock>,
        #[prost(int32, optional, tag = "7")]
        pub playercolor: ::core::option::Option<i32>,
        #[prost(bool, optional, tag = "8")]
        pub isbot: ::core::option::Option<bool>,
    }
}

pub mod ccs_usr_msg_player_stats_update {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Stat {
        #[prost(int32, optional, tag = "1")]
        pub idx: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub delta: ::core::option::Option<i32>,
    }
}

pub mod ccs_usr_msg_process_spotted_entity_update {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SpottedEntityUpdate {
        #[prost(int32, optional, tag = "1", default = "-1")]
        pub entity_idx: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub class_id: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub origin_x: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub origin_y: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "5")]
        pub origin_z: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "6")]
        pub angle_y: ::core::option::Option<i32>,
        #[prost(bool, optional, tag = "7")]
        pub defuser: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "8")]
        pub player_has_defuser: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "9")]
        pub player_has_c4: ::core::option::Option<bool>,
    }
}

pub mod ccs_usr_msg_round_end_report_data {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct RerEvent {
        #[prost(float, optional, tag = "1")]
        pub timestamp: ::core::option::Option<f32>,
        #[prost(int32, optional, tag = "2")]
        pub terrorist_odds: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub ct_alive: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub t_alive: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "5")]
        pub victim_data: ::core::option::Option<rer_event::Victim>,
        #[prost(message, optional, tag = "6")]
        pub objective_data: ::core::option::Option<rer_event::Objective>,
        #[prost(message, repeated, tag = "7")]
        pub all_damage_data: ::prost::alloc::vec::Vec<rer_event::Damage>,
    }
    pub mod rer_event {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct Victim {
            #[prost(int32, optional, tag = "1")]
            pub team_number: ::core::option::Option<i32>,
            #[prost(int32, optional, tag = "2", default = "-1")]
            pub playerslot: ::core::option::Option<i32>,
            #[prost(uint64, optional, tag = "3")]
            pub xuid: ::core::option::Option<u64>,
            #[prost(int32, optional, tag = "4")]
            pub color: ::core::option::Option<i32>,
            #[prost(bool, optional, tag = "5")]
            pub is_bot: ::core::option::Option<bool>,
            #[prost(bool, optional, tag = "6")]
            pub is_dead: ::core::option::Option<bool>,
        }
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct Objective {
            #[prost(int32, optional, tag = "1")]
            pub r#type: ::core::option::Option<i32>,
        }
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct Damage {
            #[prost(int32, optional, tag = "1", default = "-1")]
            pub other_playerslot: ::core::option::Option<i32>,
            #[prost(uint64, optional, tag = "2")]
            pub other_xuid: ::core::option::Option<u64>,
            #[prost(int32, optional, tag = "3")]
            pub health_removed: ::core::option::Option<i32>,
            #[prost(int32, optional, tag = "4")]
            pub num_hits: ::core::option::Option<i32>,
            #[prost(int32, optional, tag = "5")]
            pub return_health_removed: ::core::option::Option<i32>,
            #[prost(int32, optional, tag = "6")]
            pub return_num_hits: ::core::option::Option<i32>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct InitialConditions {
        #[prost(int32, optional, tag = "1")]
        pub ct_equip_value: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub t_equip_value: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub terrorist_odds: ::core::option::Option<i32>,
    }
}

pub mod ccs_usr_msg_send_player_loadout {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct LoadoutItem {
        #[prost(message, optional, tag = "1")]
        pub econ_item: ::core::option::Option<super::CEconItemPreviewDataBlock>,
        #[prost(int32, optional, tag = "2")]
        pub team: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub slot: ::core::option::Option<i32>,
    }
}

pub mod ccs_usr_msg_server_rank_update {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RankUpdate {
        #[prost(int32, optional, tag = "1")]
        pub account_id: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub rank_old: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub rank_new: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub num_wins: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "5")]
        pub rank_change: ::core::option::Option<f32>,
        #[prost(int32, optional, tag = "6")]
        pub rank_type_id: ::core::option::Option<i32>,
    }
}

pub mod ccs_usr_msg_survival_stats {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Fact {
        #[prost(int32, optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub display: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub value: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "4")]
        pub interestingness: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Placement {
        #[prost(uint64, optional, tag = "1")]
        pub xuid: ::core::option::Option<u64>,
        #[prost(int32, optional, tag = "2")]
        pub teamnumber: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub placement: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Damage {
        #[prost(uint64, optional, tag = "1")]
        pub xuid: ::core::option::Option<u64>,
        #[prost(int32, optional, tag = "2")]
        pub to: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub to_hits: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub from: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "5")]
        pub from_hits: ::core::option::Option<i32>,
    }
}

pub mod ccs_usr_msg_vgui_menu {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Keys {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
    }
}

pub mod ccs_usr_msg_voice_mask {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct PlayerMask {
        #[prost(int32, optional, tag = "1")]
        pub game_rules_mask: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub ban_masks: ::core::option::Option<i32>,
    }
}

pub mod player_quest_data {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct QuestItemData {
        #[prost(uint64, optional, tag = "1")]
        pub quest_id: ::core::option::Option<u64>,
        #[prost(int32, optional, tag = "2")]
        pub quest_normal_points_earned: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub quest_bonus_points_earned: ::core::option::Option<i32>,
        #[prost(int32, repeated, packed = "false", tag = "4")]
        pub quest_normal_points_required: ::prost::alloc::vec::Vec<i32>,
        #[prost(int32, repeated, packed = "false", tag = "5")]
        pub quest_reward_xp: ::prost::alloc::vec::Vec<i32>,
        #[prost(int32, optional, tag = "6")]
        pub quest_period: ::core::option::Option<i32>,
        #[prost(enumeration = "super::QuestType", optional, tag = "7", default = "KEQuestTypeOperation")]
        pub quest_type: ::core::option::Option<i32>,
    }
}

pub mod player_ranking_info {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct PerMapRank {
        #[prost(uint32, optional, tag = "1")]
        pub map_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub rank_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub wins: ::core::option::Option<u32>,
    }
}

pub mod score_leaderboard_data {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Entry {
        #[prost(uint32, optional, tag = "1")]
        pub tag: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub val: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct AccountEntries {
        #[prost(uint32, optional, tag = "1")]
        pub accountid: ::core::option::Option<u32>,
        #[prost(message, repeated, tag = "2")]
        pub entries: ::prost::alloc::vec::Vec<Entry>,
    }
}
