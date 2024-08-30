#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoFileHeader {
    #[prost(string, required, tag = "1")]
    pub demo_file_stamp: ::prost::alloc::string::String,
    #[prost(int32, optional, tag = "2")]
    pub network_protocol: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub server_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub client_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub map_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub game_directory: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "7")]
    pub fullpackets_version: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "8")]
    pub allow_clientside_entities: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "9")]
    pub allow_clientside_particles: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "10")]
    pub addons: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub demo_version_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub demo_version_guid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "13")]
    pub build_num: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "14")]
    pub game: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "15")]
    pub server_start_tick: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CGameInfo {
    #[prost(message, optional, tag = "4")]
    pub dota: ::core::option::Option<c_game_info::CDotaGameInfo>,
    #[prost(message, optional, tag = "5")]
    pub cs: ::core::option::Option<c_game_info::CcsGameInfo>,
}
pub mod c_game_info {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CDotaGameInfo {
        #[prost(uint64, optional, tag = "1")]
        pub match_id: ::core::option::Option<u64>,
        #[prost(int32, optional, tag = "2")]
        pub game_mode: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub game_winner: ::core::option::Option<i32>,
        #[prost(message, repeated, tag = "4")]
        pub player_info: ::prost::alloc::vec::Vec<c_dota_game_info::CPlayerInfo>,
        #[prost(uint32, optional, tag = "5")]
        pub leagueid: ::core::option::Option<u32>,
        #[prost(message, repeated, tag = "6")]
        pub picks_bans: ::prost::alloc::vec::Vec<c_dota_game_info::CHeroSelectEvent>,
        #[prost(uint32, optional, tag = "7")]
        pub radiant_team_id: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "8")]
        pub dire_team_id: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "9")]
        pub radiant_team_tag: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "10")]
        pub dire_team_tag: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "11")]
        pub end_time: ::core::option::Option<u32>,
    }
    pub mod c_dota_game_info {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CPlayerInfo {
            #[prost(string, optional, tag = "1")]
            pub hero_name: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(string, optional, tag = "2")]
            pub player_name: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(bool, optional, tag = "3")]
            pub is_fake_client: ::core::option::Option<bool>,
            #[prost(uint64, optional, tag = "4")]
            pub steamid: ::core::option::Option<u64>,
            #[prost(int32, optional, tag = "5")]
            pub game_team: ::core::option::Option<i32>,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct CHeroSelectEvent {
            #[prost(bool, optional, tag = "1")]
            pub is_pick: ::core::option::Option<bool>,
            #[prost(uint32, optional, tag = "2")]
            pub team: ::core::option::Option<u32>,
            #[prost(int32, optional, tag = "3")]
            pub hero_id: ::core::option::Option<i32>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CcsGameInfo {
        #[prost(int32, repeated, packed = "false", tag = "1")]
        pub round_start_ticks: ::prost::alloc::vec::Vec<i32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoFileInfo {
    #[prost(float, optional, tag = "1")]
    pub playback_time: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "2")]
    pub playback_ticks: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub playback_frames: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub game_info: ::core::option::Option<CGameInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoPacket {
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoFullPacket {
    #[prost(message, optional, tag = "1")]
    pub string_table: ::core::option::Option<CDemoStringTables>,
    #[prost(message, optional, tag = "2")]
    pub packet: ::core::option::Option<CDemoPacket>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoSaveGame {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag = "3")]
    pub signature: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "4")]
    pub version: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDemoSyncTick {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoConsoleCmd {
    #[prost(string, optional, tag = "1")]
    pub cmdstring: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoSendTables {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoClassInfo {
    #[prost(message, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<c_demo_class_info::ClassT>,
}
pub mod c_demo_class_info {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClassT {
        #[prost(int32, optional, tag = "1")]
        pub class_id: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub network_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "3")]
        pub table_name: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoCustomData {
    #[prost(int32, optional, tag = "1")]
    pub callback_index: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoCustomDataCallbacks {
    #[prost(string, repeated, tag = "1")]
    pub save_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoAnimationHeader {
    #[prost(sint32, optional, tag = "1")]
    pub entity_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub tick: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoAnimationData {
    #[prost(sint32, optional, tag = "1")]
    pub entity_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub start_tick: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub end_tick: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, optional, tag = "5")]
    pub data_checksum: ::core::option::Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoStringTables {
    #[prost(message, repeated, tag = "1")]
    pub tables: ::prost::alloc::vec::Vec<c_demo_string_tables::TableT>,
}
pub mod c_demo_string_tables {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ItemsT {
        #[prost(string, optional, tag = "1")]
        pub str: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableT {
        #[prost(string, optional, tag = "1")]
        pub table_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub items: ::prost::alloc::vec::Vec<ItemsT>,
        #[prost(message, repeated, tag = "3")]
        pub items_clientside: ::prost::alloc::vec::Vec<ItemsT>,
        #[prost(int32, optional, tag = "4")]
        pub table_flags: ::core::option::Option<i32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CDemoStop {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoUserCmd {
    #[prost(int32, optional, tag = "1")]
    pub cmd_number: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoSpawnGroups {
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub msgs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EDemoCommands {
    DemError = -1,
    DemStop = 0,
    DemFileHeader = 1,
    DemFileInfo = 2,
    DemSyncTick = 3,
    DemSendTables = 4,
    DemClassInfo = 5,
    DemStringTables = 6,
    DemPacket = 7,
    DemSignonPacket = 8,
    DemConsoleCmd = 9,
    DemCustomData = 10,
    DemCustomDataCallbacks = 11,
    DemUserCmd = 12,
    DemFullPacket = 13,
    DemSaveGame = 14,
    DemSpawnGroups = 15,
    DemAnimationData = 16,
    DemAnimationHeader = 17,
    DemMax = 18,
    DemIsCompressed = 64,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ENetworkDisconnectionReason {
    NetworkDisconnectInvalid = 0,
    NetworkDisconnectShutdown = 1,
    NetworkDisconnectDisconnectByUser = 2,
    NetworkDisconnectDisconnectByServer = 3,
    NetworkDisconnectLost = 4,
    NetworkDisconnectOverflow = 5,
    NetworkDisconnectSteamBanned = 6,
    NetworkDisconnectSteamInuse = 7,
    NetworkDisconnectSteamTicket = 8,
    NetworkDisconnectSteamLogon = 9,
    NetworkDisconnectSteamAuthcancelled = 10,
    NetworkDisconnectSteamAuthalreadyused = 11,
    NetworkDisconnectSteamAuthinvalid = 12,
    NetworkDisconnectSteamVacbanstate = 13,
    NetworkDisconnectSteamLoggedInElsewhere = 14,
    NetworkDisconnectSteamVacCheckTimedout = 15,
    NetworkDisconnectSteamDropped = 16,
    NetworkDisconnectSteamOwnership = 17,
    NetworkDisconnectServerinfoOverflow = 18,
    NetworkDisconnectTickmsgOverflow = 19,
    NetworkDisconnectStringtablemsgOverflow = 20,
    NetworkDisconnectDeltaentmsgOverflow = 21,
    NetworkDisconnectTempentmsgOverflow = 22,
    NetworkDisconnectSoundsmsgOverflow = 23,
    NetworkDisconnectSnapshotoverflow = 24,
    NetworkDisconnectSnapshoterror = 25,
    NetworkDisconnectReliableoverflow = 26,
    NetworkDisconnectBaddeltatick = 27,
    NetworkDisconnectNomoresplits = 28,
    NetworkDisconnectTimedout = 29,
    NetworkDisconnectDisconnected = 30,
    NetworkDisconnectLeavingsplit = 31,
    NetworkDisconnectDifferentclasstables = 32,
    NetworkDisconnectBadrelaypassword = 33,
    NetworkDisconnectBadspectatorpassword = 34,
    NetworkDisconnectHltvrestricted = 35,
    NetworkDisconnectNospectators = 36,
    NetworkDisconnectHltvunavailable = 37,
    NetworkDisconnectHltvstop = 38,
    NetworkDisconnectKicked = 39,
    NetworkDisconnectBanadded = 40,
    NetworkDisconnectKickbanadded = 41,
    NetworkDisconnectHltvdirect = 42,
    NetworkDisconnectPureserverClientextra = 43,
    NetworkDisconnectPureserverMismatch = 44,
    NetworkDisconnectUsercmd = 45,
    NetworkDisconnectRejectedByGame = 46,
    NetworkDisconnectMessageParseError = 47,
    NetworkDisconnectInvalidMessageError = 48,
    NetworkDisconnectBadServerPassword = 49,
    NetworkDisconnectDirectConnectReservation = 50,
    NetworkDisconnectConnectionFailure = 51,
    NetworkDisconnectNoPeerGroupHandlers = 52,
    NetworkDisconnectReconnection = 53,
    NetworkDisconnectLoopshutdown = 54,
    NetworkDisconnectLoopdeactivate = 55,
    NetworkDisconnectHostEndgame = 56,
    NetworkDisconnectLoopLevelloadActivate = 57,
    NetworkDisconnectCreateServerFailed = 58,
    NetworkDisconnectExiting = 59,
    NetworkDisconnectRequestHoststateIdle = 60,
    NetworkDisconnectRequestHoststateHltvrelay = 61,
    NetworkDisconnectClientConsistencyFail = 62,
    NetworkDisconnectClientUnableToCrcMap = 63,
    NetworkDisconnectClientNoMap = 64,
    NetworkDisconnectClientDifferentMap = 65,
    NetworkDisconnectServerRequiresSteam = 66,
    NetworkDisconnectSteamDenyMisc = 67,
    NetworkDisconnectSteamDenyBadAntiCheat = 68,
    NetworkDisconnectServerShutdown = 69,
    NetworkDisconnectReplayIncompatible = 71,
    NetworkDisconnectConnectRequestTimedout = 72,
    NetworkDisconnectServerIncompatible = 73,
    NetworkDisconnectLocalproblemManyrelays = 74,
    NetworkDisconnectLocalproblemHostedserverprimaryrelay = 75,
    NetworkDisconnectLocalproblemNetworkconfig = 76,
    NetworkDisconnectLocalproblemOther = 77,
    NetworkDisconnectRemoteTimeout = 79,
    NetworkDisconnectRemoteTimeoutConnecting = 80,
    NetworkDisconnectRemoteOther = 81,
    NetworkDisconnectRemoteBadcrypt = 82,
    NetworkDisconnectRemoteCertnottrusted = 83,
    NetworkDisconnectUnusual = 84,
    NetworkDisconnectInternalError = 85,
    NetworkDisconnectRejectBadchallenge = 128,
    NetworkDisconnectRejectNolobby = 129,
    NetworkDisconnectRejectBackgroundMap = 130,
    NetworkDisconnectRejectSinglePlayer = 131,
    NetworkDisconnectRejectHiddenGame = 132,
    NetworkDisconnectRejectLanrestrict = 133,
    NetworkDisconnectRejectBadpassword = 134,
    NetworkDisconnectRejectServerfull = 135,
    NetworkDisconnectRejectInvalidreservation = 136,
    NetworkDisconnectRejectFailedchannel = 137,
    NetworkDisconnectRejectConnectFromLobby = 138,
    NetworkDisconnectRejectReservedForLobby = 139,
    NetworkDisconnectRejectInvalidkeylength = 140,
    NetworkDisconnectRejectOldprotocol = 141,
    NetworkDisconnectRejectNewprotocol = 142,
    NetworkDisconnectRejectInvalidconnection = 143,
    NetworkDisconnectRejectInvalidcertlen = 144,
    NetworkDisconnectRejectInvalidsteamcertlen = 145,
    NetworkDisconnectRejectSteam = 146,
    NetworkDisconnectRejectServerauthdisabled = 147,
    NetworkDisconnectRejectServercdkeyauthinvalid = 148,
    NetworkDisconnectRejectBanned = 149,
    NetworkDisconnectKickedTeamkilling = 150,
    NetworkDisconnectKickedTkStart = 151,
    NetworkDisconnectKickedUntrustedaccount = 152,
    NetworkDisconnectKickedConvictedaccount = 153,
    NetworkDisconnectKickedCompetitivecooldown = 154,
    NetworkDisconnectKickedTeamhurting = 155,
    NetworkDisconnectKickedHostagekilling = 156,
    NetworkDisconnectKickedVotedoff = 157,
    NetworkDisconnectKickedIdle = 158,
    NetworkDisconnectKickedSuicide = 159,
    NetworkDisconnectKickedNosteamlogin = 160,
    NetworkDisconnectKickedNosteamticket = 161,
    NetworkDisconnectKickedInputautomation = 162,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgVector {
    #[prost(float, optional, tag = "1")]
    pub x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub z: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub w: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgVector2D {
    #[prost(float, optional, tag = "1")]
    pub x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgQAngle {
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
pub struct CMsgQuaternion {
    #[prost(float, optional, tag = "1")]
    pub x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub z: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub w: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgTransform {
    #[prost(message, optional, tag = "1")]
    pub position: ::core::option::Option<CMsgVector>,
    #[prost(float, optional, tag = "2")]
    pub scale: ::core::option::Option<f32>,
    #[prost(message, optional, tag = "3")]
    pub orientation: ::core::option::Option<CMsgQuaternion>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgRgba {
    #[prost(int32, optional, tag = "1")]
    pub r: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub g: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub b: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub a: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgPlayerInfo {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(fixed64, optional, tag = "2")]
    pub xuid: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "3")]
    pub userid: ::core::option::Option<i32>,
    #[prost(fixed64, optional, tag = "4")]
    pub steamid: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "5")]
    pub fakeplayer: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub ishltv: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CEntityMsg {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub target_entity: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgCVars {
    #[prost(message, repeated, tag = "1")]
    pub cvars: ::prost::alloc::vec::Vec<c_msg_c_vars::CVar>,
}
pub mod c_msg_c_vars {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CVar {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CNetMsgNop {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CNetMsgSplitScreenUser {
    #[prost(int32, optional, tag = "1")]
    pub slot: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CNetMsgTick {
    #[prost(uint32, optional, tag = "1")]
    pub tick: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub host_frametime: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub host_frametime_std_deviation: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub host_computationtime: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub host_computationtime_std_deviation: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub host_framestarttime_std_deviation: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub host_loss: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub host_unfiltered_frametime: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub hltv_replay_flags: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub expected_long_tick: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "11")]
    pub expected_long_tick_reason: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "12")]
    pub jitter: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CNetMsgStringCmd {
    #[prost(string, optional, tag = "1")]
    pub command: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "2")]
    pub prediction_sync: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CNetMsgSetConVar {
    #[prost(message, optional, tag = "1")]
    pub convars: ::core::option::Option<CMsgCVars>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CNetMsgSignonState {
    #[prost(enumeration = "SignonStateT", optional, tag = "1", default = "SignonstateNone")]
    pub signon_state: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub spawn_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub num_server_players: ::core::option::Option<u32>,
    #[prost(string, repeated, tag = "4")]
    pub players_networkids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub map_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub addons: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgGameEvent {
    #[prost(string, optional, tag = "1")]
    pub event_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub eventid: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub keys: ::prost::alloc::vec::Vec<csvc_msg_game_event::KeyT>,
}
pub mod csvc_msg_game_event {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyT {
        #[prost(int32, optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub val_string: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(float, optional, tag = "3")]
        pub val_float: ::core::option::Option<f32>,
        #[prost(int32, optional, tag = "4")]
        pub val_long: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "5")]
        pub val_short: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "6")]
        pub val_byte: ::core::option::Option<i32>,
        #[prost(bool, optional, tag = "7")]
        pub val_bool: ::core::option::Option<bool>,
        #[prost(uint64, optional, tag = "8")]
        pub val_uint64: ::core::option::Option<u64>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgListGameEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<csvc_msg_list_game_events::EventT>,
}
pub mod csvc_msg_list_game_events {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EventT {
        #[prost(int32, optional, tag = "1")]
        pub tick: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub event: ::core::option::Option<super::CSvcMsgGameEvent>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CNetMsgSpawnGroupLoad {
    #[prost(string, optional, tag = "1")]
    pub worldname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub entitylumpname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub entityfiltername: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub spawngrouphandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub spawngroupownerhandle: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "6")]
    pub world_offset_pos: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "7")]
    pub world_offset_angle: ::core::option::Option<CMsgQAngle>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub spawngroupmanifest: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "9")]
    pub flags: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "10")]
    pub tickcount: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "11")]
    pub manifestincomplete: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "12")]
    pub localnamefixup: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub parentnamefixup: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "14")]
    pub manifestloadpriority: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "15")]
    pub worldgroupid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "16")]
    pub creationsequence: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "17")]
    pub savegamefilename: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "18")]
    pub spawngroupparenthandle: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "19")]
    pub leveltransition: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "20")]
    pub worldgroupname: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CNetMsgSpawnGroupManifestUpdate {
    #[prost(uint32, optional, tag = "1")]
    pub spawngrouphandle: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub spawngroupmanifest: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "3")]
    pub manifestincomplete: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CNetMsgSpawnGroupSetCreationTick {
    #[prost(uint32, optional, tag = "1")]
    pub spawngrouphandle: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub tickcount: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub creationsequence: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CNetMsgSpawnGroupUnload {
    #[prost(uint32, optional, tag = "1")]
    pub spawngrouphandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub flags: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub tickcount: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CNetMsgSpawnGroupLoadCompleted {
    #[prost(uint32, optional, tag = "1")]
    pub spawngrouphandle: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgGameSessionConfiguration {
    #[prost(bool, optional, tag = "1")]
    pub is_multiplayer: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "2")]
    pub is_loadsavegame: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub is_background_map: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub is_headless: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "5")]
    pub min_client_limit: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub max_client_limit: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub max_clients: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "8")]
    pub tick_interval: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "9")]
    pub hostname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub savegamename: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub s1_mapname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub gamemode: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub server_ip_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "14")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "15")]
    pub is_localonly: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "19")]
    pub no_steam_server: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "16")]
    pub is_transition: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "17")]
    pub previouslevel: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub landmarkname: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CNetMsgDebugOverlay {
    #[prost(int32, optional, tag = "1")]
    pub etype: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub vectors: ::prost::alloc::vec::Vec<CMsgVector>,
    #[prost(message, repeated, tag = "3")]
    pub colors: ::prost::alloc::vec::Vec<CMsgRgba>,
    #[prost(float, repeated, packed = "false", tag = "4")]
    pub dimensions: ::prost::alloc::vec::Vec<f32>,
    #[prost(float, repeated, packed = "false", tag = "5")]
    pub times: ::prost::alloc::vec::Vec<f32>,
    #[prost(bool, repeated, packed = "false", tag = "6")]
    pub bools: ::prost::alloc::vec::Vec<bool>,
    #[prost(uint64, repeated, packed = "false", tag = "7")]
    pub uint64s: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, repeated, tag = "8")]
    pub strings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignonStateT {
    SignonstateNone = 0,
    SignonstateChallenge = 1,
    SignonstateConnected = 2,
    SignonstateNew = 3,
    SignonstatePrespawn = 4,
    SignonstateSpawn = 5,
    SignonstateFull = 6,
    SignonstateChangelevel = 7,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NetMessages {
    NetNop = 0,
    NetDisconnectLegacy = 1,
    NetSplitScreenUser = 3,
    NetTick = 4,
    NetStringCmd = 5,
    NetSetConVar = 6,
    NetSignonState = 7,
    NetSpawnGroupLoad = 8,
    NetSpawnGroupManifestUpdate = 9,
    NetSpawnGroupSetCreationTick = 11,
    NetSpawnGroupUnload = 12,
    NetSpawnGroupLoadCompleted = 13,
    NetDebugOverlay = 15,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SpawnGroupFlagsT {
    SpawnGroupLoadEntitiesFromSave = 1,
    SpawnGroupDontSpawnEntities = 2,
    SpawnGroupSynchronousSpawn = 4,
    SpawnGroupIsInitialSpawnGroup = 8,
    SpawnGroupCreateClientOnlyEntities = 16,
    SpawnGroupBlockUntilLoaded = 64,
    SpawnGroupLoadStreamingData = 128,
    SpawnGroupCreateNewSceneWorld = 256,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgVDebugGameSessionIdEvent {
    #[prost(int32, optional, tag = "1")]
    pub clientid: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub gamesessionid: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgPlaceDecalEvent {
    #[prost(message, optional, tag = "1")]
    pub position: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "2")]
    pub normal: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "3")]
    pub saxis: ::core::option::Option<CMsgVector>,
    #[prost(uint32, optional, tag = "4")]
    pub decalmaterialindex: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub flags: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "6")]
    pub color: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "7")]
    pub width: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "8")]
    pub height: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "9")]
    pub depth: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "10")]
    pub entityhandleindex: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "11")]
    pub skeletoninstancehash: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "12")]
    pub boneindex: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "13")]
    pub translucenthit: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "14")]
    pub is_adjacent: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgClearWorldDecalsEvent {
    #[prost(uint32, optional, tag = "1")]
    pub flagstoclear: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgClearEntityDecalsEvent {
    #[prost(uint32, optional, tag = "1")]
    pub flagstoclear: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgClearDecalsForSkeletonInstanceEvent {
    #[prost(uint32, optional, tag = "1")]
    pub flagstoclear: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub entityhandleindex: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub skeletoninstancehash: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSource1LegacyGameEventList {
    #[prost(message, repeated, tag = "1")]
    pub descriptors: ::prost::alloc::vec::Vec<c_msg_source1_legacy_game_event_list::DescriptorT>,
}
pub mod c_msg_source1_legacy_game_event_list {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyT {
        #[prost(int32, optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DescriptorT {
        #[prost(int32, optional, tag = "1")]
        pub eventid: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "3")]
        pub keys: ::prost::alloc::vec::Vec<KeyT>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSource1LegacyListenEvents {
    #[prost(int32, optional, tag = "1")]
    pub playerslot: ::core::option::Option<i32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub eventarraybits: ::prost::alloc::vec::Vec<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSource1LegacyGameEvent {
    #[prost(string, optional, tag = "1")]
    pub event_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub eventid: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub keys: ::prost::alloc::vec::Vec<c_msg_source1_legacy_game_event::KeyT>,
    #[prost(int32, optional, tag = "4")]
    pub server_tick: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub passthrough: ::core::option::Option<i32>,
}
pub mod c_msg_source1_legacy_game_event {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyT {
        #[prost(int32, optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub val_string: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(float, optional, tag = "3")]
        pub val_float: ::core::option::Option<f32>,
        #[prost(int32, optional, tag = "4")]
        pub val_long: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "5")]
        pub val_short: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "6")]
        pub val_byte: ::core::option::Option<i32>,
        #[prost(bool, optional, tag = "7")]
        pub val_bool: ::core::option::Option<bool>,
        #[prost(uint64, optional, tag = "8")]
        pub val_uint64: ::core::option::Option<u64>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSosStartSoundEvent {
    #[prost(int32, optional, tag = "1")]
    pub soundevent_guid: ::core::option::Option<i32>,
    #[prost(fixed32, optional, tag = "2")]
    pub soundevent_hash: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub source_entity_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub seed: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub packed_params: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(float, optional, tag = "6")]
    pub start_time: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgSosStopSoundEvent {
    #[prost(int32, optional, tag = "1")]
    pub soundevent_guid: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgSosStopSoundEventHash {
    #[prost(fixed32, optional, tag = "1")]
    pub soundevent_hash: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub source_entity_index: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSosSetSoundEventParams {
    #[prost(int32, optional, tag = "1")]
    pub soundevent_guid: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub packed_params: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSosSetLibraryStackFields {
    #[prost(fixed32, optional, tag = "1")]
    pub stack_hash: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub packed_fields: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EBaseGameEvents {
    GeVDebugGameSessionIdEvent = 200,
    GePlaceDecalEvent = 201,
    GeClearWorldDecalsEvent = 202,
    GeClearEntityDecalsEvent = 203,
    GeClearDecalsForSkeletonInstanceEvent = 204,
    GeSource1LegacyGameEventList = 205,
    GeSource1LegacyListenEvents = 206,
    GeSource1LegacyGameEvent = 207,
    GeSosStartSoundEvent = 208,
    GeSosStopSoundEvent = 209,
    GeSosSetSoundEventParams = 210,
    GeSosSetLibraryStackFields = 211,
    GeSosStopSoundEventHash = 212,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgClientInfo {
    #[prost(fixed32, optional, tag = "1")]
    pub send_table_crc: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub server_count: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub is_hltv: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "5")]
    pub friends_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "6")]
    pub friends_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgMove {
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "4")]
    pub last_command_number: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgVoiceAudio {
    #[prost(enumeration = "VoiceDataFormatT", optional, tag = "1", default = "VoicedataFormatSteam")]
    pub format: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub voice_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "3")]
    pub sequence_bytes: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "4")]
    pub section_number: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub sample_rate: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub uncompressed_sample_offset: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub num_packets: ::core::option::Option<u32>,
    #[prost(uint32, repeated, tag = "8")]
    pub packet_offsets: ::prost::alloc::vec::Vec<u32>,
    #[prost(float, optional, tag = "9")]
    pub voice_level: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgVoiceData {
    #[prost(message, optional, tag = "1")]
    pub audio: ::core::option::Option<CMsgVoiceAudio>,
    #[prost(fixed64, optional, tag = "2")]
    pub xuid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub tick: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CclcMsgBaselineAck {
    #[prost(int32, optional, tag = "1")]
    pub baseline_tick: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub baseline_nr: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgListenEvents {
    #[prost(fixed32, repeated, packed = "false", tag = "1")]
    pub event_mask: ::prost::alloc::vec::Vec<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgRespondCvarValue {
    #[prost(int32, optional, tag = "1")]
    pub cookie: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub status_code: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgFileCrcCheck {
    #[prost(int32, optional, tag = "1")]
    pub code_path: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub code_filename: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub filename: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(fixed32, optional, tag = "5")]
    pub crc: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CclcMsgLoadingProgress {
    #[prost(int32, optional, tag = "1")]
    pub progress: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgSplitPlayerConnect {
    #[prost(string, optional, tag = "1")]
    pub playername: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CclcMsgSplitPlayerDisconnect {
    #[prost(int32, optional, tag = "1")]
    pub slot: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CclcMsgServerStatus {
    #[prost(bool, optional, tag = "1")]
    pub simplified: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CclcMsgRequestPause {
    #[prost(enumeration = "RequestPauseT", optional, tag = "1", default = "RpPause")]
    pub pause_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub pause_group: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgCmdKeyValues {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgRconServerDetails {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSource2SystemSpecs {
    #[prost(string, optional, tag = "1")]
    pub cpu_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub cpu_brand: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub cpu_model: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub cpu_num_physical: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "21")]
    pub ram_physical_total_mb: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "41")]
    pub gpu_rendersystem_dll_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "42")]
    pub gpu_vendor_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "43")]
    pub gpu_driver_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "44")]
    pub gpu_driver_version_high: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "45")]
    pub gpu_driver_version_low: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "46")]
    pub gpu_dx_support_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "47")]
    pub gpu_texture_memory_size_mb: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSource2VProfLiteReportItem {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "2")]
    pub active_samples: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub usec_max: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub usec_avg_active: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub usec_p50_active: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "13")]
    pub usec_p99_active: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "21")]
    pub usec_avg_all: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "22")]
    pub usec_p50_all: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "23")]
    pub usec_p99_all: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSource2VProfLiteReport {
    #[prost(message, optional, tag = "1")]
    pub total: ::core::option::Option<CMsgSource2VProfLiteReportItem>,
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<CMsgSource2VProfLiteReportItem>,
    #[prost(uint32, optional, tag = "3")]
    pub discarded_frames: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgDiagnostic {
    #[prost(message, optional, tag = "1")]
    pub system_specs: ::core::option::Option<CMsgSource2SystemSpecs>,
    #[prost(message, optional, tag = "2")]
    pub vprof_report: ::core::option::Option<CMsgSource2VProfLiteReport>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSource2MetricsMatchPerfSummaryNotification {
    #[prost(uint32, optional, tag = "1")]
    pub appid: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub game_mode: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub server_build_id: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "10")]
    pub server_profile: ::core::option::Option<CMsgSource2VProfLiteReport>,
    #[prost(message, repeated, tag = "11")]
    pub clients: ::prost::alloc::vec::Vec<c_source2_metrics_match_perf_summary_notification::Client>,
    #[prost(string, optional, tag = "20")]
    pub map: ::core::option::Option<::prost::alloc::string::String>,
}
pub mod c_source2_metrics_match_perf_summary_notification {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Client {
        #[prost(message, optional, tag = "1")]
        pub system_specs: ::core::option::Option<super::CMsgSource2SystemSpecs>,
        #[prost(message, optional, tag = "2")]
        pub profile: ::core::option::Option<super::CMsgSource2VProfLiteReport>,
        #[prost(uint32, optional, tag = "3")]
        pub build_id: ::core::option::Option<u32>,
        #[prost(fixed64, optional, tag = "10")]
        pub steamid: ::core::option::Option<u64>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgServerInfo {
    #[prost(int32, optional, tag = "1")]
    pub protocol: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub server_count: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub is_dedicated: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub is_hltv: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "6")]
    pub c_os: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub max_clients: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub max_classes: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12", default = "-1")]
    pub player_slot: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "13")]
    pub tick_interval: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "14")]
    pub game_dir: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub map_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub sky_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "17")]
    pub host_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub addon_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "19")]
    pub game_session_config: ::core::option::Option<CSvcMsgGameSessionConfiguration>,
    #[prost(bytes = "vec", optional, tag = "20")]
    pub game_session_manifest: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgClassInfo {
    #[prost(bool, optional, tag = "1")]
    pub create_on_client: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "2")]
    pub classes: ::prost::alloc::vec::Vec<csvc_msg_class_info::ClassT>,
}
pub mod csvc_msg_class_info {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClassT {
        #[prost(int32, optional, tag = "1")]
        pub class_id: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "3")]
        pub class_name: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgSetPause {
    #[prost(bool, optional, tag = "1")]
    pub paused: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgVoiceInit {
    #[prost(int32, optional, tag = "1")]
    pub quality: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub codec: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3", default = "0")]
    pub version: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgPrint {
    #[prost(string, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgSounds {
    #[prost(bool, optional, tag = "1")]
    pub reliable_sound: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "2")]
    pub sounds: ::prost::alloc::vec::Vec<csvc_msg_sounds::SounddataT>,
}
pub mod csvc_msg_sounds {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SounddataT {
        #[prost(sint32, optional, tag = "1")]
        pub origin_x: ::core::option::Option<i32>,
        #[prost(sint32, optional, tag = "2")]
        pub origin_y: ::core::option::Option<i32>,
        #[prost(sint32, optional, tag = "3")]
        pub origin_z: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "4")]
        pub volume: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "5")]
        pub delay_value: ::core::option::Option<f32>,
        #[prost(int32, optional, tag = "6")]
        pub sequence_number: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "7", default = "-1")]
        pub entity_index: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "8")]
        pub channel: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "9")]
        pub pitch: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "10")]
        pub flags: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "11")]
        pub sound_num: ::core::option::Option<u32>,
        #[prost(fixed32, optional, tag = "12")]
        pub sound_num_handle: ::core::option::Option<u32>,
        #[prost(int32, optional, tag = "13")]
        pub speaker_entity: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "14")]
        pub random_seed: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "15")]
        pub sound_level: ::core::option::Option<i32>,
        #[prost(bool, optional, tag = "16")]
        pub is_sentence: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "17")]
        pub is_ambient: ::core::option::Option<bool>,
        #[prost(uint32, optional, tag = "18")]
        pub guid: ::core::option::Option<u32>,
        #[prost(fixed64, optional, tag = "19")]
        pub sound_resource_id: ::core::option::Option<u64>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgPrefetch {
    #[prost(int32, optional, tag = "1")]
    pub sound_index: ::core::option::Option<i32>,
    #[prost(enumeration = "PrefetchType", optional, tag = "2", default = "PftSound")]
    pub resource_type: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgSetView {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entity_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub slot: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgFixAngle {
    #[prost(bool, optional, tag = "1")]
    pub relative: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub angle: ::core::option::Option<CMsgQAngle>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgCrosshairAngle {
    #[prost(message, optional, tag = "1")]
    pub angle: ::core::option::Option<CMsgQAngle>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgBspDecal {
    #[prost(message, optional, tag = "1")]
    pub pos: ::core::option::Option<CMsgVector>,
    #[prost(int32, optional, tag = "2")]
    pub decal_texture_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub entity_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub model_index: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub low_priority: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgSplitScreen {
    #[prost(enumeration = "ESplitScreenMessageType", optional, tag = "1", default = "MsgSplitscreenAdduser")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub slot: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub player_index: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgGetCvarValue {
    #[prost(int32, optional, tag = "1")]
    pub cookie: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub cvar_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgMenu {
    #[prost(int32, optional, tag = "1")]
    pub dialog_type: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub menu_key_values: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgUserMessage {
    #[prost(int32, optional, tag = "1")]
    pub msg_type: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub msg_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "3")]
    pub passthrough: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgSendTable {
    #[prost(bool, optional, tag = "1")]
    pub is_end: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub net_table_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub needs_decoder: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "4")]
    pub props: ::prost::alloc::vec::Vec<csvc_msg_send_table::SendpropT>,
}
pub mod csvc_msg_send_table {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SendpropT {
        #[prost(int32, optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub var_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "3")]
        pub flags: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub priority: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "5")]
        pub dt_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "6")]
        pub num_elements: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "7")]
        pub low_value: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "8")]
        pub high_value: ::core::option::Option<f32>,
        #[prost(int32, optional, tag = "9")]
        pub num_bits: ::core::option::Option<i32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgGameEventList {
    #[prost(message, repeated, tag = "1")]
    pub descriptors: ::prost::alloc::vec::Vec<csvc_msg_game_event_list::DescriptorT>,
}
pub mod csvc_msg_game_event_list {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyT {
        #[prost(int32, optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DescriptorT {
        #[prost(int32, optional, tag = "1")]
        pub eventid: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "3")]
        pub keys: ::prost::alloc::vec::Vec<KeyT>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgPacketEntities {
    #[prost(int32, optional, tag = "1")]
    pub max_entries: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub updated_entries: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub legacy_is_delta: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub update_baseline: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "5")]
    pub baseline: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub delta_from: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub entity_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "8")]
    pub pending_full_frame: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "9")]
    pub active_spawngroup_handle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub max_spawngroup_creationsequence: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub last_cmd_number_executed: ::core::option::Option<u32>,
    #[prost(sint32, optional, tag = "17")]
    pub last_cmd_number_recv_delta: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "12")]
    pub server_tick: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub serialized_entities: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "15")]
    pub alternate_baselines: ::prost::alloc::vec::Vec<csvc_msg_packet_entities::AlternateBaselineT>,
    #[prost(uint32, optional, tag = "16")]
    pub has_pvs_vis_bits: ::core::option::Option<u32>,
    #[prost(sint32, repeated, tag = "22")]
    pub cmd_recv_status: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, optional, tag = "19")]
    pub non_transmitted_entities: ::core::option::Option<csvc_msg_packet_entities::NonTransmittedEntitiesT>,
    #[prost(uint32, optional, tag = "20")]
    pub cq_starved_command_ticks: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "21")]
    pub cq_discarded_command_ticks: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "999")]
    pub dev_padding: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
pub mod csvc_msg_packet_entities {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct AlternateBaselineT {
        #[prost(int32, optional, tag = "1")]
        pub entity_index: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub baseline_index: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NonTransmittedEntitiesT {
        #[prost(int32, optional, tag = "1")]
        pub header_count: ::core::option::Option<i32>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgTempEntities {
    #[prost(bool, optional, tag = "1")]
    pub reliable: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "2")]
    pub num_entries: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub entity_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgCreateStringTable {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub num_entries: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub user_data_fixed_size: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "4")]
    pub user_data_size: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub user_data_size_bits: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub flags: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub string_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "8")]
    pub uncompressed_size: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "9")]
    pub data_compressed: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "10")]
    pub using_varint_bitcounts: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgUpdateStringTable {
    #[prost(int32, optional, tag = "1")]
    pub table_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub num_changed_entries: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub string_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgVoiceData {
    #[prost(message, optional, tag = "1")]
    pub audio: ::core::option::Option<CMsgVoiceAudio>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub client: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub proximity: ::core::option::Option<bool>,
    #[prost(fixed64, optional, tag = "4")]
    pub xuid: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "5")]
    pub audible_mask: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "6")]
    pub tick: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "7")]
    pub passthrough: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgPacketReliable {
    #[prost(int32, optional, tag = "1")]
    pub tick: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub messagessize: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub state: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgFullFrameSplit {
    #[prost(int32, optional, tag = "1")]
    pub tick: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub section: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub total: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgHltvStatus {
    #[prost(string, optional, tag = "1")]
    pub master: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub clients: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub slots: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub proxies: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgServerSteamId {
    #[prost(uint64, optional, tag = "1")]
    pub steam_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgCmdKeyValues {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgRconServerDetails {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "2")]
    pub details: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgIpcAddress {
    #[prost(fixed64, optional, tag = "1")]
    pub computer_guid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub process_id: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgServerPeer {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_slot: ::core::option::Option<i32>,
    #[prost(fixed64, optional, tag = "2")]
    pub steamid: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "3")]
    pub ipc: ::core::option::Option<CMsgIpcAddress>,
    #[prost(bool, optional, tag = "4")]
    pub they_hear_you: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub you_hear_them: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub is_listenserver_host: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgPeerList {
    #[prost(message, repeated, tag = "1")]
    pub peer: ::prost::alloc::vec::Vec<CMsgServerPeer>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgClearAllStringTables {
    #[prost(string, optional, tag = "1")]
    pub mapname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub create_tables_skipped: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoFlattenedSerializerFieldT {
    #[prost(int32, optional, tag = "1")]
    pub var_type_sym: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub var_name_sym: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub bit_count: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "4")]
    pub low_value: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "5")]
    pub high_value: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "6")]
    pub encode_flags: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub field_serializer_name_sym: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub field_serializer_version: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub send_node_sym: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub var_encoder_sym: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "11")]
    pub polymorphic_types: ::prost::alloc::vec::Vec<proto_flattened_serializer_field_t::PolymorphicFieldT>,
    #[prost(int32, optional, tag = "12")]
    pub var_serializer_sym: ::core::option::Option<i32>,
}
pub mod proto_flattened_serializer_field_t {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct PolymorphicFieldT {
        #[prost(int32, optional, tag = "1")]
        pub polymorphic_field_serializer_name_sym: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub polymorphic_field_serializer_version: ::core::option::Option<i32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoFlattenedSerializerT {
    #[prost(int32, optional, tag = "1")]
    pub serializer_name_sym: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub serializer_version: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub fields_index: ::prost::alloc::vec::Vec<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgFlattenedSerializer {
    #[prost(message, repeated, tag = "1")]
    pub serializers: ::prost::alloc::vec::Vec<ProtoFlattenedSerializerT>,
    #[prost(string, repeated, tag = "2")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub fields: ::prost::alloc::vec::Vec<ProtoFlattenedSerializerFieldT>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgStopSound {
    #[prost(fixed32, optional, tag = "1")]
    pub guid: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CBidirMsgRebroadcastGameEvent {
    #[prost(bool, optional, tag = "1")]
    pub posttoserver: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "2")]
    pub buftype: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub clientbitcount: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "4")]
    pub receivingclients: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CBidirMsgRebroadcastSource {
    #[prost(int32, optional, tag = "1")]
    pub eventsource: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgServerNetworkStats {
    #[prost(bool, optional, tag = "1")]
    pub dedicated: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "2")]
    pub cpu_usage: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub memory_used_mb: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub memory_free_mb: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub uptime: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub spawn_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub num_clients: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub num_bots: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub num_spectators: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub num_tv_relays: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "12")]
    pub fps: ::core::option::Option<f32>,
    #[prost(message, repeated, tag = "17")]
    pub ports: ::prost::alloc::vec::Vec<c_msg_server_network_stats::Port>,
    #[prost(float, optional, tag = "18")]
    pub avg_ping_ms: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "19")]
    pub avg_engine_latency_out: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "20")]
    pub avg_packets_out: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "21")]
    pub avg_packets_in: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "22")]
    pub avg_loss_out: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "23")]
    pub avg_loss_in: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "24")]
    pub avg_data_out: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "25")]
    pub avg_data_in: ::core::option::Option<f32>,
    #[prost(uint64, optional, tag = "26")]
    pub total_data_in: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "27")]
    pub total_packets_in: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "28")]
    pub total_data_out: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "29")]
    pub total_packets_out: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "30")]
    pub players: ::prost::alloc::vec::Vec<c_msg_server_network_stats::Player>,
}
pub mod c_msg_server_network_stats {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Port {
        #[prost(int32, optional, tag = "1")]
        pub port: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Player {
        #[prost(uint64, optional, tag = "1")]
        pub steamid: ::core::option::Option<u64>,
        #[prost(string, optional, tag = "2")]
        pub remote_addr: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "4")]
        pub ping_avg_ms: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "5")]
        pub packet_loss_pct: ::core::option::Option<f32>,
        #[prost(bool, optional, tag = "6")]
        pub is_bot: ::core::option::Option<bool>,
        #[prost(float, optional, tag = "7")]
        pub loss_in: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "8")]
        pub loss_out: ::core::option::Option<f32>,
        #[prost(int32, optional, tag = "9")]
        pub engine_latency_ms: ::core::option::Option<i32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgHltvReplay {
    #[prost(int32, optional, tag = "1")]
    pub delay: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub primary_target: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub replay_stop_at: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub replay_start_at: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub replay_slowdown_begin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub replay_slowdown_end: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "7")]
    pub replay_slowdown_rate: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "8")]
    pub reason: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CclcMsgHltvReplay {
    #[prost(int32, optional, tag = "1")]
    pub request: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub slowdown_length: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub slowdown_rate: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub primary_target: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "5")]
    pub event_time: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgBroadcastCommand {
    #[prost(string, optional, tag = "1")]
    pub cmd: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgHltvFixupOperatorTick {
    #[prost(int32, optional, tag = "1")]
    pub tick: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub props_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "3")]
    pub origin: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "4")]
    pub eye_angles: ::core::option::Option<CMsgQAngle>,
    #[prost(int32, optional, tag = "5")]
    pub observer_mode: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "6")]
    pub cameraman_scoreboard: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "7")]
    pub observer_target: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "8")]
    pub view_offset: ::core::option::Option<CMsgVector>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgHltvFixupOperatorStatus {
    #[prost(uint32, optional, tag = "1")]
    pub mode: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub override_operator_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgServerUserCmd {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "2")]
    pub cmd_number: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub player_slot: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub server_tick_executed: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub client_tick: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgUserCommands {
    #[prost(message, repeated, tag = "1")]
    pub commands: ::prost::alloc::vec::Vec<CMsgServerUserCmd>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClcMessages {
    ClcClientInfo = 20,
    ClcMove = 21,
    ClcVoiceData = 22,
    ClcBaselineAck = 23,
    ClcRespondCvarValue = 25,
    ClcFileCrcCheck = 26,
    ClcLoadingProgress = 27,
    ClcSplitPlayerConnect = 28,
    ClcSplitPlayerDisconnect = 30,
    ClcServerStatus = 31,
    ClcRequestPause = 33,
    ClcCmdKeyValues = 34,
    ClcRconServerDetails = 35,
    ClcHltvReplay = 36,
    ClcDiagnostic = 37,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SvcMessages {
    SvcServerInfo = 40,
    SvcFlattenedSerializer = 41,
    SvcClassInfo = 42,
    SvcSetPause = 43,
    SvcCreateStringTable = 44,
    SvcUpdateStringTable = 45,
    SvcVoiceInit = 46,
    SvcVoiceData = 47,
    SvcPrint = 48,
    SvcSounds = 49,
    SvcSetView = 50,
    SvcClearAllStringTables = 51,
    SvcCmdKeyValues = 52,
    SvcBspDecal = 53,
    SvcSplitScreen = 54,
    SvcPacketEntities = 55,
    SvcPrefetch = 56,
    SvcMenu = 57,
    SvcGetCvarValue = 58,
    SvcStopSound = 59,
    SvcPeerList = 60,
    SvcPacketReliable = 61,
    SvcHltvStatus = 62,
    SvcServerSteamId = 63,
    SvcFullFrameSplit = 70,
    SvcRconServerDetails = 71,
    SvcUserMessage = 72,
    SvcBroadcastCommand = 74,
    SvcHltvFixupOperatorStatus = 75,
    SvcUserCmds = 76,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VoiceDataFormatT {
    VoicedataFormatSteam = 0,
    VoicedataFormatEngine = 1,
    VoicedataFormatOpus = 2,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RequestPauseT {
    RpPause = 0,
    RpUnpause = 1,
    RpTogglepause = 2,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PrefetchType {
    PftSound = 0,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ESplitScreenMessageType {
    MsgSplitscreenAdduser = 0,
    MsgSplitscreenRemoveuser = 1,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EQueryCvarValueStatus {
    ValueIntact = 0,
    CvarNotFound = 1,
    NotACvar = 2,
    CvarProtected = 3,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DialogType {
    DialogMsg = 0,
    DialogMenu = 1,
    DialogText = 2,
    DialogEntry = 3,
    DialogAskconnect = 4,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SvcMessagesLowFrequency {
    SvcDummy = 600,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BidirectionalMessages {
    BiRebroadcastGameEvent = 16,
    BiRebroadcastSource = 17,
    BiGameEvent = 18,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BidirectionalMessagesLowFrequency {
    BiRelayInfo = 700,
    BiRelayPacket = 701,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReplayEventTypeT {
    ReplayEventCancel = 0,
    ReplayEventDeath = 1,
    ReplayEventGeneric = 2,
    ReplayEventStuckNeedFullUpdate = 3,
    ReplayEventVictory = 4,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageAchievementEvent {
    #[prost(uint32, optional, tag = "1")]
    pub achievement: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageCloseCaption {
    #[prost(fixed32, optional, tag = "1")]
    pub hash: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "3")]
    pub from_player: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub ent_index: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageCloseCaptionDirect {
    #[prost(fixed32, optional, tag = "1")]
    pub hash: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "3")]
    pub from_player: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub ent_index: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageCloseCaptionPlaceholder {
    #[prost(string, optional, tag = "1")]
    pub string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
    #[prost(bool, optional, tag = "3")]
    pub from_player: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub ent_index: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageCurrentTimescale {
    #[prost(float, optional, tag = "1")]
    pub current: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageDesiredTimescale {
    #[prost(float, optional, tag = "1")]
    pub desired: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub acceleration: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub minblendrate: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub blenddeltamultiplier: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageFade {
    #[prost(uint32, optional, tag = "1")]
    pub duration: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub hold_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub flags: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "4")]
    pub color: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageShake {
    #[prost(uint32, optional, tag = "1")]
    pub command: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub amplitude: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub frequency: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub duration: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageShakeDir {
    #[prost(message, optional, tag = "1")]
    pub shake: ::core::option::Option<CUserMessageShake>,
    #[prost(message, optional, tag = "2")]
    pub direction: ::core::option::Option<CMsgVector>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageWaterShake {
    #[prost(uint32, optional, tag = "1")]
    pub command: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub amplitude: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub frequency: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub duration: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageScreenTilt {
    #[prost(uint32, optional, tag = "1")]
    pub command: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "2")]
    pub ease_in_out: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "3")]
    pub angle: ::core::option::Option<CMsgVector>,
    #[prost(float, optional, tag = "4")]
    pub duration: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "5")]
    pub time: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageSayText {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub playerindex: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub chat: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageSayText2 {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entityindex: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub chat: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "3")]
    pub messagename: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub param1: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub param2: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub param3: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub param4: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageHudMsg {
    #[prost(uint32, optional, tag = "1")]
    pub channel: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub y: ::core::option::Option<f32>,
    #[prost(fixed32, optional, tag = "4")]
    pub color1: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "5")]
    pub color2: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub effect: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "11")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageHudText {
    #[prost(string, optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageTextMsg {
    #[prost(uint32, optional, tag = "1")]
    pub dest: ::core::option::Option<u32>,
    #[prost(string, repeated, tag = "2")]
    pub param: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageGameTitle {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageResetHud {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageSendAudio {
    #[prost(string, optional, tag = "1")]
    pub soundname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub stop: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageAudioParameter {
    #[prost(uint32, optional, tag = "1")]
    pub parameter_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub name_hash_code: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "3")]
    pub value: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "4")]
    pub int_value: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageVoiceMask {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub gamerules_masks: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub ban_masks: ::prost::alloc::vec::Vec<u32>,
    #[prost(bool, optional, tag = "3")]
    pub mod_enable: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageRequestState {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageRumble {
    #[prost(int32, optional, tag = "1")]
    pub index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub data: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub flags: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageSayTextChannel {
    #[prost(int32, optional, tag = "1")]
    pub player: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub channel: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageColoredText {
    #[prost(uint32, optional, tag = "1")]
    pub color: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub reset: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub context_player_slot: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub context_value: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub context_team_id: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageItemPickup {
    #[prost(string, optional, tag = "1")]
    pub itemname: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageAmmoDenied {
    #[prost(uint32, optional, tag = "1")]
    pub ammo_id: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageShowMenu {
    #[prost(uint32, optional, tag = "1")]
    pub validslots: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub displaytime: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub needmore: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub menustring: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageCreditsMsg {
    #[prost(enumeration = "ERollType", optional, tag = "1", default = "RollNone")]
    pub rolltype: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub logo_length: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CEntityMessagePlayJingle {
    #[prost(message, optional, tag = "1")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CEntityMessageScreenOverlay {
    #[prost(bool, optional, tag = "1")]
    pub start_effect: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CEntityMessageRemoveAllDecals {
    #[prost(bool, optional, tag = "1")]
    pub remove_decals: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CEntityMessagePropagateForce {
    #[prost(message, optional, tag = "1")]
    pub impulse: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "2")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CEntityMessageDoSpark {
    #[prost(message, optional, tag = "1")]
    pub origin: ::core::option::Option<CMsgVector>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub entityindex: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "3")]
    pub radius: ::core::option::Option<f32>,
    #[prost(fixed32, optional, tag = "4")]
    pub color: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub beams: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "6")]
    pub thick: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "7")]
    pub duration: ::core::option::Option<f32>,
    #[prost(message, optional, tag = "8")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CEntityMessageFixAngle {
    #[prost(bool, optional, tag = "1")]
    pub relative: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub angle: ::core::option::Option<CMsgQAngle>,
    #[prost(message, optional, tag = "3")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageCameraTransition {
    #[prost(uint32, optional, tag = "1")]
    pub camera_type: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
    #[prost(message, optional, tag = "3")]
    pub params_data_driven: ::core::option::Option<c_user_message_camera_transition::TransitionDataDriven>,
}
pub mod c_user_message_camera_transition {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransitionDataDriven {
        #[prost(string, optional, tag = "1")]
        pub filename: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "2", default = "-1")]
        pub attach_ent_index: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "3")]
        pub duration: ::core::option::Option<f32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMsgParticleManager {
    #[prost(enumeration = "ParticleMessage", required, tag = "1", default = "GameParticleManagerEventCreate")]
    pub r#type: i32,
    #[prost(uint32, required, tag = "2")]
    pub index: u32,
    #[prost(message, optional, tag = "3")]
    pub release_particle_index: ::core::option::Option<c_user_msg_particle_manager::ReleaseParticleIndex>,
    #[prost(message, optional, tag = "4")]
    pub create_particle: ::core::option::Option<c_user_msg_particle_manager::CreateParticle>,
    #[prost(message, optional, tag = "5")]
    pub destroy_particle: ::core::option::Option<c_user_msg_particle_manager::DestroyParticle>,
    #[prost(message, optional, tag = "6")]
    pub destroy_particle_involving: ::core::option::Option<c_user_msg_particle_manager::DestroyParticleInvolving>,
    #[prost(message, optional, tag = "7")]
    pub update_particle: ::core::option::Option<c_user_msg_particle_manager::UpdateParticleObsolete>,
    #[prost(message, optional, tag = "8")]
    pub update_particle_fwd: ::core::option::Option<c_user_msg_particle_manager::UpdateParticleFwdObsolete>,
    #[prost(message, optional, tag = "9")]
    pub update_particle_orient: ::core::option::Option<c_user_msg_particle_manager::UpdateParticleOrientObsolete>,
    #[prost(message, optional, tag = "10")]
    pub update_particle_fallback: ::core::option::Option<c_user_msg_particle_manager::UpdateParticleFallback>,
    #[prost(message, optional, tag = "11")]
    pub update_particle_offset: ::core::option::Option<c_user_msg_particle_manager::UpdateParticleOffset>,
    #[prost(message, optional, tag = "12")]
    pub update_particle_ent: ::core::option::Option<c_user_msg_particle_manager::UpdateParticleEnt>,
    #[prost(message, optional, tag = "14")]
    pub update_particle_should_draw: ::core::option::Option<c_user_msg_particle_manager::UpdateParticleShouldDraw>,
    #[prost(message, optional, tag = "15")]
    pub update_particle_set_frozen: ::core::option::Option<c_user_msg_particle_manager::UpdateParticleSetFrozen>,
    #[prost(message, optional, tag = "16")]
    pub change_control_point_attachment: ::core::option::Option<c_user_msg_particle_manager::ChangeControlPointAttachment>,
    #[prost(message, optional, tag = "17")]
    pub update_entity_position: ::core::option::Option<c_user_msg_particle_manager::UpdateEntityPosition>,
    #[prost(message, optional, tag = "18")]
    pub set_particle_fow_properties: ::core::option::Option<c_user_msg_particle_manager::SetParticleFoWProperties>,
    #[prost(message, optional, tag = "19")]
    pub set_particle_text: ::core::option::Option<c_user_msg_particle_manager::SetParticleText>,
    #[prost(message, optional, tag = "20")]
    pub set_particle_should_check_fow: ::core::option::Option<c_user_msg_particle_manager::SetParticleShouldCheckFoW>,
    #[prost(message, optional, tag = "21")]
    pub set_control_point_model: ::core::option::Option<c_user_msg_particle_manager::SetControlPointModel>,
    #[prost(message, optional, tag = "22")]
    pub set_control_point_snapshot: ::core::option::Option<c_user_msg_particle_manager::SetControlPointSnapshot>,
    #[prost(message, optional, tag = "23")]
    pub set_texture_attribute: ::core::option::Option<c_user_msg_particle_manager::SetTextureAttribute>,
    #[prost(message, optional, tag = "24")]
    pub set_scene_object_generic_flag: ::core::option::Option<c_user_msg_particle_manager::SetSceneObjectGenericFlag>,
    #[prost(message, optional, tag = "25")]
    pub set_scene_object_tint_and_desat: ::core::option::Option<c_user_msg_particle_manager::SetSceneObjectTintAndDesat>,
    #[prost(message, optional, tag = "26")]
    pub destroy_particle_named: ::core::option::Option<c_user_msg_particle_manager::DestroyParticleNamed>,
    #[prost(message, optional, tag = "27")]
    pub particle_skip_to_time: ::core::option::Option<c_user_msg_particle_manager::ParticleSkipToTime>,
    #[prost(message, optional, tag = "28")]
    pub particle_can_freeze: ::core::option::Option<c_user_msg_particle_manager::ParticleCanFreeze>,
    #[prost(message, optional, tag = "29")]
    pub set_named_value_context: ::core::option::Option<c_user_msg_particle_manager::SetParticleNamedValueContext>,
    #[prost(message, optional, tag = "30")]
    pub update_particle_transform: ::core::option::Option<c_user_msg_particle_manager::UpdateParticleTransform>,
    #[prost(message, optional, tag = "31")]
    pub particle_freeze_transition_override: ::core::option::Option<c_user_msg_particle_manager::ParticleFreezeTransitionOverride>,
    #[prost(message, optional, tag = "32")]
    pub freeze_particle_involving: ::core::option::Option<c_user_msg_particle_manager::FreezeParticleInvolving>,
    #[prost(message, optional, tag = "33")]
    pub add_modellist_override_element: ::core::option::Option<c_user_msg_particle_manager::AddModellistOverrideElement>,
    #[prost(message, optional, tag = "34")]
    pub clear_modellist_override: ::core::option::Option<c_user_msg_particle_manager::ClearModellistOverride>,
    #[prost(message, optional, tag = "35")]
    pub create_physics_sim: ::core::option::Option<c_user_msg_particle_manager::CreatePhysicsSim>,
    #[prost(message, optional, tag = "36")]
    pub destroy_physics_sim: ::core::option::Option<c_user_msg_particle_manager::DestroyPhysicsSim>,
    #[prost(message, optional, tag = "37")]
    pub set_vdata: ::core::option::Option<c_user_msg_particle_manager::SetVData>,
    #[prost(message, optional, tag = "38")]
    pub set_material_override: ::core::option::Option<c_user_msg_particle_manager::SetMaterialOverride>,
}
pub mod c_user_msg_particle_manager {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ReleaseParticleIndex {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateParticle {
        #[prost(fixed64, optional, tag = "1")]
        pub particle_name_index: ::core::option::Option<u64>,
        #[prost(int32, optional, tag = "2")]
        pub attach_type: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "3", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4", default = "16777215")]
        pub entity_handle_for_modifiers: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "5")]
        pub apply_voice_ban_rules: ::core::option::Option<bool>,
        #[prost(int32, optional, tag = "6")]
        pub team_behavior: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "7")]
        pub control_point_configuration: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bool, optional, tag = "8")]
        pub cluster: ::core::option::Option<bool>,
        #[prost(float, optional, tag = "9")]
        pub endcap_time: ::core::option::Option<f32>,
        #[prost(message, optional, tag = "10")]
        pub aggregation_position: ::core::option::Option<super::CMsgVector>,
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
    pub struct DestroyParticleNamed {
        #[prost(fixed64, optional, tag = "1")]
        pub particle_name_index: ::core::option::Option<u64>,
        #[prost(uint32, optional, tag = "2", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "3")]
        pub destroy_immediately: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "4")]
        pub play_endcap: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleObsolete {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleFwdObsolete {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub forward: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleOrientObsolete {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub forward: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "3")]
        pub deprecated_right: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "4")]
        pub up: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "5")]
        pub left: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleTransform {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "3")]
        pub orientation: ::core::option::Option<super::CMsgQuaternion>,
        #[prost(float, optional, tag = "4")]
        pub interpolation_interval: ::core::option::Option<f32>,
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
    pub struct UpdateParticleOffset {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub origin_offset: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "3")]
        pub angle_offset: ::core::option::Option<super::CMsgQAngle>,
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
        #[prost(bool, optional, tag = "6")]
        pub include_wearables: ::core::option::Option<bool>,
        #[prost(message, optional, tag = "7")]
        pub offset_position: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "8")]
        pub offset_angles: ::core::option::Option<super::CMsgQAngle>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleSetFrozen {
        #[prost(bool, optional, tag = "1")]
        pub set_frozen: ::core::option::Option<bool>,
        #[prost(float, optional, tag = "2")]
        pub transition_duration: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleShouldDraw {
        #[prost(bool, optional, tag = "1")]
        pub should_draw: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ChangeControlPointAttachment {
        #[prost(int32, optional, tag = "1")]
        pub attachment_old: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub attachment_new: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "3", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateEntityPosition {
        #[prost(uint32, optional, tag = "1", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SetParticleFoWProperties {
        #[prost(int32, optional, tag = "1")]
        pub fow_control_point: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub fow_control_point2: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "3")]
        pub fow_radius: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SetParticleShouldCheckFoW {
        #[prost(bool, optional, tag = "1")]
        pub check_fow: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetControlPointModel {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub model_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetControlPointSnapshot {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub snapshot_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetParticleText {
        #[prost(string, optional, tag = "1")]
        pub text: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetTextureAttribute {
        #[prost(string, optional, tag = "1")]
        pub attribute_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub texture_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SetSceneObjectGenericFlag {
        #[prost(bool, optional, tag = "1")]
        pub flag_value: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SetSceneObjectTintAndDesat {
        #[prost(fixed32, optional, tag = "1")]
        pub tint: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "2")]
        pub desat: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ParticleSkipToTime {
        #[prost(float, optional, tag = "1")]
        pub skip_to_time: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ParticleCanFreeze {
        #[prost(bool, optional, tag = "1")]
        pub can_freeze: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ParticleFreezeTransitionOverride {
        #[prost(float, optional, tag = "1")]
        pub freeze_transition_override: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct FreezeParticleInvolving {
        #[prost(bool, optional, tag = "1")]
        pub set_frozen: ::core::option::Option<bool>,
        #[prost(float, optional, tag = "2")]
        pub transition_duration: ::core::option::Option<f32>,
        #[prost(uint32, optional, tag = "3", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AddModellistOverrideElement {
        #[prost(string, optional, tag = "1")]
        pub model_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(float, optional, tag = "2")]
        pub spawn_probability: ::core::option::Option<f32>,
        #[prost(uint32, optional, tag = "3")]
        pub groupid: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ClearModellistOverride {
        #[prost(uint32, optional, tag = "1")]
        pub groupid: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetParticleNamedValueContext {
        #[prost(message, repeated, tag = "1")]
        pub float_values: ::prost::alloc::vec::Vec<set_particle_named_value_context::FloatContextValue>,
        #[prost(message, repeated, tag = "2")]
        pub vector_values: ::prost::alloc::vec::Vec<set_particle_named_value_context::VectorContextValue>,
        #[prost(message, repeated, tag = "3")]
        pub transform_values: ::prost::alloc::vec::Vec<set_particle_named_value_context::TransformContextValue>,
        #[prost(message, repeated, tag = "4")]
        pub ehandle_values: ::prost::alloc::vec::Vec<set_particle_named_value_context::EHandleContext>,
    }
    pub mod set_particle_named_value_context {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct FloatContextValue {
            #[prost(uint32, optional, tag = "1")]
            pub value_name_hash: ::core::option::Option<u32>,
            #[prost(float, optional, tag = "2")]
            pub value: ::core::option::Option<f32>,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct VectorContextValue {
            #[prost(uint32, optional, tag = "1")]
            pub value_name_hash: ::core::option::Option<u32>,
            #[prost(message, optional, tag = "2")]
            pub value: ::core::option::Option<super::super::CMsgVector>,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct TransformContextValue {
            #[prost(uint32, optional, tag = "1")]
            pub value_name_hash: ::core::option::Option<u32>,
            #[prost(message, optional, tag = "2")]
            pub angles: ::core::option::Option<super::super::CMsgQAngle>,
            #[prost(message, optional, tag = "3")]
            pub translation: ::core::option::Option<super::super::CMsgVector>,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct EHandleContext {
            #[prost(uint32, optional, tag = "1")]
            pub value_name_hash: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "2", default = "16777215")]
            pub ent_index: ::core::option::Option<u32>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreatePhysicsSim {
        #[prost(string, optional, tag = "1")]
        pub prop_group_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bool, optional, tag = "2")]
        pub use_high_quality_simulation: ::core::option::Option<bool>,
        #[prost(uint32, optional, tag = "3")]
        pub max_particle_count: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct DestroyPhysicsSim {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetVData {
        #[prost(string, optional, tag = "1")]
        pub vdata_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetMaterialOverride {
        #[prost(string, optional, tag = "1")]
        pub material_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bool, optional, tag = "2")]
        pub include_children: ::core::option::Option<bool>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMsgHudError {
    #[prost(int32, optional, tag = "1")]
    pub order_id: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMsgCustomGameEvent {
    #[prost(string, optional, tag = "1")]
    pub event_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageHapticsManagerPulse {
    #[prost(int32, optional, tag = "1")]
    pub hand_id: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub effect_amplitude: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub effect_frequency: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub effect_duration: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageHapticsManagerEffect {
    #[prost(int32, optional, tag = "1")]
    pub hand_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub effect_name_hash_code: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "3")]
    pub effect_scale: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageAnimStateGraphState {
    #[prost(int32, optional, tag = "1")]
    pub entity_index: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageUpdateCssClasses {
    #[prost(int32, optional, tag = "1")]
    pub target_world_panel: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub css_classes: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub is_add: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageServerFrameTime {
    #[prost(float, optional, tag = "1")]
    pub frame_time: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageLagCompensationError {
    #[prost(float, optional, tag = "1")]
    pub distance: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageRequestDllStatus {
    #[prost(string, optional, tag = "1")]
    pub dll_action: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub full_report: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageRequestUtilAction {
    #[prost(int32, optional, tag = "2")]
    pub util1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub util2: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub util3: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub util4: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub util5: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageUtilMsgResponse {
    #[prost(fixed32, optional, tag = "1")]
    pub crc: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub item_count: ::core::option::Option<i32>,
    #[prost(fixed32, optional, tag = "3")]
    pub crc2: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "4")]
    pub item_count2: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "5")]
    pub crc_part: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "6")]
    pub crc_part2: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, optional, tag = "7")]
    pub client_timestamp: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub platform: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "9")]
    pub itemdetails: ::prost::alloc::vec::Vec<c_user_message_util_msg_response::ItemDetail>,
    #[prost(int32, optional, tag = "10")]
    pub itemgroup: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub total_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub total_count2: ::core::option::Option<i32>,
}
pub mod c_user_message_util_msg_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ItemDetail {
        #[prost(int32, optional, tag = "1")]
        pub index: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub hash: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub crc: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "4")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageDllStatus {
    #[prost(string, optional, tag = "1")]
    pub file_report: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub command_line: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub total_files: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub process_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "5")]
    pub osversion: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "6")]
    pub client_time: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "7")]
    pub diagnostics: ::prost::alloc::vec::Vec<c_user_message_dll_status::CvDiagnostic>,
    #[prost(message, repeated, tag = "8")]
    pub modules: ::prost::alloc::vec::Vec<c_user_message_dll_status::CModule>,
}
pub mod c_user_message_dll_status {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CModule {
        #[prost(uint64, optional, tag = "1")]
        pub base_addr: ::core::option::Option<u64>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "3")]
        pub size: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub timestamp: ::core::option::Option<u32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageRequestInventory {
    #[prost(int32, optional, tag = "1")]
    pub inventory: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub offset: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub options: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageInventoryResponse {
    #[prost(fixed32, optional, tag = "1")]
    pub crc: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub item_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub osversion: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub perf_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub client_timestamp: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub platform: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "9")]
    pub inventories: ::prost::alloc::vec::Vec<c_user_message_inventory_response::InventoryDetail>,
    #[prost(message, repeated, tag = "10")]
    pub inventories2: ::prost::alloc::vec::Vec<c_user_message_inventory_response::InventoryDetail>,
    #[prost(message, repeated, tag = "14")]
    pub inventories3: ::prost::alloc::vec::Vec<c_user_message_inventory_response::InventoryDetail>,
    #[prost(int32, optional, tag = "11")]
    pub inv_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub build_version: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub instance: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "15")]
    pub start_time: ::core::option::Option<i64>,
}
pub mod c_user_message_inventory_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InventoryDetail {
        #[prost(int32, optional, tag = "1")]
        pub index: ::core::option::Option<i32>,
        #[prost(int64, optional, tag = "2")]
        pub primary: ::core::option::Option<i64>,
        #[prost(int64, optional, tag = "3")]
        pub offset: ::core::option::Option<i64>,
        #[prost(int64, optional, tag = "4")]
        pub first: ::core::option::Option<i64>,
        #[prost(int64, optional, tag = "5")]
        pub base: ::core::option::Option<i64>,
        #[prost(string, optional, tag = "6")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "7")]
        pub base_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "8")]
        pub base_detail: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "9")]
        pub base_time: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "10")]
        pub base_hash: ::core::option::Option<i32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageRequestDiagnostic {
    #[prost(message, repeated, tag = "1")]
    pub diagnostics: ::prost::alloc::vec::Vec<c_user_message_request_diagnostic::Diagnostic>,
}
pub mod c_user_message_request_diagnostic {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Diagnostic {
        #[prost(int32, optional, tag = "1")]
        pub index: ::core::option::Option<i32>,
        #[prost(int64, optional, tag = "2")]
        pub offset: ::core::option::Option<i64>,
        #[prost(int32, optional, tag = "3")]
        pub param: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub length: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "5")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(int64, optional, tag = "6")]
        pub base: ::core::option::Option<i64>,
        #[prost(int64, optional, tag = "7")]
        pub range: ::core::option::Option<i64>,
        #[prost(int64, optional, tag = "8")]
        pub extent: ::core::option::Option<i64>,
        #[prost(int64, optional, tag = "9")]
        pub detail: ::core::option::Option<i64>,
        #[prost(string, optional, tag = "10")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "11")]
        pub alias: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes = "vec", optional, tag = "12")]
        pub vardetail: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(int32, optional, tag = "13")]
        pub context: ::core::option::Option<i32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageDiagnosticResponse {
    #[prost(message, repeated, tag = "1")]
    pub diagnostics: ::prost::alloc::vec::Vec<c_user_message_diagnostic_response::Diagnostic>,
    #[prost(int32, optional, tag = "2")]
    pub build_version: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub instance: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "4")]
    pub start_time: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "5")]
    pub osversion: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub platform: ::core::option::Option<i32>,
}
pub mod c_user_message_diagnostic_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Diagnostic {
        #[prost(int32, optional, tag = "1")]
        pub index: ::core::option::Option<i32>,
        #[prost(int64, optional, tag = "2")]
        pub offset: ::core::option::Option<i64>,
        #[prost(int32, optional, tag = "3")]
        pub param: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub length: ::core::option::Option<i32>,
        #[prost(bytes = "vec", optional, tag = "5")]
        pub detail: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(int64, optional, tag = "6")]
        pub base: ::core::option::Option<i64>,
        #[prost(int64, optional, tag = "7")]
        pub range: ::core::option::Option<i64>,
        #[prost(int32, optional, tag = "8")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "10")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "11")]
        pub alias: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes = "vec", optional, tag = "12")]
        pub backup: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(int32, optional, tag = "13")]
        pub context: ::core::option::Option<i32>,
        #[prost(int64, optional, tag = "14")]
        pub control: ::core::option::Option<i64>,
        #[prost(int64, optional, tag = "15")]
        pub augment: ::core::option::Option<i64>,
        #[prost(int64, optional, tag = "16")]
        pub placebo: ::core::option::Option<i64>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageExtraUserData {
    #[prost(int32, optional, tag = "1")]
    pub item: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "2")]
    pub value1: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub value2: ::core::option::Option<i64>,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub detail1: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub detail2: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageNotifyResponseFound {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub ent_index: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub rule_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub response_value: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub response_concept: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub criteria: ::prost::alloc::vec::Vec<c_user_message_notify_response_found::Criteria>,
    #[prost(uint32, repeated, tag = "6")]
    pub int_criteria_names: ::prost::alloc::vec::Vec<u32>,
    #[prost(int32, repeated, tag = "7")]
    pub int_criteria_values: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint32, repeated, tag = "8")]
    pub float_criteria_names: ::prost::alloc::vec::Vec<u32>,
    #[prost(float, repeated, packed = "false", tag = "9")]
    pub float_criteria_values: ::prost::alloc::vec::Vec<f32>,
    #[prost(uint32, repeated, tag = "10")]
    pub symbol_criteria_names: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "11")]
    pub symbol_criteria_values: ::prost::alloc::vec::Vec<u32>,
    #[prost(int32, optional, tag = "12")]
    pub speak_result: ::core::option::Option<i32>,
}
pub mod c_user_message_notify_response_found {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Criteria {
        #[prost(uint32, optional, tag = "1")]
        pub name_symbol: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessagePlayResponseConditional {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub ent_index: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_slots: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, optional, tag = "3")]
    pub response: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub ent_origin: ::core::option::Option<CMsgVector>,
    #[prost(float, optional, tag = "5")]
    pub pre_delay: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "6")]
    pub mix_priority: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EBaseUserMessages {
    UmAchievementEvent = 101,
    UmCloseCaption = 102,
    UmCloseCaptionDirect = 103,
    UmCurrentTimescale = 104,
    UmDesiredTimescale = 105,
    UmFade = 106,
    UmGameTitle = 107,
    UmHudMsg = 110,
    UmHudText = 111,
    UmColoredText = 113,
    UmRequestState = 114,
    UmResetHud = 115,
    UmRumble = 116,
    UmSayText = 117,
    UmSayText2 = 118,
    UmSayTextChannel = 119,
    UmShake = 120,
    UmShakeDir = 121,
    UmWaterShake = 122,
    UmTextMsg = 124,
    UmScreenTilt = 125,
    UmVoiceMask = 128,
    UmSendAudio = 130,
    UmItemPickup = 131,
    UmAmmoDenied = 132,
    UmShowMenu = 134,
    UmCreditsMsg = 135,
    UmCloseCaptionPlaceholder = 142,
    UmCameraTransition = 143,
    UmAudioParameter = 144,
    UmParticleManager = 145,
    UmHudError = 146,
    UmCustomGameEvent = 148,
    UmAnimGraphUpdate = 149,
    UmHapticsManagerPulse = 150,
    UmHapticsManagerEffect = 151,
    UmCommandQueueState = 152,
    UmUpdateCssClasses = 153,
    UmServerFrameTime = 154,
    UmLagCompensationError = 155,
    UmRequestDllStatus = 156,
    UmRequestUtilAction = 157,
    UmUtilActionResponse = 158,
    UmDllStatusResponse = 159,
    UmRequestInventory = 160,
    UmInventoryResponse = 161,
    UmRequestDiagnostic = 162,
    UmDiagnosticResponse = 163,
    UmExtraUserData = 164,
    UmNotifyResponseFound = 165,
    UmPlayResponseConditional = 166,
    UmMaxBase = 200,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EBaseEntityMessages {
    EmPlayJingle = 136,
    EmScreenOverlay = 137,
    EmRemoveAllDecals = 138,
    EmPropagateForce = 139,
    EmDoSpark = 140,
    EmFixAngle = 141,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ERollType {
    RollNone = -1,
    RollStats = 0,
    RollCredits = 1,
    RollLateJoinLogo = 2,
    RollOuttro = 3,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ParticleMessage {
    GameParticleManagerEventCreate = 0,
    GameParticleManagerEventUpdate = 1,
    GameParticleManagerEventUpdateForward = 2,
    GameParticleManagerEventUpdateOrientation = 3,
    GameParticleManagerEventUpdateFallback = 4,
    GameParticleManagerEventUpdateEnt = 5,
    GameParticleManagerEventUpdateOffset = 6,
    GameParticleManagerEventDestroy = 7,
    GameParticleManagerEventDestroyInvolving = 8,
    GameParticleManagerEventRelease = 9,
    GameParticleManagerEventLatency = 10,
    GameParticleManagerEventShouldDraw = 11,
    GameParticleManagerEventFrozen = 12,
    GameParticleManagerEventChangeControlPointAttachment = 13,
    GameParticleManagerEventUpdateEntityPosition = 14,
    GameParticleManagerEventSetFowProperties = 15,
    GameParticleManagerEventSetText = 16,
    GameParticleManagerEventSetShouldCheckFow = 17,
    GameParticleManagerEventSetControlPointModel = 18,
    GameParticleManagerEventSetControlPointSnapshot = 19,
    GameParticleManagerEventSetTextureAttribute = 20,
    GameParticleManagerEventSetSceneObjectGenericFlag = 21,
    GameParticleManagerEventSetSceneObjectTintAndDesat = 22,
    GameParticleManagerEventDestroyNamed = 23,
    GameParticleManagerEventSkipToTime = 24,
    GameParticleManagerEventCanFreeze = 25,
    GameParticleManagerEventSetNamedValueContext = 26,
    GameParticleManagerEventUpdateTransform = 27,
    GameParticleManagerEventFreezeTransitionOverride = 28,
    GameParticleManagerEventFreezeInvolving = 29,
    GameParticleManagerEventAddModellistOverrideElement = 30,
    GameParticleManagerEventClearModellistOverride = 31,
    GameParticleManagerEventCreatePhysicsSim = 32,
    GameParticleManagerEventDestroyPhysicsSim = 33,
    GameParticleManagerEventSetVdata = 34,
    GameParticleManagerEventSetMaterialOverride = 35,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EHapticPulseType {
    VrHandHapticPulseLight = 0,
    VrHandHapticPulseMedium = 1,
    VrHandHapticPulseStrong = 2,
}
