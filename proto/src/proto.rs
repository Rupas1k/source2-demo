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
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CGameInfo {
    #[prost(message, optional, tag = "4")]
    pub dota: ::core::option::Option<c_game_info::CDotaGameInfo>,
}
/// Nested message and enum types in `CGameInfo`.
pub mod c_game_info {
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
    /// Nested message and enum types in `CDotaGameInfo`.
    pub mod c_dota_game_info {
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
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CHeroSelectEvent {
            #[prost(bool, optional, tag = "1")]
            pub is_pick: ::core::option::Option<bool>,
            #[prost(uint32, optional, tag = "2")]
            pub team: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "3")]
            pub hero_id: ::core::option::Option<u32>,
        }
    }
}
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoPacket {
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoFullPacket {
    #[prost(message, optional, tag = "1")]
    pub string_table: ::core::option::Option<CDemoStringTables>,
    #[prost(message, optional, tag = "2")]
    pub packet: ::core::option::Option<CDemoPacket>,
}
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoSyncTick {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoConsoleCmd {
    #[prost(string, optional, tag = "1")]
    pub cmdstring: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoSendTables {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoClassInfo {
    #[prost(message, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<c_demo_class_info::ClassT>,
}
/// Nested message and enum types in `CDemoClassInfo`.
pub mod c_demo_class_info {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoCustomData {
    #[prost(int32, optional, tag = "1")]
    pub callback_index: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoCustomDataCallbacks {
    #[prost(string, repeated, tag = "1")]
    pub save_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoStringTables {
    #[prost(message, repeated, tag = "1")]
    pub tables: ::prost::alloc::vec::Vec<c_demo_string_tables::TableT>,
}
/// Nested message and enum types in `CDemoStringTables`.
pub mod c_demo_string_tables {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ItemsT {
        #[prost(string, optional, tag = "1")]
        pub str: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoStop {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoUserCmd {
    #[prost(int32, optional, tag = "1")]
    pub cmd_number: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CDemoSpawnGroups {
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub msgs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    DemMax = 17,
    DemIsCompressed = 64,
}
impl EDemoCommands {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EDemoCommands::DemError => "DEM_Error",
            EDemoCommands::DemStop => "DEM_Stop",
            EDemoCommands::DemFileHeader => "DEM_FileHeader",
            EDemoCommands::DemFileInfo => "DEM_FileInfo",
            EDemoCommands::DemSyncTick => "DEM_SyncTick",
            EDemoCommands::DemSendTables => "DEM_SendTables",
            EDemoCommands::DemClassInfo => "DEM_ClassInfo",
            EDemoCommands::DemStringTables => "DEM_StringTables",
            EDemoCommands::DemPacket => "DEM_Packet",
            EDemoCommands::DemSignonPacket => "DEM_SignonPacket",
            EDemoCommands::DemConsoleCmd => "DEM_ConsoleCmd",
            EDemoCommands::DemCustomData => "DEM_CustomData",
            EDemoCommands::DemCustomDataCallbacks => "DEM_CustomDataCallbacks",
            EDemoCommands::DemUserCmd => "DEM_UserCmd",
            EDemoCommands::DemFullPacket => "DEM_FullPacket",
            EDemoCommands::DemSaveGame => "DEM_SaveGame",
            EDemoCommands::DemSpawnGroups => "DEM_SpawnGroups",
            EDemoCommands::DemAnimationData => "DEM_AnimationData",
            EDemoCommands::DemMax => "DEM_Max",
            EDemoCommands::DemIsCompressed => "DEM_IsCompressed",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEM_Error" => Some(Self::DemError),
            "DEM_Stop" => Some(Self::DemStop),
            "DEM_FileHeader" => Some(Self::DemFileHeader),
            "DEM_FileInfo" => Some(Self::DemFileInfo),
            "DEM_SyncTick" => Some(Self::DemSyncTick),
            "DEM_SendTables" => Some(Self::DemSendTables),
            "DEM_ClassInfo" => Some(Self::DemClassInfo),
            "DEM_StringTables" => Some(Self::DemStringTables),
            "DEM_Packet" => Some(Self::DemPacket),
            "DEM_SignonPacket" => Some(Self::DemSignonPacket),
            "DEM_ConsoleCmd" => Some(Self::DemConsoleCmd),
            "DEM_CustomData" => Some(Self::DemCustomData),
            "DEM_CustomDataCallbacks" => Some(Self::DemCustomDataCallbacks),
            "DEM_UserCmd" => Some(Self::DemUserCmd),
            "DEM_FullPacket" => Some(Self::DemFullPacket),
            "DEM_SaveGame" => Some(Self::DemSaveGame),
            "DEM_SpawnGroups" => Some(Self::DemSpawnGroups),
            "DEM_AnimationData" => Some(Self::DemAnimationData),
            "DEM_Max" => Some(Self::DemMax),
            "DEM_IsCompressed" => Some(Self::DemIsCompressed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
impl ENetworkDisconnectionReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ENetworkDisconnectionReason::NetworkDisconnectInvalid => {
                "NETWORK_DISCONNECT_INVALID"
            }
            ENetworkDisconnectionReason::NetworkDisconnectShutdown => {
                "NETWORK_DISCONNECT_SHUTDOWN"
            }
            ENetworkDisconnectionReason::NetworkDisconnectDisconnectByUser => {
                "NETWORK_DISCONNECT_DISCONNECT_BY_USER"
            }
            ENetworkDisconnectionReason::NetworkDisconnectDisconnectByServer => {
                "NETWORK_DISCONNECT_DISCONNECT_BY_SERVER"
            }
            ENetworkDisconnectionReason::NetworkDisconnectLost => {
                "NETWORK_DISCONNECT_LOST"
            }
            ENetworkDisconnectionReason::NetworkDisconnectOverflow => {
                "NETWORK_DISCONNECT_OVERFLOW"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamBanned => {
                "NETWORK_DISCONNECT_STEAM_BANNED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamInuse => {
                "NETWORK_DISCONNECT_STEAM_INUSE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamTicket => {
                "NETWORK_DISCONNECT_STEAM_TICKET"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamLogon => {
                "NETWORK_DISCONNECT_STEAM_LOGON"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamAuthcancelled => {
                "NETWORK_DISCONNECT_STEAM_AUTHCANCELLED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamAuthalreadyused => {
                "NETWORK_DISCONNECT_STEAM_AUTHALREADYUSED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamAuthinvalid => {
                "NETWORK_DISCONNECT_STEAM_AUTHINVALID"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamVacbanstate => {
                "NETWORK_DISCONNECT_STEAM_VACBANSTATE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamLoggedInElsewhere => {
                "NETWORK_DISCONNECT_STEAM_LOGGED_IN_ELSEWHERE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamVacCheckTimedout => {
                "NETWORK_DISCONNECT_STEAM_VAC_CHECK_TIMEDOUT"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamDropped => {
                "NETWORK_DISCONNECT_STEAM_DROPPED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamOwnership => {
                "NETWORK_DISCONNECT_STEAM_OWNERSHIP"
            }
            ENetworkDisconnectionReason::NetworkDisconnectServerinfoOverflow => {
                "NETWORK_DISCONNECT_SERVERINFO_OVERFLOW"
            }
            ENetworkDisconnectionReason::NetworkDisconnectTickmsgOverflow => {
                "NETWORK_DISCONNECT_TICKMSG_OVERFLOW"
            }
            ENetworkDisconnectionReason::NetworkDisconnectStringtablemsgOverflow => {
                "NETWORK_DISCONNECT_STRINGTABLEMSG_OVERFLOW"
            }
            ENetworkDisconnectionReason::NetworkDisconnectDeltaentmsgOverflow => {
                "NETWORK_DISCONNECT_DELTAENTMSG_OVERFLOW"
            }
            ENetworkDisconnectionReason::NetworkDisconnectTempentmsgOverflow => {
                "NETWORK_DISCONNECT_TEMPENTMSG_OVERFLOW"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSoundsmsgOverflow => {
                "NETWORK_DISCONNECT_SOUNDSMSG_OVERFLOW"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSnapshotoverflow => {
                "NETWORK_DISCONNECT_SNAPSHOTOVERFLOW"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSnapshoterror => {
                "NETWORK_DISCONNECT_SNAPSHOTERROR"
            }
            ENetworkDisconnectionReason::NetworkDisconnectReliableoverflow => {
                "NETWORK_DISCONNECT_RELIABLEOVERFLOW"
            }
            ENetworkDisconnectionReason::NetworkDisconnectBaddeltatick => {
                "NETWORK_DISCONNECT_BADDELTATICK"
            }
            ENetworkDisconnectionReason::NetworkDisconnectNomoresplits => {
                "NETWORK_DISCONNECT_NOMORESPLITS"
            }
            ENetworkDisconnectionReason::NetworkDisconnectTimedout => {
                "NETWORK_DISCONNECT_TIMEDOUT"
            }
            ENetworkDisconnectionReason::NetworkDisconnectDisconnected => {
                "NETWORK_DISCONNECT_DISCONNECTED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectLeavingsplit => {
                "NETWORK_DISCONNECT_LEAVINGSPLIT"
            }
            ENetworkDisconnectionReason::NetworkDisconnectDifferentclasstables => {
                "NETWORK_DISCONNECT_DIFFERENTCLASSTABLES"
            }
            ENetworkDisconnectionReason::NetworkDisconnectBadrelaypassword => {
                "NETWORK_DISCONNECT_BADRELAYPASSWORD"
            }
            ENetworkDisconnectionReason::NetworkDisconnectBadspectatorpassword => {
                "NETWORK_DISCONNECT_BADSPECTATORPASSWORD"
            }
            ENetworkDisconnectionReason::NetworkDisconnectHltvrestricted => {
                "NETWORK_DISCONNECT_HLTVRESTRICTED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectNospectators => {
                "NETWORK_DISCONNECT_NOSPECTATORS"
            }
            ENetworkDisconnectionReason::NetworkDisconnectHltvunavailable => {
                "NETWORK_DISCONNECT_HLTVUNAVAILABLE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectHltvstop => {
                "NETWORK_DISCONNECT_HLTVSTOP"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKicked => {
                "NETWORK_DISCONNECT_KICKED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectBanadded => {
                "NETWORK_DISCONNECT_BANADDED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickbanadded => {
                "NETWORK_DISCONNECT_KICKBANADDED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectHltvdirect => {
                "NETWORK_DISCONNECT_HLTVDIRECT"
            }
            ENetworkDisconnectionReason::NetworkDisconnectPureserverClientextra => {
                "NETWORK_DISCONNECT_PURESERVER_CLIENTEXTRA"
            }
            ENetworkDisconnectionReason::NetworkDisconnectPureserverMismatch => {
                "NETWORK_DISCONNECT_PURESERVER_MISMATCH"
            }
            ENetworkDisconnectionReason::NetworkDisconnectUsercmd => {
                "NETWORK_DISCONNECT_USERCMD"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectedByGame => {
                "NETWORK_DISCONNECT_REJECTED_BY_GAME"
            }
            ENetworkDisconnectionReason::NetworkDisconnectMessageParseError => {
                "NETWORK_DISCONNECT_MESSAGE_PARSE_ERROR"
            }
            ENetworkDisconnectionReason::NetworkDisconnectInvalidMessageError => {
                "NETWORK_DISCONNECT_INVALID_MESSAGE_ERROR"
            }
            ENetworkDisconnectionReason::NetworkDisconnectBadServerPassword => {
                "NETWORK_DISCONNECT_BAD_SERVER_PASSWORD"
            }
            ENetworkDisconnectionReason::NetworkDisconnectDirectConnectReservation => {
                "NETWORK_DISCONNECT_DIRECT_CONNECT_RESERVATION"
            }
            ENetworkDisconnectionReason::NetworkDisconnectConnectionFailure => {
                "NETWORK_DISCONNECT_CONNECTION_FAILURE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectNoPeerGroupHandlers => {
                "NETWORK_DISCONNECT_NO_PEER_GROUP_HANDLERS"
            }
            ENetworkDisconnectionReason::NetworkDisconnectReconnection => {
                "NETWORK_DISCONNECT_RECONNECTION"
            }
            ENetworkDisconnectionReason::NetworkDisconnectLoopshutdown => {
                "NETWORK_DISCONNECT_LOOPSHUTDOWN"
            }
            ENetworkDisconnectionReason::NetworkDisconnectLoopdeactivate => {
                "NETWORK_DISCONNECT_LOOPDEACTIVATE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectHostEndgame => {
                "NETWORK_DISCONNECT_HOST_ENDGAME"
            }
            ENetworkDisconnectionReason::NetworkDisconnectLoopLevelloadActivate => {
                "NETWORK_DISCONNECT_LOOP_LEVELLOAD_ACTIVATE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectCreateServerFailed => {
                "NETWORK_DISCONNECT_CREATE_SERVER_FAILED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectExiting => {
                "NETWORK_DISCONNECT_EXITING"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRequestHoststateIdle => {
                "NETWORK_DISCONNECT_REQUEST_HOSTSTATE_IDLE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRequestHoststateHltvrelay => {
                "NETWORK_DISCONNECT_REQUEST_HOSTSTATE_HLTVRELAY"
            }
            ENetworkDisconnectionReason::NetworkDisconnectClientConsistencyFail => {
                "NETWORK_DISCONNECT_CLIENT_CONSISTENCY_FAIL"
            }
            ENetworkDisconnectionReason::NetworkDisconnectClientUnableToCrcMap => {
                "NETWORK_DISCONNECT_CLIENT_UNABLE_TO_CRC_MAP"
            }
            ENetworkDisconnectionReason::NetworkDisconnectClientNoMap => {
                "NETWORK_DISCONNECT_CLIENT_NO_MAP"
            }
            ENetworkDisconnectionReason::NetworkDisconnectClientDifferentMap => {
                "NETWORK_DISCONNECT_CLIENT_DIFFERENT_MAP"
            }
            ENetworkDisconnectionReason::NetworkDisconnectServerRequiresSteam => {
                "NETWORK_DISCONNECT_SERVER_REQUIRES_STEAM"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamDenyMisc => {
                "NETWORK_DISCONNECT_STEAM_DENY_MISC"
            }
            ENetworkDisconnectionReason::NetworkDisconnectSteamDenyBadAntiCheat => {
                "NETWORK_DISCONNECT_STEAM_DENY_BAD_ANTI_CHEAT"
            }
            ENetworkDisconnectionReason::NetworkDisconnectServerShutdown => {
                "NETWORK_DISCONNECT_SERVER_SHUTDOWN"
            }
            ENetworkDisconnectionReason::NetworkDisconnectReplayIncompatible => {
                "NETWORK_DISCONNECT_REPLAY_INCOMPATIBLE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectConnectRequestTimedout => {
                "NETWORK_DISCONNECT_CONNECT_REQUEST_TIMEDOUT"
            }
            ENetworkDisconnectionReason::NetworkDisconnectServerIncompatible => {
                "NETWORK_DISCONNECT_SERVER_INCOMPATIBLE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectLocalproblemManyrelays => {
                "NETWORK_DISCONNECT_LOCALPROBLEM_MANYRELAYS"
            }
            ENetworkDisconnectionReason::NetworkDisconnectLocalproblemHostedserverprimaryrelay => {
                "NETWORK_DISCONNECT_LOCALPROBLEM_HOSTEDSERVERPRIMARYRELAY"
            }
            ENetworkDisconnectionReason::NetworkDisconnectLocalproblemNetworkconfig => {
                "NETWORK_DISCONNECT_LOCALPROBLEM_NETWORKCONFIG"
            }
            ENetworkDisconnectionReason::NetworkDisconnectLocalproblemOther => {
                "NETWORK_DISCONNECT_LOCALPROBLEM_OTHER"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRemoteTimeout => {
                "NETWORK_DISCONNECT_REMOTE_TIMEOUT"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRemoteTimeoutConnecting => {
                "NETWORK_DISCONNECT_REMOTE_TIMEOUT_CONNECTING"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRemoteOther => {
                "NETWORK_DISCONNECT_REMOTE_OTHER"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRemoteBadcrypt => {
                "NETWORK_DISCONNECT_REMOTE_BADCRYPT"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRemoteCertnottrusted => {
                "NETWORK_DISCONNECT_REMOTE_CERTNOTTRUSTED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectUnusual => {
                "NETWORK_DISCONNECT_UNUSUAL"
            }
            ENetworkDisconnectionReason::NetworkDisconnectInternalError => {
                "NETWORK_DISCONNECT_INTERNAL_ERROR"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectBadchallenge => {
                "NETWORK_DISCONNECT_REJECT_BADCHALLENGE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectNolobby => {
                "NETWORK_DISCONNECT_REJECT_NOLOBBY"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectBackgroundMap => {
                "NETWORK_DISCONNECT_REJECT_BACKGROUND_MAP"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectSinglePlayer => {
                "NETWORK_DISCONNECT_REJECT_SINGLE_PLAYER"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectHiddenGame => {
                "NETWORK_DISCONNECT_REJECT_HIDDEN_GAME"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectLanrestrict => {
                "NETWORK_DISCONNECT_REJECT_LANRESTRICT"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectBadpassword => {
                "NETWORK_DISCONNECT_REJECT_BADPASSWORD"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectServerfull => {
                "NETWORK_DISCONNECT_REJECT_SERVERFULL"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectInvalidreservation => {
                "NETWORK_DISCONNECT_REJECT_INVALIDRESERVATION"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectFailedchannel => {
                "NETWORK_DISCONNECT_REJECT_FAILEDCHANNEL"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectConnectFromLobby => {
                "NETWORK_DISCONNECT_REJECT_CONNECT_FROM_LOBBY"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectReservedForLobby => {
                "NETWORK_DISCONNECT_REJECT_RESERVED_FOR_LOBBY"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectInvalidkeylength => {
                "NETWORK_DISCONNECT_REJECT_INVALIDKEYLENGTH"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectOldprotocol => {
                "NETWORK_DISCONNECT_REJECT_OLDPROTOCOL"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectNewprotocol => {
                "NETWORK_DISCONNECT_REJECT_NEWPROTOCOL"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectInvalidconnection => {
                "NETWORK_DISCONNECT_REJECT_INVALIDCONNECTION"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectInvalidcertlen => {
                "NETWORK_DISCONNECT_REJECT_INVALIDCERTLEN"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectInvalidsteamcertlen => {
                "NETWORK_DISCONNECT_REJECT_INVALIDSTEAMCERTLEN"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectSteam => {
                "NETWORK_DISCONNECT_REJECT_STEAM"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectServerauthdisabled => {
                "NETWORK_DISCONNECT_REJECT_SERVERAUTHDISABLED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectServercdkeyauthinvalid => {
                "NETWORK_DISCONNECT_REJECT_SERVERCDKEYAUTHINVALID"
            }
            ENetworkDisconnectionReason::NetworkDisconnectRejectBanned => {
                "NETWORK_DISCONNECT_REJECT_BANNED"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickedTeamkilling => {
                "NETWORK_DISCONNECT_KICKED_TEAMKILLING"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickedTkStart => {
                "NETWORK_DISCONNECT_KICKED_TK_START"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickedUntrustedaccount => {
                "NETWORK_DISCONNECT_KICKED_UNTRUSTEDACCOUNT"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickedConvictedaccount => {
                "NETWORK_DISCONNECT_KICKED_CONVICTEDACCOUNT"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickedCompetitivecooldown => {
                "NETWORK_DISCONNECT_KICKED_COMPETITIVECOOLDOWN"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickedTeamhurting => {
                "NETWORK_DISCONNECT_KICKED_TEAMHURTING"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickedHostagekilling => {
                "NETWORK_DISCONNECT_KICKED_HOSTAGEKILLING"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickedVotedoff => {
                "NETWORK_DISCONNECT_KICKED_VOTEDOFF"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickedIdle => {
                "NETWORK_DISCONNECT_KICKED_IDLE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickedSuicide => {
                "NETWORK_DISCONNECT_KICKED_SUICIDE"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickedNosteamlogin => {
                "NETWORK_DISCONNECT_KICKED_NOSTEAMLOGIN"
            }
            ENetworkDisconnectionReason::NetworkDisconnectKickedNosteamticket => {
                "NETWORK_DISCONNECT_KICKED_NOSTEAMTICKET"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NETWORK_DISCONNECT_INVALID" => Some(Self::NetworkDisconnectInvalid),
            "NETWORK_DISCONNECT_SHUTDOWN" => Some(Self::NetworkDisconnectShutdown),
            "NETWORK_DISCONNECT_DISCONNECT_BY_USER" => {
                Some(Self::NetworkDisconnectDisconnectByUser)
            }
            "NETWORK_DISCONNECT_DISCONNECT_BY_SERVER" => {
                Some(Self::NetworkDisconnectDisconnectByServer)
            }
            "NETWORK_DISCONNECT_LOST" => Some(Self::NetworkDisconnectLost),
            "NETWORK_DISCONNECT_OVERFLOW" => Some(Self::NetworkDisconnectOverflow),
            "NETWORK_DISCONNECT_STEAM_BANNED" => Some(Self::NetworkDisconnectSteamBanned),
            "NETWORK_DISCONNECT_STEAM_INUSE" => Some(Self::NetworkDisconnectSteamInuse),
            "NETWORK_DISCONNECT_STEAM_TICKET" => Some(Self::NetworkDisconnectSteamTicket),
            "NETWORK_DISCONNECT_STEAM_LOGON" => Some(Self::NetworkDisconnectSteamLogon),
            "NETWORK_DISCONNECT_STEAM_AUTHCANCELLED" => {
                Some(Self::NetworkDisconnectSteamAuthcancelled)
            }
            "NETWORK_DISCONNECT_STEAM_AUTHALREADYUSED" => {
                Some(Self::NetworkDisconnectSteamAuthalreadyused)
            }
            "NETWORK_DISCONNECT_STEAM_AUTHINVALID" => {
                Some(Self::NetworkDisconnectSteamAuthinvalid)
            }
            "NETWORK_DISCONNECT_STEAM_VACBANSTATE" => {
                Some(Self::NetworkDisconnectSteamVacbanstate)
            }
            "NETWORK_DISCONNECT_STEAM_LOGGED_IN_ELSEWHERE" => {
                Some(Self::NetworkDisconnectSteamLoggedInElsewhere)
            }
            "NETWORK_DISCONNECT_STEAM_VAC_CHECK_TIMEDOUT" => {
                Some(Self::NetworkDisconnectSteamVacCheckTimedout)
            }
            "NETWORK_DISCONNECT_STEAM_DROPPED" => {
                Some(Self::NetworkDisconnectSteamDropped)
            }
            "NETWORK_DISCONNECT_STEAM_OWNERSHIP" => {
                Some(Self::NetworkDisconnectSteamOwnership)
            }
            "NETWORK_DISCONNECT_SERVERINFO_OVERFLOW" => {
                Some(Self::NetworkDisconnectServerinfoOverflow)
            }
            "NETWORK_DISCONNECT_TICKMSG_OVERFLOW" => {
                Some(Self::NetworkDisconnectTickmsgOverflow)
            }
            "NETWORK_DISCONNECT_STRINGTABLEMSG_OVERFLOW" => {
                Some(Self::NetworkDisconnectStringtablemsgOverflow)
            }
            "NETWORK_DISCONNECT_DELTAENTMSG_OVERFLOW" => {
                Some(Self::NetworkDisconnectDeltaentmsgOverflow)
            }
            "NETWORK_DISCONNECT_TEMPENTMSG_OVERFLOW" => {
                Some(Self::NetworkDisconnectTempentmsgOverflow)
            }
            "NETWORK_DISCONNECT_SOUNDSMSG_OVERFLOW" => {
                Some(Self::NetworkDisconnectSoundsmsgOverflow)
            }
            "NETWORK_DISCONNECT_SNAPSHOTOVERFLOW" => {
                Some(Self::NetworkDisconnectSnapshotoverflow)
            }
            "NETWORK_DISCONNECT_SNAPSHOTERROR" => {
                Some(Self::NetworkDisconnectSnapshoterror)
            }
            "NETWORK_DISCONNECT_RELIABLEOVERFLOW" => {
                Some(Self::NetworkDisconnectReliableoverflow)
            }
            "NETWORK_DISCONNECT_BADDELTATICK" => {
                Some(Self::NetworkDisconnectBaddeltatick)
            }
            "NETWORK_DISCONNECT_NOMORESPLITS" => {
                Some(Self::NetworkDisconnectNomoresplits)
            }
            "NETWORK_DISCONNECT_TIMEDOUT" => Some(Self::NetworkDisconnectTimedout),
            "NETWORK_DISCONNECT_DISCONNECTED" => {
                Some(Self::NetworkDisconnectDisconnected)
            }
            "NETWORK_DISCONNECT_LEAVINGSPLIT" => {
                Some(Self::NetworkDisconnectLeavingsplit)
            }
            "NETWORK_DISCONNECT_DIFFERENTCLASSTABLES" => {
                Some(Self::NetworkDisconnectDifferentclasstables)
            }
            "NETWORK_DISCONNECT_BADRELAYPASSWORD" => {
                Some(Self::NetworkDisconnectBadrelaypassword)
            }
            "NETWORK_DISCONNECT_BADSPECTATORPASSWORD" => {
                Some(Self::NetworkDisconnectBadspectatorpassword)
            }
            "NETWORK_DISCONNECT_HLTVRESTRICTED" => {
                Some(Self::NetworkDisconnectHltvrestricted)
            }
            "NETWORK_DISCONNECT_NOSPECTATORS" => {
                Some(Self::NetworkDisconnectNospectators)
            }
            "NETWORK_DISCONNECT_HLTVUNAVAILABLE" => {
                Some(Self::NetworkDisconnectHltvunavailable)
            }
            "NETWORK_DISCONNECT_HLTVSTOP" => Some(Self::NetworkDisconnectHltvstop),
            "NETWORK_DISCONNECT_KICKED" => Some(Self::NetworkDisconnectKicked),
            "NETWORK_DISCONNECT_BANADDED" => Some(Self::NetworkDisconnectBanadded),
            "NETWORK_DISCONNECT_KICKBANADDED" => {
                Some(Self::NetworkDisconnectKickbanadded)
            }
            "NETWORK_DISCONNECT_HLTVDIRECT" => Some(Self::NetworkDisconnectHltvdirect),
            "NETWORK_DISCONNECT_PURESERVER_CLIENTEXTRA" => {
                Some(Self::NetworkDisconnectPureserverClientextra)
            }
            "NETWORK_DISCONNECT_PURESERVER_MISMATCH" => {
                Some(Self::NetworkDisconnectPureserverMismatch)
            }
            "NETWORK_DISCONNECT_USERCMD" => Some(Self::NetworkDisconnectUsercmd),
            "NETWORK_DISCONNECT_REJECTED_BY_GAME" => {
                Some(Self::NetworkDisconnectRejectedByGame)
            }
            "NETWORK_DISCONNECT_MESSAGE_PARSE_ERROR" => {
                Some(Self::NetworkDisconnectMessageParseError)
            }
            "NETWORK_DISCONNECT_INVALID_MESSAGE_ERROR" => {
                Some(Self::NetworkDisconnectInvalidMessageError)
            }
            "NETWORK_DISCONNECT_BAD_SERVER_PASSWORD" => {
                Some(Self::NetworkDisconnectBadServerPassword)
            }
            "NETWORK_DISCONNECT_DIRECT_CONNECT_RESERVATION" => {
                Some(Self::NetworkDisconnectDirectConnectReservation)
            }
            "NETWORK_DISCONNECT_CONNECTION_FAILURE" => {
                Some(Self::NetworkDisconnectConnectionFailure)
            }
            "NETWORK_DISCONNECT_NO_PEER_GROUP_HANDLERS" => {
                Some(Self::NetworkDisconnectNoPeerGroupHandlers)
            }
            "NETWORK_DISCONNECT_RECONNECTION" => {
                Some(Self::NetworkDisconnectReconnection)
            }
            "NETWORK_DISCONNECT_LOOPSHUTDOWN" => {
                Some(Self::NetworkDisconnectLoopshutdown)
            }
            "NETWORK_DISCONNECT_LOOPDEACTIVATE" => {
                Some(Self::NetworkDisconnectLoopdeactivate)
            }
            "NETWORK_DISCONNECT_HOST_ENDGAME" => Some(Self::NetworkDisconnectHostEndgame),
            "NETWORK_DISCONNECT_LOOP_LEVELLOAD_ACTIVATE" => {
                Some(Self::NetworkDisconnectLoopLevelloadActivate)
            }
            "NETWORK_DISCONNECT_CREATE_SERVER_FAILED" => {
                Some(Self::NetworkDisconnectCreateServerFailed)
            }
            "NETWORK_DISCONNECT_EXITING" => Some(Self::NetworkDisconnectExiting),
            "NETWORK_DISCONNECT_REQUEST_HOSTSTATE_IDLE" => {
                Some(Self::NetworkDisconnectRequestHoststateIdle)
            }
            "NETWORK_DISCONNECT_REQUEST_HOSTSTATE_HLTVRELAY" => {
                Some(Self::NetworkDisconnectRequestHoststateHltvrelay)
            }
            "NETWORK_DISCONNECT_CLIENT_CONSISTENCY_FAIL" => {
                Some(Self::NetworkDisconnectClientConsistencyFail)
            }
            "NETWORK_DISCONNECT_CLIENT_UNABLE_TO_CRC_MAP" => {
                Some(Self::NetworkDisconnectClientUnableToCrcMap)
            }
            "NETWORK_DISCONNECT_CLIENT_NO_MAP" => {
                Some(Self::NetworkDisconnectClientNoMap)
            }
            "NETWORK_DISCONNECT_CLIENT_DIFFERENT_MAP" => {
                Some(Self::NetworkDisconnectClientDifferentMap)
            }
            "NETWORK_DISCONNECT_SERVER_REQUIRES_STEAM" => {
                Some(Self::NetworkDisconnectServerRequiresSteam)
            }
            "NETWORK_DISCONNECT_STEAM_DENY_MISC" => {
                Some(Self::NetworkDisconnectSteamDenyMisc)
            }
            "NETWORK_DISCONNECT_STEAM_DENY_BAD_ANTI_CHEAT" => {
                Some(Self::NetworkDisconnectSteamDenyBadAntiCheat)
            }
            "NETWORK_DISCONNECT_SERVER_SHUTDOWN" => {
                Some(Self::NetworkDisconnectServerShutdown)
            }
            "NETWORK_DISCONNECT_REPLAY_INCOMPATIBLE" => {
                Some(Self::NetworkDisconnectReplayIncompatible)
            }
            "NETWORK_DISCONNECT_CONNECT_REQUEST_TIMEDOUT" => {
                Some(Self::NetworkDisconnectConnectRequestTimedout)
            }
            "NETWORK_DISCONNECT_SERVER_INCOMPATIBLE" => {
                Some(Self::NetworkDisconnectServerIncompatible)
            }
            "NETWORK_DISCONNECT_LOCALPROBLEM_MANYRELAYS" => {
                Some(Self::NetworkDisconnectLocalproblemManyrelays)
            }
            "NETWORK_DISCONNECT_LOCALPROBLEM_HOSTEDSERVERPRIMARYRELAY" => {
                Some(Self::NetworkDisconnectLocalproblemHostedserverprimaryrelay)
            }
            "NETWORK_DISCONNECT_LOCALPROBLEM_NETWORKCONFIG" => {
                Some(Self::NetworkDisconnectLocalproblemNetworkconfig)
            }
            "NETWORK_DISCONNECT_LOCALPROBLEM_OTHER" => {
                Some(Self::NetworkDisconnectLocalproblemOther)
            }
            "NETWORK_DISCONNECT_REMOTE_TIMEOUT" => {
                Some(Self::NetworkDisconnectRemoteTimeout)
            }
            "NETWORK_DISCONNECT_REMOTE_TIMEOUT_CONNECTING" => {
                Some(Self::NetworkDisconnectRemoteTimeoutConnecting)
            }
            "NETWORK_DISCONNECT_REMOTE_OTHER" => Some(Self::NetworkDisconnectRemoteOther),
            "NETWORK_DISCONNECT_REMOTE_BADCRYPT" => {
                Some(Self::NetworkDisconnectRemoteBadcrypt)
            }
            "NETWORK_DISCONNECT_REMOTE_CERTNOTTRUSTED" => {
                Some(Self::NetworkDisconnectRemoteCertnottrusted)
            }
            "NETWORK_DISCONNECT_UNUSUAL" => Some(Self::NetworkDisconnectUnusual),
            "NETWORK_DISCONNECT_INTERNAL_ERROR" => {
                Some(Self::NetworkDisconnectInternalError)
            }
            "NETWORK_DISCONNECT_REJECT_BADCHALLENGE" => {
                Some(Self::NetworkDisconnectRejectBadchallenge)
            }
            "NETWORK_DISCONNECT_REJECT_NOLOBBY" => {
                Some(Self::NetworkDisconnectRejectNolobby)
            }
            "NETWORK_DISCONNECT_REJECT_BACKGROUND_MAP" => {
                Some(Self::NetworkDisconnectRejectBackgroundMap)
            }
            "NETWORK_DISCONNECT_REJECT_SINGLE_PLAYER" => {
                Some(Self::NetworkDisconnectRejectSinglePlayer)
            }
            "NETWORK_DISCONNECT_REJECT_HIDDEN_GAME" => {
                Some(Self::NetworkDisconnectRejectHiddenGame)
            }
            "NETWORK_DISCONNECT_REJECT_LANRESTRICT" => {
                Some(Self::NetworkDisconnectRejectLanrestrict)
            }
            "NETWORK_DISCONNECT_REJECT_BADPASSWORD" => {
                Some(Self::NetworkDisconnectRejectBadpassword)
            }
            "NETWORK_DISCONNECT_REJECT_SERVERFULL" => {
                Some(Self::NetworkDisconnectRejectServerfull)
            }
            "NETWORK_DISCONNECT_REJECT_INVALIDRESERVATION" => {
                Some(Self::NetworkDisconnectRejectInvalidreservation)
            }
            "NETWORK_DISCONNECT_REJECT_FAILEDCHANNEL" => {
                Some(Self::NetworkDisconnectRejectFailedchannel)
            }
            "NETWORK_DISCONNECT_REJECT_CONNECT_FROM_LOBBY" => {
                Some(Self::NetworkDisconnectRejectConnectFromLobby)
            }
            "NETWORK_DISCONNECT_REJECT_RESERVED_FOR_LOBBY" => {
                Some(Self::NetworkDisconnectRejectReservedForLobby)
            }
            "NETWORK_DISCONNECT_REJECT_INVALIDKEYLENGTH" => {
                Some(Self::NetworkDisconnectRejectInvalidkeylength)
            }
            "NETWORK_DISCONNECT_REJECT_OLDPROTOCOL" => {
                Some(Self::NetworkDisconnectRejectOldprotocol)
            }
            "NETWORK_DISCONNECT_REJECT_NEWPROTOCOL" => {
                Some(Self::NetworkDisconnectRejectNewprotocol)
            }
            "NETWORK_DISCONNECT_REJECT_INVALIDCONNECTION" => {
                Some(Self::NetworkDisconnectRejectInvalidconnection)
            }
            "NETWORK_DISCONNECT_REJECT_INVALIDCERTLEN" => {
                Some(Self::NetworkDisconnectRejectInvalidcertlen)
            }
            "NETWORK_DISCONNECT_REJECT_INVALIDSTEAMCERTLEN" => {
                Some(Self::NetworkDisconnectRejectInvalidsteamcertlen)
            }
            "NETWORK_DISCONNECT_REJECT_STEAM" => Some(Self::NetworkDisconnectRejectSteam),
            "NETWORK_DISCONNECT_REJECT_SERVERAUTHDISABLED" => {
                Some(Self::NetworkDisconnectRejectServerauthdisabled)
            }
            "NETWORK_DISCONNECT_REJECT_SERVERCDKEYAUTHINVALID" => {
                Some(Self::NetworkDisconnectRejectServercdkeyauthinvalid)
            }
            "NETWORK_DISCONNECT_REJECT_BANNED" => {
                Some(Self::NetworkDisconnectRejectBanned)
            }
            "NETWORK_DISCONNECT_KICKED_TEAMKILLING" => {
                Some(Self::NetworkDisconnectKickedTeamkilling)
            }
            "NETWORK_DISCONNECT_KICKED_TK_START" => {
                Some(Self::NetworkDisconnectKickedTkStart)
            }
            "NETWORK_DISCONNECT_KICKED_UNTRUSTEDACCOUNT" => {
                Some(Self::NetworkDisconnectKickedUntrustedaccount)
            }
            "NETWORK_DISCONNECT_KICKED_CONVICTEDACCOUNT" => {
                Some(Self::NetworkDisconnectKickedConvictedaccount)
            }
            "NETWORK_DISCONNECT_KICKED_COMPETITIVECOOLDOWN" => {
                Some(Self::NetworkDisconnectKickedCompetitivecooldown)
            }
            "NETWORK_DISCONNECT_KICKED_TEAMHURTING" => {
                Some(Self::NetworkDisconnectKickedTeamhurting)
            }
            "NETWORK_DISCONNECT_KICKED_HOSTAGEKILLING" => {
                Some(Self::NetworkDisconnectKickedHostagekilling)
            }
            "NETWORK_DISCONNECT_KICKED_VOTEDOFF" => {
                Some(Self::NetworkDisconnectKickedVotedoff)
            }
            "NETWORK_DISCONNECT_KICKED_IDLE" => Some(Self::NetworkDisconnectKickedIdle),
            "NETWORK_DISCONNECT_KICKED_SUICIDE" => {
                Some(Self::NetworkDisconnectKickedSuicide)
            }
            "NETWORK_DISCONNECT_KICKED_NOSTEAMLOGIN" => {
                Some(Self::NetworkDisconnectKickedNosteamlogin)
            }
            "NETWORK_DISCONNECT_KICKED_NOSTEAMTICKET" => {
                Some(Self::NetworkDisconnectKickedNosteamticket)
            }
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgVector2D {
    #[prost(float, optional, tag = "1")]
    pub x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgQAngle {
    #[prost(float, optional, tag = "1")]
    pub x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub z: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgTransform {
    #[prost(message, optional, tag = "1")]
    pub position: ::core::option::Option<CMsgVector>,
    #[prost(float, optional, tag = "2")]
    pub scale: ::core::option::Option<f32>,
    #[prost(message, optional, tag = "3")]
    pub orientation: ::core::option::Option<CMsgQuaternion>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CEntityMsg {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub target_entity: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgCVars {
    #[prost(message, repeated, tag = "1")]
    pub cvars: ::prost::alloc::vec::Vec<c_msg_c_vars::CVar>,
}
/// Nested message and enum types in `CMsg_CVars`.
pub mod c_msg_c_vars {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CVar {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgNop {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgSplitScreenUser {
    #[prost(int32, optional, tag = "1")]
    pub slot: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgDisconnect {
    #[prost(
        enumeration = "ENetworkDisconnectionReason",
        optional,
        tag = "2",
        default = "NetworkDisconnectInvalid"
    )]
    pub reason: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgTick {
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
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgStringCmd {
    #[prost(string, optional, tag = "1")]
    pub command: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "2")]
    pub prediction_sync: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgSetConVar {
    #[prost(message, optional, tag = "1")]
    pub convars: ::core::option::Option<CMsgCVars>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgSignonState {
    #[prost(
        enumeration = "SignonStateT",
        optional,
        tag = "1",
        default = "SignonstateNone"
    )]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgGameEvent {
    #[prost(string, optional, tag = "1")]
    pub event_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub eventid: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub keys: ::prost::alloc::vec::Vec<csvc_msg_game_event::KeyT>,
}
/// Nested message and enum types in `CSVCMsg_GameEvent`.
pub mod csvc_msg_game_event {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgListGameEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<csvc_msg_list_game_events::EventT>,
}
/// Nested message and enum types in `CSVCMsgList_GameEvents`.
pub mod csvc_msg_list_game_events {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EventT {
        #[prost(int32, optional, tag = "1")]
        pub tick: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub event: ::core::option::Option<super::CsvcMsgGameEvent>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgSpawnGroupLoad {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgSpawnGroupManifestUpdate {
    #[prost(uint32, optional, tag = "1")]
    pub spawngrouphandle: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub spawngroupmanifest: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "3")]
    pub manifestincomplete: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgSpawnGroupSetCreationTick {
    #[prost(uint32, optional, tag = "1")]
    pub spawngrouphandle: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub tickcount: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub creationsequence: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgSpawnGroupUnload {
    #[prost(uint32, optional, tag = "1")]
    pub spawngrouphandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub flags: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub tickcount: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgSpawnGroupLoadCompleted {
    #[prost(uint32, optional, tag = "1")]
    pub spawngrouphandle: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgGameSessionConfiguration {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CnetMsgDebugOverlay {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl SignonStateT {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignonStateT::SignonstateNone => "SIGNONSTATE_NONE",
            SignonStateT::SignonstateChallenge => "SIGNONSTATE_CHALLENGE",
            SignonStateT::SignonstateConnected => "SIGNONSTATE_CONNECTED",
            SignonStateT::SignonstateNew => "SIGNONSTATE_NEW",
            SignonStateT::SignonstatePrespawn => "SIGNONSTATE_PRESPAWN",
            SignonStateT::SignonstateSpawn => "SIGNONSTATE_SPAWN",
            SignonStateT::SignonstateFull => "SIGNONSTATE_FULL",
            SignonStateT::SignonstateChangelevel => "SIGNONSTATE_CHANGELEVEL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGNONSTATE_NONE" => Some(Self::SignonstateNone),
            "SIGNONSTATE_CHALLENGE" => Some(Self::SignonstateChallenge),
            "SIGNONSTATE_CONNECTED" => Some(Self::SignonstateConnected),
            "SIGNONSTATE_NEW" => Some(Self::SignonstateNew),
            "SIGNONSTATE_PRESPAWN" => Some(Self::SignonstatePrespawn),
            "SIGNONSTATE_SPAWN" => Some(Self::SignonstateSpawn),
            "SIGNONSTATE_FULL" => Some(Self::SignonstateFull),
            "SIGNONSTATE_CHANGELEVEL" => Some(Self::SignonstateChangelevel),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NetMessages {
    NetNop = 0,
    NetDisconnect = 1,
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
impl NetMessages {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NetMessages::NetNop => "net_NOP",
            NetMessages::NetDisconnect => "net_Disconnect",
            NetMessages::NetSplitScreenUser => "net_SplitScreenUser",
            NetMessages::NetTick => "net_Tick",
            NetMessages::NetStringCmd => "net_StringCmd",
            NetMessages::NetSetConVar => "net_SetConVar",
            NetMessages::NetSignonState => "net_SignonState",
            NetMessages::NetSpawnGroupLoad => "net_SpawnGroup_Load",
            NetMessages::NetSpawnGroupManifestUpdate => "net_SpawnGroup_ManifestUpdate",
            NetMessages::NetSpawnGroupSetCreationTick => "net_SpawnGroup_SetCreationTick",
            NetMessages::NetSpawnGroupUnload => "net_SpawnGroup_Unload",
            NetMessages::NetSpawnGroupLoadCompleted => "net_SpawnGroup_LoadCompleted",
            NetMessages::NetDebugOverlay => "net_DebugOverlay",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "net_NOP" => Some(Self::NetNop),
            "net_Disconnect" => Some(Self::NetDisconnect),
            "net_SplitScreenUser" => Some(Self::NetSplitScreenUser),
            "net_Tick" => Some(Self::NetTick),
            "net_StringCmd" => Some(Self::NetStringCmd),
            "net_SetConVar" => Some(Self::NetSetConVar),
            "net_SignonState" => Some(Self::NetSignonState),
            "net_SpawnGroup_Load" => Some(Self::NetSpawnGroupLoad),
            "net_SpawnGroup_ManifestUpdate" => Some(Self::NetSpawnGroupManifestUpdate),
            "net_SpawnGroup_SetCreationTick" => Some(Self::NetSpawnGroupSetCreationTick),
            "net_SpawnGroup_Unload" => Some(Self::NetSpawnGroupUnload),
            "net_SpawnGroup_LoadCompleted" => Some(Self::NetSpawnGroupLoadCompleted),
            "net_DebugOverlay" => Some(Self::NetDebugOverlay),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl SpawnGroupFlagsT {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SpawnGroupFlagsT::SpawnGroupLoadEntitiesFromSave => {
                "SPAWN_GROUP_LOAD_ENTITIES_FROM_SAVE"
            }
            SpawnGroupFlagsT::SpawnGroupDontSpawnEntities => {
                "SPAWN_GROUP_DONT_SPAWN_ENTITIES"
            }
            SpawnGroupFlagsT::SpawnGroupSynchronousSpawn => {
                "SPAWN_GROUP_SYNCHRONOUS_SPAWN"
            }
            SpawnGroupFlagsT::SpawnGroupIsInitialSpawnGroup => {
                "SPAWN_GROUP_IS_INITIAL_SPAWN_GROUP"
            }
            SpawnGroupFlagsT::SpawnGroupCreateClientOnlyEntities => {
                "SPAWN_GROUP_CREATE_CLIENT_ONLY_ENTITIES"
            }
            SpawnGroupFlagsT::SpawnGroupBlockUntilLoaded => {
                "SPAWN_GROUP_BLOCK_UNTIL_LOADED"
            }
            SpawnGroupFlagsT::SpawnGroupLoadStreamingData => {
                "SPAWN_GROUP_LOAD_STREAMING_DATA"
            }
            SpawnGroupFlagsT::SpawnGroupCreateNewSceneWorld => {
                "SPAWN_GROUP_CREATE_NEW_SCENE_WORLD"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SPAWN_GROUP_LOAD_ENTITIES_FROM_SAVE" => {
                Some(Self::SpawnGroupLoadEntitiesFromSave)
            }
            "SPAWN_GROUP_DONT_SPAWN_ENTITIES" => Some(Self::SpawnGroupDontSpawnEntities),
            "SPAWN_GROUP_SYNCHRONOUS_SPAWN" => Some(Self::SpawnGroupSynchronousSpawn),
            "SPAWN_GROUP_IS_INITIAL_SPAWN_GROUP" => {
                Some(Self::SpawnGroupIsInitialSpawnGroup)
            }
            "SPAWN_GROUP_CREATE_CLIENT_ONLY_ENTITIES" => {
                Some(Self::SpawnGroupCreateClientOnlyEntities)
            }
            "SPAWN_GROUP_BLOCK_UNTIL_LOADED" => Some(Self::SpawnGroupBlockUntilLoaded),
            "SPAWN_GROUP_LOAD_STREAMING_DATA" => Some(Self::SpawnGroupLoadStreamingData),
            "SPAWN_GROUP_CREATE_NEW_SCENE_WORLD" => {
                Some(Self::SpawnGroupCreateNewSceneWorld)
            }
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaMsgLocationPing {
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
    #[prost(
        enumeration = "EPingSource",
        optional,
        tag = "6",
        default = "KEPingSourceDefault"
    )]
    pub ping_source: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaMsgItemAlert {
    #[prost(int32, optional, tag = "1")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub y: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaMsgMapLine {
    #[prost(int32, optional, tag = "1")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub y: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub initial: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaMsgWorldLine {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaMsgSendStatPopup {
    #[prost(
        enumeration = "EdotaStatPopupTypes",
        optional,
        tag = "1",
        default = "KEdotaSptTextline"
    )]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaMsgDismissAllStatPopups {
    #[prost(float, optional, tag = "1")]
    pub time_delay: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaMsgCoachHudPing {
    #[prost(uint32, optional, tag = "1")]
    pub x: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub y: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub tgtpath: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaMsgUnitOrder {
    #[prost(
        enumeration = "DotaunitorderT",
        optional,
        tag = "2",
        default = "DotaUnitOrderNone"
    )]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersusScenePlayActivity {
    #[prost(message, repeated, tag = "1")]
    pub activities: ::prost::alloc::vec::Vec<versus_scene_play_activity::ActivityInfo>,
    #[prost(float, optional, tag = "2")]
    pub playback_rate: ::core::option::Option<f32>,
}
/// Nested message and enum types in `VersusScene_PlayActivity`.
pub mod versus_scene_play_activity {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersusSceneChatWheel {
    #[prost(uint32, optional, tag = "1", default = "4294967295")]
    pub chat_message_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub emoticon_id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersusScenePlaybackRate {
    #[prost(float, optional, tag = "1")]
    pub rate: ::core::option::Option<f32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EPingSource {
    KEPingSourceDefault = 0,
    KEPingSourceWarning = 1,
    KEPingSourceWheel = 2,
    KEPingSourceSystem = 3,
}
impl EPingSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EPingSource::KEPingSourceDefault => "k_ePingSource_Default",
            EPingSource::KEPingSourceWarning => "k_ePingSource_Warning",
            EPingSource::KEPingSourceWheel => "k_ePingSource_Wheel",
            EPingSource::KEPingSourceSystem => "k_ePingSource_System",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_ePingSource_Default" => Some(Self::KEPingSourceDefault),
            "k_ePingSource_Warning" => Some(Self::KEPingSourceWarning),
            "k_ePingSource_Wheel" => Some(Self::KEPingSourceWheel),
            "k_ePingSource_System" => Some(Self::KEPingSourceSystem),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EdotaStatPopupTypes {
    KEdotaSptTextline = 0,
    KEdotaSptBasic = 1,
    KEdotaSptPoll = 2,
    KEdotaSptGrid = 3,
    KEdotaSptDualImage = 4,
    KEdotaSptMovie = 5,
}
impl EdotaStatPopupTypes {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EdotaStatPopupTypes::KEdotaSptTextline => "k_EDOTA_SPT_Textline",
            EdotaStatPopupTypes::KEdotaSptBasic => "k_EDOTA_SPT_Basic",
            EdotaStatPopupTypes::KEdotaSptPoll => "k_EDOTA_SPT_Poll",
            EdotaStatPopupTypes::KEdotaSptGrid => "k_EDOTA_SPT_Grid",
            EdotaStatPopupTypes::KEdotaSptDualImage => "k_EDOTA_SPT_DualImage",
            EdotaStatPopupTypes::KEdotaSptMovie => "k_EDOTA_SPT_Movie",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_EDOTA_SPT_Textline" => Some(Self::KEdotaSptTextline),
            "k_EDOTA_SPT_Basic" => Some(Self::KEdotaSptBasic),
            "k_EDOTA_SPT_Poll" => Some(Self::KEdotaSptPoll),
            "k_EDOTA_SPT_Grid" => Some(Self::KEdotaSptGrid),
            "k_EDOTA_SPT_DualImage" => Some(Self::KEdotaSptDualImage),
            "k_EDOTA_SPT_Movie" => Some(Self::KEdotaSptMovie),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
impl DotaunitorderT {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaunitorderT::DotaUnitOrderNone => "DOTA_UNIT_ORDER_NONE",
            DotaunitorderT::DotaUnitOrderMoveToPosition => {
                "DOTA_UNIT_ORDER_MOVE_TO_POSITION"
            }
            DotaunitorderT::DotaUnitOrderMoveToTarget => "DOTA_UNIT_ORDER_MOVE_TO_TARGET",
            DotaunitorderT::DotaUnitOrderAttackMove => "DOTA_UNIT_ORDER_ATTACK_MOVE",
            DotaunitorderT::DotaUnitOrderAttackTarget => "DOTA_UNIT_ORDER_ATTACK_TARGET",
            DotaunitorderT::DotaUnitOrderCastPosition => "DOTA_UNIT_ORDER_CAST_POSITION",
            DotaunitorderT::DotaUnitOrderCastTarget => "DOTA_UNIT_ORDER_CAST_TARGET",
            DotaunitorderT::DotaUnitOrderCastTargetTree => {
                "DOTA_UNIT_ORDER_CAST_TARGET_TREE"
            }
            DotaunitorderT::DotaUnitOrderCastNoTarget => "DOTA_UNIT_ORDER_CAST_NO_TARGET",
            DotaunitorderT::DotaUnitOrderCastToggle => "DOTA_UNIT_ORDER_CAST_TOGGLE",
            DotaunitorderT::DotaUnitOrderHoldPosition => "DOTA_UNIT_ORDER_HOLD_POSITION",
            DotaunitorderT::DotaUnitOrderTrainAbility => "DOTA_UNIT_ORDER_TRAIN_ABILITY",
            DotaunitorderT::DotaUnitOrderDropItem => "DOTA_UNIT_ORDER_DROP_ITEM",
            DotaunitorderT::DotaUnitOrderGiveItem => "DOTA_UNIT_ORDER_GIVE_ITEM",
            DotaunitorderT::DotaUnitOrderPickupItem => "DOTA_UNIT_ORDER_PICKUP_ITEM",
            DotaunitorderT::DotaUnitOrderPickupRune => "DOTA_UNIT_ORDER_PICKUP_RUNE",
            DotaunitorderT::DotaUnitOrderPurchaseItem => "DOTA_UNIT_ORDER_PURCHASE_ITEM",
            DotaunitorderT::DotaUnitOrderSellItem => "DOTA_UNIT_ORDER_SELL_ITEM",
            DotaunitorderT::DotaUnitOrderDisassembleItem => {
                "DOTA_UNIT_ORDER_DISASSEMBLE_ITEM"
            }
            DotaunitorderT::DotaUnitOrderMoveItem => "DOTA_UNIT_ORDER_MOVE_ITEM",
            DotaunitorderT::DotaUnitOrderCastToggleAuto => {
                "DOTA_UNIT_ORDER_CAST_TOGGLE_AUTO"
            }
            DotaunitorderT::DotaUnitOrderStop => "DOTA_UNIT_ORDER_STOP",
            DotaunitorderT::DotaUnitOrderTaunt => "DOTA_UNIT_ORDER_TAUNT",
            DotaunitorderT::DotaUnitOrderBuyback => "DOTA_UNIT_ORDER_BUYBACK",
            DotaunitorderT::DotaUnitOrderGlyph => "DOTA_UNIT_ORDER_GLYPH",
            DotaunitorderT::DotaUnitOrderEjectItemFromStash => {
                "DOTA_UNIT_ORDER_EJECT_ITEM_FROM_STASH"
            }
            DotaunitorderT::DotaUnitOrderCastRune => "DOTA_UNIT_ORDER_CAST_RUNE",
            DotaunitorderT::DotaUnitOrderPingAbility => "DOTA_UNIT_ORDER_PING_ABILITY",
            DotaunitorderT::DotaUnitOrderMoveToDirection => {
                "DOTA_UNIT_ORDER_MOVE_TO_DIRECTION"
            }
            DotaunitorderT::DotaUnitOrderPatrol => "DOTA_UNIT_ORDER_PATROL",
            DotaunitorderT::DotaUnitOrderVectorTargetPosition => {
                "DOTA_UNIT_ORDER_VECTOR_TARGET_POSITION"
            }
            DotaunitorderT::DotaUnitOrderRadar => "DOTA_UNIT_ORDER_RADAR",
            DotaunitorderT::DotaUnitOrderSetItemCombineLock => {
                "DOTA_UNIT_ORDER_SET_ITEM_COMBINE_LOCK"
            }
            DotaunitorderT::DotaUnitOrderContinue => "DOTA_UNIT_ORDER_CONTINUE",
            DotaunitorderT::DotaUnitOrderVectorTargetCanceled => {
                "DOTA_UNIT_ORDER_VECTOR_TARGET_CANCELED"
            }
            DotaunitorderT::DotaUnitOrderCastRiverPaint => {
                "DOTA_UNIT_ORDER_CAST_RIVER_PAINT"
            }
            DotaunitorderT::DotaUnitOrderPregameAdjustItemAssignment => {
                "DOTA_UNIT_ORDER_PREGAME_ADJUST_ITEM_ASSIGNMENT"
            }
            DotaunitorderT::DotaUnitOrderDropItemAtFountain => {
                "DOTA_UNIT_ORDER_DROP_ITEM_AT_FOUNTAIN"
            }
            DotaunitorderT::DotaUnitOrderTakeItemFromNeutralItemStash => {
                "DOTA_UNIT_ORDER_TAKE_ITEM_FROM_NEUTRAL_ITEM_STASH"
            }
            DotaunitorderT::DotaUnitOrderMoveRelative => "DOTA_UNIT_ORDER_MOVE_RELATIVE",
            DotaunitorderT::DotaUnitOrderCastToggleAlt => {
                "DOTA_UNIT_ORDER_CAST_TOGGLE_ALT"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_UNIT_ORDER_NONE" => Some(Self::DotaUnitOrderNone),
            "DOTA_UNIT_ORDER_MOVE_TO_POSITION" => Some(Self::DotaUnitOrderMoveToPosition),
            "DOTA_UNIT_ORDER_MOVE_TO_TARGET" => Some(Self::DotaUnitOrderMoveToTarget),
            "DOTA_UNIT_ORDER_ATTACK_MOVE" => Some(Self::DotaUnitOrderAttackMove),
            "DOTA_UNIT_ORDER_ATTACK_TARGET" => Some(Self::DotaUnitOrderAttackTarget),
            "DOTA_UNIT_ORDER_CAST_POSITION" => Some(Self::DotaUnitOrderCastPosition),
            "DOTA_UNIT_ORDER_CAST_TARGET" => Some(Self::DotaUnitOrderCastTarget),
            "DOTA_UNIT_ORDER_CAST_TARGET_TREE" => Some(Self::DotaUnitOrderCastTargetTree),
            "DOTA_UNIT_ORDER_CAST_NO_TARGET" => Some(Self::DotaUnitOrderCastNoTarget),
            "DOTA_UNIT_ORDER_CAST_TOGGLE" => Some(Self::DotaUnitOrderCastToggle),
            "DOTA_UNIT_ORDER_HOLD_POSITION" => Some(Self::DotaUnitOrderHoldPosition),
            "DOTA_UNIT_ORDER_TRAIN_ABILITY" => Some(Self::DotaUnitOrderTrainAbility),
            "DOTA_UNIT_ORDER_DROP_ITEM" => Some(Self::DotaUnitOrderDropItem),
            "DOTA_UNIT_ORDER_GIVE_ITEM" => Some(Self::DotaUnitOrderGiveItem),
            "DOTA_UNIT_ORDER_PICKUP_ITEM" => Some(Self::DotaUnitOrderPickupItem),
            "DOTA_UNIT_ORDER_PICKUP_RUNE" => Some(Self::DotaUnitOrderPickupRune),
            "DOTA_UNIT_ORDER_PURCHASE_ITEM" => Some(Self::DotaUnitOrderPurchaseItem),
            "DOTA_UNIT_ORDER_SELL_ITEM" => Some(Self::DotaUnitOrderSellItem),
            "DOTA_UNIT_ORDER_DISASSEMBLE_ITEM" => {
                Some(Self::DotaUnitOrderDisassembleItem)
            }
            "DOTA_UNIT_ORDER_MOVE_ITEM" => Some(Self::DotaUnitOrderMoveItem),
            "DOTA_UNIT_ORDER_CAST_TOGGLE_AUTO" => Some(Self::DotaUnitOrderCastToggleAuto),
            "DOTA_UNIT_ORDER_STOP" => Some(Self::DotaUnitOrderStop),
            "DOTA_UNIT_ORDER_TAUNT" => Some(Self::DotaUnitOrderTaunt),
            "DOTA_UNIT_ORDER_BUYBACK" => Some(Self::DotaUnitOrderBuyback),
            "DOTA_UNIT_ORDER_GLYPH" => Some(Self::DotaUnitOrderGlyph),
            "DOTA_UNIT_ORDER_EJECT_ITEM_FROM_STASH" => {
                Some(Self::DotaUnitOrderEjectItemFromStash)
            }
            "DOTA_UNIT_ORDER_CAST_RUNE" => Some(Self::DotaUnitOrderCastRune),
            "DOTA_UNIT_ORDER_PING_ABILITY" => Some(Self::DotaUnitOrderPingAbility),
            "DOTA_UNIT_ORDER_MOVE_TO_DIRECTION" => {
                Some(Self::DotaUnitOrderMoveToDirection)
            }
            "DOTA_UNIT_ORDER_PATROL" => Some(Self::DotaUnitOrderPatrol),
            "DOTA_UNIT_ORDER_VECTOR_TARGET_POSITION" => {
                Some(Self::DotaUnitOrderVectorTargetPosition)
            }
            "DOTA_UNIT_ORDER_RADAR" => Some(Self::DotaUnitOrderRadar),
            "DOTA_UNIT_ORDER_SET_ITEM_COMBINE_LOCK" => {
                Some(Self::DotaUnitOrderSetItemCombineLock)
            }
            "DOTA_UNIT_ORDER_CONTINUE" => Some(Self::DotaUnitOrderContinue),
            "DOTA_UNIT_ORDER_VECTOR_TARGET_CANCELED" => {
                Some(Self::DotaUnitOrderVectorTargetCanceled)
            }
            "DOTA_UNIT_ORDER_CAST_RIVER_PAINT" => Some(Self::DotaUnitOrderCastRiverPaint),
            "DOTA_UNIT_ORDER_PREGAME_ADJUST_ITEM_ASSIGNMENT" => {
                Some(Self::DotaUnitOrderPregameAdjustItemAssignment)
            }
            "DOTA_UNIT_ORDER_DROP_ITEM_AT_FOUNTAIN" => {
                Some(Self::DotaUnitOrderDropItemAtFountain)
            }
            "DOTA_UNIT_ORDER_TAKE_ITEM_FROM_NEUTRAL_ITEM_STASH" => {
                Some(Self::DotaUnitOrderTakeItemFromNeutralItemStash)
            }
            "DOTA_UNIT_ORDER_MOVE_RELATIVE" => Some(Self::DotaUnitOrderMoveRelative),
            "DOTA_UNIT_ORDER_CAST_TOGGLE_ALT" => Some(Self::DotaUnitOrderCastToggleAlt),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EdotaVersusScenePlayerBehavior {
    VsPlayerBehaviorPlayActivity = 1,
    VsPlayerBehaviorChatWheel = 2,
    VsPlayerBehaviorPlaybackRate = 3,
}
impl EdotaVersusScenePlayerBehavior {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EdotaVersusScenePlayerBehavior::VsPlayerBehaviorPlayActivity => {
                "VS_PLAYER_BEHAVIOR_PLAY_ACTIVITY"
            }
            EdotaVersusScenePlayerBehavior::VsPlayerBehaviorChatWheel => {
                "VS_PLAYER_BEHAVIOR_CHAT_WHEEL"
            }
            EdotaVersusScenePlayerBehavior::VsPlayerBehaviorPlaybackRate => {
                "VS_PLAYER_BEHAVIOR_PLAYBACK_RATE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VS_PLAYER_BEHAVIOR_PLAY_ACTIVITY" => {
                Some(Self::VsPlayerBehaviorPlayActivity)
            }
            "VS_PLAYER_BEHAVIOR_CHAT_WHEEL" => Some(Self::VsPlayerBehaviorChatWheel),
            "VS_PLAYER_BEHAVIOR_PLAYBACK_RATE" => {
                Some(Self::VsPlayerBehaviorPlaybackRate)
            }
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaClientHardwareSpecs {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaSaveGame {
    #[prost(uint64, optional, tag = "5")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub save_time: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "3")]
    pub players: ::prost::alloc::vec::Vec<cdota_save_game::Player>,
    #[prost(message, repeated, tag = "4")]
    pub save_instances: ::prost::alloc::vec::Vec<cdota_save_game::SaveInstance>,
}
/// Nested message and enum types in `CDOTASaveGame`.
pub mod cdota_save_game {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Player {
        #[prost(
            enumeration = "super::DotaGcTeam",
            optional,
            tag = "1",
            default = "GoodGuys"
        )]
        pub team: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "3")]
        pub hero: ::core::option::Option<::prost::alloc::string::String>,
    }
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
    /// Nested message and enum types in `SaveInstance`.
    pub mod save_instance {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PlayerPositions {
            #[prost(float, optional, tag = "1")]
            pub x: ::core::option::Option<f32>,
            #[prost(float, optional, tag = "2")]
            pub y: ::core::option::Option<f32>,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgDotaCombatLogEntry {
    #[prost(
        enumeration = "DotaCombatlogTypes",
        optional,
        tag = "1",
        default = "DotaCombatlogInvalid"
    )]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgPendingEventAward {
    #[prost(enumeration = "EEvent", optional, tag = "1", default = "EventIdNone")]
    pub event_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub action_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub num_to_grant: ::core::option::Option<u32>,
    #[prost(
        enumeration = "EEventActionScoreMode",
        optional,
        tag = "4",
        default = "KEEventActionScoreModeAdd"
    )]
    pub score_mode: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "5")]
    pub audit_action: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "6")]
    pub audit_data: ::core::option::Option<u64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaGameMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaGameMode::DotaGamemodeNone => "DOTA_GAMEMODE_NONE",
            DotaGameMode::DotaGamemodeAp => "DOTA_GAMEMODE_AP",
            DotaGameMode::DotaGamemodeCm => "DOTA_GAMEMODE_CM",
            DotaGameMode::DotaGamemodeRd => "DOTA_GAMEMODE_RD",
            DotaGameMode::DotaGamemodeSd => "DOTA_GAMEMODE_SD",
            DotaGameMode::DotaGamemodeAr => "DOTA_GAMEMODE_AR",
            DotaGameMode::DotaGamemodeIntro => "DOTA_GAMEMODE_INTRO",
            DotaGameMode::DotaGamemodeHw => "DOTA_GAMEMODE_HW",
            DotaGameMode::DotaGamemodeReverseCm => "DOTA_GAMEMODE_REVERSE_CM",
            DotaGameMode::DotaGamemodeXmas => "DOTA_GAMEMODE_XMAS",
            DotaGameMode::DotaGamemodeTutorial => "DOTA_GAMEMODE_TUTORIAL",
            DotaGameMode::DotaGamemodeMo => "DOTA_GAMEMODE_MO",
            DotaGameMode::DotaGamemodeLp => "DOTA_GAMEMODE_LP",
            DotaGameMode::DotaGamemodePool1 => "DOTA_GAMEMODE_POOL1",
            DotaGameMode::DotaGamemodeFh => "DOTA_GAMEMODE_FH",
            DotaGameMode::DotaGamemodeCustom => "DOTA_GAMEMODE_CUSTOM",
            DotaGameMode::DotaGamemodeCd => "DOTA_GAMEMODE_CD",
            DotaGameMode::DotaGamemodeBd => "DOTA_GAMEMODE_BD",
            DotaGameMode::DotaGamemodeAbilityDraft => "DOTA_GAMEMODE_ABILITY_DRAFT",
            DotaGameMode::DotaGamemodeEvent => "DOTA_GAMEMODE_EVENT",
            DotaGameMode::DotaGamemodeArdm => "DOTA_GAMEMODE_ARDM",
            DotaGameMode::DotaGamemode1v1mid => "DOTA_GAMEMODE_1V1MID",
            DotaGameMode::DotaGamemodeAllDraft => "DOTA_GAMEMODE_ALL_DRAFT",
            DotaGameMode::DotaGamemodeTurbo => "DOTA_GAMEMODE_TURBO",
            DotaGameMode::DotaGamemodeMutation => "DOTA_GAMEMODE_MUTATION",
            DotaGameMode::DotaGamemodeCoachesChallenge => {
                "DOTA_GAMEMODE_COACHES_CHALLENGE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_GAMEMODE_NONE" => Some(Self::DotaGamemodeNone),
            "DOTA_GAMEMODE_AP" => Some(Self::DotaGamemodeAp),
            "DOTA_GAMEMODE_CM" => Some(Self::DotaGamemodeCm),
            "DOTA_GAMEMODE_RD" => Some(Self::DotaGamemodeRd),
            "DOTA_GAMEMODE_SD" => Some(Self::DotaGamemodeSd),
            "DOTA_GAMEMODE_AR" => Some(Self::DotaGamemodeAr),
            "DOTA_GAMEMODE_INTRO" => Some(Self::DotaGamemodeIntro),
            "DOTA_GAMEMODE_HW" => Some(Self::DotaGamemodeHw),
            "DOTA_GAMEMODE_REVERSE_CM" => Some(Self::DotaGamemodeReverseCm),
            "DOTA_GAMEMODE_XMAS" => Some(Self::DotaGamemodeXmas),
            "DOTA_GAMEMODE_TUTORIAL" => Some(Self::DotaGamemodeTutorial),
            "DOTA_GAMEMODE_MO" => Some(Self::DotaGamemodeMo),
            "DOTA_GAMEMODE_LP" => Some(Self::DotaGamemodeLp),
            "DOTA_GAMEMODE_POOL1" => Some(Self::DotaGamemodePool1),
            "DOTA_GAMEMODE_FH" => Some(Self::DotaGamemodeFh),
            "DOTA_GAMEMODE_CUSTOM" => Some(Self::DotaGamemodeCustom),
            "DOTA_GAMEMODE_CD" => Some(Self::DotaGamemodeCd),
            "DOTA_GAMEMODE_BD" => Some(Self::DotaGamemodeBd),
            "DOTA_GAMEMODE_ABILITY_DRAFT" => Some(Self::DotaGamemodeAbilityDraft),
            "DOTA_GAMEMODE_EVENT" => Some(Self::DotaGamemodeEvent),
            "DOTA_GAMEMODE_ARDM" => Some(Self::DotaGamemodeArdm),
            "DOTA_GAMEMODE_1V1MID" => Some(Self::DotaGamemode1v1mid),
            "DOTA_GAMEMODE_ALL_DRAFT" => Some(Self::DotaGamemodeAllDraft),
            "DOTA_GAMEMODE_TURBO" => Some(Self::DotaGamemodeTurbo),
            "DOTA_GAMEMODE_MUTATION" => Some(Self::DotaGamemodeMutation),
            "DOTA_GAMEMODE_COACHES_CHALLENGE" => Some(Self::DotaGamemodeCoachesChallenge),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaGameState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaGameState::DotaGamerulesStateInit => "DOTA_GAMERULES_STATE_INIT",
            DotaGameState::DotaGamerulesStateWaitForPlayersToLoad => {
                "DOTA_GAMERULES_STATE_WAIT_FOR_PLAYERS_TO_LOAD"
            }
            DotaGameState::DotaGamerulesStateHeroSelection => {
                "DOTA_GAMERULES_STATE_HERO_SELECTION"
            }
            DotaGameState::DotaGamerulesStateStrategyTime => {
                "DOTA_GAMERULES_STATE_STRATEGY_TIME"
            }
            DotaGameState::DotaGamerulesStatePreGame => "DOTA_GAMERULES_STATE_PRE_GAME",
            DotaGameState::DotaGamerulesStateGameInProgress => {
                "DOTA_GAMERULES_STATE_GAME_IN_PROGRESS"
            }
            DotaGameState::DotaGamerulesStatePostGame => "DOTA_GAMERULES_STATE_POST_GAME",
            DotaGameState::DotaGamerulesStateDisconnect => {
                "DOTA_GAMERULES_STATE_DISCONNECT"
            }
            DotaGameState::DotaGamerulesStateTeamShowcase => {
                "DOTA_GAMERULES_STATE_TEAM_SHOWCASE"
            }
            DotaGameState::DotaGamerulesStateCustomGameSetup => {
                "DOTA_GAMERULES_STATE_CUSTOM_GAME_SETUP"
            }
            DotaGameState::DotaGamerulesStateWaitForMapToLoad => {
                "DOTA_GAMERULES_STATE_WAIT_FOR_MAP_TO_LOAD"
            }
            DotaGameState::DotaGamerulesStateScenarioSetup => {
                "DOTA_GAMERULES_STATE_SCENARIO_SETUP"
            }
            DotaGameState::DotaGamerulesStatePlayerDraft => {
                "DOTA_GAMERULES_STATE_PLAYER_DRAFT"
            }
            DotaGameState::DotaGamerulesStateLast => "DOTA_GAMERULES_STATE_LAST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_GAMERULES_STATE_INIT" => Some(Self::DotaGamerulesStateInit),
            "DOTA_GAMERULES_STATE_WAIT_FOR_PLAYERS_TO_LOAD" => {
                Some(Self::DotaGamerulesStateWaitForPlayersToLoad)
            }
            "DOTA_GAMERULES_STATE_HERO_SELECTION" => {
                Some(Self::DotaGamerulesStateHeroSelection)
            }
            "DOTA_GAMERULES_STATE_STRATEGY_TIME" => {
                Some(Self::DotaGamerulesStateStrategyTime)
            }
            "DOTA_GAMERULES_STATE_PRE_GAME" => Some(Self::DotaGamerulesStatePreGame),
            "DOTA_GAMERULES_STATE_GAME_IN_PROGRESS" => {
                Some(Self::DotaGamerulesStateGameInProgress)
            }
            "DOTA_GAMERULES_STATE_POST_GAME" => Some(Self::DotaGamerulesStatePostGame),
            "DOTA_GAMERULES_STATE_DISCONNECT" => Some(Self::DotaGamerulesStateDisconnect),
            "DOTA_GAMERULES_STATE_TEAM_SHOWCASE" => {
                Some(Self::DotaGamerulesStateTeamShowcase)
            }
            "DOTA_GAMERULES_STATE_CUSTOM_GAME_SETUP" => {
                Some(Self::DotaGamerulesStateCustomGameSetup)
            }
            "DOTA_GAMERULES_STATE_WAIT_FOR_MAP_TO_LOAD" => {
                Some(Self::DotaGamerulesStateWaitForMapToLoad)
            }
            "DOTA_GAMERULES_STATE_SCENARIO_SETUP" => {
                Some(Self::DotaGamerulesStateScenarioSetup)
            }
            "DOTA_GAMERULES_STATE_PLAYER_DRAFT" => {
                Some(Self::DotaGamerulesStatePlayerDraft)
            }
            "DOTA_GAMERULES_STATE_LAST" => Some(Self::DotaGamerulesStateLast),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaGcTeam {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaGcTeam::GoodGuys => "DOTA_GC_TEAM_GOOD_GUYS",
            DotaGcTeam::BadGuys => "DOTA_GC_TEAM_BAD_GUYS",
            DotaGcTeam::Broadcaster => "DOTA_GC_TEAM_BROADCASTER",
            DotaGcTeam::Spectator => "DOTA_GC_TEAM_SPECTATOR",
            DotaGcTeam::PlayerPool => "DOTA_GC_TEAM_PLAYER_POOL",
            DotaGcTeam::Noteam => "DOTA_GC_TEAM_NOTEAM",
            DotaGcTeam::Custom1 => "DOTA_GC_TEAM_CUSTOM_1",
            DotaGcTeam::Custom2 => "DOTA_GC_TEAM_CUSTOM_2",
            DotaGcTeam::Custom3 => "DOTA_GC_TEAM_CUSTOM_3",
            DotaGcTeam::Custom4 => "DOTA_GC_TEAM_CUSTOM_4",
            DotaGcTeam::Custom5 => "DOTA_GC_TEAM_CUSTOM_5",
            DotaGcTeam::Custom6 => "DOTA_GC_TEAM_CUSTOM_6",
            DotaGcTeam::Custom7 => "DOTA_GC_TEAM_CUSTOM_7",
            DotaGcTeam::Custom8 => "DOTA_GC_TEAM_CUSTOM_8",
            DotaGcTeam::Neutrals => "DOTA_GC_TEAM_NEUTRALS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_GC_TEAM_GOOD_GUYS" => Some(Self::GoodGuys),
            "DOTA_GC_TEAM_BAD_GUYS" => Some(Self::BadGuys),
            "DOTA_GC_TEAM_BROADCASTER" => Some(Self::Broadcaster),
            "DOTA_GC_TEAM_SPECTATOR" => Some(Self::Spectator),
            "DOTA_GC_TEAM_PLAYER_POOL" => Some(Self::PlayerPool),
            "DOTA_GC_TEAM_NOTEAM" => Some(Self::Noteam),
            "DOTA_GC_TEAM_CUSTOM_1" => Some(Self::Custom1),
            "DOTA_GC_TEAM_CUSTOM_2" => Some(Self::Custom2),
            "DOTA_GC_TEAM_CUSTOM_3" => Some(Self::Custom3),
            "DOTA_GC_TEAM_CUSTOM_4" => Some(Self::Custom4),
            "DOTA_GC_TEAM_CUSTOM_5" => Some(Self::Custom5),
            "DOTA_GC_TEAM_CUSTOM_6" => Some(Self::Custom6),
            "DOTA_GC_TEAM_CUSTOM_7" => Some(Self::Custom7),
            "DOTA_GC_TEAM_CUSTOM_8" => Some(Self::Custom8),
            "DOTA_GC_TEAM_NEUTRALS" => Some(Self::Neutrals),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    EventIdFrostivus2023 = 48,
}
impl EEvent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EEvent::EventIdNone => "EVENT_ID_NONE",
            EEvent::EventIdDiretide => "EVENT_ID_DIRETIDE",
            EEvent::EventIdSpringFestival => "EVENT_ID_SPRING_FESTIVAL",
            EEvent::EventIdFrostivus2013 => "EVENT_ID_FROSTIVUS_2013",
            EEvent::EventIdCompendium2014 => "EVENT_ID_COMPENDIUM_2014",
            EEvent::EventIdNexonPcBang => "EVENT_ID_NEXON_PC_BANG",
            EEvent::EventIdPwrdDac2015 => "EVENT_ID_PWRD_DAC_2015",
            EEvent::EventIdNewBloom2015 => "EVENT_ID_NEW_BLOOM_2015",
            EEvent::EventIdInternational2015 => "EVENT_ID_INTERNATIONAL_2015",
            EEvent::EventIdFallMajor2015 => "EVENT_ID_FALL_MAJOR_2015",
            EEvent::EventIdOraclePa => "EVENT_ID_ORACLE_PA",
            EEvent::EventIdNewBloom2015Prebeast => "EVENT_ID_NEW_BLOOM_2015_PREBEAST",
            EEvent::EventIdFrostivus => "EVENT_ID_FROSTIVUS",
            EEvent::EventIdWinterMajor2016 => "EVENT_ID_WINTER_MAJOR_2016",
            EEvent::EventIdInternational2016 => "EVENT_ID_INTERNATIONAL_2016",
            EEvent::EventIdFallMajor2016 => "EVENT_ID_FALL_MAJOR_2016",
            EEvent::EventIdWinterMajor2017 => "EVENT_ID_WINTER_MAJOR_2017",
            EEvent::EventIdNewBloom2017 => "EVENT_ID_NEW_BLOOM_2017",
            EEvent::EventIdInternational2017 => "EVENT_ID_INTERNATIONAL_2017",
            EEvent::EventIdPlusSubscription => "EVENT_ID_PLUS_SUBSCRIPTION",
            EEvent::EventIdSinglesDay2017 => "EVENT_ID_SINGLES_DAY_2017",
            EEvent::EventIdFrostivus2017 => "EVENT_ID_FROSTIVUS_2017",
            EEvent::EventIdInternational2018 => "EVENT_ID_INTERNATIONAL_2018",
            EEvent::EventIdFrostivus2018 => "EVENT_ID_FROSTIVUS_2018",
            EEvent::EventIdNewBloom2019 => "EVENT_ID_NEW_BLOOM_2019",
            EEvent::EventIdInternational2019 => "EVENT_ID_INTERNATIONAL_2019",
            EEvent::EventIdNewPlayerExperience => "EVENT_ID_NEW_PLAYER_EXPERIENCE",
            EEvent::EventIdFrostivus2019 => "EVENT_ID_FROSTIVUS_2019",
            EEvent::EventIdNewBloom2020 => "EVENT_ID_NEW_BLOOM_2020",
            EEvent::EventIdInternational2020 => "EVENT_ID_INTERNATIONAL_2020",
            EEvent::EventIdTeamFandom => "EVENT_ID_TEAM_FANDOM",
            EEvent::EventIdDiretide2020 => "EVENT_ID_DIRETIDE_2020",
            EEvent::EventIdSpring2021 => "EVENT_ID_SPRING_2021",
            EEvent::EventIdFall2021 => "EVENT_ID_FALL_2021",
            EEvent::EventIdTeamFandomFall2021 => "EVENT_ID_TEAM_FANDOM_FALL_2021",
            EEvent::EventIdTeam20212022Tour2 => "EVENT_ID_TEAM_2021_2022_TOUR2",
            EEvent::EventIdInternational2022 => "EVENT_ID_INTERNATIONAL_2022",
            EEvent::EventIdTeam20212022Tour3 => "EVENT_ID_TEAM_2021_2022_TOUR3",
            EEvent::EventIdTeamInternational2022 => "EVENT_ID_TEAM_INTERNATIONAL_2022",
            EEvent::EventIdPermanentGrants => "EVENT_ID_PERMANENT_GRANTS",
            EEvent::EventIdMuertaReleaseSpring2023 => {
                "EVENT_ID_MUERTA_RELEASE_SPRING2023"
            }
            EEvent::EventIdTeam2023Tour1 => "EVENT_ID_TEAM_2023_TOUR1",
            EEvent::EventIdTeam2023Tour2 => "EVENT_ID_TEAM_2023_TOUR2",
            EEvent::EventIdTeam2023Tour3 => "EVENT_ID_TEAM_2023_TOUR3",
            EEvent::EventIdInternational2023 => "EVENT_ID_INTERNATIONAL_2023",
            EEvent::EventId10thAnniversary => "EVENT_ID_10TH_ANNIVERSARY",
            EEvent::EventIdFrostivus2023 => "EVENT_ID_FROSTIVUS_2023",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EVENT_ID_NONE" => Some(Self::EventIdNone),
            "EVENT_ID_DIRETIDE" => Some(Self::EventIdDiretide),
            "EVENT_ID_SPRING_FESTIVAL" => Some(Self::EventIdSpringFestival),
            "EVENT_ID_FROSTIVUS_2013" => Some(Self::EventIdFrostivus2013),
            "EVENT_ID_COMPENDIUM_2014" => Some(Self::EventIdCompendium2014),
            "EVENT_ID_NEXON_PC_BANG" => Some(Self::EventIdNexonPcBang),
            "EVENT_ID_PWRD_DAC_2015" => Some(Self::EventIdPwrdDac2015),
            "EVENT_ID_NEW_BLOOM_2015" => Some(Self::EventIdNewBloom2015),
            "EVENT_ID_INTERNATIONAL_2015" => Some(Self::EventIdInternational2015),
            "EVENT_ID_FALL_MAJOR_2015" => Some(Self::EventIdFallMajor2015),
            "EVENT_ID_ORACLE_PA" => Some(Self::EventIdOraclePa),
            "EVENT_ID_NEW_BLOOM_2015_PREBEAST" => Some(Self::EventIdNewBloom2015Prebeast),
            "EVENT_ID_FROSTIVUS" => Some(Self::EventIdFrostivus),
            "EVENT_ID_WINTER_MAJOR_2016" => Some(Self::EventIdWinterMajor2016),
            "EVENT_ID_INTERNATIONAL_2016" => Some(Self::EventIdInternational2016),
            "EVENT_ID_FALL_MAJOR_2016" => Some(Self::EventIdFallMajor2016),
            "EVENT_ID_WINTER_MAJOR_2017" => Some(Self::EventIdWinterMajor2017),
            "EVENT_ID_NEW_BLOOM_2017" => Some(Self::EventIdNewBloom2017),
            "EVENT_ID_INTERNATIONAL_2017" => Some(Self::EventIdInternational2017),
            "EVENT_ID_PLUS_SUBSCRIPTION" => Some(Self::EventIdPlusSubscription),
            "EVENT_ID_SINGLES_DAY_2017" => Some(Self::EventIdSinglesDay2017),
            "EVENT_ID_FROSTIVUS_2017" => Some(Self::EventIdFrostivus2017),
            "EVENT_ID_INTERNATIONAL_2018" => Some(Self::EventIdInternational2018),
            "EVENT_ID_FROSTIVUS_2018" => Some(Self::EventIdFrostivus2018),
            "EVENT_ID_NEW_BLOOM_2019" => Some(Self::EventIdNewBloom2019),
            "EVENT_ID_INTERNATIONAL_2019" => Some(Self::EventIdInternational2019),
            "EVENT_ID_NEW_PLAYER_EXPERIENCE" => Some(Self::EventIdNewPlayerExperience),
            "EVENT_ID_FROSTIVUS_2019" => Some(Self::EventIdFrostivus2019),
            "EVENT_ID_NEW_BLOOM_2020" => Some(Self::EventIdNewBloom2020),
            "EVENT_ID_INTERNATIONAL_2020" => Some(Self::EventIdInternational2020),
            "EVENT_ID_TEAM_FANDOM" => Some(Self::EventIdTeamFandom),
            "EVENT_ID_DIRETIDE_2020" => Some(Self::EventIdDiretide2020),
            "EVENT_ID_SPRING_2021" => Some(Self::EventIdSpring2021),
            "EVENT_ID_FALL_2021" => Some(Self::EventIdFall2021),
            "EVENT_ID_TEAM_FANDOM_FALL_2021" => Some(Self::EventIdTeamFandomFall2021),
            "EVENT_ID_TEAM_2021_2022_TOUR2" => Some(Self::EventIdTeam20212022Tour2),
            "EVENT_ID_INTERNATIONAL_2022" => Some(Self::EventIdInternational2022),
            "EVENT_ID_TEAM_2021_2022_TOUR3" => Some(Self::EventIdTeam20212022Tour3),
            "EVENT_ID_TEAM_INTERNATIONAL_2022" => {
                Some(Self::EventIdTeamInternational2022)
            }
            "EVENT_ID_PERMANENT_GRANTS" => Some(Self::EventIdPermanentGrants),
            "EVENT_ID_MUERTA_RELEASE_SPRING2023" => {
                Some(Self::EventIdMuertaReleaseSpring2023)
            }
            "EVENT_ID_TEAM_2023_TOUR1" => Some(Self::EventIdTeam2023Tour1),
            "EVENT_ID_TEAM_2023_TOUR2" => Some(Self::EventIdTeam2023Tour2),
            "EVENT_ID_TEAM_2023_TOUR3" => Some(Self::EventIdTeam2023Tour3),
            "EVENT_ID_INTERNATIONAL_2023" => Some(Self::EventIdInternational2023),
            "EVENT_ID_10TH_ANNIVERSARY" => Some(Self::EventId10thAnniversary),
            "EVENT_ID_FROSTIVUS_2023" => Some(Self::EventIdFrostivus2023),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl ERankType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ERankType::KERankTypeInvalid => "k_ERankType_Invalid",
            ERankType::KERankTypeCasual => "k_ERankType_Casual",
            ERankType::KERankTypeRanked => "k_ERankType_Ranked",
            ERankType::KERankTypeCasualLegacy => "k_ERankType_CasualLegacy",
            ERankType::KERankTypeRankedLegacy => "k_ERankType_RankedLegacy",
            ERankType::KERankTypeCasualGlicko => "k_ERankType_CasualGlicko",
            ERankType::KERankTypeRankedGlicko => "k_ERankType_RankedGlicko",
            ERankType::KERankTypeRankMax => "k_ERankType_RankMax",
            ERankType::KERankTypeBehaviorPrivate => "k_ERankType_BehaviorPrivate",
            ERankType::KERankTypeBehaviorPublic => "k_ERankType_BehaviorPublic",
            ERankType::KERankTypeMax => "k_ERankType_Max",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_ERankType_Invalid" => Some(Self::KERankTypeInvalid),
            "k_ERankType_Casual" => Some(Self::KERankTypeCasual),
            "k_ERankType_Ranked" => Some(Self::KERankTypeRanked),
            "k_ERankType_CasualLegacy" => Some(Self::KERankTypeCasualLegacy),
            "k_ERankType_RankedLegacy" => Some(Self::KERankTypeRankedLegacy),
            "k_ERankType_CasualGlicko" => Some(Self::KERankTypeCasualGlicko),
            "k_ERankType_RankedGlicko" => Some(Self::KERankTypeRankedGlicko),
            "k_ERankType_RankMax" => Some(Self::KERankTypeRankMax),
            "k_ERankType_BehaviorPrivate" => Some(Self::KERankTypeBehaviorPrivate),
            "k_ERankType_BehaviorPublic" => Some(Self::KERankTypeBehaviorPublic),
            "k_ERankType_Max" => Some(Self::KERankTypeMax),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
impl DotaLeaverStatusT {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaLeaverStatusT::DotaLeaverNone => "DOTA_LEAVER_NONE",
            DotaLeaverStatusT::DotaLeaverDisconnected => "DOTA_LEAVER_DISCONNECTED",
            DotaLeaverStatusT::DotaLeaverDisconnectedTooLong => {
                "DOTA_LEAVER_DISCONNECTED_TOO_LONG"
            }
            DotaLeaverStatusT::DotaLeaverAbandoned => "DOTA_LEAVER_ABANDONED",
            DotaLeaverStatusT::DotaLeaverAfk => "DOTA_LEAVER_AFK",
            DotaLeaverStatusT::DotaLeaverNeverConnected => "DOTA_LEAVER_NEVER_CONNECTED",
            DotaLeaverStatusT::DotaLeaverNeverConnectedTooLong => {
                "DOTA_LEAVER_NEVER_CONNECTED_TOO_LONG"
            }
            DotaLeaverStatusT::DotaLeaverFailedToReadyUp => {
                "DOTA_LEAVER_FAILED_TO_READY_UP"
            }
            DotaLeaverStatusT::DotaLeaverDeclined => "DOTA_LEAVER_DECLINED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_LEAVER_NONE" => Some(Self::DotaLeaverNone),
            "DOTA_LEAVER_DISCONNECTED" => Some(Self::DotaLeaverDisconnected),
            "DOTA_LEAVER_DISCONNECTED_TOO_LONG" => {
                Some(Self::DotaLeaverDisconnectedTooLong)
            }
            "DOTA_LEAVER_ABANDONED" => Some(Self::DotaLeaverAbandoned),
            "DOTA_LEAVER_AFK" => Some(Self::DotaLeaverAfk),
            "DOTA_LEAVER_NEVER_CONNECTED" => Some(Self::DotaLeaverNeverConnected),
            "DOTA_LEAVER_NEVER_CONNECTED_TOO_LONG" => {
                Some(Self::DotaLeaverNeverConnectedTooLong)
            }
            "DOTA_LEAVER_FAILED_TO_READY_UP" => Some(Self::DotaLeaverFailedToReadyUp),
            "DOTA_LEAVER_DECLINED" => Some(Self::DotaLeaverDeclined),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaConnectionStateT {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaConnectionStateT::DotaConnectionStateUnknown => {
                "DOTA_CONNECTION_STATE_UNKNOWN"
            }
            DotaConnectionStateT::DotaConnectionStateNotYetConnected => {
                "DOTA_CONNECTION_STATE_NOT_YET_CONNECTED"
            }
            DotaConnectionStateT::DotaConnectionStateConnected => {
                "DOTA_CONNECTION_STATE_CONNECTED"
            }
            DotaConnectionStateT::DotaConnectionStateDisconnected => {
                "DOTA_CONNECTION_STATE_DISCONNECTED"
            }
            DotaConnectionStateT::DotaConnectionStateAbandoned => {
                "DOTA_CONNECTION_STATE_ABANDONED"
            }
            DotaConnectionStateT::DotaConnectionStateLoading => {
                "DOTA_CONNECTION_STATE_LOADING"
            }
            DotaConnectionStateT::DotaConnectionStateFailed => {
                "DOTA_CONNECTION_STATE_FAILED"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_CONNECTION_STATE_UNKNOWN" => Some(Self::DotaConnectionStateUnknown),
            "DOTA_CONNECTION_STATE_NOT_YET_CONNECTED" => {
                Some(Self::DotaConnectionStateNotYetConnected)
            }
            "DOTA_CONNECTION_STATE_CONNECTED" => Some(Self::DotaConnectionStateConnected),
            "DOTA_CONNECTION_STATE_DISCONNECTED" => {
                Some(Self::DotaConnectionStateDisconnected)
            }
            "DOTA_CONNECTION_STATE_ABANDONED" => Some(Self::DotaConnectionStateAbandoned),
            "DOTA_CONNECTION_STATE_LOADING" => Some(Self::DotaConnectionStateLoading),
            "DOTA_CONNECTION_STATE_FAILED" => Some(Self::DotaConnectionStateFailed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FantasyRoles {
    FantasyRoleUndefined = 0,
    FantasyRoleCore = 1,
    FantasyRoleSupport = 2,
    FantasyRoleOfflane = 3,
    FantasyRoleMid = 4,
}
impl FantasyRoles {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FantasyRoles::FantasyRoleUndefined => "FANTASY_ROLE_UNDEFINED",
            FantasyRoles::FantasyRoleCore => "FANTASY_ROLE_CORE",
            FantasyRoles::FantasyRoleSupport => "FANTASY_ROLE_SUPPORT",
            FantasyRoles::FantasyRoleOfflane => "FANTASY_ROLE_OFFLANE",
            FantasyRoles::FantasyRoleMid => "FANTASY_ROLE_MID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FANTASY_ROLE_UNDEFINED" => Some(Self::FantasyRoleUndefined),
            "FANTASY_ROLE_CORE" => Some(Self::FantasyRoleCore),
            "FANTASY_ROLE_SUPPORT" => Some(Self::FantasyRoleSupport),
            "FANTASY_ROLE_OFFLANE" => Some(Self::FantasyRoleOfflane),
            "FANTASY_ROLE_MID" => Some(Self::FantasyRoleMid),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl FantasyScoring {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FantasyScoring::Kills => "FANTASY_SCORING_KILLS",
            FantasyScoring::Deaths => "FANTASY_SCORING_DEATHS",
            FantasyScoring::Cs => "FANTASY_SCORING_CS",
            FantasyScoring::Gpm => "FANTASY_SCORING_GPM",
            FantasyScoring::TowerKills => "FANTASY_SCORING_TOWER_KILLS",
            FantasyScoring::RoshanKills => "FANTASY_SCORING_ROSHAN_KILLS",
            FantasyScoring::TeamfightParticipation => {
                "FANTASY_SCORING_TEAMFIGHT_PARTICIPATION"
            }
            FantasyScoring::WardsPlanted => "FANTASY_SCORING_WARDS_PLANTED",
            FantasyScoring::CampsStacked => "FANTASY_SCORING_CAMPS_STACKED",
            FantasyScoring::RunesGrabbed => "FANTASY_SCORING_RUNES_GRABBED",
            FantasyScoring::FirstBlood => "FANTASY_SCORING_FIRST_BLOOD",
            FantasyScoring::Stuns => "FANTASY_SCORING_STUNS",
            FantasyScoring::SmokesUsed => "FANTASY_SCORING_SMOKES_USED",
            FantasyScoring::NeutralTokensFound => "FANTASY_SCORING_NEUTRAL_TOKENS_FOUND",
            FantasyScoring::WatchersTaken => "FANTASY_SCORING_WATCHERS_TAKEN",
            FantasyScoring::LotusesGained => "FANTASY_SCORING_LOTUSES_GAINED",
            FantasyScoring::TormentorKills => "FANTASY_SCORING_TORMENTOR_KILLS",
            FantasyScoring::CourierKills => "FANTASY_SCORING_COURIER_KILLS",
            FantasyScoring::Types => "FANTASY_SCORING_TYPES",
            FantasyScoring::Invalid => "FANTASY_SCORING_INVALID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FANTASY_SCORING_KILLS" => Some(Self::Kills),
            "FANTASY_SCORING_DEATHS" => Some(Self::Deaths),
            "FANTASY_SCORING_CS" => Some(Self::Cs),
            "FANTASY_SCORING_GPM" => Some(Self::Gpm),
            "FANTASY_SCORING_TOWER_KILLS" => Some(Self::TowerKills),
            "FANTASY_SCORING_ROSHAN_KILLS" => Some(Self::RoshanKills),
            "FANTASY_SCORING_TEAMFIGHT_PARTICIPATION" => {
                Some(Self::TeamfightParticipation)
            }
            "FANTASY_SCORING_WARDS_PLANTED" => Some(Self::WardsPlanted),
            "FANTASY_SCORING_CAMPS_STACKED" => Some(Self::CampsStacked),
            "FANTASY_SCORING_RUNES_GRABBED" => Some(Self::RunesGrabbed),
            "FANTASY_SCORING_FIRST_BLOOD" => Some(Self::FirstBlood),
            "FANTASY_SCORING_STUNS" => Some(Self::Stuns),
            "FANTASY_SCORING_SMOKES_USED" => Some(Self::SmokesUsed),
            "FANTASY_SCORING_NEUTRAL_TOKENS_FOUND" => Some(Self::NeutralTokensFound),
            "FANTASY_SCORING_WATCHERS_TAKEN" => Some(Self::WatchersTaken),
            "FANTASY_SCORING_LOTUSES_GAINED" => Some(Self::LotusesGained),
            "FANTASY_SCORING_TORMENTOR_KILLS" => Some(Self::TormentorKills),
            "FANTASY_SCORING_COURIER_KILLS" => Some(Self::CourierKills),
            "FANTASY_SCORING_TYPES" => Some(Self::Types),
            "FANTASY_SCORING_INVALID" => Some(Self::Invalid),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FantasyTeamSlots {
    FantasySlotNone = 0,
    FantasySlotCore = 1,
    FantasySlotSupport = 2,
    FantasySlotAny = 3,
    FantasySlotBench = 4,
}
impl FantasyTeamSlots {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FantasyTeamSlots::FantasySlotNone => "FANTASY_SLOT_NONE",
            FantasyTeamSlots::FantasySlotCore => "FANTASY_SLOT_CORE",
            FantasyTeamSlots::FantasySlotSupport => "FANTASY_SLOT_SUPPORT",
            FantasyTeamSlots::FantasySlotAny => "FANTASY_SLOT_ANY",
            FantasyTeamSlots::FantasySlotBench => "FANTASY_SLOT_BENCH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FANTASY_SLOT_NONE" => Some(Self::FantasySlotNone),
            "FANTASY_SLOT_CORE" => Some(Self::FantasySlotCore),
            "FANTASY_SLOT_SUPPORT" => Some(Self::FantasySlotSupport),
            "FANTASY_SLOT_ANY" => Some(Self::FantasySlotAny),
            "FANTASY_SLOT_BENCH" => Some(Self::FantasySlotBench),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl FantasySelectionMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FantasySelectionMode::FantasySelectionInvalid => "FANTASY_SELECTION_INVALID",
            FantasySelectionMode::FantasySelectionLocked => "FANTASY_SELECTION_LOCKED",
            FantasySelectionMode::FantasySelectionShuffle => "FANTASY_SELECTION_SHUFFLE",
            FantasySelectionMode::FantasySelectionFreePick => {
                "FANTASY_SELECTION_FREE_PICK"
            }
            FantasySelectionMode::FantasySelectionEnded => "FANTASY_SELECTION_ENDED",
            FantasySelectionMode::FantasySelectionPreSeason => {
                "FANTASY_SELECTION_PRE_SEASON"
            }
            FantasySelectionMode::FantasySelectionPreDraft => {
                "FANTASY_SELECTION_PRE_DRAFT"
            }
            FantasySelectionMode::FantasySelectionDrafting => {
                "FANTASY_SELECTION_DRAFTING"
            }
            FantasySelectionMode::FantasySelectionRegularSeason => {
                "FANTASY_SELECTION_REGULAR_SEASON"
            }
            FantasySelectionMode::FantasySelectionCardBased => {
                "FANTASY_SELECTION_CARD_BASED"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FANTASY_SELECTION_INVALID" => Some(Self::FantasySelectionInvalid),
            "FANTASY_SELECTION_LOCKED" => Some(Self::FantasySelectionLocked),
            "FANTASY_SELECTION_SHUFFLE" => Some(Self::FantasySelectionShuffle),
            "FANTASY_SELECTION_FREE_PICK" => Some(Self::FantasySelectionFreePick),
            "FANTASY_SELECTION_ENDED" => Some(Self::FantasySelectionEnded),
            "FANTASY_SELECTION_PRE_SEASON" => Some(Self::FantasySelectionPreSeason),
            "FANTASY_SELECTION_PRE_DRAFT" => Some(Self::FantasySelectionPreDraft),
            "FANTASY_SELECTION_DRAFTING" => Some(Self::FantasySelectionDrafting),
            "FANTASY_SELECTION_REGULAR_SEASON" => {
                Some(Self::FantasySelectionRegularSeason)
            }
            "FANTASY_SELECTION_CARD_BASED" => Some(Self::FantasySelectionCardBased),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FantasyGemType {
    Ruby = 0,
    Sapphire = 1,
    Emerald = 2,
}
impl FantasyGemType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FantasyGemType::Ruby => "FANTASY_GEM_TYPE_RUBY",
            FantasyGemType::Sapphire => "FANTASY_GEM_TYPE_SAPPHIRE",
            FantasyGemType::Emerald => "FANTASY_GEM_TYPE_EMERALD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FANTASY_GEM_TYPE_RUBY" => Some(Self::Ruby),
            "FANTASY_GEM_TYPE_SAPPHIRE" => Some(Self::Sapphire),
            "FANTASY_GEM_TYPE_EMERALD" => Some(Self::Emerald),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaChatChannelTypeT {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaChatChannelTypeT::DotaChannelTypeRegional => "DOTAChannelType_Regional",
            DotaChatChannelTypeT::DotaChannelTypeCustom => "DOTAChannelType_Custom",
            DotaChatChannelTypeT::DotaChannelTypeParty => "DOTAChannelType_Party",
            DotaChatChannelTypeT::DotaChannelTypeLobby => "DOTAChannelType_Lobby",
            DotaChatChannelTypeT::DotaChannelTypeTeam => "DOTAChannelType_Team",
            DotaChatChannelTypeT::DotaChannelTypeGuild => "DOTAChannelType_Guild",
            DotaChatChannelTypeT::DotaChannelTypeFantasy => "DOTAChannelType_Fantasy",
            DotaChatChannelTypeT::DotaChannelTypeWhisper => "DOTAChannelType_Whisper",
            DotaChatChannelTypeT::DotaChannelTypeConsole => "DOTAChannelType_Console",
            DotaChatChannelTypeT::DotaChannelTypeTab => "DOTAChannelType_Tab",
            DotaChatChannelTypeT::DotaChannelTypeInvalid => "DOTAChannelType_Invalid",
            DotaChatChannelTypeT::DotaChannelTypeGameAll => "DOTAChannelType_GameAll",
            DotaChatChannelTypeT::DotaChannelTypeGameAllies => {
                "DOTAChannelType_GameAllies"
            }
            DotaChatChannelTypeT::DotaChannelTypeGameSpectator => {
                "DOTAChannelType_GameSpectator"
            }
            DotaChatChannelTypeT::DotaChannelTypeGameCoaching => {
                "DOTAChannelType_GameCoaching"
            }
            DotaChatChannelTypeT::DotaChannelTypeCafe => "DOTAChannelType_Cafe",
            DotaChatChannelTypeT::DotaChannelTypeCustomGame => {
                "DOTAChannelType_CustomGame"
            }
            DotaChatChannelTypeT::DotaChannelTypePrivate => "DOTAChannelType_Private",
            DotaChatChannelTypeT::DotaChannelTypePostGame => "DOTAChannelType_PostGame",
            DotaChatChannelTypeT::DotaChannelTypeBattleCup => "DOTAChannelType_BattleCup",
            DotaChatChannelTypeT::DotaChannelTypeHltvSpectator => {
                "DOTAChannelType_HLTVSpectator"
            }
            DotaChatChannelTypeT::DotaChannelTypeGameEvents => {
                "DOTAChannelType_GameEvents"
            }
            DotaChatChannelTypeT::DotaChannelTypeTrivia => "DOTAChannelType_Trivia",
            DotaChatChannelTypeT::DotaChannelTypeNewPlayer => "DOTAChannelType_NewPlayer",
            DotaChatChannelTypeT::DotaChannelTypePrivateCoaching => {
                "DOTAChannelType_PrivateCoaching"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTAChannelType_Regional" => Some(Self::DotaChannelTypeRegional),
            "DOTAChannelType_Custom" => Some(Self::DotaChannelTypeCustom),
            "DOTAChannelType_Party" => Some(Self::DotaChannelTypeParty),
            "DOTAChannelType_Lobby" => Some(Self::DotaChannelTypeLobby),
            "DOTAChannelType_Team" => Some(Self::DotaChannelTypeTeam),
            "DOTAChannelType_Guild" => Some(Self::DotaChannelTypeGuild),
            "DOTAChannelType_Fantasy" => Some(Self::DotaChannelTypeFantasy),
            "DOTAChannelType_Whisper" => Some(Self::DotaChannelTypeWhisper),
            "DOTAChannelType_Console" => Some(Self::DotaChannelTypeConsole),
            "DOTAChannelType_Tab" => Some(Self::DotaChannelTypeTab),
            "DOTAChannelType_Invalid" => Some(Self::DotaChannelTypeInvalid),
            "DOTAChannelType_GameAll" => Some(Self::DotaChannelTypeGameAll),
            "DOTAChannelType_GameAllies" => Some(Self::DotaChannelTypeGameAllies),
            "DOTAChannelType_GameSpectator" => Some(Self::DotaChannelTypeGameSpectator),
            "DOTAChannelType_GameCoaching" => Some(Self::DotaChannelTypeGameCoaching),
            "DOTAChannelType_Cafe" => Some(Self::DotaChannelTypeCafe),
            "DOTAChannelType_CustomGame" => Some(Self::DotaChannelTypeCustomGame),
            "DOTAChannelType_Private" => Some(Self::DotaChannelTypePrivate),
            "DOTAChannelType_PostGame" => Some(Self::DotaChannelTypePostGame),
            "DOTAChannelType_BattleCup" => Some(Self::DotaChannelTypeBattleCup),
            "DOTAChannelType_HLTVSpectator" => Some(Self::DotaChannelTypeHltvSpectator),
            "DOTAChannelType_GameEvents" => Some(Self::DotaChannelTypeGameEvents),
            "DOTAChannelType_Trivia" => Some(Self::DotaChannelTypeTrivia),
            "DOTAChannelType_NewPlayer" => Some(Self::DotaChannelTypeNewPlayer),
            "DOTAChannelType_PrivateCoaching" => {
                Some(Self::DotaChannelTypePrivateCoaching)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EChatSpecialPrivileges {
    KEChatSpecialPrivilegesNone = 0,
    KEChatSpecialPrivilegesModerator = 1,
    KEChatSpecialPrivilegesSuperModerator = 2,
}
impl EChatSpecialPrivileges {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EChatSpecialPrivileges::KEChatSpecialPrivilegesNone => {
                "k_EChatSpecialPrivileges_None"
            }
            EChatSpecialPrivileges::KEChatSpecialPrivilegesModerator => {
                "k_EChatSpecialPrivileges_Moderator"
            }
            EChatSpecialPrivileges::KEChatSpecialPrivilegesSuperModerator => {
                "k_EChatSpecialPrivileges_SuperModerator"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_EChatSpecialPrivileges_None" => Some(Self::KEChatSpecialPrivilegesNone),
            "k_EChatSpecialPrivileges_Moderator" => {
                Some(Self::KEChatSpecialPrivilegesModerator)
            }
            "k_EChatSpecialPrivileges_SuperModerator" => {
                Some(Self::KEChatSpecialPrivilegesSuperModerator)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaCommTypeT {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaCommTypeT::DotaCommTypeNone => "DOTA_COMM_TYPE_NONE",
            DotaCommTypeT::DotaCommTypePing => "DOTA_COMM_TYPE_PING",
            DotaCommTypeT::DotaCommTypeChatwheel => "DOTA_COMM_TYPE_CHATWHEEL",
            DotaCommTypeT::DotaCommTypeTip => "DOTA_COMM_TYPE_TIP",
            DotaCommTypeT::DotaCommTypeText => "DOTA_COMM_TYPE_TEXT",
            DotaCommTypeT::DotaCommTypeShowcase => "DOTA_COMM_TYPE_SHOWCASE",
            DotaCommTypeT::DotaCommTypeVoice => "DOTA_COMM_TYPE_VOICE",
            DotaCommTypeT::DotaCommTypeAllyAbility => "DOTA_COMM_TYPE_ALLY_ABILITY",
            DotaCommTypeT::DotaCommTypePause => "DOTA_COMM_TYPE_PAUSE",
            DotaCommTypeT::DotaCommTypeCoaching => "DOTA_COMM_TYPE_COACHING",
            DotaCommTypeT::DotaCommTypeNocooldown => "DOTA_COMM_TYPE_NOCOOLDOWN",
            DotaCommTypeT::DotaCommTypeRankedmatchmake => {
                "DOTA_COMM_TYPE_RANKEDMATCHMAKE"
            }
            DotaCommTypeT::DotaCommTypeDrops => "DOTA_COMM_TYPE_DROPS",
            DotaCommTypeT::DotaCommTypeNewplayerExpert => {
                "DOTA_COMM_TYPE_NEWPLAYER_EXPERT"
            }
            DotaCommTypeT::DotaCommTypeCoached => "DOTA_COMM_TYPE_COACHED",
            DotaCommTypeT::DotaCommTypeMapdrawing => "DOTA_COMM_TYPE_MAPDRAWING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_COMM_TYPE_NONE" => Some(Self::DotaCommTypeNone),
            "DOTA_COMM_TYPE_PING" => Some(Self::DotaCommTypePing),
            "DOTA_COMM_TYPE_CHATWHEEL" => Some(Self::DotaCommTypeChatwheel),
            "DOTA_COMM_TYPE_TIP" => Some(Self::DotaCommTypeTip),
            "DOTA_COMM_TYPE_TEXT" => Some(Self::DotaCommTypeText),
            "DOTA_COMM_TYPE_SHOWCASE" => Some(Self::DotaCommTypeShowcase),
            "DOTA_COMM_TYPE_VOICE" => Some(Self::DotaCommTypeVoice),
            "DOTA_COMM_TYPE_ALLY_ABILITY" => Some(Self::DotaCommTypeAllyAbility),
            "DOTA_COMM_TYPE_PAUSE" => Some(Self::DotaCommTypePause),
            "DOTA_COMM_TYPE_COACHING" => Some(Self::DotaCommTypeCoaching),
            "DOTA_COMM_TYPE_NOCOOLDOWN" => Some(Self::DotaCommTypeNocooldown),
            "DOTA_COMM_TYPE_RANKEDMATCHMAKE" => Some(Self::DotaCommTypeRankedmatchmake),
            "DOTA_COMM_TYPE_DROPS" => Some(Self::DotaCommTypeDrops),
            "DOTA_COMM_TYPE_NEWPLAYER_EXPERT" => Some(Self::DotaCommTypeNewplayerExpert),
            "DOTA_COMM_TYPE_COACHED" => Some(Self::DotaCommTypeCoached),
            "DOTA_COMM_TYPE_MAPDRAWING" => Some(Self::DotaCommTypeMapdrawing),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaCommLevelT {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaCommLevelT::DotaCommLevelNone => "DOTA_COMM_LEVEL_NONE",
            DotaCommLevelT::DotaCommLevelCooldown => "DOTA_COMM_LEVEL_COOLDOWN",
            DotaCommLevelT::DotaCommLevelPings => "DOTA_COMM_LEVEL_PINGS",
            DotaCommLevelT::DotaCommLevelMapdrawing => "DOTA_COMM_LEVEL_MAPDRAWING",
            DotaCommLevelT::DotaCommLevelChat => "DOTA_COMM_LEVEL_CHAT",
            DotaCommLevelT::DotaCommLevelTipping => "DOTA_COMM_LEVEL_TIPPING",
            DotaCommLevelT::DotaCommLevelVoice => "DOTA_COMM_LEVEL_VOICE",
            DotaCommLevelT::DotaCommLevelAlliedAbility => {
                "DOTA_COMM_LEVEL_ALLIED_ABILITY"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_COMM_LEVEL_NONE" => Some(Self::DotaCommLevelNone),
            "DOTA_COMM_LEVEL_COOLDOWN" => Some(Self::DotaCommLevelCooldown),
            "DOTA_COMM_LEVEL_PINGS" => Some(Self::DotaCommLevelPings),
            "DOTA_COMM_LEVEL_MAPDRAWING" => Some(Self::DotaCommLevelMapdrawing),
            "DOTA_COMM_LEVEL_CHAT" => Some(Self::DotaCommLevelChat),
            "DOTA_COMM_LEVEL_TIPPING" => Some(Self::DotaCommLevelTipping),
            "DOTA_COMM_LEVEL_VOICE" => Some(Self::DotaCommLevelVoice),
            "DOTA_COMM_LEVEL_ALLIED_ABILITY" => Some(Self::DotaCommLevelAlliedAbility),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaBehaviorLevelT {
    DotaBehaviorLevelNone = 0,
    DotaBehaviorLevelRankedAllowed = 1,
    DotaBehaviorLevelPausing = 2,
    DotaBehaviorLevelDrops = 3,
    DotaBehaviorLevelCoaching = 4,
}
impl DotaBehaviorLevelT {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaBehaviorLevelT::DotaBehaviorLevelNone => "DOTA_BEHAVIOR_LEVEL_NONE",
            DotaBehaviorLevelT::DotaBehaviorLevelRankedAllowed => {
                "DOTA_BEHAVIOR_LEVEL_RANKED_ALLOWED"
            }
            DotaBehaviorLevelT::DotaBehaviorLevelPausing => "DOTA_BEHAVIOR_LEVEL_PAUSING",
            DotaBehaviorLevelT::DotaBehaviorLevelDrops => "DOTA_BEHAVIOR_LEVEL_DROPS",
            DotaBehaviorLevelT::DotaBehaviorLevelCoaching => {
                "DOTA_BEHAVIOR_LEVEL_COACHING"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_BEHAVIOR_LEVEL_NONE" => Some(Self::DotaBehaviorLevelNone),
            "DOTA_BEHAVIOR_LEVEL_RANKED_ALLOWED" => {
                Some(Self::DotaBehaviorLevelRankedAllowed)
            }
            "DOTA_BEHAVIOR_LEVEL_PAUSING" => Some(Self::DotaBehaviorLevelPausing),
            "DOTA_BEHAVIOR_LEVEL_DROPS" => Some(Self::DotaBehaviorLevelDrops),
            "DOTA_BEHAVIOR_LEVEL_COACHING" => Some(Self::DotaBehaviorLevelCoaching),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl EProfileCardSlotType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EProfileCardSlotType::KEProfileCardSlotTypeEmpty => {
                "k_EProfileCardSlotType_Empty"
            }
            EProfileCardSlotType::KEProfileCardSlotTypeStat => {
                "k_EProfileCardSlotType_Stat"
            }
            EProfileCardSlotType::KEProfileCardSlotTypeTrophy => {
                "k_EProfileCardSlotType_Trophy"
            }
            EProfileCardSlotType::KEProfileCardSlotTypeItem => {
                "k_EProfileCardSlotType_Item"
            }
            EProfileCardSlotType::KEProfileCardSlotTypeHero => {
                "k_EProfileCardSlotType_Hero"
            }
            EProfileCardSlotType::KEProfileCardSlotTypeEmoticon => {
                "k_EProfileCardSlotType_Emoticon"
            }
            EProfileCardSlotType::KEProfileCardSlotTypeTeam => {
                "k_EProfileCardSlotType_Team"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_EProfileCardSlotType_Empty" => Some(Self::KEProfileCardSlotTypeEmpty),
            "k_EProfileCardSlotType_Stat" => Some(Self::KEProfileCardSlotTypeStat),
            "k_EProfileCardSlotType_Trophy" => Some(Self::KEProfileCardSlotTypeTrophy),
            "k_EProfileCardSlotType_Item" => Some(Self::KEProfileCardSlotTypeItem),
            "k_EProfileCardSlotType_Hero" => Some(Self::KEProfileCardSlotTypeHero),
            "k_EProfileCardSlotType_Emoticon" => {
                Some(Self::KEProfileCardSlotTypeEmoticon)
            }
            "k_EProfileCardSlotType_Team" => Some(Self::KEProfileCardSlotTypeTeam),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EMatchGroupServerStatus {
    KEMatchGroupServerStatusOk = 0,
    KEMatchGroupServerStatusLimitedAvailability = 1,
    KEMatchGroupServerStatusOffline = 2,
}
impl EMatchGroupServerStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EMatchGroupServerStatus::KEMatchGroupServerStatusOk => {
                "k_EMatchGroupServerStatus_OK"
            }
            EMatchGroupServerStatus::KEMatchGroupServerStatusLimitedAvailability => {
                "k_EMatchGroupServerStatus_LimitedAvailability"
            }
            EMatchGroupServerStatus::KEMatchGroupServerStatusOffline => {
                "k_EMatchGroupServerStatus_Offline"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_EMatchGroupServerStatus_OK" => Some(Self::KEMatchGroupServerStatusOk),
            "k_EMatchGroupServerStatus_LimitedAvailability" => {
                Some(Self::KEMatchGroupServerStatusLimitedAvailability)
            }
            "k_EMatchGroupServerStatus_Offline" => {
                Some(Self::KEMatchGroupServerStatusOffline)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaCmPick {
    DotaCmRandom = 0,
    DotaCmGoodGuys = 1,
    DotaCmBadGuys = 2,
}
impl DotaCmPick {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaCmPick::DotaCmRandom => "DOTA_CM_RANDOM",
            DotaCmPick::DotaCmGoodGuys => "DOTA_CM_GOOD_GUYS",
            DotaCmPick::DotaCmBadGuys => "DOTA_CM_BAD_GUYS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_CM_RANDOM" => Some(Self::DotaCmRandom),
            "DOTA_CM_GOOD_GUYS" => Some(Self::DotaCmGoodGuys),
            "DOTA_CM_BAD_GUYS" => Some(Self::DotaCmBadGuys),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaLowPriorityBanType {
    DotaLowPriorityBanAbandon = 0,
    DotaLowPriorityBanReports = 1,
    DotaLowPriorityBanSecondaryAbandon = 2,
    DotaLowPriorityBanPreGameRole = 3,
}
impl DotaLowPriorityBanType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaLowPriorityBanType::DotaLowPriorityBanAbandon => {
                "DOTA_LOW_PRIORITY_BAN_ABANDON"
            }
            DotaLowPriorityBanType::DotaLowPriorityBanReports => {
                "DOTA_LOW_PRIORITY_BAN_REPORTS"
            }
            DotaLowPriorityBanType::DotaLowPriorityBanSecondaryAbandon => {
                "DOTA_LOW_PRIORITY_BAN_SECONDARY_ABANDON"
            }
            DotaLowPriorityBanType::DotaLowPriorityBanPreGameRole => {
                "DOTA_LOW_PRIORITY_BAN_PRE_GAME_ROLE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_LOW_PRIORITY_BAN_ABANDON" => Some(Self::DotaLowPriorityBanAbandon),
            "DOTA_LOW_PRIORITY_BAN_REPORTS" => Some(Self::DotaLowPriorityBanReports),
            "DOTA_LOW_PRIORITY_BAN_SECONDARY_ABANDON" => {
                Some(Self::DotaLowPriorityBanSecondaryAbandon)
            }
            "DOTA_LOW_PRIORITY_BAN_PRE_GAME_ROLE" => {
                Some(Self::DotaLowPriorityBanPreGameRole)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaLobbyReadyState {
    Undeclared = 0,
    Accepted = 1,
    Declined = 2,
}
impl DotaLobbyReadyState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaLobbyReadyState::Undeclared => "DOTALobbyReadyState_UNDECLARED",
            DotaLobbyReadyState::Accepted => "DOTALobbyReadyState_ACCEPTED",
            DotaLobbyReadyState::Declined => "DOTALobbyReadyState_DECLINED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTALobbyReadyState_UNDECLARED" => Some(Self::Undeclared),
            "DOTALobbyReadyState_ACCEPTED" => Some(Self::Accepted),
            "DOTALobbyReadyState_DECLINED" => Some(Self::Declined),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaJoinLobbyResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaJoinLobbyResult::DotaJoinResultSuccess => "DOTA_JOIN_RESULT_SUCCESS",
            DotaJoinLobbyResult::DotaJoinResultAlreadyInGame => {
                "DOTA_JOIN_RESULT_ALREADY_IN_GAME"
            }
            DotaJoinLobbyResult::DotaJoinResultInvalidLobby => {
                "DOTA_JOIN_RESULT_INVALID_LOBBY"
            }
            DotaJoinLobbyResult::DotaJoinResultIncorrectPassword => {
                "DOTA_JOIN_RESULT_INCORRECT_PASSWORD"
            }
            DotaJoinLobbyResult::DotaJoinResultAccessDenied => {
                "DOTA_JOIN_RESULT_ACCESS_DENIED"
            }
            DotaJoinLobbyResult::DotaJoinResultGenericError => {
                "DOTA_JOIN_RESULT_GENERIC_ERROR"
            }
            DotaJoinLobbyResult::DotaJoinResultIncorrectVersion => {
                "DOTA_JOIN_RESULT_INCORRECT_VERSION"
            }
            DotaJoinLobbyResult::DotaJoinResultInTeamParty => {
                "DOTA_JOIN_RESULT_IN_TEAM_PARTY"
            }
            DotaJoinLobbyResult::DotaJoinResultNoLobbyFound => {
                "DOTA_JOIN_RESULT_NO_LOBBY_FOUND"
            }
            DotaJoinLobbyResult::DotaJoinResultLobbyFull => "DOTA_JOIN_RESULT_LOBBY_FULL",
            DotaJoinLobbyResult::DotaJoinResultCustomGameIncorrectVersion => {
                "DOTA_JOIN_RESULT_CUSTOM_GAME_INCORRECT_VERSION"
            }
            DotaJoinLobbyResult::DotaJoinResultTimeout => "DOTA_JOIN_RESULT_TIMEOUT",
            DotaJoinLobbyResult::DotaJoinResultCustomGameCooldown => {
                "DOTA_JOIN_RESULT_CUSTOM_GAME_COOLDOWN"
            }
            DotaJoinLobbyResult::DotaJoinResultBusy => "DOTA_JOIN_RESULT_BUSY",
            DotaJoinLobbyResult::DotaJoinResultNoPlaytime => {
                "DOTA_JOIN_RESULT_NO_PLAYTIME"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_JOIN_RESULT_SUCCESS" => Some(Self::DotaJoinResultSuccess),
            "DOTA_JOIN_RESULT_ALREADY_IN_GAME" => Some(Self::DotaJoinResultAlreadyInGame),
            "DOTA_JOIN_RESULT_INVALID_LOBBY" => Some(Self::DotaJoinResultInvalidLobby),
            "DOTA_JOIN_RESULT_INCORRECT_PASSWORD" => {
                Some(Self::DotaJoinResultIncorrectPassword)
            }
            "DOTA_JOIN_RESULT_ACCESS_DENIED" => Some(Self::DotaJoinResultAccessDenied),
            "DOTA_JOIN_RESULT_GENERIC_ERROR" => Some(Self::DotaJoinResultGenericError),
            "DOTA_JOIN_RESULT_INCORRECT_VERSION" => {
                Some(Self::DotaJoinResultIncorrectVersion)
            }
            "DOTA_JOIN_RESULT_IN_TEAM_PARTY" => Some(Self::DotaJoinResultInTeamParty),
            "DOTA_JOIN_RESULT_NO_LOBBY_FOUND" => Some(Self::DotaJoinResultNoLobbyFound),
            "DOTA_JOIN_RESULT_LOBBY_FULL" => Some(Self::DotaJoinResultLobbyFull),
            "DOTA_JOIN_RESULT_CUSTOM_GAME_INCORRECT_VERSION" => {
                Some(Self::DotaJoinResultCustomGameIncorrectVersion)
            }
            "DOTA_JOIN_RESULT_TIMEOUT" => Some(Self::DotaJoinResultTimeout),
            "DOTA_JOIN_RESULT_CUSTOM_GAME_COOLDOWN" => {
                Some(Self::DotaJoinResultCustomGameCooldown)
            }
            "DOTA_JOIN_RESULT_BUSY" => Some(Self::DotaJoinResultBusy),
            "DOTA_JOIN_RESULT_NO_PLAYTIME" => Some(Self::DotaJoinResultNoPlaytime),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaSelectionPriorityRules {
    KDotaSelectionPriorityRulesManual = 0,
    KDotaSelectionPriorityRulesAutomatic = 1,
}
impl DotaSelectionPriorityRules {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaSelectionPriorityRules::KDotaSelectionPriorityRulesManual => {
                "k_DOTASelectionPriorityRules_Manual"
            }
            DotaSelectionPriorityRules::KDotaSelectionPriorityRulesAutomatic => {
                "k_DOTASelectionPriorityRules_Automatic"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_DOTASelectionPriorityRules_Manual" => {
                Some(Self::KDotaSelectionPriorityRulesManual)
            }
            "k_DOTASelectionPriorityRules_Automatic" => {
                Some(Self::KDotaSelectionPriorityRulesAutomatic)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaSelectionPriorityChoice {
    KDotaSelectionPriorityChoiceInvalid = 0,
    KDotaSelectionPriorityChoiceFirstPick = 1,
    KDotaSelectionPriorityChoiceSecondPick = 2,
    KDotaSelectionPriorityChoiceRadiant = 3,
    KDotaSelectionPriorityChoiceDire = 4,
}
impl DotaSelectionPriorityChoice {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaSelectionPriorityChoice::KDotaSelectionPriorityChoiceInvalid => {
                "k_DOTASelectionPriorityChoice_Invalid"
            }
            DotaSelectionPriorityChoice::KDotaSelectionPriorityChoiceFirstPick => {
                "k_DOTASelectionPriorityChoice_FirstPick"
            }
            DotaSelectionPriorityChoice::KDotaSelectionPriorityChoiceSecondPick => {
                "k_DOTASelectionPriorityChoice_SecondPick"
            }
            DotaSelectionPriorityChoice::KDotaSelectionPriorityChoiceRadiant => {
                "k_DOTASelectionPriorityChoice_Radiant"
            }
            DotaSelectionPriorityChoice::KDotaSelectionPriorityChoiceDire => {
                "k_DOTASelectionPriorityChoice_Dire"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_DOTASelectionPriorityChoice_Invalid" => {
                Some(Self::KDotaSelectionPriorityChoiceInvalid)
            }
            "k_DOTASelectionPriorityChoice_FirstPick" => {
                Some(Self::KDotaSelectionPriorityChoiceFirstPick)
            }
            "k_DOTASelectionPriorityChoice_SecondPick" => {
                Some(Self::KDotaSelectionPriorityChoiceSecondPick)
            }
            "k_DOTASelectionPriorityChoice_Radiant" => {
                Some(Self::KDotaSelectionPriorityChoiceRadiant)
            }
            "k_DOTASelectionPriorityChoice_Dire" => {
                Some(Self::KDotaSelectionPriorityChoiceDire)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaMatchVote {
    Invalid = 0,
    Positive = 1,
    Negative = 2,
}
impl DotaMatchVote {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaMatchVote::Invalid => "DOTAMatchVote_INVALID",
            DotaMatchVote::Positive => "DOTAMatchVote_POSITIVE",
            DotaMatchVote::Negative => "DOTAMatchVote_NEGATIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTAMatchVote_INVALID" => Some(Self::Invalid),
            "DOTAMatchVote_POSITIVE" => Some(Self::Positive),
            "DOTAMatchVote_NEGATIVE" => Some(Self::Negative),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaLobbyVisibility {
    Public = 0,
    Friends = 1,
    Unlisted = 2,
}
impl DotaLobbyVisibility {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaLobbyVisibility::Public => "DOTALobbyVisibility_Public",
            DotaLobbyVisibility::Friends => "DOTALobbyVisibility_Friends",
            DotaLobbyVisibility::Unlisted => "DOTALobbyVisibility_Unlisted",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTALobbyVisibility_Public" => Some(Self::Public),
            "DOTALobbyVisibility_Friends" => Some(Self::Friends),
            "DOTALobbyVisibility_Unlisted" => Some(Self::Unlisted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EdotaPlayerMmrType {
    KEdotaPlayerMmrTypeInvalid = 0,
    KEdotaPlayerMmrTypeGeneralHidden = 1,
    KEdotaPlayerMmrTypeGeneralCompetitive = 3,
}
impl EdotaPlayerMmrType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EdotaPlayerMmrType::KEdotaPlayerMmrTypeInvalid => {
                "k_EDOTAPlayerMMRType_Invalid"
            }
            EdotaPlayerMmrType::KEdotaPlayerMmrTypeGeneralHidden => {
                "k_EDOTAPlayerMMRType_GeneralHidden"
            }
            EdotaPlayerMmrType::KEdotaPlayerMmrTypeGeneralCompetitive => {
                "k_EDOTAPlayerMMRType_GeneralCompetitive"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_EDOTAPlayerMMRType_Invalid" => Some(Self::KEdotaPlayerMmrTypeInvalid),
            "k_EDOTAPlayerMMRType_GeneralHidden" => {
                Some(Self::KEdotaPlayerMmrTypeGeneralHidden)
            }
            "k_EDOTAPlayerMMRType_GeneralCompetitive" => {
                Some(Self::KEdotaPlayerMmrTypeGeneralCompetitive)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EdotammrBoostType {
    KEdotammrBoostTypeNone = 0,
    KEdotammrBoostTypeLeader = 1,
    KEdotammrBoostTypeFollower = 2,
}
impl EdotammrBoostType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EdotammrBoostType::KEdotammrBoostTypeNone => "k_EDOTAMMRBoostType_None",
            EdotammrBoostType::KEdotammrBoostTypeLeader => "k_EDOTAMMRBoostType_Leader",
            EdotammrBoostType::KEdotammrBoostTypeFollower => {
                "k_EDOTAMMRBoostType_Follower"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_EDOTAMMRBoostType_None" => Some(Self::KEdotammrBoostTypeNone),
            "k_EDOTAMMRBoostType_Leader" => Some(Self::KEdotammrBoostTypeLeader),
            "k_EDOTAMMRBoostType_Follower" => Some(Self::KEdotammrBoostTypeFollower),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl MatchType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MatchType::Casual => "MATCH_TYPE_CASUAL",
            MatchType::CoopBots => "MATCH_TYPE_COOP_BOTS",
            MatchType::Competitive => "MATCH_TYPE_COMPETITIVE",
            MatchType::WeekendTourney => "MATCH_TYPE_WEEKEND_TOURNEY",
            MatchType::Event => "MATCH_TYPE_EVENT",
            MatchType::CoachesChallenge => "MATCH_TYPE_COACHES_CHALLENGE",
            MatchType::NewPlayerPool => "MATCH_TYPE_NEW_PLAYER_POOL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MATCH_TYPE_CASUAL" => Some(Self::Casual),
            "MATCH_TYPE_COOP_BOTS" => Some(Self::CoopBots),
            "MATCH_TYPE_COMPETITIVE" => Some(Self::Competitive),
            "MATCH_TYPE_WEEKEND_TOURNEY" => Some(Self::WeekendTourney),
            "MATCH_TYPE_EVENT" => Some(Self::Event),
            "MATCH_TYPE_COACHES_CHALLENGE" => Some(Self::CoachesChallenge),
            "MATCH_TYPE_NEW_PLAYER_POOL" => Some(Self::NewPlayerPool),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaBotDifficulty {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaBotDifficulty::BotDifficultyPassive => "BOT_DIFFICULTY_PASSIVE",
            DotaBotDifficulty::BotDifficultyEasy => "BOT_DIFFICULTY_EASY",
            DotaBotDifficulty::BotDifficultyMedium => "BOT_DIFFICULTY_MEDIUM",
            DotaBotDifficulty::BotDifficultyHard => "BOT_DIFFICULTY_HARD",
            DotaBotDifficulty::BotDifficultyUnfair => "BOT_DIFFICULTY_UNFAIR",
            DotaBotDifficulty::BotDifficultyInvalid => "BOT_DIFFICULTY_INVALID",
            DotaBotDifficulty::BotDifficultyExtra1 => "BOT_DIFFICULTY_EXTRA1",
            DotaBotDifficulty::BotDifficultyExtra2 => "BOT_DIFFICULTY_EXTRA2",
            DotaBotDifficulty::BotDifficultyExtra3 => "BOT_DIFFICULTY_EXTRA3",
            DotaBotDifficulty::BotDifficultyNpx => "BOT_DIFFICULTY_NPX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BOT_DIFFICULTY_PASSIVE" => Some(Self::BotDifficultyPassive),
            "BOT_DIFFICULTY_EASY" => Some(Self::BotDifficultyEasy),
            "BOT_DIFFICULTY_MEDIUM" => Some(Self::BotDifficultyMedium),
            "BOT_DIFFICULTY_HARD" => Some(Self::BotDifficultyHard),
            "BOT_DIFFICULTY_UNFAIR" => Some(Self::BotDifficultyUnfair),
            "BOT_DIFFICULTY_INVALID" => Some(Self::BotDifficultyInvalid),
            "BOT_DIFFICULTY_EXTRA1" => Some(Self::BotDifficultyExtra1),
            "BOT_DIFFICULTY_EXTRA2" => Some(Self::BotDifficultyExtra2),
            "BOT_DIFFICULTY_EXTRA3" => Some(Self::BotDifficultyExtra3),
            "BOT_DIFFICULTY_NPX" => Some(Self::BotDifficultyNpx),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaBotMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaBotMode::None => "DOTA_BOT_MODE_NONE",
            DotaBotMode::Laning => "DOTA_BOT_MODE_LANING",
            DotaBotMode::Attack => "DOTA_BOT_MODE_ATTACK",
            DotaBotMode::Roam => "DOTA_BOT_MODE_ROAM",
            DotaBotMode::Retreat => "DOTA_BOT_MODE_RETREAT",
            DotaBotMode::SecretShop => "DOTA_BOT_MODE_SECRET_SHOP",
            DotaBotMode::SideShop => "DOTA_BOT_MODE_SIDE_SHOP",
            DotaBotMode::Rune => "DOTA_BOT_MODE_RUNE",
            DotaBotMode::PushTowerTop => "DOTA_BOT_MODE_PUSH_TOWER_TOP",
            DotaBotMode::PushTowerMid => "DOTA_BOT_MODE_PUSH_TOWER_MID",
            DotaBotMode::PushTowerBot => "DOTA_BOT_MODE_PUSH_TOWER_BOT",
            DotaBotMode::DefendTowerTop => "DOTA_BOT_MODE_DEFEND_TOWER_TOP",
            DotaBotMode::DefendTowerMid => "DOTA_BOT_MODE_DEFEND_TOWER_MID",
            DotaBotMode::DefendTowerBot => "DOTA_BOT_MODE_DEFEND_TOWER_BOT",
            DotaBotMode::Assemble => "DOTA_BOT_MODE_ASSEMBLE",
            DotaBotMode::AssembleWithHumans => "DOTA_BOT_MODE_ASSEMBLE_WITH_HUMANS",
            DotaBotMode::TeamRoam => "DOTA_BOT_MODE_TEAM_ROAM",
            DotaBotMode::Farm => "DOTA_BOT_MODE_FARM",
            DotaBotMode::DefendAlly => "DOTA_BOT_MODE_DEFEND_ALLY",
            DotaBotMode::EvasiveManeuvers => "DOTA_BOT_MODE_EVASIVE_MANEUVERS",
            DotaBotMode::Roshan => "DOTA_BOT_MODE_ROSHAN",
            DotaBotMode::Item => "DOTA_BOT_MODE_ITEM",
            DotaBotMode::Ward => "DOTA_BOT_MODE_WARD",
            DotaBotMode::Companion => "DOTA_BOT_MODE_COMPANION",
            DotaBotMode::TutorialBoss => "DOTA_BOT_MODE_TUTORIAL_BOSS",
            DotaBotMode::Minion => "DOTA_BOT_MODE_MINION",
            DotaBotMode::Outpost => "DOTA_BOT_MODE_OUTPOST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_BOT_MODE_NONE" => Some(Self::None),
            "DOTA_BOT_MODE_LANING" => Some(Self::Laning),
            "DOTA_BOT_MODE_ATTACK" => Some(Self::Attack),
            "DOTA_BOT_MODE_ROAM" => Some(Self::Roam),
            "DOTA_BOT_MODE_RETREAT" => Some(Self::Retreat),
            "DOTA_BOT_MODE_SECRET_SHOP" => Some(Self::SecretShop),
            "DOTA_BOT_MODE_SIDE_SHOP" => Some(Self::SideShop),
            "DOTA_BOT_MODE_RUNE" => Some(Self::Rune),
            "DOTA_BOT_MODE_PUSH_TOWER_TOP" => Some(Self::PushTowerTop),
            "DOTA_BOT_MODE_PUSH_TOWER_MID" => Some(Self::PushTowerMid),
            "DOTA_BOT_MODE_PUSH_TOWER_BOT" => Some(Self::PushTowerBot),
            "DOTA_BOT_MODE_DEFEND_TOWER_TOP" => Some(Self::DefendTowerTop),
            "DOTA_BOT_MODE_DEFEND_TOWER_MID" => Some(Self::DefendTowerMid),
            "DOTA_BOT_MODE_DEFEND_TOWER_BOT" => Some(Self::DefendTowerBot),
            "DOTA_BOT_MODE_ASSEMBLE" => Some(Self::Assemble),
            "DOTA_BOT_MODE_ASSEMBLE_WITH_HUMANS" => Some(Self::AssembleWithHumans),
            "DOTA_BOT_MODE_TEAM_ROAM" => Some(Self::TeamRoam),
            "DOTA_BOT_MODE_FARM" => Some(Self::Farm),
            "DOTA_BOT_MODE_DEFEND_ALLY" => Some(Self::DefendAlly),
            "DOTA_BOT_MODE_EVASIVE_MANEUVERS" => Some(Self::EvasiveManeuvers),
            "DOTA_BOT_MODE_ROSHAN" => Some(Self::Roshan),
            "DOTA_BOT_MODE_ITEM" => Some(Self::Item),
            "DOTA_BOT_MODE_WARD" => Some(Self::Ward),
            "DOTA_BOT_MODE_COMPANION" => Some(Self::Companion),
            "DOTA_BOT_MODE_TUTORIAL_BOSS" => Some(Self::TutorialBoss),
            "DOTA_BOT_MODE_MINION" => Some(Self::Minion),
            "DOTA_BOT_MODE_OUTPOST" => Some(Self::Outpost),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl MatchLanguages {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MatchLanguages::MatchLanguageInvalid => "MATCH_LANGUAGE_INVALID",
            MatchLanguages::MatchLanguageEnglish => "MATCH_LANGUAGE_ENGLISH",
            MatchLanguages::MatchLanguageRussian => "MATCH_LANGUAGE_RUSSIAN",
            MatchLanguages::MatchLanguageChinese => "MATCH_LANGUAGE_CHINESE",
            MatchLanguages::MatchLanguageKorean => "MATCH_LANGUAGE_KOREAN",
            MatchLanguages::MatchLanguageSpanish => "MATCH_LANGUAGE_SPANISH",
            MatchLanguages::MatchLanguagePortuguese => "MATCH_LANGUAGE_PORTUGUESE",
            MatchLanguages::MatchLanguageEnglish2 => "MATCH_LANGUAGE_ENGLISH2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MATCH_LANGUAGE_INVALID" => Some(Self::MatchLanguageInvalid),
            "MATCH_LANGUAGE_ENGLISH" => Some(Self::MatchLanguageEnglish),
            "MATCH_LANGUAGE_RUSSIAN" => Some(Self::MatchLanguageRussian),
            "MATCH_LANGUAGE_CHINESE" => Some(Self::MatchLanguageChinese),
            "MATCH_LANGUAGE_KOREAN" => Some(Self::MatchLanguageKorean),
            "MATCH_LANGUAGE_SPANISH" => Some(Self::MatchLanguageSpanish),
            "MATCH_LANGUAGE_PORTUGUESE" => Some(Self::MatchLanguagePortuguese),
            "MATCH_LANGUAGE_ENGLISH2" => Some(Self::MatchLanguageEnglish2),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl ETourneyQueueDeadlineState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ETourneyQueueDeadlineState::KETourneyQueueDeadlineStateNormal => {
                "k_ETourneyQueueDeadlineState_Normal"
            }
            ETourneyQueueDeadlineState::KETourneyQueueDeadlineStateMissed => {
                "k_ETourneyQueueDeadlineState_Missed"
            }
            ETourneyQueueDeadlineState::KETourneyQueueDeadlineStateExpiredOk => {
                "k_ETourneyQueueDeadlineState_ExpiredOK"
            }
            ETourneyQueueDeadlineState::KETourneyQueueDeadlineStateSeekingBye => {
                "k_ETourneyQueueDeadlineState_SeekingBye"
            }
            ETourneyQueueDeadlineState::KETourneyQueueDeadlineStateEligibleForRefund => {
                "k_ETourneyQueueDeadlineState_EligibleForRefund"
            }
            ETourneyQueueDeadlineState::KETourneyQueueDeadlineStateNa => {
                "k_ETourneyQueueDeadlineState_NA"
            }
            ETourneyQueueDeadlineState::KETourneyQueueDeadlineStateExpiringSoon => {
                "k_ETourneyQueueDeadlineState_ExpiringSoon"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_ETourneyQueueDeadlineState_Normal" => {
                Some(Self::KETourneyQueueDeadlineStateNormal)
            }
            "k_ETourneyQueueDeadlineState_Missed" => {
                Some(Self::KETourneyQueueDeadlineStateMissed)
            }
            "k_ETourneyQueueDeadlineState_ExpiredOK" => {
                Some(Self::KETourneyQueueDeadlineStateExpiredOk)
            }
            "k_ETourneyQueueDeadlineState_SeekingBye" => {
                Some(Self::KETourneyQueueDeadlineStateSeekingBye)
            }
            "k_ETourneyQueueDeadlineState_EligibleForRefund" => {
                Some(Self::KETourneyQueueDeadlineStateEligibleForRefund)
            }
            "k_ETourneyQueueDeadlineState_NA" => {
                Some(Self::KETourneyQueueDeadlineStateNa)
            }
            "k_ETourneyQueueDeadlineState_ExpiringSoon" => {
                Some(Self::KETourneyQueueDeadlineStateExpiringSoon)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl EMatchOutcome {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EMatchOutcome::KEMatchOutcomeUnknown => "k_EMatchOutcome_Unknown",
            EMatchOutcome::KEMatchOutcomeRadVictory => "k_EMatchOutcome_RadVictory",
            EMatchOutcome::KEMatchOutcomeDireVictory => "k_EMatchOutcome_DireVictory",
            EMatchOutcome::KEMatchOutcomeNeutralVictory => {
                "k_EMatchOutcome_NeutralVictory"
            }
            EMatchOutcome::KEMatchOutcomeNoTeamWinner => "k_EMatchOutcome_NoTeamWinner",
            EMatchOutcome::KEMatchOutcomeCustom1Victory => {
                "k_EMatchOutcome_Custom1Victory"
            }
            EMatchOutcome::KEMatchOutcomeCustom2Victory => {
                "k_EMatchOutcome_Custom2Victory"
            }
            EMatchOutcome::KEMatchOutcomeCustom3Victory => {
                "k_EMatchOutcome_Custom3Victory"
            }
            EMatchOutcome::KEMatchOutcomeCustom4Victory => {
                "k_EMatchOutcome_Custom4Victory"
            }
            EMatchOutcome::KEMatchOutcomeCustom5Victory => {
                "k_EMatchOutcome_Custom5Victory"
            }
            EMatchOutcome::KEMatchOutcomeCustom6Victory => {
                "k_EMatchOutcome_Custom6Victory"
            }
            EMatchOutcome::KEMatchOutcomeCustom7Victory => {
                "k_EMatchOutcome_Custom7Victory"
            }
            EMatchOutcome::KEMatchOutcomeCustom8Victory => {
                "k_EMatchOutcome_Custom8Victory"
            }
            EMatchOutcome::KEMatchOutcomeNotScoredPoorNetworkConditions => {
                "k_EMatchOutcome_NotScored_PoorNetworkConditions"
            }
            EMatchOutcome::KEMatchOutcomeNotScoredLeaver => {
                "k_EMatchOutcome_NotScored_Leaver"
            }
            EMatchOutcome::KEMatchOutcomeNotScoredServerCrash => {
                "k_EMatchOutcome_NotScored_ServerCrash"
            }
            EMatchOutcome::KEMatchOutcomeNotScoredNeverStarted => {
                "k_EMatchOutcome_NotScored_NeverStarted"
            }
            EMatchOutcome::KEMatchOutcomeNotScoredCanceled => {
                "k_EMatchOutcome_NotScored_Canceled"
            }
            EMatchOutcome::KEMatchOutcomeNotScoredSuspicious => {
                "k_EMatchOutcome_NotScored_Suspicious"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_EMatchOutcome_Unknown" => Some(Self::KEMatchOutcomeUnknown),
            "k_EMatchOutcome_RadVictory" => Some(Self::KEMatchOutcomeRadVictory),
            "k_EMatchOutcome_DireVictory" => Some(Self::KEMatchOutcomeDireVictory),
            "k_EMatchOutcome_NeutralVictory" => Some(Self::KEMatchOutcomeNeutralVictory),
            "k_EMatchOutcome_NoTeamWinner" => Some(Self::KEMatchOutcomeNoTeamWinner),
            "k_EMatchOutcome_Custom1Victory" => Some(Self::KEMatchOutcomeCustom1Victory),
            "k_EMatchOutcome_Custom2Victory" => Some(Self::KEMatchOutcomeCustom2Victory),
            "k_EMatchOutcome_Custom3Victory" => Some(Self::KEMatchOutcomeCustom3Victory),
            "k_EMatchOutcome_Custom4Victory" => Some(Self::KEMatchOutcomeCustom4Victory),
            "k_EMatchOutcome_Custom5Victory" => Some(Self::KEMatchOutcomeCustom5Victory),
            "k_EMatchOutcome_Custom6Victory" => Some(Self::KEMatchOutcomeCustom6Victory),
            "k_EMatchOutcome_Custom7Victory" => Some(Self::KEMatchOutcomeCustom7Victory),
            "k_EMatchOutcome_Custom8Victory" => Some(Self::KEMatchOutcomeCustom8Victory),
            "k_EMatchOutcome_NotScored_PoorNetworkConditions" => {
                Some(Self::KEMatchOutcomeNotScoredPoorNetworkConditions)
            }
            "k_EMatchOutcome_NotScored_Leaver" => {
                Some(Self::KEMatchOutcomeNotScoredLeaver)
            }
            "k_EMatchOutcome_NotScored_ServerCrash" => {
                Some(Self::KEMatchOutcomeNotScoredServerCrash)
            }
            "k_EMatchOutcome_NotScored_NeverStarted" => {
                Some(Self::KEMatchOutcomeNotScoredNeverStarted)
            }
            "k_EMatchOutcome_NotScored_Canceled" => {
                Some(Self::KEMatchOutcomeNotScoredCanceled)
            }
            "k_EMatchOutcome_NotScored_Suspicious" => {
                Some(Self::KEMatchOutcomeNotScoredSuspicious)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELaneType {
    LaneTypeUnknown = 0,
    LaneTypeSafe = 1,
    LaneTypeOff = 2,
    LaneTypeMid = 3,
    LaneTypeJungle = 4,
    LaneTypeRoam = 5,
}
impl ELaneType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ELaneType::LaneTypeUnknown => "LANE_TYPE_UNKNOWN",
            ELaneType::LaneTypeSafe => "LANE_TYPE_SAFE",
            ELaneType::LaneTypeOff => "LANE_TYPE_OFF",
            ELaneType::LaneTypeMid => "LANE_TYPE_MID",
            ELaneType::LaneTypeJungle => "LANE_TYPE_JUNGLE",
            ELaneType::LaneTypeRoam => "LANE_TYPE_ROAM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LANE_TYPE_UNKNOWN" => Some(Self::LaneTypeUnknown),
            "LANE_TYPE_SAFE" => Some(Self::LaneTypeSafe),
            "LANE_TYPE_OFF" => Some(Self::LaneTypeOff),
            "LANE_TYPE_MID" => Some(Self::LaneTypeMid),
            "LANE_TYPE_JUNGLE" => Some(Self::LaneTypeJungle),
            "LANE_TYPE_ROAM" => Some(Self::LaneTypeRoam),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl EBadgeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EBadgeType::KEBadgeTypeInvalid => "k_EBadgeType_Invalid",
            EBadgeType::KEBadgeTypeTi7Midweek => "k_EBadgeType_TI7_Midweek",
            EBadgeType::KEBadgeTypeTi7Finals => "k_EBadgeType_TI7_Finals",
            EBadgeType::KEBadgeTypeTi7AllEvent => "k_EBadgeType_TI7_AllEvent",
            EBadgeType::KEBadgeTypeTi8Midweek => "k_EBadgeType_TI8_Midweek",
            EBadgeType::KEBadgeTypeTi8Finals => "k_EBadgeType_TI8_Finals",
            EBadgeType::KEBadgeTypeTi8AllEvent => "k_EBadgeType_TI8_AllEvent",
            EBadgeType::KEBadgeTypeTi10 => "k_EBadgeType_TI10",
            EBadgeType::KEBadgeTypeTi11PlayoffsDay1 => "k_EBadgeType_TI11_PlayoffsDay1",
            EBadgeType::KEBadgeTypeTi11PlayoffsDay2 => "k_EBadgeType_TI11_PlayoffsDay2",
            EBadgeType::KEBadgeTypeTi11PlayoffsDay3 => "k_EBadgeType_TI11_PlayoffsDay3",
            EBadgeType::KEBadgeTypeTi11PlayoffsDay4 => "k_EBadgeType_TI11_PlayoffsDay4",
            EBadgeType::KEBadgeTypeTi11FinalsWeekend => "k_EBadgeType_TI11_FinalsWeekend",
            EBadgeType::KEBadgeTypeTi12PlayoffsDay1 => "k_EBadgeType_TI12_PlayoffsDay1",
            EBadgeType::KEBadgeTypeTi12PlayoffsDay2 => "k_EBadgeType_TI12_PlayoffsDay2",
            EBadgeType::KEBadgeTypeTi12PlayoffsDay3 => "k_EBadgeType_TI12_PlayoffsDay3",
            EBadgeType::KEBadgeTypeTi12FinalsWeekend => "k_EBadgeType_TI12_FinalsWeekend",
            EBadgeType::KEBadgeTypeTi12Special => "k_EBadgeType_TI12_Special",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_EBadgeType_Invalid" => Some(Self::KEBadgeTypeInvalid),
            "k_EBadgeType_TI7_Midweek" => Some(Self::KEBadgeTypeTi7Midweek),
            "k_EBadgeType_TI7_Finals" => Some(Self::KEBadgeTypeTi7Finals),
            "k_EBadgeType_TI7_AllEvent" => Some(Self::KEBadgeTypeTi7AllEvent),
            "k_EBadgeType_TI8_Midweek" => Some(Self::KEBadgeTypeTi8Midweek),
            "k_EBadgeType_TI8_Finals" => Some(Self::KEBadgeTypeTi8Finals),
            "k_EBadgeType_TI8_AllEvent" => Some(Self::KEBadgeTypeTi8AllEvent),
            "k_EBadgeType_TI10" => Some(Self::KEBadgeTypeTi10),
            "k_EBadgeType_TI11_PlayoffsDay1" => Some(Self::KEBadgeTypeTi11PlayoffsDay1),
            "k_EBadgeType_TI11_PlayoffsDay2" => Some(Self::KEBadgeTypeTi11PlayoffsDay2),
            "k_EBadgeType_TI11_PlayoffsDay3" => Some(Self::KEBadgeTypeTi11PlayoffsDay3),
            "k_EBadgeType_TI11_PlayoffsDay4" => Some(Self::KEBadgeTypeTi11PlayoffsDay4),
            "k_EBadgeType_TI11_FinalsWeekend" => Some(Self::KEBadgeTypeTi11FinalsWeekend),
            "k_EBadgeType_TI12_PlayoffsDay1" => Some(Self::KEBadgeTypeTi12PlayoffsDay1),
            "k_EBadgeType_TI12_PlayoffsDay2" => Some(Self::KEBadgeTypeTi12PlayoffsDay2),
            "k_EBadgeType_TI12_PlayoffsDay3" => Some(Self::KEBadgeTypeTi12PlayoffsDay3),
            "k_EBadgeType_TI12_FinalsWeekend" => Some(Self::KEBadgeTypeTi12FinalsWeekend),
            "k_EBadgeType_TI12_Special" => Some(Self::KEBadgeTypeTi12Special),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl ELeagueStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ELeagueStatus::LeagueStatusUnset => "LEAGUE_STATUS_UNSET",
            ELeagueStatus::LeagueStatusUnsubmitted => "LEAGUE_STATUS_UNSUBMITTED",
            ELeagueStatus::LeagueStatusSubmitted => "LEAGUE_STATUS_SUBMITTED",
            ELeagueStatus::LeagueStatusAccepted => "LEAGUE_STATUS_ACCEPTED",
            ELeagueStatus::LeagueStatusRejected => "LEAGUE_STATUS_REJECTED",
            ELeagueStatus::LeagueStatusConcluded => "LEAGUE_STATUS_CONCLUDED",
            ELeagueStatus::LeagueStatusDeleted => "LEAGUE_STATUS_DELETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEAGUE_STATUS_UNSET" => Some(Self::LeagueStatusUnset),
            "LEAGUE_STATUS_UNSUBMITTED" => Some(Self::LeagueStatusUnsubmitted),
            "LEAGUE_STATUS_SUBMITTED" => Some(Self::LeagueStatusSubmitted),
            "LEAGUE_STATUS_ACCEPTED" => Some(Self::LeagueStatusAccepted),
            "LEAGUE_STATUS_REJECTED" => Some(Self::LeagueStatusRejected),
            "LEAGUE_STATUS_CONCLUDED" => Some(Self::LeagueStatusConcluded),
            "LEAGUE_STATUS_DELETED" => Some(Self::LeagueStatusDeleted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl ELeagueRegion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ELeagueRegion::LeagueRegionUnset => "LEAGUE_REGION_UNSET",
            ELeagueRegion::LeagueRegionNa => "LEAGUE_REGION_NA",
            ELeagueRegion::LeagueRegionSa => "LEAGUE_REGION_SA",
            ELeagueRegion::LeagueRegionWeu => "LEAGUE_REGION_WEU",
            ELeagueRegion::LeagueRegionEeu => "LEAGUE_REGION_EEU",
            ELeagueRegion::LeagueRegionChina => "LEAGUE_REGION_CHINA",
            ELeagueRegion::LeagueRegionSea => "LEAGUE_REGION_SEA",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEAGUE_REGION_UNSET" => Some(Self::LeagueRegionUnset),
            "LEAGUE_REGION_NA" => Some(Self::LeagueRegionNa),
            "LEAGUE_REGION_SA" => Some(Self::LeagueRegionSa),
            "LEAGUE_REGION_WEU" => Some(Self::LeagueRegionWeu),
            "LEAGUE_REGION_EEU" => Some(Self::LeagueRegionEeu),
            "LEAGUE_REGION_CHINA" => Some(Self::LeagueRegionChina),
            "LEAGUE_REGION_SEA" => Some(Self::LeagueRegionSea),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl ELeagueTier {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ELeagueTier::LeagueTierUnset => "LEAGUE_TIER_UNSET",
            ELeagueTier::LeagueTierAmateur => "LEAGUE_TIER_AMATEUR",
            ELeagueTier::LeagueTierProfessional => "LEAGUE_TIER_PROFESSIONAL",
            ELeagueTier::LeagueTierMinor => "LEAGUE_TIER_MINOR",
            ELeagueTier::LeagueTierMajor => "LEAGUE_TIER_MAJOR",
            ELeagueTier::LeagueTierInternational => "LEAGUE_TIER_INTERNATIONAL",
            ELeagueTier::LeagueTierDpcQualifier => "LEAGUE_TIER_DPC_QUALIFIER",
            ELeagueTier::LeagueTierDpcLeagueQualifier => {
                "LEAGUE_TIER_DPC_LEAGUE_QUALIFIER"
            }
            ELeagueTier::LeagueTierDpcLeague => "LEAGUE_TIER_DPC_LEAGUE",
            ELeagueTier::LeagueTierDpcLeagueFinals => "LEAGUE_TIER_DPC_LEAGUE_FINALS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEAGUE_TIER_UNSET" => Some(Self::LeagueTierUnset),
            "LEAGUE_TIER_AMATEUR" => Some(Self::LeagueTierAmateur),
            "LEAGUE_TIER_PROFESSIONAL" => Some(Self::LeagueTierProfessional),
            "LEAGUE_TIER_MINOR" => Some(Self::LeagueTierMinor),
            "LEAGUE_TIER_MAJOR" => Some(Self::LeagueTierMajor),
            "LEAGUE_TIER_INTERNATIONAL" => Some(Self::LeagueTierInternational),
            "LEAGUE_TIER_DPC_QUALIFIER" => Some(Self::LeagueTierDpcQualifier),
            "LEAGUE_TIER_DPC_LEAGUE_QUALIFIER" => {
                Some(Self::LeagueTierDpcLeagueQualifier)
            }
            "LEAGUE_TIER_DPC_LEAGUE" => Some(Self::LeagueTierDpcLeague),
            "LEAGUE_TIER_DPC_LEAGUE_FINALS" => Some(Self::LeagueTierDpcLeagueFinals),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELeagueTierCategory {
    LeagueTierCategoryAmateur = 1,
    LeagueTierCategoryProfessional = 2,
    LeagueTierCategoryDpc = 3,
}
impl ELeagueTierCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ELeagueTierCategory::LeagueTierCategoryAmateur => {
                "LEAGUE_TIER_CATEGORY_AMATEUR"
            }
            ELeagueTierCategory::LeagueTierCategoryProfessional => {
                "LEAGUE_TIER_CATEGORY_PROFESSIONAL"
            }
            ELeagueTierCategory::LeagueTierCategoryDpc => "LEAGUE_TIER_CATEGORY_DPC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEAGUE_TIER_CATEGORY_AMATEUR" => Some(Self::LeagueTierCategoryAmateur),
            "LEAGUE_TIER_CATEGORY_PROFESSIONAL" => {
                Some(Self::LeagueTierCategoryProfessional)
            }
            "LEAGUE_TIER_CATEGORY_DPC" => Some(Self::LeagueTierCategoryDpc),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELeagueDivision {
    LeagueDivisionUnset = 0,
    LeagueDivisionI = 1,
    LeagueDivisionIi = 2,
}
impl ELeagueDivision {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ELeagueDivision::LeagueDivisionUnset => "LEAGUE_DIVISION_UNSET",
            ELeagueDivision::LeagueDivisionI => "LEAGUE_DIVISION_I",
            ELeagueDivision::LeagueDivisionIi => "LEAGUE_DIVISION_II",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEAGUE_DIVISION_UNSET" => Some(Self::LeagueDivisionUnset),
            "LEAGUE_DIVISION_I" => Some(Self::LeagueDivisionI),
            "LEAGUE_DIVISION_II" => Some(Self::LeagueDivisionIi),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELeagueBroadcastProvider {
    LeagueBroadcastUnknown = 0,
    LeagueBroadcastSteam = 1,
    LeagueBroadcastTwitch = 2,
    LeagueBroadcastYoutube = 3,
    LeagueBroadcastOther = 100,
}
impl ELeagueBroadcastProvider {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ELeagueBroadcastProvider::LeagueBroadcastUnknown => {
                "LEAGUE_BROADCAST_UNKNOWN"
            }
            ELeagueBroadcastProvider::LeagueBroadcastSteam => "LEAGUE_BROADCAST_STEAM",
            ELeagueBroadcastProvider::LeagueBroadcastTwitch => "LEAGUE_BROADCAST_TWITCH",
            ELeagueBroadcastProvider::LeagueBroadcastYoutube => {
                "LEAGUE_BROADCAST_YOUTUBE"
            }
            ELeagueBroadcastProvider::LeagueBroadcastOther => "LEAGUE_BROADCAST_OTHER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEAGUE_BROADCAST_UNKNOWN" => Some(Self::LeagueBroadcastUnknown),
            "LEAGUE_BROADCAST_STEAM" => Some(Self::LeagueBroadcastSteam),
            "LEAGUE_BROADCAST_TWITCH" => Some(Self::LeagueBroadcastTwitch),
            "LEAGUE_BROADCAST_YOUTUBE" => Some(Self::LeagueBroadcastYoutube),
            "LEAGUE_BROADCAST_OTHER" => Some(Self::LeagueBroadcastOther),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ELeaguePhase {
    LeaguePhaseUnset = 0,
    LeaguePhaseRegionalQualifier = 1,
    LeaguePhaseGroupStage = 2,
    LeaguePhaseMainEvent = 3,
}
impl ELeaguePhase {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ELeaguePhase::LeaguePhaseUnset => "LEAGUE_PHASE_UNSET",
            ELeaguePhase::LeaguePhaseRegionalQualifier => {
                "LEAGUE_PHASE_REGIONAL_QUALIFIER"
            }
            ELeaguePhase::LeaguePhaseGroupStage => "LEAGUE_PHASE_GROUP_STAGE",
            ELeaguePhase::LeaguePhaseMainEvent => "LEAGUE_PHASE_MAIN_EVENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEAGUE_PHASE_UNSET" => Some(Self::LeaguePhaseUnset),
            "LEAGUE_PHASE_REGIONAL_QUALIFIER" => Some(Self::LeaguePhaseRegionalQualifier),
            "LEAGUE_PHASE_GROUP_STAGE" => Some(Self::LeaguePhaseGroupStage),
            "LEAGUE_PHASE_MAIN_EVENT" => Some(Self::LeaguePhaseMainEvent),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl ELeagueAuditAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ELeagueAuditAction::LeagueAuditActionInvalid => "LEAGUE_AUDIT_ACTION_INVALID",
            ELeagueAuditAction::LeagueAuditActionLeagueCreate => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_CREATE"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueEdit => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_EDIT"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueDelete => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_DELETE"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueAdminAdd => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_ADMIN_ADD"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueAdminRevoke => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_ADMIN_REVOKE"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueAdminPromote => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_ADMIN_PROMOTE"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueStreamAdd => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_STREAM_ADD"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueStreamRemove => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_STREAM_REMOVE"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueImageUpdated => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_IMAGE_UPDATED"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueMessageAdded => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_MESSAGE_ADDED"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueSubmitted => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_SUBMITTED"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueSetPrizePool => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_SET_PRIZE_POOL"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueAddPrizePoolItem => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_ADD_PRIZE_POOL_ITEM"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueRemovePrizePoolItem => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_REMOVE_PRIZE_POOL_ITEM"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueMatchStart => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_MATCH_START"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueMatchEnd => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_MATCH_END"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueAddInvitedTeam => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_ADD_INVITED_TEAM"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueRemoveInvitedTeam => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_REMOVE_INVITED_TEAM"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueStatusChanged => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_STATUS_CHANGED"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueStreamEdit => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_STREAM_EDIT"
            }
            ELeagueAuditAction::LeagueAuditActionLeagueTeamSwap => {
                "LEAGUE_AUDIT_ACTION_LEAGUE_TEAM_SWAP"
            }
            ELeagueAuditAction::LeagueAuditActionNodegroupCreate => {
                "LEAGUE_AUDIT_ACTION_NODEGROUP_CREATE"
            }
            ELeagueAuditAction::LeagueAuditActionNodegroupDestroy => {
                "LEAGUE_AUDIT_ACTION_NODEGROUP_DESTROY"
            }
            ELeagueAuditAction::LeagueAuditActionNodegroupAddTeam => {
                "LEAGUE_AUDIT_ACTION_NODEGROUP_ADD_TEAM"
            }
            ELeagueAuditAction::LeagueAuditActionNodegroupRemoveTeam => {
                "LEAGUE_AUDIT_ACTION_NODEGROUP_REMOVE_TEAM"
            }
            ELeagueAuditAction::LeagueAuditActionNodegroupSetAdvancing => {
                "LEAGUE_AUDIT_ACTION_NODEGROUP_SET_ADVANCING"
            }
            ELeagueAuditAction::LeagueAuditActionNodegroupEdit => {
                "LEAGUE_AUDIT_ACTION_NODEGROUP_EDIT"
            }
            ELeagueAuditAction::LeagueAuditActionNodegroupPopulate => {
                "LEAGUE_AUDIT_ACTION_NODEGROUP_POPULATE"
            }
            ELeagueAuditAction::LeagueAuditActionNodegroupCompleted => {
                "LEAGUE_AUDIT_ACTION_NODEGROUP_COMPLETED"
            }
            ELeagueAuditAction::LeagueAuditActionNodegroupSetSecondaryAdvancing => {
                "LEAGUE_AUDIT_ACTION_NODEGROUP_SET_SECONDARY_ADVANCING"
            }
            ELeagueAuditAction::LeagueAuditActionNodegroupSetTertiaryAdvancing => {
                "LEAGUE_AUDIT_ACTION_NODEGROUP_SET_TERTIARY_ADVANCING"
            }
            ELeagueAuditAction::LeagueAuditActionNodeCreate => {
                "LEAGUE_AUDIT_ACTION_NODE_CREATE"
            }
            ELeagueAuditAction::LeagueAuditActionNodeDestroy => {
                "LEAGUE_AUDIT_ACTION_NODE_DESTROY"
            }
            ELeagueAuditAction::LeagueAuditActionNodeAutocreate => {
                "LEAGUE_AUDIT_ACTION_NODE_AUTOCREATE"
            }
            ELeagueAuditAction::LeagueAuditActionNodeSetTeam => {
                "LEAGUE_AUDIT_ACTION_NODE_SET_TEAM"
            }
            ELeagueAuditAction::LeagueAuditActionNodeSetSeriesId => {
                "LEAGUE_AUDIT_ACTION_NODE_SET_SERIES_ID"
            }
            ELeagueAuditAction::LeagueAuditActionNodeSetAdvancing => {
                "LEAGUE_AUDIT_ACTION_NODE_SET_ADVANCING"
            }
            ELeagueAuditAction::LeagueAuditActionNodeSetTime => {
                "LEAGUE_AUDIT_ACTION_NODE_SET_TIME"
            }
            ELeagueAuditAction::LeagueAuditActionNodeMatchCompleted => {
                "LEAGUE_AUDIT_ACTION_NODE_MATCH_COMPLETED"
            }
            ELeagueAuditAction::LeagueAuditActionNodeCompleted => {
                "LEAGUE_AUDIT_ACTION_NODE_COMPLETED"
            }
            ELeagueAuditAction::LeagueAuditActionNodeEdit => {
                "LEAGUE_AUDIT_ACTION_NODE_EDIT"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEAGUE_AUDIT_ACTION_INVALID" => Some(Self::LeagueAuditActionInvalid),
            "LEAGUE_AUDIT_ACTION_LEAGUE_CREATE" => {
                Some(Self::LeagueAuditActionLeagueCreate)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_EDIT" => Some(Self::LeagueAuditActionLeagueEdit),
            "LEAGUE_AUDIT_ACTION_LEAGUE_DELETE" => {
                Some(Self::LeagueAuditActionLeagueDelete)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_ADMIN_ADD" => {
                Some(Self::LeagueAuditActionLeagueAdminAdd)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_ADMIN_REVOKE" => {
                Some(Self::LeagueAuditActionLeagueAdminRevoke)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_ADMIN_PROMOTE" => {
                Some(Self::LeagueAuditActionLeagueAdminPromote)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_STREAM_ADD" => {
                Some(Self::LeagueAuditActionLeagueStreamAdd)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_STREAM_REMOVE" => {
                Some(Self::LeagueAuditActionLeagueStreamRemove)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_IMAGE_UPDATED" => {
                Some(Self::LeagueAuditActionLeagueImageUpdated)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_MESSAGE_ADDED" => {
                Some(Self::LeagueAuditActionLeagueMessageAdded)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_SUBMITTED" => {
                Some(Self::LeagueAuditActionLeagueSubmitted)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_SET_PRIZE_POOL" => {
                Some(Self::LeagueAuditActionLeagueSetPrizePool)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_ADD_PRIZE_POOL_ITEM" => {
                Some(Self::LeagueAuditActionLeagueAddPrizePoolItem)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_REMOVE_PRIZE_POOL_ITEM" => {
                Some(Self::LeagueAuditActionLeagueRemovePrizePoolItem)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_MATCH_START" => {
                Some(Self::LeagueAuditActionLeagueMatchStart)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_MATCH_END" => {
                Some(Self::LeagueAuditActionLeagueMatchEnd)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_ADD_INVITED_TEAM" => {
                Some(Self::LeagueAuditActionLeagueAddInvitedTeam)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_REMOVE_INVITED_TEAM" => {
                Some(Self::LeagueAuditActionLeagueRemoveInvitedTeam)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_STATUS_CHANGED" => {
                Some(Self::LeagueAuditActionLeagueStatusChanged)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_STREAM_EDIT" => {
                Some(Self::LeagueAuditActionLeagueStreamEdit)
            }
            "LEAGUE_AUDIT_ACTION_LEAGUE_TEAM_SWAP" => {
                Some(Self::LeagueAuditActionLeagueTeamSwap)
            }
            "LEAGUE_AUDIT_ACTION_NODEGROUP_CREATE" => {
                Some(Self::LeagueAuditActionNodegroupCreate)
            }
            "LEAGUE_AUDIT_ACTION_NODEGROUP_DESTROY" => {
                Some(Self::LeagueAuditActionNodegroupDestroy)
            }
            "LEAGUE_AUDIT_ACTION_NODEGROUP_ADD_TEAM" => {
                Some(Self::LeagueAuditActionNodegroupAddTeam)
            }
            "LEAGUE_AUDIT_ACTION_NODEGROUP_REMOVE_TEAM" => {
                Some(Self::LeagueAuditActionNodegroupRemoveTeam)
            }
            "LEAGUE_AUDIT_ACTION_NODEGROUP_SET_ADVANCING" => {
                Some(Self::LeagueAuditActionNodegroupSetAdvancing)
            }
            "LEAGUE_AUDIT_ACTION_NODEGROUP_EDIT" => {
                Some(Self::LeagueAuditActionNodegroupEdit)
            }
            "LEAGUE_AUDIT_ACTION_NODEGROUP_POPULATE" => {
                Some(Self::LeagueAuditActionNodegroupPopulate)
            }
            "LEAGUE_AUDIT_ACTION_NODEGROUP_COMPLETED" => {
                Some(Self::LeagueAuditActionNodegroupCompleted)
            }
            "LEAGUE_AUDIT_ACTION_NODEGROUP_SET_SECONDARY_ADVANCING" => {
                Some(Self::LeagueAuditActionNodegroupSetSecondaryAdvancing)
            }
            "LEAGUE_AUDIT_ACTION_NODEGROUP_SET_TERTIARY_ADVANCING" => {
                Some(Self::LeagueAuditActionNodegroupSetTertiaryAdvancing)
            }
            "LEAGUE_AUDIT_ACTION_NODE_CREATE" => Some(Self::LeagueAuditActionNodeCreate),
            "LEAGUE_AUDIT_ACTION_NODE_DESTROY" => {
                Some(Self::LeagueAuditActionNodeDestroy)
            }
            "LEAGUE_AUDIT_ACTION_NODE_AUTOCREATE" => {
                Some(Self::LeagueAuditActionNodeAutocreate)
            }
            "LEAGUE_AUDIT_ACTION_NODE_SET_TEAM" => {
                Some(Self::LeagueAuditActionNodeSetTeam)
            }
            "LEAGUE_AUDIT_ACTION_NODE_SET_SERIES_ID" => {
                Some(Self::LeagueAuditActionNodeSetSeriesId)
            }
            "LEAGUE_AUDIT_ACTION_NODE_SET_ADVANCING" => {
                Some(Self::LeagueAuditActionNodeSetAdvancing)
            }
            "LEAGUE_AUDIT_ACTION_NODE_SET_TIME" => {
                Some(Self::LeagueAuditActionNodeSetTime)
            }
            "LEAGUE_AUDIT_ACTION_NODE_MATCH_COMPLETED" => {
                Some(Self::LeagueAuditActionNodeMatchCompleted)
            }
            "LEAGUE_AUDIT_ACTION_NODE_COMPLETED" => {
                Some(Self::LeagueAuditActionNodeCompleted)
            }
            "LEAGUE_AUDIT_ACTION_NODE_EDIT" => Some(Self::LeagueAuditActionNodeEdit),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaCombatlogTypes {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaCombatlogTypes::DotaCombatlogInvalid => "DOTA_COMBATLOG_INVALID",
            DotaCombatlogTypes::DotaCombatlogDamage => "DOTA_COMBATLOG_DAMAGE",
            DotaCombatlogTypes::DotaCombatlogHeal => "DOTA_COMBATLOG_HEAL",
            DotaCombatlogTypes::DotaCombatlogModifierAdd => "DOTA_COMBATLOG_MODIFIER_ADD",
            DotaCombatlogTypes::DotaCombatlogModifierRemove => {
                "DOTA_COMBATLOG_MODIFIER_REMOVE"
            }
            DotaCombatlogTypes::DotaCombatlogDeath => "DOTA_COMBATLOG_DEATH",
            DotaCombatlogTypes::DotaCombatlogAbility => "DOTA_COMBATLOG_ABILITY",
            DotaCombatlogTypes::DotaCombatlogItem => "DOTA_COMBATLOG_ITEM",
            DotaCombatlogTypes::DotaCombatlogLocation => "DOTA_COMBATLOG_LOCATION",
            DotaCombatlogTypes::DotaCombatlogGold => "DOTA_COMBATLOG_GOLD",
            DotaCombatlogTypes::DotaCombatlogGameState => "DOTA_COMBATLOG_GAME_STATE",
            DotaCombatlogTypes::DotaCombatlogXp => "DOTA_COMBATLOG_XP",
            DotaCombatlogTypes::DotaCombatlogPurchase => "DOTA_COMBATLOG_PURCHASE",
            DotaCombatlogTypes::DotaCombatlogBuyback => "DOTA_COMBATLOG_BUYBACK",
            DotaCombatlogTypes::DotaCombatlogAbilityTrigger => {
                "DOTA_COMBATLOG_ABILITY_TRIGGER"
            }
            DotaCombatlogTypes::DotaCombatlogPlayerstats => "DOTA_COMBATLOG_PLAYERSTATS",
            DotaCombatlogTypes::DotaCombatlogMultikill => "DOTA_COMBATLOG_MULTIKILL",
            DotaCombatlogTypes::DotaCombatlogKillstreak => "DOTA_COMBATLOG_KILLSTREAK",
            DotaCombatlogTypes::DotaCombatlogTeamBuildingKill => {
                "DOTA_COMBATLOG_TEAM_BUILDING_KILL"
            }
            DotaCombatlogTypes::DotaCombatlogFirstBlood => "DOTA_COMBATLOG_FIRST_BLOOD",
            DotaCombatlogTypes::DotaCombatlogModifierStackEvent => {
                "DOTA_COMBATLOG_MODIFIER_STACK_EVENT"
            }
            DotaCombatlogTypes::DotaCombatlogNeutralCampStack => {
                "DOTA_COMBATLOG_NEUTRAL_CAMP_STACK"
            }
            DotaCombatlogTypes::DotaCombatlogPickupRune => "DOTA_COMBATLOG_PICKUP_RUNE",
            DotaCombatlogTypes::DotaCombatlogRevealedInvisible => {
                "DOTA_COMBATLOG_REVEALED_INVISIBLE"
            }
            DotaCombatlogTypes::DotaCombatlogHeroSaved => "DOTA_COMBATLOG_HERO_SAVED",
            DotaCombatlogTypes::DotaCombatlogManaRestored => {
                "DOTA_COMBATLOG_MANA_RESTORED"
            }
            DotaCombatlogTypes::DotaCombatlogHeroLevelup => "DOTA_COMBATLOG_HERO_LEVELUP",
            DotaCombatlogTypes::DotaCombatlogBottleHealAlly => {
                "DOTA_COMBATLOG_BOTTLE_HEAL_ALLY"
            }
            DotaCombatlogTypes::DotaCombatlogEndgameStats => {
                "DOTA_COMBATLOG_ENDGAME_STATS"
            }
            DotaCombatlogTypes::DotaCombatlogInterruptChannel => {
                "DOTA_COMBATLOG_INTERRUPT_CHANNEL"
            }
            DotaCombatlogTypes::DotaCombatlogAlliedGold => "DOTA_COMBATLOG_ALLIED_GOLD",
            DotaCombatlogTypes::DotaCombatlogAegisTaken => "DOTA_COMBATLOG_AEGIS_TAKEN",
            DotaCombatlogTypes::DotaCombatlogManaDamage => "DOTA_COMBATLOG_MANA_DAMAGE",
            DotaCombatlogTypes::DotaCombatlogPhysicalDamagePrevented => {
                "DOTA_COMBATLOG_PHYSICAL_DAMAGE_PREVENTED"
            }
            DotaCombatlogTypes::DotaCombatlogUnitSummoned => {
                "DOTA_COMBATLOG_UNIT_SUMMONED"
            }
            DotaCombatlogTypes::DotaCombatlogAttackEvade => "DOTA_COMBATLOG_ATTACK_EVADE",
            DotaCombatlogTypes::DotaCombatlogTreeCut => "DOTA_COMBATLOG_TREE_CUT",
            DotaCombatlogTypes::DotaCombatlogSuccessfulScan => {
                "DOTA_COMBATLOG_SUCCESSFUL_SCAN"
            }
            DotaCombatlogTypes::DotaCombatlogEndKillstreak => {
                "DOTA_COMBATLOG_END_KILLSTREAK"
            }
            DotaCombatlogTypes::DotaCombatlogBloodstoneCharge => {
                "DOTA_COMBATLOG_BLOODSTONE_CHARGE"
            }
            DotaCombatlogTypes::DotaCombatlogCriticalDamage => {
                "DOTA_COMBATLOG_CRITICAL_DAMAGE"
            }
            DotaCombatlogTypes::DotaCombatlogSpellAbsorb => "DOTA_COMBATLOG_SPELL_ABSORB",
            DotaCombatlogTypes::DotaCombatlogUnitTeleported => {
                "DOTA_COMBATLOG_UNIT_TELEPORTED"
            }
            DotaCombatlogTypes::DotaCombatlogKillEaterEvent => {
                "DOTA_COMBATLOG_KILL_EATER_EVENT"
            }
            DotaCombatlogTypes::DotaCombatlogNeutralItemEarned => {
                "DOTA_COMBATLOG_NEUTRAL_ITEM_EARNED"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_COMBATLOG_INVALID" => Some(Self::DotaCombatlogInvalid),
            "DOTA_COMBATLOG_DAMAGE" => Some(Self::DotaCombatlogDamage),
            "DOTA_COMBATLOG_HEAL" => Some(Self::DotaCombatlogHeal),
            "DOTA_COMBATLOG_MODIFIER_ADD" => Some(Self::DotaCombatlogModifierAdd),
            "DOTA_COMBATLOG_MODIFIER_REMOVE" => Some(Self::DotaCombatlogModifierRemove),
            "DOTA_COMBATLOG_DEATH" => Some(Self::DotaCombatlogDeath),
            "DOTA_COMBATLOG_ABILITY" => Some(Self::DotaCombatlogAbility),
            "DOTA_COMBATLOG_ITEM" => Some(Self::DotaCombatlogItem),
            "DOTA_COMBATLOG_LOCATION" => Some(Self::DotaCombatlogLocation),
            "DOTA_COMBATLOG_GOLD" => Some(Self::DotaCombatlogGold),
            "DOTA_COMBATLOG_GAME_STATE" => Some(Self::DotaCombatlogGameState),
            "DOTA_COMBATLOG_XP" => Some(Self::DotaCombatlogXp),
            "DOTA_COMBATLOG_PURCHASE" => Some(Self::DotaCombatlogPurchase),
            "DOTA_COMBATLOG_BUYBACK" => Some(Self::DotaCombatlogBuyback),
            "DOTA_COMBATLOG_ABILITY_TRIGGER" => Some(Self::DotaCombatlogAbilityTrigger),
            "DOTA_COMBATLOG_PLAYERSTATS" => Some(Self::DotaCombatlogPlayerstats),
            "DOTA_COMBATLOG_MULTIKILL" => Some(Self::DotaCombatlogMultikill),
            "DOTA_COMBATLOG_KILLSTREAK" => Some(Self::DotaCombatlogKillstreak),
            "DOTA_COMBATLOG_TEAM_BUILDING_KILL" => {
                Some(Self::DotaCombatlogTeamBuildingKill)
            }
            "DOTA_COMBATLOG_FIRST_BLOOD" => Some(Self::DotaCombatlogFirstBlood),
            "DOTA_COMBATLOG_MODIFIER_STACK_EVENT" => {
                Some(Self::DotaCombatlogModifierStackEvent)
            }
            "DOTA_COMBATLOG_NEUTRAL_CAMP_STACK" => {
                Some(Self::DotaCombatlogNeutralCampStack)
            }
            "DOTA_COMBATLOG_PICKUP_RUNE" => Some(Self::DotaCombatlogPickupRune),
            "DOTA_COMBATLOG_REVEALED_INVISIBLE" => {
                Some(Self::DotaCombatlogRevealedInvisible)
            }
            "DOTA_COMBATLOG_HERO_SAVED" => Some(Self::DotaCombatlogHeroSaved),
            "DOTA_COMBATLOG_MANA_RESTORED" => Some(Self::DotaCombatlogManaRestored),
            "DOTA_COMBATLOG_HERO_LEVELUP" => Some(Self::DotaCombatlogHeroLevelup),
            "DOTA_COMBATLOG_BOTTLE_HEAL_ALLY" => Some(Self::DotaCombatlogBottleHealAlly),
            "DOTA_COMBATLOG_ENDGAME_STATS" => Some(Self::DotaCombatlogEndgameStats),
            "DOTA_COMBATLOG_INTERRUPT_CHANNEL" => {
                Some(Self::DotaCombatlogInterruptChannel)
            }
            "DOTA_COMBATLOG_ALLIED_GOLD" => Some(Self::DotaCombatlogAlliedGold),
            "DOTA_COMBATLOG_AEGIS_TAKEN" => Some(Self::DotaCombatlogAegisTaken),
            "DOTA_COMBATLOG_MANA_DAMAGE" => Some(Self::DotaCombatlogManaDamage),
            "DOTA_COMBATLOG_PHYSICAL_DAMAGE_PREVENTED" => {
                Some(Self::DotaCombatlogPhysicalDamagePrevented)
            }
            "DOTA_COMBATLOG_UNIT_SUMMONED" => Some(Self::DotaCombatlogUnitSummoned),
            "DOTA_COMBATLOG_ATTACK_EVADE" => Some(Self::DotaCombatlogAttackEvade),
            "DOTA_COMBATLOG_TREE_CUT" => Some(Self::DotaCombatlogTreeCut),
            "DOTA_COMBATLOG_SUCCESSFUL_SCAN" => Some(Self::DotaCombatlogSuccessfulScan),
            "DOTA_COMBATLOG_END_KILLSTREAK" => Some(Self::DotaCombatlogEndKillstreak),
            "DOTA_COMBATLOG_BLOODSTONE_CHARGE" => {
                Some(Self::DotaCombatlogBloodstoneCharge)
            }
            "DOTA_COMBATLOG_CRITICAL_DAMAGE" => Some(Self::DotaCombatlogCriticalDamage),
            "DOTA_COMBATLOG_SPELL_ABSORB" => Some(Self::DotaCombatlogSpellAbsorb),
            "DOTA_COMBATLOG_UNIT_TELEPORTED" => Some(Self::DotaCombatlogUnitTeleported),
            "DOTA_COMBATLOG_KILL_EATER_EVENT" => Some(Self::DotaCombatlogKillEaterEvent),
            "DOTA_COMBATLOG_NEUTRAL_ITEM_EARNED" => {
                Some(Self::DotaCombatlogNeutralItemEarned)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EdpcFavoriteType {
    FavoriteTypeAll = 0,
    FavoriteTypePlayer = 1,
    FavoriteTypeTeam = 2,
    FavoriteTypeLeague = 3,
}
impl EdpcFavoriteType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EdpcFavoriteType::FavoriteTypeAll => "FAVORITE_TYPE_ALL",
            EdpcFavoriteType::FavoriteTypePlayer => "FAVORITE_TYPE_PLAYER",
            EdpcFavoriteType::FavoriteTypeTeam => "FAVORITE_TYPE_TEAM",
            EdpcFavoriteType::FavoriteTypeLeague => "FAVORITE_TYPE_LEAGUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FAVORITE_TYPE_ALL" => Some(Self::FavoriteTypeAll),
            "FAVORITE_TYPE_PLAYER" => Some(Self::FavoriteTypePlayer),
            "FAVORITE_TYPE_TEAM" => Some(Self::FavoriteTypeTeam),
            "FAVORITE_TYPE_LEAGUE" => Some(Self::FavoriteTypeLeague),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl EdpcPushNotification {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EdpcPushNotification::DpcPushNotificationMatchStarting => {
                "DPC_PUSH_NOTIFICATION_MATCH_STARTING"
            }
            EdpcPushNotification::DpcPushNotificationPlayerLeftTeam => {
                "DPC_PUSH_NOTIFICATION_PLAYER_LEFT_TEAM"
            }
            EdpcPushNotification::DpcPushNotificationPlayerJoinedTeam => {
                "DPC_PUSH_NOTIFICATION_PLAYER_JOINED_TEAM"
            }
            EdpcPushNotification::DpcPushNotificationPlayerJoinedTeamAsCoach => {
                "DPC_PUSH_NOTIFICATION_PLAYER_JOINED_TEAM_AS_COACH"
            }
            EdpcPushNotification::DpcPushNotificationPlayerLeftTeamAsCoach => {
                "DPC_PUSH_NOTIFICATION_PLAYER_LEFT_TEAM_AS_COACH"
            }
            EdpcPushNotification::DpcPushNotificationLeagueResult => {
                "DPC_PUSH_NOTIFICATION_LEAGUE_RESULT"
            }
            EdpcPushNotification::DpcPushNotificationPredictionMatchesAvailable => {
                "DPC_PUSH_NOTIFICATION_PREDICTION_MATCHES_AVAILABLE"
            }
            EdpcPushNotification::DpcPushNotificationPredictionResult => {
                "DPC_PUSH_NOTIFICATION_PREDICTION_RESULT"
            }
            EdpcPushNotification::DpcPushNotificationFantasyPlayerCleared => {
                "DPC_PUSH_NOTIFICATION_FANTASY_PLAYER_CLEARED"
            }
            EdpcPushNotification::DpcPushNotificationFantasyDailySummary => {
                "DPC_PUSH_NOTIFICATION_FANTASY_DAILY_SUMMARY"
            }
            EdpcPushNotification::DpcPushNotificationFantasyFinalResults => {
                "DPC_PUSH_NOTIFICATION_FANTASY_FINAL_RESULTS"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DPC_PUSH_NOTIFICATION_MATCH_STARTING" => {
                Some(Self::DpcPushNotificationMatchStarting)
            }
            "DPC_PUSH_NOTIFICATION_PLAYER_LEFT_TEAM" => {
                Some(Self::DpcPushNotificationPlayerLeftTeam)
            }
            "DPC_PUSH_NOTIFICATION_PLAYER_JOINED_TEAM" => {
                Some(Self::DpcPushNotificationPlayerJoinedTeam)
            }
            "DPC_PUSH_NOTIFICATION_PLAYER_JOINED_TEAM_AS_COACH" => {
                Some(Self::DpcPushNotificationPlayerJoinedTeamAsCoach)
            }
            "DPC_PUSH_NOTIFICATION_PLAYER_LEFT_TEAM_AS_COACH" => {
                Some(Self::DpcPushNotificationPlayerLeftTeamAsCoach)
            }
            "DPC_PUSH_NOTIFICATION_LEAGUE_RESULT" => {
                Some(Self::DpcPushNotificationLeagueResult)
            }
            "DPC_PUSH_NOTIFICATION_PREDICTION_MATCHES_AVAILABLE" => {
                Some(Self::DpcPushNotificationPredictionMatchesAvailable)
            }
            "DPC_PUSH_NOTIFICATION_PREDICTION_RESULT" => {
                Some(Self::DpcPushNotificationPredictionResult)
            }
            "DPC_PUSH_NOTIFICATION_FANTASY_PLAYER_CLEARED" => {
                Some(Self::DpcPushNotificationFantasyPlayerCleared)
            }
            "DPC_PUSH_NOTIFICATION_FANTASY_DAILY_SUMMARY" => {
                Some(Self::DpcPushNotificationFantasyDailySummary)
            }
            "DPC_PUSH_NOTIFICATION_FANTASY_FINAL_RESULTS" => {
                Some(Self::DpcPushNotificationFantasyFinalResults)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EEventActionScoreMode {
    KEEventActionScoreModeAdd = 0,
    KEEventActionScoreModeMin = 1,
}
impl EEventActionScoreMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EEventActionScoreMode::KEEventActionScoreModeAdd => {
                "k_eEventActionScoreMode_Add"
            }
            EEventActionScoreMode::KEEventActionScoreModeMin => {
                "k_eEventActionScoreMode_Min"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_eEventActionScoreMode_Add" => Some(Self::KEEventActionScoreModeAdd),
            "k_eEventActionScoreMode_Min" => Some(Self::KEEventActionScoreModeMin),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EPlayerChallengeHistoryType {
    KEPlayerChallengeHistoryTypeInvalid = 0,
    KEPlayerChallengeHistoryTypeKillEater = 1,
    KEPlayerChallengeHistoryTypeDotaPlusRelic = 2,
    KEPlayerChallengeHistoryTypeDotaPlusHeroPlayerChallenge = 3,
    KEPlayerChallengeHistoryTypeInGameEventChallenge = 4,
    KEPlayerChallengeHistoryTypeGuildContract = 5,
}
impl EPlayerChallengeHistoryType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EPlayerChallengeHistoryType::KEPlayerChallengeHistoryTypeInvalid => {
                "k_EPlayerChallengeHistoryType_Invalid"
            }
            EPlayerChallengeHistoryType::KEPlayerChallengeHistoryTypeKillEater => {
                "k_EPlayerChallengeHistoryType_KillEater"
            }
            EPlayerChallengeHistoryType::KEPlayerChallengeHistoryTypeDotaPlusRelic => {
                "k_EPlayerChallengeHistoryType_DotaPlusRelic"
            }
            EPlayerChallengeHistoryType::KEPlayerChallengeHistoryTypeDotaPlusHeroPlayerChallenge => {
                "k_EPlayerChallengeHistoryType_DotaPlusHeroPlayerChallenge"
            }
            EPlayerChallengeHistoryType::KEPlayerChallengeHistoryTypeInGameEventChallenge => {
                "k_EPlayerChallengeHistoryType_InGameEventChallenge"
            }
            EPlayerChallengeHistoryType::KEPlayerChallengeHistoryTypeGuildContract => {
                "k_EPlayerChallengeHistoryType_GuildContract"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_EPlayerChallengeHistoryType_Invalid" => {
                Some(Self::KEPlayerChallengeHistoryTypeInvalid)
            }
            "k_EPlayerChallengeHistoryType_KillEater" => {
                Some(Self::KEPlayerChallengeHistoryTypeKillEater)
            }
            "k_EPlayerChallengeHistoryType_DotaPlusRelic" => {
                Some(Self::KEPlayerChallengeHistoryTypeDotaPlusRelic)
            }
            "k_EPlayerChallengeHistoryType_DotaPlusHeroPlayerChallenge" => {
                Some(Self::KEPlayerChallengeHistoryTypeDotaPlusHeroPlayerChallenge)
            }
            "k_EPlayerChallengeHistoryType_InGameEventChallenge" => {
                Some(Self::KEPlayerChallengeHistoryTypeInGameEventChallenge)
            }
            "k_EPlayerChallengeHistoryType_GuildContract" => {
                Some(Self::KEPlayerChallengeHistoryTypeGuildContract)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EOverwatchReportReason {
    KEOverwatchReportReasonUnknown = 0,
    KEOverwatchReportReasonCheating = 1,
    KEOverwatchReportReasonFeeding = 2,
    KEOverwatchReportReasonGriefing = 3,
    KEOverwatchReportReasonSuspicious = 4,
    KEOverwatchReportReasonAbilityAbuse = 5,
}
impl EOverwatchReportReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EOverwatchReportReason::KEOverwatchReportReasonUnknown => {
                "k_EOverwatchReportReason_Unknown"
            }
            EOverwatchReportReason::KEOverwatchReportReasonCheating => {
                "k_EOverwatchReportReason_Cheating"
            }
            EOverwatchReportReason::KEOverwatchReportReasonFeeding => {
                "k_EOverwatchReportReason_Feeding"
            }
            EOverwatchReportReason::KEOverwatchReportReasonGriefing => {
                "k_EOverwatchReportReason_Griefing"
            }
            EOverwatchReportReason::KEOverwatchReportReasonSuspicious => {
                "k_EOverwatchReportReason_Suspicious"
            }
            EOverwatchReportReason::KEOverwatchReportReasonAbilityAbuse => {
                "k_EOverwatchReportReason_AbilityAbuse"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_EOverwatchReportReason_Unknown" => {
                Some(Self::KEOverwatchReportReasonUnknown)
            }
            "k_EOverwatchReportReason_Cheating" => {
                Some(Self::KEOverwatchReportReasonCheating)
            }
            "k_EOverwatchReportReason_Feeding" => {
                Some(Self::KEOverwatchReportReasonFeeding)
            }
            "k_EOverwatchReportReason_Griefing" => {
                Some(Self::KEOverwatchReportReasonGriefing)
            }
            "k_EOverwatchReportReason_Suspicious" => {
                Some(Self::KEOverwatchReportReasonSuspicious)
            }
            "k_EOverwatchReportReason_AbilityAbuse" => {
                Some(Self::KEOverwatchReportReasonAbilityAbuse)
            }
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgAiDebugLine {
    #[prost(string, optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgPing {
    #[prost(uint32, optional, tag = "2")]
    pub ping: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub loss: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSwapVerify {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgChatEvent {
    #[prost(
        enumeration = "DotaChatMessage",
        required,
        tag = "1",
        default = "ChatMessageInvalid"
    )]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgBotChat {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub target: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "5")]
    pub team_only: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgCombatHeroPositions {
    #[prost(uint32, optional, tag = "1")]
    pub index: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub time: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub world_pos: ::core::option::Option<CMsgVector2D>,
    #[prost(int32, optional, tag = "4")]
    pub health: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgCombatLogBulkData {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgProjectileParticleCpData {
    #[prost(int32, optional, tag = "1")]
    pub control_point: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub vector: ::core::option::Option<CMsgVector>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgMiniKillCamInfo {
    #[prost(message, repeated, tag = "1")]
    pub attackers: ::prost::alloc::vec::Vec<cdota_user_msg_mini_kill_cam_info::Attacker>,
}
/// Nested message and enum types in `CDOTAUserMsg_MiniKillCamInfo`.
pub mod cdota_user_msg_mini_kill_cam_info {
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
    /// Nested message and enum types in `Attacker`.
    pub mod attacker {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Ability {
            #[prost(int32, optional, tag = "1", default = "-1")]
            pub ability_id: ::core::option::Option<i32>,
            #[prost(int32, optional, tag = "2")]
            pub damage: ::core::option::Option<i32>,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgGlobalLightColor {
    #[prost(uint32, optional, tag = "1")]
    pub color: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgGlobalLightDirection {
    #[prost(message, optional, tag = "1")]
    pub direction: ::core::option::Option<CMsgVector>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgLocationPing {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub location_ping: ::core::option::Option<CdotaMsgLocationPing>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgPingConfirmation {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_of_original_pinger: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub entity_index: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub icon_type: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "4")]
    pub location: ::core::option::Option<CMsgVector>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgItemAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub item_alert: ::core::option::Option<CdotaMsgItemAlert>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgEnemyItemAlert {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgModifierAlert {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgHpManaAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_entindex: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub show_raw_values: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgNeutralCampAlert {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgGlyphAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub negative: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgRadarAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub negative: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgWillPurchaseAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub gold_remaining: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub suggestion_player_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgEmptyTeleportAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub cooldown_seconds: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgMarsArenaOfBloodAttack {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub source_ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2", default = "16777215")]
    pub target_ehandle: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub warrior_index: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaEntityMsgInvokerSpellCast {
    #[prost(message, optional, tag = "1")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
    #[prost(int32, optional, tag = "2")]
    pub cast_activity: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgBuyBackStateAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgQuickBuyAlert {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgCourierKilledAlert {
    #[prost(uint32, optional, tag = "1")]
    pub team: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub gold_value: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3", default = "16777215")]
    pub entity_handle: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "4")]
    pub timestamp: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "5")]
    pub lost_items: ::prost::alloc::vec::Vec<
        cdota_user_msg_courier_killed_alert::LostItem,
    >,
    #[prost(int32, optional, tag = "6", default = "-1")]
    pub killer_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "7", default = "-1")]
    pub owning_player_id: ::core::option::Option<i32>,
}
/// Nested message and enum types in `CDOTAUserMsg_CourierKilledAlert`.
pub mod cdota_user_msg_courier_killed_alert {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LostItem {
        #[prost(int32, optional, tag = "1", default = "-1")]
        pub item_ability_id: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "2")]
        pub quantity: ::core::option::Option<u32>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgMinimapEvent {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgMapLine {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub mapline: ::core::option::Option<CdotaMsgMapLine>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgMinimapDebugPoint {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgCreateLinearProjectile {
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
    pub particle_cp_data: ::prost::alloc::vec::Vec<CdotaUserMsgProjectileParticleCpData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgDestroyLinearProjectile {
    #[prost(int32, optional, tag = "1")]
    pub handle: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgDodgeTrackingProjectiles {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub entindex: i32,
    #[prost(bool, optional, tag = "2")]
    pub attacks_only: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSpectatorPlayerClick {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub entindex: i32,
    #[prost(int32, optional, tag = "2")]
    pub order_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "0")]
    pub target_index: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSpectatorPlayerUnitOrders {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgNevermoreRequiem {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub entity_handle: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub lines: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub origin: ::core::option::Option<CMsgVector>,
    #[prost(bool, optional, tag = "4")]
    pub reverse: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgInvalidCommand {
    #[prost(string, optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub sequence_number: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgHudError {
    #[prost(int32, optional, tag = "1")]
    pub order_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub sequence_number: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSharedCooldown {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entindex: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, optional, tag = "3")]
    pub cooldown: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "4")]
    pub name_index: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSetNextAutobuyItem {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgHalloweenDrops {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub item_defs: ::prost::alloc::vec::Vec<u32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub player_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub prize_list: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgPredictionResult {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "3")]
    pub correct: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "4")]
    pub predictions: ::prost::alloc::vec::Vec<
        cdota_user_msg_prediction_result::Prediction,
    >,
}
/// Nested message and enum types in `CDOTAUserMsg_PredictionResult`.
pub mod cdota_user_msg_prediction_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Prediction {
        #[prost(uint32, optional, tag = "1")]
        pub item_def: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub num_correct: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub num_fails: ::core::option::Option<u32>,
        #[prost(
            enumeration = "prediction::EResult",
            optional,
            tag = "4",
            default = "KEResultItemGranted"
        )]
        pub result: ::core::option::Option<i32>,
        #[prost(uint32, repeated, packed = "false", tag = "6")]
        pub granted_item_defs: ::prost::alloc::vec::Vec<u32>,
    }
    /// Nested message and enum types in `Prediction`.
    pub mod prediction {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum EResult {
            KEResultItemGranted = 1,
            KEResultDestroyed = 2,
        }
        impl EResult {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    EResult::KEResultItemGranted => "k_eResult_ItemGranted",
                    EResult::KEResultDestroyed => "k_eResult_Destroyed",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "k_eResult_ItemGranted" => Some(Self::KEResultItemGranted),
                    "k_eResult_Destroyed" => Some(Self::KEResultDestroyed),
                    _ => None,
                }
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaResponseQuerySerialized {
    #[prost(message, repeated, tag = "1")]
    pub facts: ::prost::alloc::vec::Vec<cdota_response_query_serialized::Fact>,
}
/// Nested message and enum types in `CDOTAResponseQuerySerialized`.
pub mod cdota_response_query_serialized {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Fact {
        #[prost(int32, required, tag = "1")]
        pub key: i32,
        #[prost(
            enumeration = "fact::ValueType",
            required,
            tag = "2",
            default = "Numeric"
        )]
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
    /// Nested message and enum types in `Fact`.
    pub mod fact {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum ValueType {
            Numeric = 1,
            String = 2,
            StringtableIndex = 3,
            IntNumeric = 4,
        }
        impl ValueType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ValueType::Numeric => "NUMERIC",
                    ValueType::String => "STRING",
                    ValueType::StringtableIndex => "STRINGTABLE_INDEX",
                    ValueType::IntNumeric => "INT_NUMERIC",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "NUMERIC" => Some(Self::Numeric),
                    "STRING" => Some(Self::String),
                    "STRINGTABLE_INDEX" => Some(Self::StringtableIndex),
                    "INT_NUMERIC" => Some(Self::IntNumeric),
                    _ => None,
                }
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaSpeechMatchOnClient {
    #[prost(int32, optional, tag = "1")]
    pub speech_concept: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub recipient_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub responsequery: ::core::option::Option<CdotaResponseQuerySerialized>,
    #[prost(sfixed32, optional, tag = "4", default = "0")]
    pub randomseed: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgUnitEvent {
    #[prost(
        enumeration = "EDotaEntityMessages",
        required,
        tag = "1",
        default = "DotaUnitSpeech"
    )]
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
    pub speech_match_on_client: ::core::option::Option<CdotaSpeechMatchOnClient>,
}
/// Nested message and enum types in `CDOTAUserMsg_UnitEvent`.
pub mod cdota_user_msg_unit_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Interval {
        #[prost(float, optional, tag = "1")]
        pub start: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "2")]
        pub range: ::core::option::Option<f32>,
    }
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
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SpeechMute {
        #[prost(float, optional, tag = "1", default = "0.5")]
        pub delay: ::core::option::Option<f32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
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
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RemoveGesture {
        #[prost(int32, optional, tag = "1")]
        pub activity: ::core::option::Option<i32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BloodImpact {
        #[prost(int32, optional, tag = "1")]
        pub scale: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub x_normal: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub y_normal: ::core::option::Option<i32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FadeGesture {
        #[prost(int32, optional, tag = "1")]
        pub activity: ::core::option::Option<i32>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgItemPurchased {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgItemSold {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgItemFound {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgOverheadEvent {
    #[prost(
        enumeration = "DotaOverheadAlert",
        required,
        tag = "1",
        default = "OverheadAlertGold"
    )]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTutorialTipInfo {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub progress: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTutorialFinish {
    #[prost(string, optional, tag = "1")]
    pub heading: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub emblem: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub body: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "4")]
    pub success: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTutorialMinimapPosition {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSendGenericToolTip {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub entindex: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub close: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgWorldLine {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub worldline: ::core::option::Option<CdotaMsgWorldLine>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgChatWheel {
    #[prost(uint32, optional, tag = "1", default = "4294967295")]
    pub chat_message_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub param_hero_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub emoticon_id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgReceivedXmasGift {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub item_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub inventory_slot: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgShowSurvey {
    #[prost(int32, optional, tag = "1")]
    pub survey_id: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "2")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub response_style: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub teammate_hero_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "5")]
    pub teammate_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "6")]
    pub teammate_account_id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgUpdateSharedContent {
    #[prost(int32, optional, tag = "1")]
    pub slot_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTutorialRequestExp {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTutorialFade {
    #[prost(int32, optional, tag = "1")]
    pub tgt_alpha: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTutorialPingMinimap {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgGamerulesStateChanged {
    #[prost(uint32, optional, tag = "1")]
    pub state: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgAddQuestLogEntry {
    #[prost(string, optional, tag = "1")]
    pub npc_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub npc_dialog: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSendStatPopup {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub statpopup: ::core::option::Option<CdotaMsgSendStatPopup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgDismissAllStatPopups {
    #[prost(message, optional, tag = "1")]
    pub dismissallmsg: ::core::option::Option<CdotaMsgDismissAllStatPopups>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSendRoshanSpectatorPhase {
    #[prost(
        enumeration = "DotaRoshanPhase",
        optional,
        tag = "1",
        default = "KSrspRoshanAlive"
    )]
    pub phase: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub phase_start_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub phase_length: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSendRoshanPopup {
    #[prost(bool, optional, tag = "1")]
    pub reclaimed: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "2")]
    pub gametime: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSendFinalGold {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub reliable_gold: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub unreliable_gold: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgCustomMsg {
    #[prost(string, optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub value: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgCoachHudPing {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub hud_ping: ::core::option::Option<CdotaMsgCoachHudPing>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgClientLoadGridNav {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTeProjectile {
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
    pub particle_cp_data: ::prost::alloc::vec::Vec<CdotaUserMsgProjectileParticleCpData>,
    #[prost(int64, optional, tag = "16")]
    pub additional_particle_system_handle: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "17")]
    pub original_move_speed: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "18", default = "16777215")]
    pub ability: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTeProjectileLoc {
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
    pub particle_cp_data: ::prost::alloc::vec::Vec<CdotaUserMsgProjectileParticleCpData>,
    #[prost(int64, optional, tag = "17")]
    pub additional_particle_system_handle: ::core::option::Option<i64>,
    #[prost(int32, optional, tag = "18")]
    pub original_move_speed: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTeDestroyProjectile {
    #[prost(int32, optional, tag = "1")]
    pub handle: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTeDotaBloodImpact {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub entity: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub scale: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub xnormal: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub ynormal: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgAbilityPing {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub ability_id: ::core::option::Option<i32>,
    #[prost(
        enumeration = "DotaAbilityPingType",
        optional,
        tag = "3",
        default = "AbilityPingReady"
    )]
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
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTeUnitAnimation {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTeUnitAnimationEnd {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub entity: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "2")]
    pub snap: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgShowGenericPopup {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgVoteStart {
    #[prost(string, optional, tag = "1")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "3")]
    pub choice_count: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "4")]
    pub choices: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgVoteUpdate {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub choice_counts: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgVoteEnd {
    #[prost(int32, optional, tag = "1")]
    pub selected_choice: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgBoosterStatePlayer {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgBoosterState {
    #[prost(message, repeated, tag = "1")]
    pub boosted_players: ::prost::alloc::vec::Vec<CdotaUserMsgBoosterStatePlayer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgPlayerMmr {
    #[prost(sint32, repeated, tag = "1")]
    pub mmr: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgAbilitySteal {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub ability_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub ability_level: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgStatsHeroLookup {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub hero_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub hero_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub persona: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgStatsHeroPositionInfo {
    #[prost(float, optional, tag = "1")]
    pub average_position: ::core::option::Option<f32>,
    #[prost(message, repeated, tag = "2")]
    pub position_details: ::prost::alloc::vec::Vec<
        cdota_user_msg_stats_hero_position_info::PositionPair,
    >,
}
/// Nested message and enum types in `CDOTAUserMsg_StatsHeroPositionInfo`.
pub mod cdota_user_msg_stats_hero_position_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PositionPair {
        #[prost(
            enumeration = "super::DotaPositionCategory",
            optional,
            tag = "1",
            default = "DotaPositionNone"
        )]
        pub position_category: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "2")]
        pub position_count: ::core::option::Option<u32>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgStatsHeroMinuteDetails {
    #[prost(uint32, optional, tag = "1")]
    pub last_hits: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub hero_kills: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub hero_damage: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub tower_damage: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "5")]
    pub position_info: ::core::option::Option<CdotaUserMsgStatsHeroPositionInfo>,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgStatsTeamMinuteDetails {
    #[prost(message, repeated, tag = "1")]
    pub player_stats: ::prost::alloc::vec::Vec<CdotaUserMsgStatsHeroMinuteDetails>,
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
    pub lane_performance: ::prost::alloc::vec::Vec<
        cdota_user_msg_stats_team_minute_details::LocationPerformance,
    >,
}
/// Nested message and enum types in `CDOTAUserMsg_StatsTeamMinuteDetails`.
pub mod cdota_user_msg_stats_team_minute_details {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationPerformance {
        #[prost(uint32, optional, tag = "1")]
        pub location_category: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub stat_type: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub value: ::core::option::Option<u32>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgStatsPlayerKillShare {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgStatsKillDetails {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub victim_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub kill_shares: ::prost::alloc::vec::Vec<CdotaUserMsgStatsPlayerKillShare>,
    #[prost(uint32, optional, tag = "3")]
    pub damage_to_kill: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub effective_health: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "5")]
    pub death_time: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "6", default = "-1")]
    pub killer_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgStatsMatchDetails {
    #[prost(message, repeated, tag = "1")]
    pub hero_lookup: ::prost::alloc::vec::Vec<CdotaUserMsgStatsHeroLookup>,
    #[prost(message, repeated, tag = "2")]
    pub radiant_stats: ::prost::alloc::vec::Vec<CdotaUserMsgStatsTeamMinuteDetails>,
    #[prost(message, repeated, tag = "3")]
    pub dire_stats: ::prost::alloc::vec::Vec<CdotaUserMsgStatsTeamMinuteDetails>,
    #[prost(message, repeated, tag = "4")]
    pub radiant_kills: ::prost::alloc::vec::Vec<CdotaUserMsgStatsKillDetails>,
    #[prost(message, repeated, tag = "5")]
    pub dire_kills: ::prost::alloc::vec::Vec<CdotaUserMsgStatsKillDetails>,
    #[prost(message, repeated, tag = "6")]
    pub fight_details: ::prost::alloc::vec::Vec<
        cdota_user_msg_stats_match_details::CdotaUserMsgStatsFightDetails,
    >,
}
/// Nested message and enum types in `CDOTAUserMsg_StatsMatchDetails`.
pub mod cdota_user_msg_stats_match_details {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CdotaUserMsgStatsFightTeamDetails {
        #[prost(int32, repeated, packed = "false", tag = "1")]
        pub participants: ::prost::alloc::vec::Vec<i32>,
        #[prost(int32, repeated, packed = "false", tag = "2")]
        pub deaths: ::prost::alloc::vec::Vec<i32>,
        #[prost(uint32, optional, tag = "3")]
        pub gold_delta: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub xp_delta: ::core::option::Option<u32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CdotaUserMsgStatsFightDetails {
        #[prost(float, optional, tag = "1")]
        pub start_time: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "2")]
        pub end_time: ::core::option::Option<f32>,
        #[prost(message, optional, tag = "3")]
        pub radiant_fight_details: ::core::option::Option<
            CdotaUserMsgStatsFightTeamDetails,
        >,
        #[prost(message, optional, tag = "4")]
        pub dire_fight_details: ::core::option::Option<
            CdotaUserMsgStatsFightTeamDetails,
        >,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgMiniTaunt {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub taunting_player_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSpeechBubble {
    #[prost(bool, optional, tag = "1")]
    pub destroy_all: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgCustomHeaderMessage {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
    #[prost(string, optional, tag = "3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub value: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgHeroAbilityStat {
    #[prost(
        enumeration = "EHeroStatType",
        optional,
        tag = "1",
        default = "KEHeroStatTypeNone"
    )]
    pub stat_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub int_value: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "3")]
    pub float_value: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgCombatAnalyzerPlayerStat {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub hero_ability_stats: ::prost::alloc::vec::Vec<CMsgHeroAbilityStat>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgCombatAnalyzerStats {
    #[prost(uint64, optional, tag = "1")]
    pub match_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "2")]
    pub player_stats: ::prost::alloc::vec::Vec<CMsgCombatAnalyzerPlayerStat>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgBeastChat {
    #[prost(uint32, optional, tag = "1")]
    pub team: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub format: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub target: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgCustomHudElementCreate {
    #[prost(string, optional, tag = "1")]
    pub element_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub layout_filename: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgCustomHudElementModify {
    #[prost(string, optional, tag = "1")]
    pub element_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub modify_visible: ::core::option::Option<bool>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgCustomHudElementDestroy {
    #[prost(string, optional, tag = "1")]
    pub element_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgCompendiumStatePlayer {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub level: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgCompendiumState {
    #[prost(message, repeated, tag = "1")]
    pub compendium_players: ::prost::alloc::vec::Vec<CdotaUserMsgCompendiumStatePlayer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgProjectionAbility {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgProjectionEvent {
    #[prost(
        enumeration = "EProjectionEvent",
        optional,
        tag = "1",
        default = "EPeFirstBlood"
    )]
    pub event_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub team: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgXpAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_entindex: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTalentTreeAlert {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgUpdateQuestProgress {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgQuestStatus {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSuggestHeroPick {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub player_id: i32,
    #[prost(uint32, optional, tag = "2")]
    pub hero_id: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub ban: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSuggestHeroRole {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub player_id: i32,
    #[prost(string, optional, tag = "2")]
    pub hero_role: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgKillcamDamageTaken {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSelectPenaltyGold {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub player_id: i32,
    #[prost(sint32, optional, tag = "2")]
    pub cost: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgRollDiceResult {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgFlipCoinResult {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub channel_type: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub result: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMessageRequestItemSuggestions {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMessageTeamCaptainChanged {
    #[prost(uint32, optional, tag = "1")]
    pub team: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub captain_player_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgChatWheelCooldown {
    #[prost(uint32, optional, tag = "1", default = "4294967295")]
    pub message_id: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub cooldown_remaining: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgHeroRelicProgress {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgAbilityDraftRequestAbility {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub requested_ability_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub ctrl_is_down: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgDamageReport {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub target_hero_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub source_hero_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "4")]
    pub damage_amount: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "5")]
    pub broadcast: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgSalutePlayer {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgTipAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub tip_text: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgReplaceQueryUnit {
    #[prost(int32, required, tag = "1", default = "-1")]
    pub player_id: i32,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub source_entindex: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub target_entindex: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgEsArcanaCombo {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub combo_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub arcana_level: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgEsArcanaComboSummary {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub combo_count: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub damage_amount: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgOmArcanaCombo {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub multicast_amount: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub arcana_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub multicast_chance: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgHighFiveCompleted {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id_2: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub special_high_five: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "4", default = "-1")]
    pub special_entindex: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgHighFiveLeftHanging {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgShovelUnearth {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub all_chat: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "3")]
    pub locstring: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub quantity: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgAllStarEvent {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub point_amount: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub event_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "5")]
    pub player_scores: ::prost::alloc::vec::Vec<
        cdota_user_msg_all_star_event::PlayerScore,
    >,
}
/// Nested message and enum types in `CDOTAUserMsg_AllStarEvent`.
pub mod cdota_user_msg_all_star_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlayerScore {
        #[prost(int32, optional, tag = "1", default = "-1")]
        pub player_id: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "2")]
        pub score_sans_kda: ::core::option::Option<u32>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgQueuedOrderRemoved {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub unit_order_sequence: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgDebugChallenge {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgFoundNeutralItem {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub item_ability_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub item_tier: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub tier_item_count: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgOutpostCaptured {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub outpost_entindex: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub team_id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgOutpostGrantedXp {
    #[prost(uint32, optional, tag = "1")]
    pub team_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub xp_amount: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgMoveCameraToUnit {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub unit_ehandle: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgPauseMinigameData {
    #[prost(message, repeated, tag = "1")]
    pub data_bits: ::prost::alloc::vec::Vec<cdota_user_msg_pause_minigame_data::DataBit>,
}
/// Nested message and enum types in `CDOTAUserMsg_PauseMinigameData`.
pub mod cdota_user_msg_pause_minigame_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DataBit {
        #[prost(uint32, optional, tag = "1")]
        pub index: ::core::option::Option<u32>,
        #[prost(int32, optional, tag = "2")]
        pub data: ::core::option::Option<i32>,
        #[prost(int64, optional, tag = "3")]
        pub data_extra: ::core::option::Option<i64>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgVersusScenePlayerBehavior {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(
        enumeration = "EdotaVersusScenePlayerBehavior",
        optional,
        tag = "2",
        default = "VsPlayerBehaviorPlayActivity"
    )]
    pub behavior: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub play_activity: ::core::option::Option<VersusScenePlayActivity>,
    #[prost(message, optional, tag = "4")]
    pub chat_wheel: ::core::option::Option<VersusSceneChatWheel>,
    #[prost(message, optional, tag = "5")]
    pub playback_rate: ::core::option::Option<VersusScenePlaybackRate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgQoPArcanaSummary {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub arcana_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub players_hit: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub players_killed: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgHotPotatoCreated {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id_2: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgHotPotatoExploded {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgWkArcanaProgress {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub ehandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub arcana_level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub hero_id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgGuildChallengeProgress {
    #[prost(message, repeated, tag = "1")]
    pub player_progress: ::prost::alloc::vec::Vec<
        cdota_user_msg_guild_challenge_progress::PlayerProgress,
    >,
    #[prost(uint32, optional, tag = "2")]
    pub guild_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub challenge_instance_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub challenge_parameter: ::core::option::Option<u32>,
    #[prost(
        enumeration = "cdota_user_msg_guild_challenge_progress::EChallengeType",
        optional,
        tag = "5",
        default = "KEChallengeTypeInvalid"
    )]
    pub challenge_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "7")]
    pub challenge_progress_at_start: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "8")]
    pub complete: ::core::option::Option<bool>,
}
/// Nested message and enum types in `CDOTAUserMsg_GuildChallenge_Progress`.
pub mod cdota_user_msg_guild_challenge_progress {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlayerProgress {
        #[prost(int32, optional, tag = "1", default = "-1")]
        pub player_id: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "6")]
        pub progress: ::core::option::Option<u32>,
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EChallengeType {
        KEChallengeTypeInvalid = 0,
        KEChallengeTypeCooperative = 1,
        KEChallengeTypeContract = 2,
    }
    impl EChallengeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EChallengeType::KEChallengeTypeInvalid => "k_EChallengeType_Invalid",
                EChallengeType::KEChallengeTypeCooperative => {
                    "k_EChallengeType_Cooperative"
                }
                EChallengeType::KEChallengeTypeContract => "k_EChallengeType_Contract",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "k_EChallengeType_Invalid" => Some(Self::KEChallengeTypeInvalid),
                "k_EChallengeType_Cooperative" => Some(Self::KEChallengeTypeCooperative),
                "k_EChallengeType_Contract" => Some(Self::KEChallengeTypeContract),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgWrArcanaProgress {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgWrArcanaSummary {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgEmptyItemSlotAlert {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub target_player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub slot_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub cooldown_seconds: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgAghsStatusAlert {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgMutedPlayers {
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub text_muted_player_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "2")]
    pub voice_muted_player_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgContextualTip {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgChatMessage {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_player_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub channel_type: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub message_text: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgRockPaperScissorsStarted {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_source: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id_target: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgRockPaperScissorsFinished {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id_2: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub player_1_choice: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub player_2_choice: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgDuelOpponentKilled {
    #[prost(int32, optional, tag = "1")]
    pub player_id_winner: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub player_id_loser: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgDuelAccepted {
    #[prost(int32, optional, tag = "1")]
    pub player_id_1: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub player_id_2: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgDuelRequested {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_requestor: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgMuertaReleaseEventAssignedTargetKilled {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgPlayerDraftSuggestPick {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub suggestion_player_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdotaUserMsgPlayerDraftPick {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_id_captain: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub player_id_target: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub team: ::core::option::Option<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    DotaUmPlayerMmr = 531,
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
}
impl EDotaUserMessages {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EDotaUserMessages::DotaUmAddUnitToSelection => "DOTA_UM_AddUnitToSelection",
            EDotaUserMessages::DotaUmAiDebugLine => "DOTA_UM_AIDebugLine",
            EDotaUserMessages::DotaUmChatEvent => "DOTA_UM_ChatEvent",
            EDotaUserMessages::DotaUmCombatHeroPositions => "DOTA_UM_CombatHeroPositions",
            EDotaUserMessages::DotaUmCombatLogData => "DOTA_UM_CombatLogData",
            EDotaUserMessages::DotaUmCombatLogBulkData => "DOTA_UM_CombatLogBulkData",
            EDotaUserMessages::DotaUmCreateLinearProjectile => {
                "DOTA_UM_CreateLinearProjectile"
            }
            EDotaUserMessages::DotaUmDestroyLinearProjectile => {
                "DOTA_UM_DestroyLinearProjectile"
            }
            EDotaUserMessages::DotaUmDodgeTrackingProjectiles => {
                "DOTA_UM_DodgeTrackingProjectiles"
            }
            EDotaUserMessages::DotaUmGlobalLightColor => "DOTA_UM_GlobalLightColor",
            EDotaUserMessages::DotaUmGlobalLightDirection => {
                "DOTA_UM_GlobalLightDirection"
            }
            EDotaUserMessages::DotaUmInvalidCommand => "DOTA_UM_InvalidCommand",
            EDotaUserMessages::DotaUmLocationPing => "DOTA_UM_LocationPing",
            EDotaUserMessages::DotaUmMapLine => "DOTA_UM_MapLine",
            EDotaUserMessages::DotaUmMiniKillCamInfo => "DOTA_UM_MiniKillCamInfo",
            EDotaUserMessages::DotaUmMinimapDebugPoint => "DOTA_UM_MinimapDebugPoint",
            EDotaUserMessages::DotaUmMinimapEvent => "DOTA_UM_MinimapEvent",
            EDotaUserMessages::DotaUmNevermoreRequiem => "DOTA_UM_NevermoreRequiem",
            EDotaUserMessages::DotaUmOverheadEvent => "DOTA_UM_OverheadEvent",
            EDotaUserMessages::DotaUmSetNextAutobuyItem => "DOTA_UM_SetNextAutobuyItem",
            EDotaUserMessages::DotaUmSharedCooldown => "DOTA_UM_SharedCooldown",
            EDotaUserMessages::DotaUmSpectatorPlayerClick => {
                "DOTA_UM_SpectatorPlayerClick"
            }
            EDotaUserMessages::DotaUmTutorialTipInfo => "DOTA_UM_TutorialTipInfo",
            EDotaUserMessages::DotaUmUnitEvent => "DOTA_UM_UnitEvent",
            EDotaUserMessages::DotaUmParticleManager => "DOTA_UM_ParticleManager",
            EDotaUserMessages::DotaUmBotChat => "DOTA_UM_BotChat",
            EDotaUserMessages::DotaUmHudError => "DOTA_UM_HudError",
            EDotaUserMessages::DotaUmItemPurchased => "DOTA_UM_ItemPurchased",
            EDotaUserMessages::DotaUmPing => "DOTA_UM_Ping",
            EDotaUserMessages::DotaUmItemFound => "DOTA_UM_ItemFound",
            EDotaUserMessages::DotaUmCharacterSpeakConcept => {
                "DOTA_UM_CharacterSpeakConcept"
            }
            EDotaUserMessages::DotaUmSwapVerify => "DOTA_UM_SwapVerify",
            EDotaUserMessages::DotaUmWorldLine => "DOTA_UM_WorldLine",
            EDotaUserMessages::DotaUmTournamentDrop => "DOTA_UM_TournamentDrop",
            EDotaUserMessages::DotaUmItemAlert => "DOTA_UM_ItemAlert",
            EDotaUserMessages::DotaUmHalloweenDrops => "DOTA_UM_HalloweenDrops",
            EDotaUserMessages::DotaUmChatWheel => "DOTA_UM_ChatWheel",
            EDotaUserMessages::DotaUmReceivedXmasGift => "DOTA_UM_ReceivedXmasGift",
            EDotaUserMessages::DotaUmUpdateSharedContent => "DOTA_UM_UpdateSharedContent",
            EDotaUserMessages::DotaUmTutorialRequestExp => "DOTA_UM_TutorialRequestExp",
            EDotaUserMessages::DotaUmTutorialPingMinimap => "DOTA_UM_TutorialPingMinimap",
            EDotaUserMessages::DotaUmGamerulesStateChanged => {
                "DOTA_UM_GamerulesStateChanged"
            }
            EDotaUserMessages::DotaUmShowSurvey => "DOTA_UM_ShowSurvey",
            EDotaUserMessages::DotaUmTutorialFade => "DOTA_UM_TutorialFade",
            EDotaUserMessages::DotaUmAddQuestLogEntry => "DOTA_UM_AddQuestLogEntry",
            EDotaUserMessages::DotaUmSendStatPopup => "DOTA_UM_SendStatPopup",
            EDotaUserMessages::DotaUmTutorialFinish => "DOTA_UM_TutorialFinish",
            EDotaUserMessages::DotaUmSendRoshanPopup => "DOTA_UM_SendRoshanPopup",
            EDotaUserMessages::DotaUmSendGenericToolTip => "DOTA_UM_SendGenericToolTip",
            EDotaUserMessages::DotaUmSendFinalGold => "DOTA_UM_SendFinalGold",
            EDotaUserMessages::DotaUmCustomMsg => "DOTA_UM_CustomMsg",
            EDotaUserMessages::DotaUmCoachHudPing => "DOTA_UM_CoachHUDPing",
            EDotaUserMessages::DotaUmClientLoadGridNav => "DOTA_UM_ClientLoadGridNav",
            EDotaUserMessages::DotaUmTeProjectile => "DOTA_UM_TE_Projectile",
            EDotaUserMessages::DotaUmTeProjectileLoc => "DOTA_UM_TE_ProjectileLoc",
            EDotaUserMessages::DotaUmTeDotaBloodImpact => "DOTA_UM_TE_DotaBloodImpact",
            EDotaUserMessages::DotaUmTeUnitAnimation => "DOTA_UM_TE_UnitAnimation",
            EDotaUserMessages::DotaUmTeUnitAnimationEnd => "DOTA_UM_TE_UnitAnimationEnd",
            EDotaUserMessages::DotaUmAbilityPing => "DOTA_UM_AbilityPing",
            EDotaUserMessages::DotaUmShowGenericPopup => "DOTA_UM_ShowGenericPopup",
            EDotaUserMessages::DotaUmVoteStart => "DOTA_UM_VoteStart",
            EDotaUserMessages::DotaUmVoteUpdate => "DOTA_UM_VoteUpdate",
            EDotaUserMessages::DotaUmVoteEnd => "DOTA_UM_VoteEnd",
            EDotaUserMessages::DotaUmBoosterState => "DOTA_UM_BoosterState",
            EDotaUserMessages::DotaUmWillPurchaseAlert => "DOTA_UM_WillPurchaseAlert",
            EDotaUserMessages::DotaUmTutorialMinimapPosition => {
                "DOTA_UM_TutorialMinimapPosition"
            }
            EDotaUserMessages::DotaUmPlayerMmr => "DOTA_UM_PlayerMMR",
            EDotaUserMessages::DotaUmAbilitySteal => "DOTA_UM_AbilitySteal",
            EDotaUserMessages::DotaUmCourierKilledAlert => "DOTA_UM_CourierKilledAlert",
            EDotaUserMessages::DotaUmEnemyItemAlert => "DOTA_UM_EnemyItemAlert",
            EDotaUserMessages::DotaUmStatsMatchDetails => "DOTA_UM_StatsMatchDetails",
            EDotaUserMessages::DotaUmMiniTaunt => "DOTA_UM_MiniTaunt",
            EDotaUserMessages::DotaUmBuyBackStateAlert => "DOTA_UM_BuyBackStateAlert",
            EDotaUserMessages::DotaUmSpeechBubble => "DOTA_UM_SpeechBubble",
            EDotaUserMessages::DotaUmCustomHeaderMessage => "DOTA_UM_CustomHeaderMessage",
            EDotaUserMessages::DotaUmQuickBuyAlert => "DOTA_UM_QuickBuyAlert",
            EDotaUserMessages::DotaUmStatsHeroDetails => "DOTA_UM_StatsHeroDetails",
            EDotaUserMessages::DotaUmPredictionResult => "DOTA_UM_PredictionResult",
            EDotaUserMessages::DotaUmModifierAlert => "DOTA_UM_ModifierAlert",
            EDotaUserMessages::DotaUmHpManaAlert => "DOTA_UM_HPManaAlert",
            EDotaUserMessages::DotaUmGlyphAlert => "DOTA_UM_GlyphAlert",
            EDotaUserMessages::DotaUmBeastChat => "DOTA_UM_BeastChat",
            EDotaUserMessages::DotaUmSpectatorPlayerUnitOrders => {
                "DOTA_UM_SpectatorPlayerUnitOrders"
            }
            EDotaUserMessages::DotaUmCustomHudElementCreate => {
                "DOTA_UM_CustomHudElement_Create"
            }
            EDotaUserMessages::DotaUmCustomHudElementModify => {
                "DOTA_UM_CustomHudElement_Modify"
            }
            EDotaUserMessages::DotaUmCustomHudElementDestroy => {
                "DOTA_UM_CustomHudElement_Destroy"
            }
            EDotaUserMessages::DotaUmCompendiumState => "DOTA_UM_CompendiumState",
            EDotaUserMessages::DotaUmProjectionAbility => "DOTA_UM_ProjectionAbility",
            EDotaUserMessages::DotaUmProjectionEvent => "DOTA_UM_ProjectionEvent",
            EDotaUserMessages::DotaUmCombatLogDataHltv => "DOTA_UM_CombatLogDataHLTV",
            EDotaUserMessages::DotaUmXpAlert => "DOTA_UM_XPAlert",
            EDotaUserMessages::DotaUmUpdateQuestProgress => "DOTA_UM_UpdateQuestProgress",
            EDotaUserMessages::DotaUmMatchMetadata => "DOTA_UM_MatchMetadata",
            EDotaUserMessages::DotaUmMatchDetails => "DOTA_UM_MatchDetails",
            EDotaUserMessages::DotaUmQuestStatus => "DOTA_UM_QuestStatus",
            EDotaUserMessages::DotaUmSuggestHeroPick => "DOTA_UM_SuggestHeroPick",
            EDotaUserMessages::DotaUmSuggestHeroRole => "DOTA_UM_SuggestHeroRole",
            EDotaUserMessages::DotaUmKillcamDamageTaken => "DOTA_UM_KillcamDamageTaken",
            EDotaUserMessages::DotaUmSelectPenaltyGold => "DOTA_UM_SelectPenaltyGold",
            EDotaUserMessages::DotaUmRollDiceResult => "DOTA_UM_RollDiceResult",
            EDotaUserMessages::DotaUmFlipCoinResult => "DOTA_UM_FlipCoinResult",
            EDotaUserMessages::DotaUmRequestItemSuggestions => {
                "DOTA_UM_RequestItemSuggestions"
            }
            EDotaUserMessages::DotaUmTeamCaptainChanged => "DOTA_UM_TeamCaptainChanged",
            EDotaUserMessages::DotaUmSendRoshanSpectatorPhase => {
                "DOTA_UM_SendRoshanSpectatorPhase"
            }
            EDotaUserMessages::DotaUmChatWheelCooldown => "DOTA_UM_ChatWheelCooldown",
            EDotaUserMessages::DotaUmDismissAllStatPopups => {
                "DOTA_UM_DismissAllStatPopups"
            }
            EDotaUserMessages::DotaUmTeDestroyProjectile => {
                "DOTA_UM_TE_DestroyProjectile"
            }
            EDotaUserMessages::DotaUmHeroRelicProgress => "DOTA_UM_HeroRelicProgress",
            EDotaUserMessages::DotaUmAbilityDraftRequestAbility => {
                "DOTA_UM_AbilityDraftRequestAbility"
            }
            EDotaUserMessages::DotaUmItemSold => "DOTA_UM_ItemSold",
            EDotaUserMessages::DotaUmDamageReport => "DOTA_UM_DamageReport",
            EDotaUserMessages::DotaUmSalutePlayer => "DOTA_UM_SalutePlayer",
            EDotaUserMessages::DotaUmTipAlert => "DOTA_UM_TipAlert",
            EDotaUserMessages::DotaUmReplaceQueryUnit => "DOTA_UM_ReplaceQueryUnit",
            EDotaUserMessages::DotaUmEmptyTeleportAlert => "DOTA_UM_EmptyTeleportAlert",
            EDotaUserMessages::DotaUmMarsArenaOfBloodAttack => {
                "DOTA_UM_MarsArenaOfBloodAttack"
            }
            EDotaUserMessages::DotaUmEsArcanaCombo => "DOTA_UM_ESArcanaCombo",
            EDotaUserMessages::DotaUmEsArcanaComboSummary => {
                "DOTA_UM_ESArcanaComboSummary"
            }
            EDotaUserMessages::DotaUmHighFiveLeftHanging => "DOTA_UM_HighFiveLeftHanging",
            EDotaUserMessages::DotaUmHighFiveCompleted => "DOTA_UM_HighFiveCompleted",
            EDotaUserMessages::DotaUmShovelUnearth => "DOTA_UM_ShovelUnearth",
            EDotaUserMessages::DotaEmInvokerSpellCast => "DOTA_EM_InvokerSpellCast",
            EDotaUserMessages::DotaUmRadarAlert => "DOTA_UM_RadarAlert",
            EDotaUserMessages::DotaUmAllStarEvent => "DOTA_UM_AllStarEvent",
            EDotaUserMessages::DotaUmTalentTreeAlert => "DOTA_UM_TalentTreeAlert",
            EDotaUserMessages::DotaUmQueuedOrderRemoved => "DOTA_UM_QueuedOrderRemoved",
            EDotaUserMessages::DotaUmDebugChallenge => "DOTA_UM_DebugChallenge",
            EDotaUserMessages::DotaUmOmArcanaCombo => "DOTA_UM_OMArcanaCombo",
            EDotaUserMessages::DotaUmFoundNeutralItem => "DOTA_UM_FoundNeutralItem",
            EDotaUserMessages::DotaUmOutpostCaptured => "DOTA_UM_OutpostCaptured",
            EDotaUserMessages::DotaUmOutpostGrantedXp => "DOTA_UM_OutpostGrantedXP",
            EDotaUserMessages::DotaUmMoveCameraToUnit => "DOTA_UM_MoveCameraToUnit",
            EDotaUserMessages::DotaUmPauseMinigameData => "DOTA_UM_PauseMinigameData",
            EDotaUserMessages::DotaUmVersusScenePlayerBehavior => {
                "DOTA_UM_VersusScene_PlayerBehavior"
            }
            EDotaUserMessages::DotaUmQoPArcanaSummary => "DOTA_UM_QoP_ArcanaSummary",
            EDotaUserMessages::DotaUmHotPotatoCreated => "DOTA_UM_HotPotato_Created",
            EDotaUserMessages::DotaUmHotPotatoExploded => "DOTA_UM_HotPotato_Exploded",
            EDotaUserMessages::DotaUmWkArcanaProgress => "DOTA_UM_WK_Arcana_Progress",
            EDotaUserMessages::DotaUmGuildChallengeProgress => {
                "DOTA_UM_GuildChallenge_Progress"
            }
            EDotaUserMessages::DotaUmWrArcanaProgress => "DOTA_UM_WRArcanaProgress",
            EDotaUserMessages::DotaUmWrArcanaSummary => "DOTA_UM_WRArcanaSummary",
            EDotaUserMessages::DotaUmEmptyItemSlotAlert => "DOTA_UM_EmptyItemSlotAlert",
            EDotaUserMessages::DotaUmAghsStatusAlert => "DOTA_UM_AghsStatusAlert",
            EDotaUserMessages::DotaUmPingConfirmation => "DOTA_UM_PingConfirmation",
            EDotaUserMessages::DotaUmMutedPlayers => "DOTA_UM_MutedPlayers",
            EDotaUserMessages::DotaUmContextualTip => "DOTA_UM_ContextualTip",
            EDotaUserMessages::DotaUmChatMessage => "DOTA_UM_ChatMessage",
            EDotaUserMessages::DotaUmNeutralCampAlert => "DOTA_UM_NeutralCampAlert",
            EDotaUserMessages::DotaUmRockPaperScissorsStarted => {
                "DOTA_UM_RockPaperScissorsStarted"
            }
            EDotaUserMessages::DotaUmRockPaperScissorsFinished => {
                "DOTA_UM_RockPaperScissorsFinished"
            }
            EDotaUserMessages::DotaUmDuelOpponentKilled => "DOTA_UM_DuelOpponentKilled",
            EDotaUserMessages::DotaUmDuelAccepted => "DOTA_UM_DuelAccepted",
            EDotaUserMessages::DotaUmDuelRequested => "DOTA_UM_DuelRequested",
            EDotaUserMessages::DotaUmMuertaReleaseEventAssignedTargetKilled => {
                "DOTA_UM_MuertaReleaseEvent_AssignedTargetKilled"
            }
            EDotaUserMessages::DotaUmPlayerDraftSuggestPick => {
                "DOTA_UM_PlayerDraftSuggestPick"
            }
            EDotaUserMessages::DotaUmPlayerDraftPick => "DOTA_UM_PlayerDraftPick",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_UM_AddUnitToSelection" => Some(Self::DotaUmAddUnitToSelection),
            "DOTA_UM_AIDebugLine" => Some(Self::DotaUmAiDebugLine),
            "DOTA_UM_ChatEvent" => Some(Self::DotaUmChatEvent),
            "DOTA_UM_CombatHeroPositions" => Some(Self::DotaUmCombatHeroPositions),
            "DOTA_UM_CombatLogData" => Some(Self::DotaUmCombatLogData),
            "DOTA_UM_CombatLogBulkData" => Some(Self::DotaUmCombatLogBulkData),
            "DOTA_UM_CreateLinearProjectile" => Some(Self::DotaUmCreateLinearProjectile),
            "DOTA_UM_DestroyLinearProjectile" => {
                Some(Self::DotaUmDestroyLinearProjectile)
            }
            "DOTA_UM_DodgeTrackingProjectiles" => {
                Some(Self::DotaUmDodgeTrackingProjectiles)
            }
            "DOTA_UM_GlobalLightColor" => Some(Self::DotaUmGlobalLightColor),
            "DOTA_UM_GlobalLightDirection" => Some(Self::DotaUmGlobalLightDirection),
            "DOTA_UM_InvalidCommand" => Some(Self::DotaUmInvalidCommand),
            "DOTA_UM_LocationPing" => Some(Self::DotaUmLocationPing),
            "DOTA_UM_MapLine" => Some(Self::DotaUmMapLine),
            "DOTA_UM_MiniKillCamInfo" => Some(Self::DotaUmMiniKillCamInfo),
            "DOTA_UM_MinimapDebugPoint" => Some(Self::DotaUmMinimapDebugPoint),
            "DOTA_UM_MinimapEvent" => Some(Self::DotaUmMinimapEvent),
            "DOTA_UM_NevermoreRequiem" => Some(Self::DotaUmNevermoreRequiem),
            "DOTA_UM_OverheadEvent" => Some(Self::DotaUmOverheadEvent),
            "DOTA_UM_SetNextAutobuyItem" => Some(Self::DotaUmSetNextAutobuyItem),
            "DOTA_UM_SharedCooldown" => Some(Self::DotaUmSharedCooldown),
            "DOTA_UM_SpectatorPlayerClick" => Some(Self::DotaUmSpectatorPlayerClick),
            "DOTA_UM_TutorialTipInfo" => Some(Self::DotaUmTutorialTipInfo),
            "DOTA_UM_UnitEvent" => Some(Self::DotaUmUnitEvent),
            "DOTA_UM_ParticleManager" => Some(Self::DotaUmParticleManager),
            "DOTA_UM_BotChat" => Some(Self::DotaUmBotChat),
            "DOTA_UM_HudError" => Some(Self::DotaUmHudError),
            "DOTA_UM_ItemPurchased" => Some(Self::DotaUmItemPurchased),
            "DOTA_UM_Ping" => Some(Self::DotaUmPing),
            "DOTA_UM_ItemFound" => Some(Self::DotaUmItemFound),
            "DOTA_UM_CharacterSpeakConcept" => Some(Self::DotaUmCharacterSpeakConcept),
            "DOTA_UM_SwapVerify" => Some(Self::DotaUmSwapVerify),
            "DOTA_UM_WorldLine" => Some(Self::DotaUmWorldLine),
            "DOTA_UM_TournamentDrop" => Some(Self::DotaUmTournamentDrop),
            "DOTA_UM_ItemAlert" => Some(Self::DotaUmItemAlert),
            "DOTA_UM_HalloweenDrops" => Some(Self::DotaUmHalloweenDrops),
            "DOTA_UM_ChatWheel" => Some(Self::DotaUmChatWheel),
            "DOTA_UM_ReceivedXmasGift" => Some(Self::DotaUmReceivedXmasGift),
            "DOTA_UM_UpdateSharedContent" => Some(Self::DotaUmUpdateSharedContent),
            "DOTA_UM_TutorialRequestExp" => Some(Self::DotaUmTutorialRequestExp),
            "DOTA_UM_TutorialPingMinimap" => Some(Self::DotaUmTutorialPingMinimap),
            "DOTA_UM_GamerulesStateChanged" => Some(Self::DotaUmGamerulesStateChanged),
            "DOTA_UM_ShowSurvey" => Some(Self::DotaUmShowSurvey),
            "DOTA_UM_TutorialFade" => Some(Self::DotaUmTutorialFade),
            "DOTA_UM_AddQuestLogEntry" => Some(Self::DotaUmAddQuestLogEntry),
            "DOTA_UM_SendStatPopup" => Some(Self::DotaUmSendStatPopup),
            "DOTA_UM_TutorialFinish" => Some(Self::DotaUmTutorialFinish),
            "DOTA_UM_SendRoshanPopup" => Some(Self::DotaUmSendRoshanPopup),
            "DOTA_UM_SendGenericToolTip" => Some(Self::DotaUmSendGenericToolTip),
            "DOTA_UM_SendFinalGold" => Some(Self::DotaUmSendFinalGold),
            "DOTA_UM_CustomMsg" => Some(Self::DotaUmCustomMsg),
            "DOTA_UM_CoachHUDPing" => Some(Self::DotaUmCoachHudPing),
            "DOTA_UM_ClientLoadGridNav" => Some(Self::DotaUmClientLoadGridNav),
            "DOTA_UM_TE_Projectile" => Some(Self::DotaUmTeProjectile),
            "DOTA_UM_TE_ProjectileLoc" => Some(Self::DotaUmTeProjectileLoc),
            "DOTA_UM_TE_DotaBloodImpact" => Some(Self::DotaUmTeDotaBloodImpact),
            "DOTA_UM_TE_UnitAnimation" => Some(Self::DotaUmTeUnitAnimation),
            "DOTA_UM_TE_UnitAnimationEnd" => Some(Self::DotaUmTeUnitAnimationEnd),
            "DOTA_UM_AbilityPing" => Some(Self::DotaUmAbilityPing),
            "DOTA_UM_ShowGenericPopup" => Some(Self::DotaUmShowGenericPopup),
            "DOTA_UM_VoteStart" => Some(Self::DotaUmVoteStart),
            "DOTA_UM_VoteUpdate" => Some(Self::DotaUmVoteUpdate),
            "DOTA_UM_VoteEnd" => Some(Self::DotaUmVoteEnd),
            "DOTA_UM_BoosterState" => Some(Self::DotaUmBoosterState),
            "DOTA_UM_WillPurchaseAlert" => Some(Self::DotaUmWillPurchaseAlert),
            "DOTA_UM_TutorialMinimapPosition" => {
                Some(Self::DotaUmTutorialMinimapPosition)
            }
            "DOTA_UM_PlayerMMR" => Some(Self::DotaUmPlayerMmr),
            "DOTA_UM_AbilitySteal" => Some(Self::DotaUmAbilitySteal),
            "DOTA_UM_CourierKilledAlert" => Some(Self::DotaUmCourierKilledAlert),
            "DOTA_UM_EnemyItemAlert" => Some(Self::DotaUmEnemyItemAlert),
            "DOTA_UM_StatsMatchDetails" => Some(Self::DotaUmStatsMatchDetails),
            "DOTA_UM_MiniTaunt" => Some(Self::DotaUmMiniTaunt),
            "DOTA_UM_BuyBackStateAlert" => Some(Self::DotaUmBuyBackStateAlert),
            "DOTA_UM_SpeechBubble" => Some(Self::DotaUmSpeechBubble),
            "DOTA_UM_CustomHeaderMessage" => Some(Self::DotaUmCustomHeaderMessage),
            "DOTA_UM_QuickBuyAlert" => Some(Self::DotaUmQuickBuyAlert),
            "DOTA_UM_StatsHeroDetails" => Some(Self::DotaUmStatsHeroDetails),
            "DOTA_UM_PredictionResult" => Some(Self::DotaUmPredictionResult),
            "DOTA_UM_ModifierAlert" => Some(Self::DotaUmModifierAlert),
            "DOTA_UM_HPManaAlert" => Some(Self::DotaUmHpManaAlert),
            "DOTA_UM_GlyphAlert" => Some(Self::DotaUmGlyphAlert),
            "DOTA_UM_BeastChat" => Some(Self::DotaUmBeastChat),
            "DOTA_UM_SpectatorPlayerUnitOrders" => {
                Some(Self::DotaUmSpectatorPlayerUnitOrders)
            }
            "DOTA_UM_CustomHudElement_Create" => Some(Self::DotaUmCustomHudElementCreate),
            "DOTA_UM_CustomHudElement_Modify" => Some(Self::DotaUmCustomHudElementModify),
            "DOTA_UM_CustomHudElement_Destroy" => {
                Some(Self::DotaUmCustomHudElementDestroy)
            }
            "DOTA_UM_CompendiumState" => Some(Self::DotaUmCompendiumState),
            "DOTA_UM_ProjectionAbility" => Some(Self::DotaUmProjectionAbility),
            "DOTA_UM_ProjectionEvent" => Some(Self::DotaUmProjectionEvent),
            "DOTA_UM_CombatLogDataHLTV" => Some(Self::DotaUmCombatLogDataHltv),
            "DOTA_UM_XPAlert" => Some(Self::DotaUmXpAlert),
            "DOTA_UM_UpdateQuestProgress" => Some(Self::DotaUmUpdateQuestProgress),
            "DOTA_UM_MatchMetadata" => Some(Self::DotaUmMatchMetadata),
            "DOTA_UM_MatchDetails" => Some(Self::DotaUmMatchDetails),
            "DOTA_UM_QuestStatus" => Some(Self::DotaUmQuestStatus),
            "DOTA_UM_SuggestHeroPick" => Some(Self::DotaUmSuggestHeroPick),
            "DOTA_UM_SuggestHeroRole" => Some(Self::DotaUmSuggestHeroRole),
            "DOTA_UM_KillcamDamageTaken" => Some(Self::DotaUmKillcamDamageTaken),
            "DOTA_UM_SelectPenaltyGold" => Some(Self::DotaUmSelectPenaltyGold),
            "DOTA_UM_RollDiceResult" => Some(Self::DotaUmRollDiceResult),
            "DOTA_UM_FlipCoinResult" => Some(Self::DotaUmFlipCoinResult),
            "DOTA_UM_RequestItemSuggestions" => Some(Self::DotaUmRequestItemSuggestions),
            "DOTA_UM_TeamCaptainChanged" => Some(Self::DotaUmTeamCaptainChanged),
            "DOTA_UM_SendRoshanSpectatorPhase" => {
                Some(Self::DotaUmSendRoshanSpectatorPhase)
            }
            "DOTA_UM_ChatWheelCooldown" => Some(Self::DotaUmChatWheelCooldown),
            "DOTA_UM_DismissAllStatPopups" => Some(Self::DotaUmDismissAllStatPopups),
            "DOTA_UM_TE_DestroyProjectile" => Some(Self::DotaUmTeDestroyProjectile),
            "DOTA_UM_HeroRelicProgress" => Some(Self::DotaUmHeroRelicProgress),
            "DOTA_UM_AbilityDraftRequestAbility" => {
                Some(Self::DotaUmAbilityDraftRequestAbility)
            }
            "DOTA_UM_ItemSold" => Some(Self::DotaUmItemSold),
            "DOTA_UM_DamageReport" => Some(Self::DotaUmDamageReport),
            "DOTA_UM_SalutePlayer" => Some(Self::DotaUmSalutePlayer),
            "DOTA_UM_TipAlert" => Some(Self::DotaUmTipAlert),
            "DOTA_UM_ReplaceQueryUnit" => Some(Self::DotaUmReplaceQueryUnit),
            "DOTA_UM_EmptyTeleportAlert" => Some(Self::DotaUmEmptyTeleportAlert),
            "DOTA_UM_MarsArenaOfBloodAttack" => Some(Self::DotaUmMarsArenaOfBloodAttack),
            "DOTA_UM_ESArcanaCombo" => Some(Self::DotaUmEsArcanaCombo),
            "DOTA_UM_ESArcanaComboSummary" => Some(Self::DotaUmEsArcanaComboSummary),
            "DOTA_UM_HighFiveLeftHanging" => Some(Self::DotaUmHighFiveLeftHanging),
            "DOTA_UM_HighFiveCompleted" => Some(Self::DotaUmHighFiveCompleted),
            "DOTA_UM_ShovelUnearth" => Some(Self::DotaUmShovelUnearth),
            "DOTA_EM_InvokerSpellCast" => Some(Self::DotaEmInvokerSpellCast),
            "DOTA_UM_RadarAlert" => Some(Self::DotaUmRadarAlert),
            "DOTA_UM_AllStarEvent" => Some(Self::DotaUmAllStarEvent),
            "DOTA_UM_TalentTreeAlert" => Some(Self::DotaUmTalentTreeAlert),
            "DOTA_UM_QueuedOrderRemoved" => Some(Self::DotaUmQueuedOrderRemoved),
            "DOTA_UM_DebugChallenge" => Some(Self::DotaUmDebugChallenge),
            "DOTA_UM_OMArcanaCombo" => Some(Self::DotaUmOmArcanaCombo),
            "DOTA_UM_FoundNeutralItem" => Some(Self::DotaUmFoundNeutralItem),
            "DOTA_UM_OutpostCaptured" => Some(Self::DotaUmOutpostCaptured),
            "DOTA_UM_OutpostGrantedXP" => Some(Self::DotaUmOutpostGrantedXp),
            "DOTA_UM_MoveCameraToUnit" => Some(Self::DotaUmMoveCameraToUnit),
            "DOTA_UM_PauseMinigameData" => Some(Self::DotaUmPauseMinigameData),
            "DOTA_UM_VersusScene_PlayerBehavior" => {
                Some(Self::DotaUmVersusScenePlayerBehavior)
            }
            "DOTA_UM_QoP_ArcanaSummary" => Some(Self::DotaUmQoPArcanaSummary),
            "DOTA_UM_HotPotato_Created" => Some(Self::DotaUmHotPotatoCreated),
            "DOTA_UM_HotPotato_Exploded" => Some(Self::DotaUmHotPotatoExploded),
            "DOTA_UM_WK_Arcana_Progress" => Some(Self::DotaUmWkArcanaProgress),
            "DOTA_UM_GuildChallenge_Progress" => Some(Self::DotaUmGuildChallengeProgress),
            "DOTA_UM_WRArcanaProgress" => Some(Self::DotaUmWrArcanaProgress),
            "DOTA_UM_WRArcanaSummary" => Some(Self::DotaUmWrArcanaSummary),
            "DOTA_UM_EmptyItemSlotAlert" => Some(Self::DotaUmEmptyItemSlotAlert),
            "DOTA_UM_AghsStatusAlert" => Some(Self::DotaUmAghsStatusAlert),
            "DOTA_UM_PingConfirmation" => Some(Self::DotaUmPingConfirmation),
            "DOTA_UM_MutedPlayers" => Some(Self::DotaUmMutedPlayers),
            "DOTA_UM_ContextualTip" => Some(Self::DotaUmContextualTip),
            "DOTA_UM_ChatMessage" => Some(Self::DotaUmChatMessage),
            "DOTA_UM_NeutralCampAlert" => Some(Self::DotaUmNeutralCampAlert),
            "DOTA_UM_RockPaperScissorsStarted" => {
                Some(Self::DotaUmRockPaperScissorsStarted)
            }
            "DOTA_UM_RockPaperScissorsFinished" => {
                Some(Self::DotaUmRockPaperScissorsFinished)
            }
            "DOTA_UM_DuelOpponentKilled" => Some(Self::DotaUmDuelOpponentKilled),
            "DOTA_UM_DuelAccepted" => Some(Self::DotaUmDuelAccepted),
            "DOTA_UM_DuelRequested" => Some(Self::DotaUmDuelRequested),
            "DOTA_UM_MuertaReleaseEvent_AssignedTargetKilled" => {
                Some(Self::DotaUmMuertaReleaseEventAssignedTargetKilled)
            }
            "DOTA_UM_PlayerDraftSuggestPick" => Some(Self::DotaUmPlayerDraftSuggestPick),
            "DOTA_UM_PlayerDraftPick" => Some(Self::DotaUmPlayerDraftPick),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaChatMessage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaChatMessage::ChatMessageInvalid => "CHAT_MESSAGE_INVALID",
            DotaChatMessage::ChatMessageHeroKill => "CHAT_MESSAGE_HERO_KILL",
            DotaChatMessage::ChatMessageHeroDeny => "CHAT_MESSAGE_HERO_DENY",
            DotaChatMessage::ChatMessageBarracksKill => "CHAT_MESSAGE_BARRACKS_KILL",
            DotaChatMessage::ChatMessageTowerKill => "CHAT_MESSAGE_TOWER_KILL",
            DotaChatMessage::ChatMessageTowerDeny => "CHAT_MESSAGE_TOWER_DENY",
            DotaChatMessage::ChatMessageFirstblood => "CHAT_MESSAGE_FIRSTBLOOD",
            DotaChatMessage::ChatMessageStreakKill => "CHAT_MESSAGE_STREAK_KILL",
            DotaChatMessage::ChatMessageBuyback => "CHAT_MESSAGE_BUYBACK",
            DotaChatMessage::ChatMessageAegis => "CHAT_MESSAGE_AEGIS",
            DotaChatMessage::ChatMessageRoshanKill => "CHAT_MESSAGE_ROSHAN_KILL",
            DotaChatMessage::ChatMessageCourierLost => "CHAT_MESSAGE_COURIER_LOST",
            DotaChatMessage::ChatMessageCourierRespawned => {
                "CHAT_MESSAGE_COURIER_RESPAWNED"
            }
            DotaChatMessage::ChatMessageGlyphUsed => "CHAT_MESSAGE_GLYPH_USED",
            DotaChatMessage::ChatMessageItemPurchase => "CHAT_MESSAGE_ITEM_PURCHASE",
            DotaChatMessage::ChatMessageConnect => "CHAT_MESSAGE_CONNECT",
            DotaChatMessage::ChatMessageDisconnect => "CHAT_MESSAGE_DISCONNECT",
            DotaChatMessage::ChatMessageDisconnectWaitForReconnect => {
                "CHAT_MESSAGE_DISCONNECT_WAIT_FOR_RECONNECT"
            }
            DotaChatMessage::ChatMessageDisconnectTimeRemaining => {
                "CHAT_MESSAGE_DISCONNECT_TIME_REMAINING"
            }
            DotaChatMessage::ChatMessageDisconnectTimeRemainingPlural => {
                "CHAT_MESSAGE_DISCONNECT_TIME_REMAINING_PLURAL"
            }
            DotaChatMessage::ChatMessageReconnect => "CHAT_MESSAGE_RECONNECT",
            DotaChatMessage::ChatMessagePlayerLeft => "CHAT_MESSAGE_PLAYER_LEFT",
            DotaChatMessage::ChatMessageSafeToLeave => "CHAT_MESSAGE_SAFE_TO_LEAVE",
            DotaChatMessage::ChatMessageRunePickup => "CHAT_MESSAGE_RUNE_PICKUP",
            DotaChatMessage::ChatMessageRuneBottle => "CHAT_MESSAGE_RUNE_BOTTLE",
            DotaChatMessage::ChatMessageRuneDeny => "CHAT_MESSAGE_RUNE_DENY",
            DotaChatMessage::ChatMessageInthebag => "CHAT_MESSAGE_INTHEBAG",
            DotaChatMessage::ChatMessageSecretshop => "CHAT_MESSAGE_SECRETSHOP",
            DotaChatMessage::ChatMessageItemAutopurchased => {
                "CHAT_MESSAGE_ITEM_AUTOPURCHASED"
            }
            DotaChatMessage::ChatMessageItemsCombined => "CHAT_MESSAGE_ITEMS_COMBINED",
            DotaChatMessage::ChatMessageSuperCreeps => "CHAT_MESSAGE_SUPER_CREEPS",
            DotaChatMessage::ChatMessageCantUseActionItem => {
                "CHAT_MESSAGE_CANT_USE_ACTION_ITEM"
            }
            DotaChatMessage::ChatMessageCantpause => "CHAT_MESSAGE_CANTPAUSE",
            DotaChatMessage::ChatMessageNopausesleft => "CHAT_MESSAGE_NOPAUSESLEFT",
            DotaChatMessage::ChatMessageCantpauseyet => "CHAT_MESSAGE_CANTPAUSEYET",
            DotaChatMessage::ChatMessagePaused => "CHAT_MESSAGE_PAUSED",
            DotaChatMessage::ChatMessageUnpauseCountdown => {
                "CHAT_MESSAGE_UNPAUSE_COUNTDOWN"
            }
            DotaChatMessage::ChatMessageUnpaused => "CHAT_MESSAGE_UNPAUSED",
            DotaChatMessage::ChatMessageAutoUnpaused => "CHAT_MESSAGE_AUTO_UNPAUSED",
            DotaChatMessage::ChatMessageYoupaused => "CHAT_MESSAGE_YOUPAUSED",
            DotaChatMessage::ChatMessageCantunpauseteam => "CHAT_MESSAGE_CANTUNPAUSETEAM",
            DotaChatMessage::ChatMessageVoiceTextBanned => {
                "CHAT_MESSAGE_VOICE_TEXT_BANNED"
            }
            DotaChatMessage::ChatMessageSpectatorsWatchingThisGame => {
                "CHAT_MESSAGE_SPECTATORS_WATCHING_THIS_GAME"
            }
            DotaChatMessage::ChatMessageReportReminder => "CHAT_MESSAGE_REPORT_REMINDER",
            DotaChatMessage::ChatMessageEconItem => "CHAT_MESSAGE_ECON_ITEM",
            DotaChatMessage::ChatMessageTaunt => "CHAT_MESSAGE_TAUNT",
            DotaChatMessage::ChatMessageRandom => "CHAT_MESSAGE_RANDOM",
            DotaChatMessage::ChatMessageRdTurn => "CHAT_MESSAGE_RD_TURN",
            DotaChatMessage::ChatMessageDropRateBonus => "CHAT_MESSAGE_DROP_RATE_BONUS",
            DotaChatMessage::ChatMessageNoBattlePoints => "CHAT_MESSAGE_NO_BATTLE_POINTS",
            DotaChatMessage::ChatMessageDeniedAegis => "CHAT_MESSAGE_DENIED_AEGIS",
            DotaChatMessage::ChatMessageInformational => "CHAT_MESSAGE_INFORMATIONAL",
            DotaChatMessage::ChatMessageAegisStolen => "CHAT_MESSAGE_AEGIS_STOLEN",
            DotaChatMessage::ChatMessageRoshanCandy => "CHAT_MESSAGE_ROSHAN_CANDY",
            DotaChatMessage::ChatMessageItemGifted => "CHAT_MESSAGE_ITEM_GIFTED",
            DotaChatMessage::ChatMessageHeroKillWithGreevil => {
                "CHAT_MESSAGE_HERO_KILL_WITH_GREEVIL"
            }
            DotaChatMessage::ChatMessageHoldoutTowerDestroyed => {
                "CHAT_MESSAGE_HOLDOUT_TOWER_DESTROYED"
            }
            DotaChatMessage::ChatMessageHoldoutWallDestroyed => {
                "CHAT_MESSAGE_HOLDOUT_WALL_DESTROYED"
            }
            DotaChatMessage::ChatMessageHoldoutWallFinished => {
                "CHAT_MESSAGE_HOLDOUT_WALL_FINISHED"
            }
            DotaChatMessage::ChatMessagePlayerLeftLimitedHero => {
                "CHAT_MESSAGE_PLAYER_LEFT_LIMITED_HERO"
            }
            DotaChatMessage::ChatMessageAbandonLimitedHeroExplanation => {
                "CHAT_MESSAGE_ABANDON_LIMITED_HERO_EXPLANATION"
            }
            DotaChatMessage::ChatMessageDisconnectLimitedHero => {
                "CHAT_MESSAGE_DISCONNECT_LIMITED_HERO"
            }
            DotaChatMessage::ChatMessageLowPriorityCompletedExplanation => {
                "CHAT_MESSAGE_LOW_PRIORITY_COMPLETED_EXPLANATION"
            }
            DotaChatMessage::ChatMessageRecruitmentDropRateBonus => {
                "CHAT_MESSAGE_RECRUITMENT_DROP_RATE_BONUS"
            }
            DotaChatMessage::ChatMessageFrostivusShiningBoosterActive => {
                "CHAT_MESSAGE_FROSTIVUS_SHINING_BOOSTER_ACTIVE"
            }
            DotaChatMessage::ChatMessagePlayerLeftAfk => "CHAT_MESSAGE_PLAYER_LEFT_AFK",
            DotaChatMessage::ChatMessagePlayerLeftDisconnectedTooLong => {
                "CHAT_MESSAGE_PLAYER_LEFT_DISCONNECTED_TOO_LONG"
            }
            DotaChatMessage::ChatMessagePlayerAbandoned => {
                "CHAT_MESSAGE_PLAYER_ABANDONED"
            }
            DotaChatMessage::ChatMessagePlayerAbandonedAfk => {
                "CHAT_MESSAGE_PLAYER_ABANDONED_AFK"
            }
            DotaChatMessage::ChatMessagePlayerAbandonedDisconnectedTooLong => {
                "CHAT_MESSAGE_PLAYER_ABANDONED_DISCONNECTED_TOO_LONG"
            }
            DotaChatMessage::ChatMessageWillNotBeScored => {
                "CHAT_MESSAGE_WILL_NOT_BE_SCORED"
            }
            DotaChatMessage::ChatMessageWillNotBeScoredRanked => {
                "CHAT_MESSAGE_WILL_NOT_BE_SCORED_RANKED"
            }
            DotaChatMessage::ChatMessageWillNotBeScoredNetwork => {
                "CHAT_MESSAGE_WILL_NOT_BE_SCORED_NETWORK"
            }
            DotaChatMessage::ChatMessageWillNotBeScoredNetworkRanked => {
                "CHAT_MESSAGE_WILL_NOT_BE_SCORED_NETWORK_RANKED"
            }
            DotaChatMessage::ChatMessageCanQuitWithoutAbandon => {
                "CHAT_MESSAGE_CAN_QUIT_WITHOUT_ABANDON"
            }
            DotaChatMessage::ChatMessageRankedGameStillScoredLeaversGetLoss => {
                "CHAT_MESSAGE_RANKED_GAME_STILL_SCORED_LEAVERS_GET_LOSS"
            }
            DotaChatMessage::ChatMessageAbandonRankedBeforeFirstBloodParty => {
                "CHAT_MESSAGE_ABANDON_RANKED_BEFORE_FIRST_BLOOD_PARTY"
            }
            DotaChatMessage::ChatMessageCompendiumLevel => {
                "CHAT_MESSAGE_COMPENDIUM_LEVEL"
            }
            DotaChatMessage::ChatMessageVictoryPredictionStreak => {
                "CHAT_MESSAGE_VICTORY_PREDICTION_STREAK"
            }
            DotaChatMessage::ChatMessageAssassinAnnounce => {
                "CHAT_MESSAGE_ASSASSIN_ANNOUNCE"
            }
            DotaChatMessage::ChatMessageAssassinSuccess => {
                "CHAT_MESSAGE_ASSASSIN_SUCCESS"
            }
            DotaChatMessage::ChatMessageAssassinDenied => "CHAT_MESSAGE_ASSASSIN_DENIED",
            DotaChatMessage::ChatMessageVictoryPredictionSingleUserConfirm => {
                "CHAT_MESSAGE_VICTORY_PREDICTION_SINGLE_USER_CONFIRM"
            }
            DotaChatMessage::ChatMessageEffigyKill => "CHAT_MESSAGE_EFFIGY_KILL",
            DotaChatMessage::ChatMessageVoiceTextBannedOverflow => {
                "CHAT_MESSAGE_VOICE_TEXT_BANNED_OVERFLOW"
            }
            DotaChatMessage::ChatMessageYearBeastKilled => {
                "CHAT_MESSAGE_YEAR_BEAST_KILLED"
            }
            DotaChatMessage::ChatMessagePauseCountdown => "CHAT_MESSAGE_PAUSE_COUNTDOWN",
            DotaChatMessage::ChatMessageCoinsWagered => "CHAT_MESSAGE_COINS_WAGERED",
            DotaChatMessage::ChatMessageHeroNominatedBan => {
                "CHAT_MESSAGE_HERO_NOMINATED_BAN"
            }
            DotaChatMessage::ChatMessageHeroBanned => "CHAT_MESSAGE_HERO_BANNED",
            DotaChatMessage::ChatMessageHeroBanCount => "CHAT_MESSAGE_HERO_BAN_COUNT",
            DotaChatMessage::ChatMessageRiverPainted => "CHAT_MESSAGE_RIVER_PAINTED",
            DotaChatMessage::ChatMessageScanUsed => "CHAT_MESSAGE_SCAN_USED",
            DotaChatMessage::ChatMessageShrineKilled => "CHAT_MESSAGE_SHRINE_KILLED",
            DotaChatMessage::ChatMessageWagerTokenSpent => {
                "CHAT_MESSAGE_WAGER_TOKEN_SPENT"
            }
            DotaChatMessage::ChatMessageRankWager => "CHAT_MESSAGE_RANK_WAGER",
            DotaChatMessage::ChatMessageNewPlayerReminder => {
                "CHAT_MESSAGE_NEW_PLAYER_REMINDER"
            }
            DotaChatMessage::ChatMessageObserverWardKilled => {
                "CHAT_MESSAGE_OBSERVER_WARD_KILLED"
            }
            DotaChatMessage::ChatMessageSentryWardKilled => {
                "CHAT_MESSAGE_SENTRY_WARD_KILLED"
            }
            DotaChatMessage::ChatMessageItemPlacedInNeutralStash => {
                "CHAT_MESSAGE_ITEM_PLACED_IN_NEUTRAL_STASH"
            }
            DotaChatMessage::ChatMessageHeroChoiceInvalid => {
                "CHAT_MESSAGE_HERO_CHOICE_INVALID"
            }
            DotaChatMessage::ChatMessageBounty => "CHAT_MESSAGE_BOUNTY",
            DotaChatMessage::ChatMessageAbilityDraftStart => {
                "CHAT_MESSAGE_ABILITY_DRAFT_START"
            }
            DotaChatMessage::ChatMessageHeroFoundCandy => "CHAT_MESSAGE_HERO_FOUND_CANDY",
            DotaChatMessage::ChatMessageAbilityDraftRandomed => {
                "CHAT_MESSAGE_ABILITY_DRAFT_RANDOMED"
            }
            DotaChatMessage::ChatMessagePrivateCoachConnected => {
                "CHAT_MESSAGE_PRIVATE_COACH_CONNECTED"
            }
            DotaChatMessage::ChatMessageCantPauseTooEarly => {
                "CHAT_MESSAGE_CANT_PAUSE_TOO_EARLY"
            }
            DotaChatMessage::ChatMessageHeroKillWithPenguin => {
                "CHAT_MESSAGE_HERO_KILL_WITH_PENGUIN"
            }
            DotaChatMessage::ChatMessageMinibossKill => "CHAT_MESSAGE_MINIBOSS_KILL",
            DotaChatMessage::ChatMessagePlayerInGameBanText => {
                "CHAT_MESSAGE_PLAYER_IN_GAME_BAN_TEXT"
            }
            DotaChatMessage::ChatMessageBannerPlanted => "CHAT_MESSAGE_BANNER_PLANTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CHAT_MESSAGE_INVALID" => Some(Self::ChatMessageInvalid),
            "CHAT_MESSAGE_HERO_KILL" => Some(Self::ChatMessageHeroKill),
            "CHAT_MESSAGE_HERO_DENY" => Some(Self::ChatMessageHeroDeny),
            "CHAT_MESSAGE_BARRACKS_KILL" => Some(Self::ChatMessageBarracksKill),
            "CHAT_MESSAGE_TOWER_KILL" => Some(Self::ChatMessageTowerKill),
            "CHAT_MESSAGE_TOWER_DENY" => Some(Self::ChatMessageTowerDeny),
            "CHAT_MESSAGE_FIRSTBLOOD" => Some(Self::ChatMessageFirstblood),
            "CHAT_MESSAGE_STREAK_KILL" => Some(Self::ChatMessageStreakKill),
            "CHAT_MESSAGE_BUYBACK" => Some(Self::ChatMessageBuyback),
            "CHAT_MESSAGE_AEGIS" => Some(Self::ChatMessageAegis),
            "CHAT_MESSAGE_ROSHAN_KILL" => Some(Self::ChatMessageRoshanKill),
            "CHAT_MESSAGE_COURIER_LOST" => Some(Self::ChatMessageCourierLost),
            "CHAT_MESSAGE_COURIER_RESPAWNED" => Some(Self::ChatMessageCourierRespawned),
            "CHAT_MESSAGE_GLYPH_USED" => Some(Self::ChatMessageGlyphUsed),
            "CHAT_MESSAGE_ITEM_PURCHASE" => Some(Self::ChatMessageItemPurchase),
            "CHAT_MESSAGE_CONNECT" => Some(Self::ChatMessageConnect),
            "CHAT_MESSAGE_DISCONNECT" => Some(Self::ChatMessageDisconnect),
            "CHAT_MESSAGE_DISCONNECT_WAIT_FOR_RECONNECT" => {
                Some(Self::ChatMessageDisconnectWaitForReconnect)
            }
            "CHAT_MESSAGE_DISCONNECT_TIME_REMAINING" => {
                Some(Self::ChatMessageDisconnectTimeRemaining)
            }
            "CHAT_MESSAGE_DISCONNECT_TIME_REMAINING_PLURAL" => {
                Some(Self::ChatMessageDisconnectTimeRemainingPlural)
            }
            "CHAT_MESSAGE_RECONNECT" => Some(Self::ChatMessageReconnect),
            "CHAT_MESSAGE_PLAYER_LEFT" => Some(Self::ChatMessagePlayerLeft),
            "CHAT_MESSAGE_SAFE_TO_LEAVE" => Some(Self::ChatMessageSafeToLeave),
            "CHAT_MESSAGE_RUNE_PICKUP" => Some(Self::ChatMessageRunePickup),
            "CHAT_MESSAGE_RUNE_BOTTLE" => Some(Self::ChatMessageRuneBottle),
            "CHAT_MESSAGE_RUNE_DENY" => Some(Self::ChatMessageRuneDeny),
            "CHAT_MESSAGE_INTHEBAG" => Some(Self::ChatMessageInthebag),
            "CHAT_MESSAGE_SECRETSHOP" => Some(Self::ChatMessageSecretshop),
            "CHAT_MESSAGE_ITEM_AUTOPURCHASED" => Some(Self::ChatMessageItemAutopurchased),
            "CHAT_MESSAGE_ITEMS_COMBINED" => Some(Self::ChatMessageItemsCombined),
            "CHAT_MESSAGE_SUPER_CREEPS" => Some(Self::ChatMessageSuperCreeps),
            "CHAT_MESSAGE_CANT_USE_ACTION_ITEM" => {
                Some(Self::ChatMessageCantUseActionItem)
            }
            "CHAT_MESSAGE_CANTPAUSE" => Some(Self::ChatMessageCantpause),
            "CHAT_MESSAGE_NOPAUSESLEFT" => Some(Self::ChatMessageNopausesleft),
            "CHAT_MESSAGE_CANTPAUSEYET" => Some(Self::ChatMessageCantpauseyet),
            "CHAT_MESSAGE_PAUSED" => Some(Self::ChatMessagePaused),
            "CHAT_MESSAGE_UNPAUSE_COUNTDOWN" => Some(Self::ChatMessageUnpauseCountdown),
            "CHAT_MESSAGE_UNPAUSED" => Some(Self::ChatMessageUnpaused),
            "CHAT_MESSAGE_AUTO_UNPAUSED" => Some(Self::ChatMessageAutoUnpaused),
            "CHAT_MESSAGE_YOUPAUSED" => Some(Self::ChatMessageYoupaused),
            "CHAT_MESSAGE_CANTUNPAUSETEAM" => Some(Self::ChatMessageCantunpauseteam),
            "CHAT_MESSAGE_VOICE_TEXT_BANNED" => Some(Self::ChatMessageVoiceTextBanned),
            "CHAT_MESSAGE_SPECTATORS_WATCHING_THIS_GAME" => {
                Some(Self::ChatMessageSpectatorsWatchingThisGame)
            }
            "CHAT_MESSAGE_REPORT_REMINDER" => Some(Self::ChatMessageReportReminder),
            "CHAT_MESSAGE_ECON_ITEM" => Some(Self::ChatMessageEconItem),
            "CHAT_MESSAGE_TAUNT" => Some(Self::ChatMessageTaunt),
            "CHAT_MESSAGE_RANDOM" => Some(Self::ChatMessageRandom),
            "CHAT_MESSAGE_RD_TURN" => Some(Self::ChatMessageRdTurn),
            "CHAT_MESSAGE_DROP_RATE_BONUS" => Some(Self::ChatMessageDropRateBonus),
            "CHAT_MESSAGE_NO_BATTLE_POINTS" => Some(Self::ChatMessageNoBattlePoints),
            "CHAT_MESSAGE_DENIED_AEGIS" => Some(Self::ChatMessageDeniedAegis),
            "CHAT_MESSAGE_INFORMATIONAL" => Some(Self::ChatMessageInformational),
            "CHAT_MESSAGE_AEGIS_STOLEN" => Some(Self::ChatMessageAegisStolen),
            "CHAT_MESSAGE_ROSHAN_CANDY" => Some(Self::ChatMessageRoshanCandy),
            "CHAT_MESSAGE_ITEM_GIFTED" => Some(Self::ChatMessageItemGifted),
            "CHAT_MESSAGE_HERO_KILL_WITH_GREEVIL" => {
                Some(Self::ChatMessageHeroKillWithGreevil)
            }
            "CHAT_MESSAGE_HOLDOUT_TOWER_DESTROYED" => {
                Some(Self::ChatMessageHoldoutTowerDestroyed)
            }
            "CHAT_MESSAGE_HOLDOUT_WALL_DESTROYED" => {
                Some(Self::ChatMessageHoldoutWallDestroyed)
            }
            "CHAT_MESSAGE_HOLDOUT_WALL_FINISHED" => {
                Some(Self::ChatMessageHoldoutWallFinished)
            }
            "CHAT_MESSAGE_PLAYER_LEFT_LIMITED_HERO" => {
                Some(Self::ChatMessagePlayerLeftLimitedHero)
            }
            "CHAT_MESSAGE_ABANDON_LIMITED_HERO_EXPLANATION" => {
                Some(Self::ChatMessageAbandonLimitedHeroExplanation)
            }
            "CHAT_MESSAGE_DISCONNECT_LIMITED_HERO" => {
                Some(Self::ChatMessageDisconnectLimitedHero)
            }
            "CHAT_MESSAGE_LOW_PRIORITY_COMPLETED_EXPLANATION" => {
                Some(Self::ChatMessageLowPriorityCompletedExplanation)
            }
            "CHAT_MESSAGE_RECRUITMENT_DROP_RATE_BONUS" => {
                Some(Self::ChatMessageRecruitmentDropRateBonus)
            }
            "CHAT_MESSAGE_FROSTIVUS_SHINING_BOOSTER_ACTIVE" => {
                Some(Self::ChatMessageFrostivusShiningBoosterActive)
            }
            "CHAT_MESSAGE_PLAYER_LEFT_AFK" => Some(Self::ChatMessagePlayerLeftAfk),
            "CHAT_MESSAGE_PLAYER_LEFT_DISCONNECTED_TOO_LONG" => {
                Some(Self::ChatMessagePlayerLeftDisconnectedTooLong)
            }
            "CHAT_MESSAGE_PLAYER_ABANDONED" => Some(Self::ChatMessagePlayerAbandoned),
            "CHAT_MESSAGE_PLAYER_ABANDONED_AFK" => {
                Some(Self::ChatMessagePlayerAbandonedAfk)
            }
            "CHAT_MESSAGE_PLAYER_ABANDONED_DISCONNECTED_TOO_LONG" => {
                Some(Self::ChatMessagePlayerAbandonedDisconnectedTooLong)
            }
            "CHAT_MESSAGE_WILL_NOT_BE_SCORED" => Some(Self::ChatMessageWillNotBeScored),
            "CHAT_MESSAGE_WILL_NOT_BE_SCORED_RANKED" => {
                Some(Self::ChatMessageWillNotBeScoredRanked)
            }
            "CHAT_MESSAGE_WILL_NOT_BE_SCORED_NETWORK" => {
                Some(Self::ChatMessageWillNotBeScoredNetwork)
            }
            "CHAT_MESSAGE_WILL_NOT_BE_SCORED_NETWORK_RANKED" => {
                Some(Self::ChatMessageWillNotBeScoredNetworkRanked)
            }
            "CHAT_MESSAGE_CAN_QUIT_WITHOUT_ABANDON" => {
                Some(Self::ChatMessageCanQuitWithoutAbandon)
            }
            "CHAT_MESSAGE_RANKED_GAME_STILL_SCORED_LEAVERS_GET_LOSS" => {
                Some(Self::ChatMessageRankedGameStillScoredLeaversGetLoss)
            }
            "CHAT_MESSAGE_ABANDON_RANKED_BEFORE_FIRST_BLOOD_PARTY" => {
                Some(Self::ChatMessageAbandonRankedBeforeFirstBloodParty)
            }
            "CHAT_MESSAGE_COMPENDIUM_LEVEL" => Some(Self::ChatMessageCompendiumLevel),
            "CHAT_MESSAGE_VICTORY_PREDICTION_STREAK" => {
                Some(Self::ChatMessageVictoryPredictionStreak)
            }
            "CHAT_MESSAGE_ASSASSIN_ANNOUNCE" => Some(Self::ChatMessageAssassinAnnounce),
            "CHAT_MESSAGE_ASSASSIN_SUCCESS" => Some(Self::ChatMessageAssassinSuccess),
            "CHAT_MESSAGE_ASSASSIN_DENIED" => Some(Self::ChatMessageAssassinDenied),
            "CHAT_MESSAGE_VICTORY_PREDICTION_SINGLE_USER_CONFIRM" => {
                Some(Self::ChatMessageVictoryPredictionSingleUserConfirm)
            }
            "CHAT_MESSAGE_EFFIGY_KILL" => Some(Self::ChatMessageEffigyKill),
            "CHAT_MESSAGE_VOICE_TEXT_BANNED_OVERFLOW" => {
                Some(Self::ChatMessageVoiceTextBannedOverflow)
            }
            "CHAT_MESSAGE_YEAR_BEAST_KILLED" => Some(Self::ChatMessageYearBeastKilled),
            "CHAT_MESSAGE_PAUSE_COUNTDOWN" => Some(Self::ChatMessagePauseCountdown),
            "CHAT_MESSAGE_COINS_WAGERED" => Some(Self::ChatMessageCoinsWagered),
            "CHAT_MESSAGE_HERO_NOMINATED_BAN" => Some(Self::ChatMessageHeroNominatedBan),
            "CHAT_MESSAGE_HERO_BANNED" => Some(Self::ChatMessageHeroBanned),
            "CHAT_MESSAGE_HERO_BAN_COUNT" => Some(Self::ChatMessageHeroBanCount),
            "CHAT_MESSAGE_RIVER_PAINTED" => Some(Self::ChatMessageRiverPainted),
            "CHAT_MESSAGE_SCAN_USED" => Some(Self::ChatMessageScanUsed),
            "CHAT_MESSAGE_SHRINE_KILLED" => Some(Self::ChatMessageShrineKilled),
            "CHAT_MESSAGE_WAGER_TOKEN_SPENT" => Some(Self::ChatMessageWagerTokenSpent),
            "CHAT_MESSAGE_RANK_WAGER" => Some(Self::ChatMessageRankWager),
            "CHAT_MESSAGE_NEW_PLAYER_REMINDER" => {
                Some(Self::ChatMessageNewPlayerReminder)
            }
            "CHAT_MESSAGE_OBSERVER_WARD_KILLED" => {
                Some(Self::ChatMessageObserverWardKilled)
            }
            "CHAT_MESSAGE_SENTRY_WARD_KILLED" => Some(Self::ChatMessageSentryWardKilled),
            "CHAT_MESSAGE_ITEM_PLACED_IN_NEUTRAL_STASH" => {
                Some(Self::ChatMessageItemPlacedInNeutralStash)
            }
            "CHAT_MESSAGE_HERO_CHOICE_INVALID" => {
                Some(Self::ChatMessageHeroChoiceInvalid)
            }
            "CHAT_MESSAGE_BOUNTY" => Some(Self::ChatMessageBounty),
            "CHAT_MESSAGE_ABILITY_DRAFT_START" => {
                Some(Self::ChatMessageAbilityDraftStart)
            }
            "CHAT_MESSAGE_HERO_FOUND_CANDY" => Some(Self::ChatMessageHeroFoundCandy),
            "CHAT_MESSAGE_ABILITY_DRAFT_RANDOMED" => {
                Some(Self::ChatMessageAbilityDraftRandomed)
            }
            "CHAT_MESSAGE_PRIVATE_COACH_CONNECTED" => {
                Some(Self::ChatMessagePrivateCoachConnected)
            }
            "CHAT_MESSAGE_CANT_PAUSE_TOO_EARLY" => {
                Some(Self::ChatMessageCantPauseTooEarly)
            }
            "CHAT_MESSAGE_HERO_KILL_WITH_PENGUIN" => {
                Some(Self::ChatMessageHeroKillWithPenguin)
            }
            "CHAT_MESSAGE_MINIBOSS_KILL" => Some(Self::ChatMessageMinibossKill),
            "CHAT_MESSAGE_PLAYER_IN_GAME_BAN_TEXT" => {
                Some(Self::ChatMessagePlayerInGameBanText)
            }
            "CHAT_MESSAGE_BANNER_PLANTED" => Some(Self::ChatMessageBannerPlanted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaNoBattlePointsReasons {
    NoBattlePointsWrongLobbyType = 1,
    NoBattlePointsPracticeBots = 2,
    NoBattlePointsCheatsEnabled = 3,
    NoBattlePointsLowPriority = 4,
}
impl DotaNoBattlePointsReasons {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaNoBattlePointsReasons::NoBattlePointsWrongLobbyType => {
                "NO_BATTLE_POINTS_WRONG_LOBBY_TYPE"
            }
            DotaNoBattlePointsReasons::NoBattlePointsPracticeBots => {
                "NO_BATTLE_POINTS_PRACTICE_BOTS"
            }
            DotaNoBattlePointsReasons::NoBattlePointsCheatsEnabled => {
                "NO_BATTLE_POINTS_CHEATS_ENABLED"
            }
            DotaNoBattlePointsReasons::NoBattlePointsLowPriority => {
                "NO_BATTLE_POINTS_LOW_PRIORITY"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO_BATTLE_POINTS_WRONG_LOBBY_TYPE" => {
                Some(Self::NoBattlePointsWrongLobbyType)
            }
            "NO_BATTLE_POINTS_PRACTICE_BOTS" => Some(Self::NoBattlePointsPracticeBots),
            "NO_BATTLE_POINTS_CHEATS_ENABLED" => Some(Self::NoBattlePointsCheatsEnabled),
            "NO_BATTLE_POINTS_LOW_PRIORITY" => Some(Self::NoBattlePointsLowPriority),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaChatInformational {
    InfoCoopBattlePointsRules = 1,
    InfoFrostivusAbandonReminder = 2,
    InfoRankedReminder = 3,
    InfoCoopLowPriorityPassiveReminder = 4,
    InfoCustomGamePenaltyReminder = 5,
}
impl DotaChatInformational {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaChatInformational::InfoCoopBattlePointsRules => {
                "INFO_COOP_BATTLE_POINTS_RULES"
            }
            DotaChatInformational::InfoFrostivusAbandonReminder => {
                "INFO_FROSTIVUS_ABANDON_REMINDER"
            }
            DotaChatInformational::InfoRankedReminder => "INFO_RANKED_REMINDER",
            DotaChatInformational::InfoCoopLowPriorityPassiveReminder => {
                "INFO_COOP_LOW_PRIORITY_PASSIVE_REMINDER"
            }
            DotaChatInformational::InfoCustomGamePenaltyReminder => {
                "INFO_CUSTOM_GAME_PENALTY_REMINDER"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INFO_COOP_BATTLE_POINTS_RULES" => Some(Self::InfoCoopBattlePointsRules),
            "INFO_FROSTIVUS_ABANDON_REMINDER" => Some(Self::InfoFrostivusAbandonReminder),
            "INFO_RANKED_REMINDER" => Some(Self::InfoRankedReminder),
            "INFO_COOP_LOW_PRIORITY_PASSIVE_REMINDER" => {
                Some(Self::InfoCoopLowPriorityPassiveReminder)
            }
            "INFO_CUSTOM_GAME_PENALTY_REMINDER" => {
                Some(Self::InfoCustomGamePenaltyReminder)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaAbilityPingType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaAbilityPingType::AbilityPingReady => "ABILITY_PING_READY",
            DotaAbilityPingType::AbilityPingMana => "ABILITY_PING_MANA",
            DotaAbilityPingType::AbilityPingCooldown => "ABILITY_PING_COOLDOWN",
            DotaAbilityPingType::AbilityPingEnemy => "ABILITY_PING_ENEMY",
            DotaAbilityPingType::AbilityPingUnlearned => "ABILITY_PING_UNLEARNED",
            DotaAbilityPingType::AbilityPingInbackpack => "ABILITY_PING_INBACKPACK",
            DotaAbilityPingType::AbilityPingInstash => "ABILITY_PING_INSTASH",
            DotaAbilityPingType::AbilityPingOncourier => "ABILITY_PING_ONCOURIER",
            DotaAbilityPingType::AbilityPingAlly => "ABILITY_PING_ALLY",
            DotaAbilityPingType::AbilityPingLearnReady => "ABILITY_PING_LEARN_READY",
            DotaAbilityPingType::AbilityPingWillLearn => "ABILITY_PING_WILL_LEARN",
            DotaAbilityPingType::AbilityPingFutureLearn => "ABILITY_PING_FUTURE_LEARN",
            DotaAbilityPingType::AbilityPingNeutralOffer => "ABILITY_PING_NEUTRAL_OFFER",
            DotaAbilityPingType::AbilityPingNeutralRequest => {
                "ABILITY_PING_NEUTRAL_REQUEST"
            }
            DotaAbilityPingType::AbilityPingNeutralEquip => "ABILITY_PING_NEUTRAL_EQUIP",
            DotaAbilityPingType::AbilityPingIncourierbackpack => {
                "ABILITY_PING_INCOURIERBACKPACK"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ABILITY_PING_READY" => Some(Self::AbilityPingReady),
            "ABILITY_PING_MANA" => Some(Self::AbilityPingMana),
            "ABILITY_PING_COOLDOWN" => Some(Self::AbilityPingCooldown),
            "ABILITY_PING_ENEMY" => Some(Self::AbilityPingEnemy),
            "ABILITY_PING_UNLEARNED" => Some(Self::AbilityPingUnlearned),
            "ABILITY_PING_INBACKPACK" => Some(Self::AbilityPingInbackpack),
            "ABILITY_PING_INSTASH" => Some(Self::AbilityPingInstash),
            "ABILITY_PING_ONCOURIER" => Some(Self::AbilityPingOncourier),
            "ABILITY_PING_ALLY" => Some(Self::AbilityPingAlly),
            "ABILITY_PING_LEARN_READY" => Some(Self::AbilityPingLearnReady),
            "ABILITY_PING_WILL_LEARN" => Some(Self::AbilityPingWillLearn),
            "ABILITY_PING_FUTURE_LEARN" => Some(Self::AbilityPingFutureLearn),
            "ABILITY_PING_NEUTRAL_OFFER" => Some(Self::AbilityPingNeutralOffer),
            "ABILITY_PING_NEUTRAL_REQUEST" => Some(Self::AbilityPingNeutralRequest),
            "ABILITY_PING_NEUTRAL_EQUIP" => Some(Self::AbilityPingNeutralEquip),
            "ABILITY_PING_INCOURIERBACKPACK" => Some(Self::AbilityPingIncourierbackpack),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaReplayStateEvent {
    GameStart = 1,
    StartingHorn = 2,
    FirstBlood = 3,
    Showcase = 4,
    PostGame = 5,
    WaitForMap = 6,
}
impl DotaReplayStateEvent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaReplayStateEvent::GameStart => "DOTA_REPLAY_STATE_EVENT_GAME_START",
            DotaReplayStateEvent::StartingHorn => "DOTA_REPLAY_STATE_EVENT_STARTING_HORN",
            DotaReplayStateEvent::FirstBlood => "DOTA_REPLAY_STATE_EVENT_FIRST_BLOOD",
            DotaReplayStateEvent::Showcase => "DOTA_REPLAY_STATE_EVENT_SHOWCASE",
            DotaReplayStateEvent::PostGame => "DOTA_REPLAY_STATE_EVENT_POST_GAME",
            DotaReplayStateEvent::WaitForMap => "DOTA_REPLAY_STATE_EVENT_WAIT_FOR_MAP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_REPLAY_STATE_EVENT_GAME_START" => Some(Self::GameStart),
            "DOTA_REPLAY_STATE_EVENT_STARTING_HORN" => Some(Self::StartingHorn),
            "DOTA_REPLAY_STATE_EVENT_FIRST_BLOOD" => Some(Self::FirstBlood),
            "DOTA_REPLAY_STATE_EVENT_SHOWCASE" => Some(Self::Showcase),
            "DOTA_REPLAY_STATE_EVENT_POST_GAME" => Some(Self::PostGame),
            "DOTA_REPLAY_STATE_EVENT_WAIT_FOR_MAP" => Some(Self::WaitForMap),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl EDotaEntityMessages {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EDotaEntityMessages::DotaUnitSpeech => "DOTA_UNIT_SPEECH",
            EDotaEntityMessages::DotaUnitSpeechMute => "DOTA_UNIT_SPEECH_MUTE",
            EDotaEntityMessages::DotaUnitAddGesture => "DOTA_UNIT_ADD_GESTURE",
            EDotaEntityMessages::DotaUnitRemoveGesture => "DOTA_UNIT_REMOVE_GESTURE",
            EDotaEntityMessages::DotaUnitRemoveAllGestures => {
                "DOTA_UNIT_REMOVE_ALL_GESTURES"
            }
            EDotaEntityMessages::DotaUnitFadeGesture => "DOTA_UNIT_FADE_GESTURE",
            EDotaEntityMessages::DotaUnitSpeechClientsideRules => {
                "DOTA_UNIT_SPEECH_CLIENTSIDE_RULES"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_UNIT_SPEECH" => Some(Self::DotaUnitSpeech),
            "DOTA_UNIT_SPEECH_MUTE" => Some(Self::DotaUnitSpeechMute),
            "DOTA_UNIT_ADD_GESTURE" => Some(Self::DotaUnitAddGesture),
            "DOTA_UNIT_REMOVE_GESTURE" => Some(Self::DotaUnitRemoveGesture),
            "DOTA_UNIT_REMOVE_ALL_GESTURES" => Some(Self::DotaUnitRemoveAllGestures),
            "DOTA_UNIT_FADE_GESTURE" => Some(Self::DotaUnitFadeGesture),
            "DOTA_UNIT_SPEECH_CLIENTSIDE_RULES" => {
                Some(Self::DotaUnitSpeechClientsideRules)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
impl DotaOverheadAlert {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaOverheadAlert::OverheadAlertGold => "OVERHEAD_ALERT_GOLD",
            DotaOverheadAlert::OverheadAlertDeny => "OVERHEAD_ALERT_DENY",
            DotaOverheadAlert::OverheadAlertCritical => "OVERHEAD_ALERT_CRITICAL",
            DotaOverheadAlert::OverheadAlertXp => "OVERHEAD_ALERT_XP",
            DotaOverheadAlert::OverheadAlertBonusSpellDamage => {
                "OVERHEAD_ALERT_BONUS_SPELL_DAMAGE"
            }
            DotaOverheadAlert::OverheadAlertMiss => "OVERHEAD_ALERT_MISS",
            DotaOverheadAlert::OverheadAlertDamage => "OVERHEAD_ALERT_DAMAGE",
            DotaOverheadAlert::OverheadAlertEvade => "OVERHEAD_ALERT_EVADE",
            DotaOverheadAlert::OverheadAlertBlock => "OVERHEAD_ALERT_BLOCK",
            DotaOverheadAlert::OverheadAlertBonusPoisonDamage => {
                "OVERHEAD_ALERT_BONUS_POISON_DAMAGE"
            }
            DotaOverheadAlert::OverheadAlertHeal => "OVERHEAD_ALERT_HEAL",
            DotaOverheadAlert::OverheadAlertManaAdd => "OVERHEAD_ALERT_MANA_ADD",
            DotaOverheadAlert::OverheadAlertManaLoss => "OVERHEAD_ALERT_MANA_LOSS",
            DotaOverheadAlert::OverheadAlertLastHitEarly => {
                "OVERHEAD_ALERT_LAST_HIT_EARLY"
            }
            DotaOverheadAlert::OverheadAlertLastHitClose => {
                "OVERHEAD_ALERT_LAST_HIT_CLOSE"
            }
            DotaOverheadAlert::OverheadAlertLastHitMiss => "OVERHEAD_ALERT_LAST_HIT_MISS",
            DotaOverheadAlert::OverheadAlertMagicalBlock => {
                "OVERHEAD_ALERT_MAGICAL_BLOCK"
            }
            DotaOverheadAlert::OverheadAlertIncomingDamage => {
                "OVERHEAD_ALERT_INCOMING_DAMAGE"
            }
            DotaOverheadAlert::OverheadAlertOutgoingDamage => {
                "OVERHEAD_ALERT_OUTGOING_DAMAGE"
            }
            DotaOverheadAlert::OverheadAlertDisableResist => {
                "OVERHEAD_ALERT_DISABLE_RESIST"
            }
            DotaOverheadAlert::OverheadAlertDeath => "OVERHEAD_ALERT_DEATH",
            DotaOverheadAlert::OverheadAlertBlocked => "OVERHEAD_ALERT_BLOCKED",
            DotaOverheadAlert::OverheadAlertItemReceived => {
                "OVERHEAD_ALERT_ITEM_RECEIVED"
            }
            DotaOverheadAlert::OverheadAlertShard => "OVERHEAD_ALERT_SHARD",
            DotaOverheadAlert::OverheadAlertDeadlyBlow => "OVERHEAD_ALERT_DEADLY_BLOW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OVERHEAD_ALERT_GOLD" => Some(Self::OverheadAlertGold),
            "OVERHEAD_ALERT_DENY" => Some(Self::OverheadAlertDeny),
            "OVERHEAD_ALERT_CRITICAL" => Some(Self::OverheadAlertCritical),
            "OVERHEAD_ALERT_XP" => Some(Self::OverheadAlertXp),
            "OVERHEAD_ALERT_BONUS_SPELL_DAMAGE" => {
                Some(Self::OverheadAlertBonusSpellDamage)
            }
            "OVERHEAD_ALERT_MISS" => Some(Self::OverheadAlertMiss),
            "OVERHEAD_ALERT_DAMAGE" => Some(Self::OverheadAlertDamage),
            "OVERHEAD_ALERT_EVADE" => Some(Self::OverheadAlertEvade),
            "OVERHEAD_ALERT_BLOCK" => Some(Self::OverheadAlertBlock),
            "OVERHEAD_ALERT_BONUS_POISON_DAMAGE" => {
                Some(Self::OverheadAlertBonusPoisonDamage)
            }
            "OVERHEAD_ALERT_HEAL" => Some(Self::OverheadAlertHeal),
            "OVERHEAD_ALERT_MANA_ADD" => Some(Self::OverheadAlertManaAdd),
            "OVERHEAD_ALERT_MANA_LOSS" => Some(Self::OverheadAlertManaLoss),
            "OVERHEAD_ALERT_LAST_HIT_EARLY" => Some(Self::OverheadAlertLastHitEarly),
            "OVERHEAD_ALERT_LAST_HIT_CLOSE" => Some(Self::OverheadAlertLastHitClose),
            "OVERHEAD_ALERT_LAST_HIT_MISS" => Some(Self::OverheadAlertLastHitMiss),
            "OVERHEAD_ALERT_MAGICAL_BLOCK" => Some(Self::OverheadAlertMagicalBlock),
            "OVERHEAD_ALERT_INCOMING_DAMAGE" => Some(Self::OverheadAlertIncomingDamage),
            "OVERHEAD_ALERT_OUTGOING_DAMAGE" => Some(Self::OverheadAlertOutgoingDamage),
            "OVERHEAD_ALERT_DISABLE_RESIST" => Some(Self::OverheadAlertDisableResist),
            "OVERHEAD_ALERT_DEATH" => Some(Self::OverheadAlertDeath),
            "OVERHEAD_ALERT_BLOCKED" => Some(Self::OverheadAlertBlocked),
            "OVERHEAD_ALERT_ITEM_RECEIVED" => Some(Self::OverheadAlertItemReceived),
            "OVERHEAD_ALERT_SHARD" => Some(Self::OverheadAlertShard),
            "OVERHEAD_ALERT_DEADLY_BLOW" => Some(Self::OverheadAlertDeadlyBlow),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaRoshanPhase {
    KSrspRoshanAlive = 0,
    KSrspRoshanBaseTimer = 1,
    KSrspRoshanVisibleTimer = 2,
}
impl DotaRoshanPhase {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaRoshanPhase::KSrspRoshanAlive => "k_SRSP_ROSHAN_ALIVE",
            DotaRoshanPhase::KSrspRoshanBaseTimer => "k_SRSP_ROSHAN_BASE_TIMER",
            DotaRoshanPhase::KSrspRoshanVisibleTimer => "k_SRSP_ROSHAN_VISIBLE_TIMER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_SRSP_ROSHAN_ALIVE" => Some(Self::KSrspRoshanAlive),
            "k_SRSP_ROSHAN_BASE_TIMER" => Some(Self::KSrspRoshanBaseTimer),
            "k_SRSP_ROSHAN_VISIBLE_TIMER" => Some(Self::KSrspRoshanVisibleTimer),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl DotaPositionCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaPositionCategory::DotaPositionNone => "DOTA_POSITION_NONE",
            DotaPositionCategory::DotaPositionBottomLane => "DOTA_POSITION_BOTTOM_LANE",
            DotaPositionCategory::DotaPositionMidLane => "DOTA_POSITION_MID_LANE",
            DotaPositionCategory::DotaPositionTopLane => "DOTA_POSITION_TOP_LANE",
            DotaPositionCategory::DotaPositionRadiantJungle => {
                "DOTA_POSITION_RADIANT_JUNGLE"
            }
            DotaPositionCategory::DotaPositionDireJungle => "DOTA_POSITION_DIRE_JUNGLE",
            DotaPositionCategory::DotaPositionRadiantAncients => {
                "DOTA_POSITION_RADIANT_ANCIENTS"
            }
            DotaPositionCategory::DotaPositionDireAncients => {
                "DOTA_POSITION_DIRE_ANCIENTS"
            }
            DotaPositionCategory::DotaPositionRadiantSecretShop => {
                "DOTA_POSITION_RADIANT_SECRET_SHOP"
            }
            DotaPositionCategory::DotaPositionDireSecretShop => {
                "DOTA_POSITION_DIRE_SECRET_SHOP"
            }
            DotaPositionCategory::DotaPositionRiver => "DOTA_POSITION_RIVER",
            DotaPositionCategory::DotaPositionRoshanPit => "DOTA_POSITION_ROSHAN_PIT",
            DotaPositionCategory::DotaPositionRadiantBase => "DOTA_POSITION_RADIANT_BASE",
            DotaPositionCategory::DotaPositionDireBase => "DOTA_POSITION_DIRE_BASE",
            DotaPositionCategory::DotaPositionFountain => "DOTA_POSITION_FOUNTAIN",
            DotaPositionCategory::DotaPositionOther => "DOTA_POSITION_OTHER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_POSITION_NONE" => Some(Self::DotaPositionNone),
            "DOTA_POSITION_BOTTOM_LANE" => Some(Self::DotaPositionBottomLane),
            "DOTA_POSITION_MID_LANE" => Some(Self::DotaPositionMidLane),
            "DOTA_POSITION_TOP_LANE" => Some(Self::DotaPositionTopLane),
            "DOTA_POSITION_RADIANT_JUNGLE" => Some(Self::DotaPositionRadiantJungle),
            "DOTA_POSITION_DIRE_JUNGLE" => Some(Self::DotaPositionDireJungle),
            "DOTA_POSITION_RADIANT_ANCIENTS" => Some(Self::DotaPositionRadiantAncients),
            "DOTA_POSITION_DIRE_ANCIENTS" => Some(Self::DotaPositionDireAncients),
            "DOTA_POSITION_RADIANT_SECRET_SHOP" => {
                Some(Self::DotaPositionRadiantSecretShop)
            }
            "DOTA_POSITION_DIRE_SECRET_SHOP" => Some(Self::DotaPositionDireSecretShop),
            "DOTA_POSITION_RIVER" => Some(Self::DotaPositionRiver),
            "DOTA_POSITION_ROSHAN_PIT" => Some(Self::DotaPositionRoshanPit),
            "DOTA_POSITION_RADIANT_BASE" => Some(Self::DotaPositionRadiantBase),
            "DOTA_POSITION_DIRE_BASE" => Some(Self::DotaPositionDireBase),
            "DOTA_POSITION_FOUNTAIN" => Some(Self::DotaPositionFountain),
            "DOTA_POSITION_OTHER" => Some(Self::DotaPositionOther),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DotaAbilityTargetType {
    DotaAbilityTargetNone = 0,
    DotaAbilityTargetSelf = 1,
    DotaAbilityTargetAllyHero = 2,
    DotaAbilityTargetAllyCreep = 3,
    DotaAbilityTargetEnemyHero = 4,
    DotaAbilityTargetEnemyCreep = 5,
}
impl DotaAbilityTargetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DotaAbilityTargetType::DotaAbilityTargetNone => "DOTA_ABILITY_TARGET_NONE",
            DotaAbilityTargetType::DotaAbilityTargetSelf => "DOTA_ABILITY_TARGET_SELF",
            DotaAbilityTargetType::DotaAbilityTargetAllyHero => {
                "DOTA_ABILITY_TARGET_ALLY_HERO"
            }
            DotaAbilityTargetType::DotaAbilityTargetAllyCreep => {
                "DOTA_ABILITY_TARGET_ALLY_CREEP"
            }
            DotaAbilityTargetType::DotaAbilityTargetEnemyHero => {
                "DOTA_ABILITY_TARGET_ENEMY_HERO"
            }
            DotaAbilityTargetType::DotaAbilityTargetEnemyCreep => {
                "DOTA_ABILITY_TARGET_ENEMY_CREEP"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DOTA_ABILITY_TARGET_NONE" => Some(Self::DotaAbilityTargetNone),
            "DOTA_ABILITY_TARGET_SELF" => Some(Self::DotaAbilityTargetSelf),
            "DOTA_ABILITY_TARGET_ALLY_HERO" => Some(Self::DotaAbilityTargetAllyHero),
            "DOTA_ABILITY_TARGET_ALLY_CREEP" => Some(Self::DotaAbilityTargetAllyCreep),
            "DOTA_ABILITY_TARGET_ENEMY_HERO" => Some(Self::DotaAbilityTargetEnemyHero),
            "DOTA_ABILITY_TARGET_ENEMY_CREEP" => Some(Self::DotaAbilityTargetEnemyCreep),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl EHeroStatType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EHeroStatType::KEHeroStatTypeNone => "k_EHeroStatType_None",
            EHeroStatType::KEHeroStatTypeAxeTotalDamage => {
                "k_EHeroStatType_AxeTotalDamage"
            }
            EHeroStatType::KEHeroStatTypeBattleHungerDamage => {
                "k_EHeroStatType_BattleHungerDamage"
            }
            EHeroStatType::KEHeroStatTypeCounterHelixDamage => {
                "k_EHeroStatType_CounterHelixDamage"
            }
            EHeroStatType::KEHeroStatTypeCullingBladeDamage => {
                "k_EHeroStatType_CullingBladeDamage"
            }
            EHeroStatType::KEHeroStatTypeBerserkersCallCastCount => {
                "k_EHeroStatType_BerserkersCallCastCount"
            }
            EHeroStatType::KEHeroStatTypeBerserkersCallHeroesHitAverage => {
                "k_EHeroStatType_BerserkersCallHeroesHitAverage"
            }
            EHeroStatType::KEHeroStatTypeBerserkersCallOtherUnitsHit => {
                "k_EHeroStatType_BerserkersCallOtherUnitsHit"
            }
            EHeroStatType::KEHeroStatTypeBerserkersCallHeroAttacksTaken => {
                "k_EHeroStatType_BerserkersCallHeroAttacksTaken"
            }
            EHeroStatType::KEHeroStatTypeBerserkersCallOtherAttacksTaken => {
                "k_EHeroStatType_BerserkersCallOtherAttacksTaken"
            }
            EHeroStatType::KEHeroStatTypeBattleHungerCastCount => {
                "k_EHeroStatType_BattleHungerCastCount"
            }
            EHeroStatType::KEHeroStatTypeBattleHungerPotentialDuration => {
                "k_EHeroStatType_BattleHungerPotentialDuration"
            }
            EHeroStatType::KEHeroStatTypeBattleHungerAverageDuration => {
                "k_EHeroStatType_BattleHungerAverageDuration"
            }
            EHeroStatType::KEHeroStatTypeCounterHelixProcCount => {
                "k_EHeroStatType_CounterHelixProcCount"
            }
            EHeroStatType::KEHeroStatTypeCounterHelixHeroProcCount => {
                "k_EHeroStatType_CounterHelixHeroProcCount"
            }
            EHeroStatType::KEHeroStatTypeCounterHelixHeroesHitAverage => {
                "k_EHeroStatType_CounterHelixHeroesHitAverage"
            }
            EHeroStatType::KEHeroStatTypeCounterHelixOtherUnitsHitCount => {
                "k_EHeroStatType_CounterHelixOtherUnitsHitCount"
            }
            EHeroStatType::KEHeroStatTypeCullingBladeCastCount => {
                "k_EHeroStatType_CullingBladeCastCount"
            }
            EHeroStatType::KEHeroStatTypeCullingBladeKillCount => {
                "k_EHeroStatType_CullingBladeKillCount"
            }
            EHeroStatType::KEHeroStatTypeCullingBladeAverageHealthCulled => {
                "k_EHeroStatType_CullingBladeAverageHealthCulled"
            }
            EHeroStatType::KEHeroStatTypeCullingBladeAverageDamageAvailable => {
                "k_EHeroStatType_CullingBladeAverageDamageAvailable"
            }
            EHeroStatType::KEHeroStatTypeCullingBladeHeroBuffAverage => {
                "k_EHeroStatType_CullingBladeHeroBuffAverage"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "k_EHeroStatType_None" => Some(Self::KEHeroStatTypeNone),
            "k_EHeroStatType_AxeTotalDamage" => Some(Self::KEHeroStatTypeAxeTotalDamage),
            "k_EHeroStatType_BattleHungerDamage" => {
                Some(Self::KEHeroStatTypeBattleHungerDamage)
            }
            "k_EHeroStatType_CounterHelixDamage" => {
                Some(Self::KEHeroStatTypeCounterHelixDamage)
            }
            "k_EHeroStatType_CullingBladeDamage" => {
                Some(Self::KEHeroStatTypeCullingBladeDamage)
            }
            "k_EHeroStatType_BerserkersCallCastCount" => {
                Some(Self::KEHeroStatTypeBerserkersCallCastCount)
            }
            "k_EHeroStatType_BerserkersCallHeroesHitAverage" => {
                Some(Self::KEHeroStatTypeBerserkersCallHeroesHitAverage)
            }
            "k_EHeroStatType_BerserkersCallOtherUnitsHit" => {
                Some(Self::KEHeroStatTypeBerserkersCallOtherUnitsHit)
            }
            "k_EHeroStatType_BerserkersCallHeroAttacksTaken" => {
                Some(Self::KEHeroStatTypeBerserkersCallHeroAttacksTaken)
            }
            "k_EHeroStatType_BerserkersCallOtherAttacksTaken" => {
                Some(Self::KEHeroStatTypeBerserkersCallOtherAttacksTaken)
            }
            "k_EHeroStatType_BattleHungerCastCount" => {
                Some(Self::KEHeroStatTypeBattleHungerCastCount)
            }
            "k_EHeroStatType_BattleHungerPotentialDuration" => {
                Some(Self::KEHeroStatTypeBattleHungerPotentialDuration)
            }
            "k_EHeroStatType_BattleHungerAverageDuration" => {
                Some(Self::KEHeroStatTypeBattleHungerAverageDuration)
            }
            "k_EHeroStatType_CounterHelixProcCount" => {
                Some(Self::KEHeroStatTypeCounterHelixProcCount)
            }
            "k_EHeroStatType_CounterHelixHeroProcCount" => {
                Some(Self::KEHeroStatTypeCounterHelixHeroProcCount)
            }
            "k_EHeroStatType_CounterHelixHeroesHitAverage" => {
                Some(Self::KEHeroStatTypeCounterHelixHeroesHitAverage)
            }
            "k_EHeroStatType_CounterHelixOtherUnitsHitCount" => {
                Some(Self::KEHeroStatTypeCounterHelixOtherUnitsHitCount)
            }
            "k_EHeroStatType_CullingBladeCastCount" => {
                Some(Self::KEHeroStatTypeCullingBladeCastCount)
            }
            "k_EHeroStatType_CullingBladeKillCount" => {
                Some(Self::KEHeroStatTypeCullingBladeKillCount)
            }
            "k_EHeroStatType_CullingBladeAverageHealthCulled" => {
                Some(Self::KEHeroStatTypeCullingBladeAverageHealthCulled)
            }
            "k_EHeroStatType_CullingBladeAverageDamageAvailable" => {
                Some(Self::KEHeroStatTypeCullingBladeAverageDamageAvailable)
            }
            "k_EHeroStatType_CullingBladeHeroBuffAverage" => {
                Some(Self::KEHeroStatTypeCullingBladeHeroBuffAverage)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl EPlayerVoiceListenState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EPlayerVoiceListenState::KPvlsNone => "kPVLS_None",
            EPlayerVoiceListenState::KPvlsDeniedChatBanned => "kPVLS_DeniedChatBanned",
            EPlayerVoiceListenState::KPvlsDeniedPartner => "kPVLS_DeniedPartner",
            EPlayerVoiceListenState::KPvlsDeniedHltvTalkerNotSpectator => {
                "kPVLS_DeniedHLTVTalkerNotSpectator"
            }
            EPlayerVoiceListenState::KPvlsDeniedHltvNoTalkerPlayerId => {
                "kPVLS_DeniedHLTVNoTalkerPlayerID"
            }
            EPlayerVoiceListenState::KPvlsDeniedHltvTalkerNotBroadcaster => {
                "kPVLS_DeniedHLTVTalkerNotBroadcaster"
            }
            EPlayerVoiceListenState::KPvlsDeniedTeamSpectator => {
                "kPVLS_DeniedTeamSpectator"
            }
            EPlayerVoiceListenState::KPvlsDeniedStudent => "kPVLS_DeniedStudent",
            EPlayerVoiceListenState::KPvlsDeniedPrivateCoach => {
                "kPVLS_DeniedPrivateCoach"
            }
            EPlayerVoiceListenState::KPvlsDenied => "kPVLS_Denied",
            EPlayerVoiceListenState::KPvlsAllowHltvTalkerIsBroadcaster => {
                "kPVLS_AllowHLTVTalkerIsBroadcaster"
            }
            EPlayerVoiceListenState::KPvlsAllowCoBroadcaster => {
                "kPVLS_AllowCoBroadcaster"
            }
            EPlayerVoiceListenState::KPvlsAllowAllChat => "kPVLS_AllowAllChat",
            EPlayerVoiceListenState::KPvlsAllowStudentToCoach => {
                "kPVLS_AllowStudentToCoach"
            }
            EPlayerVoiceListenState::KPvlsAllowFellowStudent => {
                "kPVLS_AllowFellowStudent"
            }
            EPlayerVoiceListenState::KPvlsAllowTalkerIsCoach => {
                "kPVLS_AllowTalkerIsCoach"
            }
            EPlayerVoiceListenState::KPvlsAllowCoachHearTeam => {
                "kPVLS_AllowCoachHearTeam"
            }
            EPlayerVoiceListenState::KPvlsAllowSameTeam => "kPVLS_AllowSameTeam",
            EPlayerVoiceListenState::KPvlsAllowShowcase => "kPVLS_AllowShowcase",
            EPlayerVoiceListenState::KPvlsAllowPrivateCoach => "kPVLS_AllowPrivateCoach",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "kPVLS_None" => Some(Self::KPvlsNone),
            "kPVLS_DeniedChatBanned" => Some(Self::KPvlsDeniedChatBanned),
            "kPVLS_DeniedPartner" => Some(Self::KPvlsDeniedPartner),
            "kPVLS_DeniedHLTVTalkerNotSpectator" => {
                Some(Self::KPvlsDeniedHltvTalkerNotSpectator)
            }
            "kPVLS_DeniedHLTVNoTalkerPlayerID" => {
                Some(Self::KPvlsDeniedHltvNoTalkerPlayerId)
            }
            "kPVLS_DeniedHLTVTalkerNotBroadcaster" => {
                Some(Self::KPvlsDeniedHltvTalkerNotBroadcaster)
            }
            "kPVLS_DeniedTeamSpectator" => Some(Self::KPvlsDeniedTeamSpectator),
            "kPVLS_DeniedStudent" => Some(Self::KPvlsDeniedStudent),
            "kPVLS_DeniedPrivateCoach" => Some(Self::KPvlsDeniedPrivateCoach),
            "kPVLS_Denied" => Some(Self::KPvlsDenied),
            "kPVLS_AllowHLTVTalkerIsBroadcaster" => {
                Some(Self::KPvlsAllowHltvTalkerIsBroadcaster)
            }
            "kPVLS_AllowCoBroadcaster" => Some(Self::KPvlsAllowCoBroadcaster),
            "kPVLS_AllowAllChat" => Some(Self::KPvlsAllowAllChat),
            "kPVLS_AllowStudentToCoach" => Some(Self::KPvlsAllowStudentToCoach),
            "kPVLS_AllowFellowStudent" => Some(Self::KPvlsAllowFellowStudent),
            "kPVLS_AllowTalkerIsCoach" => Some(Self::KPvlsAllowTalkerIsCoach),
            "kPVLS_AllowCoachHearTeam" => Some(Self::KPvlsAllowCoachHearTeam),
            "kPVLS_AllowSameTeam" => Some(Self::KPvlsAllowSameTeam),
            "kPVLS_AllowShowcase" => Some(Self::KPvlsAllowShowcase),
            "kPVLS_AllowPrivateCoach" => Some(Self::KPvlsAllowPrivateCoach),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EProjectionEvent {
    EPeFirstBlood = 0,
    EPeKillstreakGodlike = 1,
}
impl EProjectionEvent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EProjectionEvent::EPeFirstBlood => "ePE_FirstBlood",
            EProjectionEvent::EPeKillstreakGodlike => "ePE_Killstreak_godlike",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ePE_FirstBlood" => Some(Self::EPeFirstBlood),
            "ePE_Killstreak_godlike" => Some(Self::EPeKillstreakGodlike),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgVDebugGameSessionIdEvent {
    #[prost(int32, optional, tag = "1")]
    pub clientid: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub gamesessionid: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgClearWorldDecalsEvent {
    #[prost(uint32, optional, tag = "1")]
    pub flagstoclear: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgClearEntityDecalsEvent {
    #[prost(uint32, optional, tag = "1")]
    pub flagstoclear: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgClearDecalsForSkeletonInstanceEvent {
    #[prost(uint32, optional, tag = "1")]
    pub flagstoclear: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub entityhandleindex: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub skeletoninstancehash: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSource1LegacyGameEventList {
    #[prost(message, repeated, tag = "1")]
    pub descriptors: ::prost::alloc::vec::Vec<
        c_msg_source1_legacy_game_event_list::DescriptorT,
    >,
}
/// Nested message and enum types in `CMsgSource1LegacyGameEventList`.
pub mod c_msg_source1_legacy_game_event_list {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyT {
        #[prost(int32, optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
    }
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSource1LegacyListenEvents {
    #[prost(int32, optional, tag = "1")]
    pub playerslot: ::core::option::Option<i32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub eventarraybits: ::prost::alloc::vec::Vec<u32>,
}
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
/// Nested message and enum types in `CMsgSource1LegacyGameEvent`.
pub mod c_msg_source1_legacy_game_event {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSosStopSoundEvent {
    #[prost(int32, optional, tag = "1")]
    pub soundevent_guid: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSosStopSoundEventHash {
    #[prost(fixed32, optional, tag = "1")]
    pub soundevent_hash: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub source_entity_index: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSosSetSoundEventParams {
    #[prost(int32, optional, tag = "1")]
    pub soundevent_guid: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub packed_params: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgSosSetLibraryStackFields {
    #[prost(fixed32, optional, tag = "1")]
    pub stack_hash: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub packed_fields: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
impl EBaseGameEvents {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EBaseGameEvents::GeVDebugGameSessionIdEvent => "GE_VDebugGameSessionIDEvent",
            EBaseGameEvents::GePlaceDecalEvent => "GE_PlaceDecalEvent",
            EBaseGameEvents::GeClearWorldDecalsEvent => "GE_ClearWorldDecalsEvent",
            EBaseGameEvents::GeClearEntityDecalsEvent => "GE_ClearEntityDecalsEvent",
            EBaseGameEvents::GeClearDecalsForSkeletonInstanceEvent => {
                "GE_ClearDecalsForSkeletonInstanceEvent"
            }
            EBaseGameEvents::GeSource1LegacyGameEventList => {
                "GE_Source1LegacyGameEventList"
            }
            EBaseGameEvents::GeSource1LegacyListenEvents => {
                "GE_Source1LegacyListenEvents"
            }
            EBaseGameEvents::GeSource1LegacyGameEvent => "GE_Source1LegacyGameEvent",
            EBaseGameEvents::GeSosStartSoundEvent => "GE_SosStartSoundEvent",
            EBaseGameEvents::GeSosStopSoundEvent => "GE_SosStopSoundEvent",
            EBaseGameEvents::GeSosSetSoundEventParams => "GE_SosSetSoundEventParams",
            EBaseGameEvents::GeSosSetLibraryStackFields => "GE_SosSetLibraryStackFields",
            EBaseGameEvents::GeSosStopSoundEventHash => "GE_SosStopSoundEventHash",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GE_VDebugGameSessionIDEvent" => Some(Self::GeVDebugGameSessionIdEvent),
            "GE_PlaceDecalEvent" => Some(Self::GePlaceDecalEvent),
            "GE_ClearWorldDecalsEvent" => Some(Self::GeClearWorldDecalsEvent),
            "GE_ClearEntityDecalsEvent" => Some(Self::GeClearEntityDecalsEvent),
            "GE_ClearDecalsForSkeletonInstanceEvent" => {
                Some(Self::GeClearDecalsForSkeletonInstanceEvent)
            }
            "GE_Source1LegacyGameEventList" => Some(Self::GeSource1LegacyGameEventList),
            "GE_Source1LegacyListenEvents" => Some(Self::GeSource1LegacyListenEvents),
            "GE_Source1LegacyGameEvent" => Some(Self::GeSource1LegacyGameEvent),
            "GE_SosStartSoundEvent" => Some(Self::GeSosStartSoundEvent),
            "GE_SosStopSoundEvent" => Some(Self::GeSosStopSoundEvent),
            "GE_SosSetSoundEventParams" => Some(Self::GeSosSetSoundEventParams),
            "GE_SosSetLibraryStackFields" => Some(Self::GeSosSetLibraryStackFields),
            "GE_SosStopSoundEventHash" => Some(Self::GeSosStopSoundEventHash),
            _ => None,
        }
    }
}
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgMove {
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "4")]
    pub command_number: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub num_commands: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgVoiceAudio {
    #[prost(
        enumeration = "VoiceDataFormatT",
        optional,
        tag = "1",
        default = "VoicedataFormatSteam"
    )]
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
}
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgBaselineAck {
    #[prost(int32, optional, tag = "1")]
    pub baseline_tick: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub baseline_nr: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgListenEvents {
    #[prost(fixed32, repeated, packed = "false", tag = "1")]
    pub event_mask: ::prost::alloc::vec::Vec<u32>,
}
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgLoadingProgress {
    #[prost(int32, optional, tag = "1")]
    pub progress: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgSplitPlayerConnect {
    #[prost(string, optional, tag = "1")]
    pub playername: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgClientMessage {
    #[prost(int32, optional, tag = "1")]
    pub msg_type: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgSplitPlayerDisconnect {
    #[prost(int32, optional, tag = "1")]
    pub slot: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgServerStatus {
    #[prost(bool, optional, tag = "1")]
    pub simplified: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgServerPing {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgRequestPause {
    #[prost(enumeration = "RequestPauseT", optional, tag = "1", default = "RpPause")]
    pub pause_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub pause_group: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgCmdKeyValues {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgRconServerDetails {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgServerInfo {
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
    pub game_session_config: ::core::option::Option<CsvcMsgGameSessionConfiguration>,
    #[prost(bytes = "vec", optional, tag = "20")]
    pub game_session_manifest: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgClassInfo {
    #[prost(bool, optional, tag = "1")]
    pub create_on_client: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "2")]
    pub classes: ::prost::alloc::vec::Vec<csvc_msg_class_info::ClassT>,
}
/// Nested message and enum types in `CSVCMsg_ClassInfo`.
pub mod csvc_msg_class_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClassT {
        #[prost(int32, optional, tag = "1")]
        pub class_id: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "3")]
        pub class_name: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgSetPause {
    #[prost(bool, optional, tag = "1")]
    pub paused: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgVoiceInit {
    #[prost(int32, optional, tag = "1")]
    pub quality: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub codec: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3", default = "0")]
    pub version: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgPrint {
    #[prost(string, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgSounds {
    #[prost(bool, optional, tag = "1")]
    pub reliable_sound: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "2")]
    pub sounds: ::prost::alloc::vec::Vec<csvc_msg_sounds::SounddataT>,
}
/// Nested message and enum types in `CSVCMsg_Sounds`.
pub mod csvc_msg_sounds {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgPrefetch {
    #[prost(int32, optional, tag = "1")]
    pub sound_index: ::core::option::Option<i32>,
    #[prost(enumeration = "PrefetchType", optional, tag = "2", default = "PftSound")]
    pub resource_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgSetView {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entity_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub slot: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgFixAngle {
    #[prost(bool, optional, tag = "1")]
    pub relative: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub angle: ::core::option::Option<CMsgQAngle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgCrosshairAngle {
    #[prost(message, optional, tag = "1")]
    pub angle: ::core::option::Option<CMsgQAngle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgBspDecal {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgSplitScreen {
    #[prost(
        enumeration = "ESplitScreenMessageType",
        optional,
        tag = "1",
        default = "MsgSplitscreenAdduser"
    )]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub slot: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub player_index: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgGetCvarValue {
    #[prost(int32, optional, tag = "1")]
    pub cookie: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub cvar_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgMenu {
    #[prost(int32, optional, tag = "1")]
    pub dialog_type: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub menu_key_values: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgUserMessage {
    #[prost(int32, optional, tag = "1")]
    pub msg_type: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub msg_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "3")]
    pub passthrough: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgSendTable {
    #[prost(bool, optional, tag = "1")]
    pub is_end: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "2")]
    pub net_table_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub needs_decoder: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "4")]
    pub props: ::prost::alloc::vec::Vec<csvc_msg_send_table::SendpropT>,
}
/// Nested message and enum types in `CSVCMsg_SendTable`.
pub mod csvc_msg_send_table {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgGameEventList {
    #[prost(message, repeated, tag = "1")]
    pub descriptors: ::prost::alloc::vec::Vec<csvc_msg_game_event_list::DescriptorT>,
}
/// Nested message and enum types in `CSVCMsg_GameEventList`.
pub mod csvc_msg_game_event_list {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyT {
        #[prost(int32, optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
    }
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgPacketEntities {
    #[prost(int32, optional, tag = "1")]
    pub max_entries: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub updated_entries: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub is_delta: ::core::option::Option<bool>,
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
    pub last_cmd_number: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub server_tick: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "13")]
    pub serialized_entities: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "14")]
    pub command_queue_info: ::core::option::Option<
        csvc_msg_packet_entities::CommandQueueInfoT,
    >,
    #[prost(message, repeated, tag = "15")]
    pub alternate_baselines: ::prost::alloc::vec::Vec<
        csvc_msg_packet_entities::AlternateBaselineT,
    >,
}
/// Nested message and enum types in `CSVCMsg_PacketEntities`.
pub mod csvc_msg_packet_entities {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommandQueueInfoT {
        #[prost(uint32, optional, tag = "1")]
        pub commands_queued: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub command_queue_desired_size: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub starved_command_ticks: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "4")]
        pub time_dilation_percent: ::core::option::Option<f32>,
        #[prost(uint32, optional, tag = "5")]
        pub discarded_command_ticks: ::core::option::Option<u32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AlternateBaselineT {
        #[prost(int32, optional, tag = "1")]
        pub entity_index: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub baseline_index: ::core::option::Option<i32>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgTempEntities {
    #[prost(bool, optional, tag = "1")]
    pub reliable: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "2")]
    pub num_entries: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub entity_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgCreateStringTable {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgUpdateStringTable {
    #[prost(int32, optional, tag = "1")]
    pub table_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub num_changed_entries: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub string_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgVoiceData {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgPacketReliable {
    #[prost(int32, optional, tag = "1")]
    pub tick: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub messagessize: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub state: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgFullFrameSplit {
    #[prost(int32, optional, tag = "1")]
    pub tick: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub section: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub total: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgHltvStatus {
    #[prost(string, optional, tag = "1")]
    pub master: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub clients: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub slots: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub proxies: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgServerSteamId {
    #[prost(uint64, optional, tag = "1")]
    pub steam_id: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgCmdKeyValues {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgRconServerDetails {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "2")]
    pub details: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CMsgIpcAddress {
    #[prost(fixed64, optional, tag = "1")]
    pub computer_guid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub process_id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgPeerList {
    #[prost(message, repeated, tag = "1")]
    pub peer: ::prost::alloc::vec::Vec<CMsgServerPeer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgClearAllStringTables {
    #[prost(string, optional, tag = "1")]
    pub mapname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub create_tables_skipped: ::core::option::Option<bool>,
}
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
    pub polymorphic_types: ::prost::alloc::vec::Vec<
        proto_flattened_serializer_field_t::PolymorphicFieldT,
    >,
}
/// Nested message and enum types in `ProtoFlattenedSerializerField_t`.
pub mod proto_flattened_serializer_field_t {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PolymorphicFieldT {
        #[prost(int32, optional, tag = "1")]
        pub polymorphic_field_serializer_name_sym: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub polymorphic_field_serializer_version: ::core::option::Option<i32>,
    }
}
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgFlattenedSerializer {
    #[prost(message, repeated, tag = "1")]
    pub serializers: ::prost::alloc::vec::Vec<ProtoFlattenedSerializerT>,
    #[prost(string, repeated, tag = "2")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub fields: ::prost::alloc::vec::Vec<ProtoFlattenedSerializerFieldT>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgStopSound {
    #[prost(fixed32, optional, tag = "1")]
    pub guid: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CBidirMsgRebroadcastSource {
    #[prost(int32, optional, tag = "1")]
    pub eventsource: ::core::option::Option<i32>,
}
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
    pub avg_latency_out: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "19")]
    pub avg_latency_in: ::core::option::Option<f32>,
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
/// Nested message and enum types in `CMsgServerNetworkStats`.
pub mod c_msg_server_network_stats {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Port {
        #[prost(int32, optional, tag = "1")]
        pub port: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Player {
        #[prost(uint64, optional, tag = "1")]
        pub steamid: ::core::option::Option<u64>,
        #[prost(string, optional, tag = "2")]
        pub remote_addr: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "3")]
        pub ping_stddev_ms: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "4")]
        pub ping_avg_ms: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "5")]
        pub packet_loss_pct: ::core::option::Option<f32>,
        #[prost(bool, optional, tag = "6")]
        pub is_bot: ::core::option::Option<bool>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgHltvReplay {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgBroadcastCommand {
    #[prost(string, optional, tag = "1")]
    pub cmd: ::core::option::Option<::prost::alloc::string::String>,
}
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvcMsgHltvFixupOperatorStatus {
    #[prost(uint32, optional, tag = "1")]
    pub mode: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub override_operator_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClcMessages {
    ClcClientInfo = 20,
    ClcMove = 21,
    ClcVoiceData = 22,
    ClcBaselineAck = 23,
    ClcListenEvents = 24,
    ClcRespondCvarValue = 25,
    ClcFileCrcCheck = 26,
    ClcLoadingProgress = 27,
    ClcSplitPlayerConnect = 28,
    ClcClientMessage = 29,
    ClcSplitPlayerDisconnect = 30,
    ClcServerStatus = 31,
    ClcServerPing = 32,
    ClcRequestPause = 33,
    ClcCmdKeyValues = 34,
    ClcRconServerDetails = 35,
    ClcHltvReplay = 36,
}
impl ClcMessages {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClcMessages::ClcClientInfo => "clc_ClientInfo",
            ClcMessages::ClcMove => "clc_Move",
            ClcMessages::ClcVoiceData => "clc_VoiceData",
            ClcMessages::ClcBaselineAck => "clc_BaselineAck",
            ClcMessages::ClcListenEvents => "clc_ListenEvents",
            ClcMessages::ClcRespondCvarValue => "clc_RespondCvarValue",
            ClcMessages::ClcFileCrcCheck => "clc_FileCRCCheck",
            ClcMessages::ClcLoadingProgress => "clc_LoadingProgress",
            ClcMessages::ClcSplitPlayerConnect => "clc_SplitPlayerConnect",
            ClcMessages::ClcClientMessage => "clc_ClientMessage",
            ClcMessages::ClcSplitPlayerDisconnect => "clc_SplitPlayerDisconnect",
            ClcMessages::ClcServerStatus => "clc_ServerStatus",
            ClcMessages::ClcServerPing => "clc_ServerPing",
            ClcMessages::ClcRequestPause => "clc_RequestPause",
            ClcMessages::ClcCmdKeyValues => "clc_CmdKeyValues",
            ClcMessages::ClcRconServerDetails => "clc_RconServerDetails",
            ClcMessages::ClcHltvReplay => "clc_HltvReplay",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "clc_ClientInfo" => Some(Self::ClcClientInfo),
            "clc_Move" => Some(Self::ClcMove),
            "clc_VoiceData" => Some(Self::ClcVoiceData),
            "clc_BaselineAck" => Some(Self::ClcBaselineAck),
            "clc_ListenEvents" => Some(Self::ClcListenEvents),
            "clc_RespondCvarValue" => Some(Self::ClcRespondCvarValue),
            "clc_FileCRCCheck" => Some(Self::ClcFileCrcCheck),
            "clc_LoadingProgress" => Some(Self::ClcLoadingProgress),
            "clc_SplitPlayerConnect" => Some(Self::ClcSplitPlayerConnect),
            "clc_ClientMessage" => Some(Self::ClcClientMessage),
            "clc_SplitPlayerDisconnect" => Some(Self::ClcSplitPlayerDisconnect),
            "clc_ServerStatus" => Some(Self::ClcServerStatus),
            "clc_ServerPing" => Some(Self::ClcServerPing),
            "clc_RequestPause" => Some(Self::ClcRequestPause),
            "clc_CmdKeyValues" => Some(Self::ClcCmdKeyValues),
            "clc_RconServerDetails" => Some(Self::ClcRconServerDetails),
            "clc_HltvReplay" => Some(Self::ClcHltvReplay),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    SvcHltvReplay = 73,
    SvcBroadcastCommand = 74,
    SvcHltvFixupOperatorStatus = 75,
}
impl SvcMessages {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SvcMessages::SvcServerInfo => "svc_ServerInfo",
            SvcMessages::SvcFlattenedSerializer => "svc_FlattenedSerializer",
            SvcMessages::SvcClassInfo => "svc_ClassInfo",
            SvcMessages::SvcSetPause => "svc_SetPause",
            SvcMessages::SvcCreateStringTable => "svc_CreateStringTable",
            SvcMessages::SvcUpdateStringTable => "svc_UpdateStringTable",
            SvcMessages::SvcVoiceInit => "svc_VoiceInit",
            SvcMessages::SvcVoiceData => "svc_VoiceData",
            SvcMessages::SvcPrint => "svc_Print",
            SvcMessages::SvcSounds => "svc_Sounds",
            SvcMessages::SvcSetView => "svc_SetView",
            SvcMessages::SvcClearAllStringTables => "svc_ClearAllStringTables",
            SvcMessages::SvcCmdKeyValues => "svc_CmdKeyValues",
            SvcMessages::SvcBspDecal => "svc_BSPDecal",
            SvcMessages::SvcSplitScreen => "svc_SplitScreen",
            SvcMessages::SvcPacketEntities => "svc_PacketEntities",
            SvcMessages::SvcPrefetch => "svc_Prefetch",
            SvcMessages::SvcMenu => "svc_Menu",
            SvcMessages::SvcGetCvarValue => "svc_GetCvarValue",
            SvcMessages::SvcStopSound => "svc_StopSound",
            SvcMessages::SvcPeerList => "svc_PeerList",
            SvcMessages::SvcPacketReliable => "svc_PacketReliable",
            SvcMessages::SvcHltvStatus => "svc_HLTVStatus",
            SvcMessages::SvcServerSteamId => "svc_ServerSteamID",
            SvcMessages::SvcFullFrameSplit => "svc_FullFrameSplit",
            SvcMessages::SvcRconServerDetails => "svc_RconServerDetails",
            SvcMessages::SvcUserMessage => "svc_UserMessage",
            SvcMessages::SvcHltvReplay => "svc_HltvReplay",
            SvcMessages::SvcBroadcastCommand => "svc_Broadcast_Command",
            SvcMessages::SvcHltvFixupOperatorStatus => "svc_HltvFixupOperatorStatus",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "svc_ServerInfo" => Some(Self::SvcServerInfo),
            "svc_FlattenedSerializer" => Some(Self::SvcFlattenedSerializer),
            "svc_ClassInfo" => Some(Self::SvcClassInfo),
            "svc_SetPause" => Some(Self::SvcSetPause),
            "svc_CreateStringTable" => Some(Self::SvcCreateStringTable),
            "svc_UpdateStringTable" => Some(Self::SvcUpdateStringTable),
            "svc_VoiceInit" => Some(Self::SvcVoiceInit),
            "svc_VoiceData" => Some(Self::SvcVoiceData),
            "svc_Print" => Some(Self::SvcPrint),
            "svc_Sounds" => Some(Self::SvcSounds),
            "svc_SetView" => Some(Self::SvcSetView),
            "svc_ClearAllStringTables" => Some(Self::SvcClearAllStringTables),
            "svc_CmdKeyValues" => Some(Self::SvcCmdKeyValues),
            "svc_BSPDecal" => Some(Self::SvcBspDecal),
            "svc_SplitScreen" => Some(Self::SvcSplitScreen),
            "svc_PacketEntities" => Some(Self::SvcPacketEntities),
            "svc_Prefetch" => Some(Self::SvcPrefetch),
            "svc_Menu" => Some(Self::SvcMenu),
            "svc_GetCvarValue" => Some(Self::SvcGetCvarValue),
            "svc_StopSound" => Some(Self::SvcStopSound),
            "svc_PeerList" => Some(Self::SvcPeerList),
            "svc_PacketReliable" => Some(Self::SvcPacketReliable),
            "svc_HLTVStatus" => Some(Self::SvcHltvStatus),
            "svc_ServerSteamID" => Some(Self::SvcServerSteamId),
            "svc_FullFrameSplit" => Some(Self::SvcFullFrameSplit),
            "svc_RconServerDetails" => Some(Self::SvcRconServerDetails),
            "svc_UserMessage" => Some(Self::SvcUserMessage),
            "svc_HltvReplay" => Some(Self::SvcHltvReplay),
            "svc_Broadcast_Command" => Some(Self::SvcBroadcastCommand),
            "svc_HltvFixupOperatorStatus" => Some(Self::SvcHltvFixupOperatorStatus),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VoiceDataFormatT {
    VoicedataFormatSteam = 0,
    VoicedataFormatEngine = 1,
}
impl VoiceDataFormatT {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VoiceDataFormatT::VoicedataFormatSteam => "VOICEDATA_FORMAT_STEAM",
            VoiceDataFormatT::VoicedataFormatEngine => "VOICEDATA_FORMAT_ENGINE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOICEDATA_FORMAT_STEAM" => Some(Self::VoicedataFormatSteam),
            "VOICEDATA_FORMAT_ENGINE" => Some(Self::VoicedataFormatEngine),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RequestPauseT {
    RpPause = 0,
    RpUnpause = 1,
    RpTogglepause = 2,
}
impl RequestPauseT {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RequestPauseT::RpPause => "RP_PAUSE",
            RequestPauseT::RpUnpause => "RP_UNPAUSE",
            RequestPauseT::RpTogglepause => "RP_TOGGLEPAUSE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RP_PAUSE" => Some(Self::RpPause),
            "RP_UNPAUSE" => Some(Self::RpUnpause),
            "RP_TOGGLEPAUSE" => Some(Self::RpTogglepause),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PrefetchType {
    PftSound = 0,
}
impl PrefetchType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PrefetchType::PftSound => "PFT_SOUND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PFT_SOUND" => Some(Self::PftSound),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ESplitScreenMessageType {
    MsgSplitscreenAdduser = 0,
    MsgSplitscreenRemoveuser = 1,
}
impl ESplitScreenMessageType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ESplitScreenMessageType::MsgSplitscreenAdduser => "MSG_SPLITSCREEN_ADDUSER",
            ESplitScreenMessageType::MsgSplitscreenRemoveuser => {
                "MSG_SPLITSCREEN_REMOVEUSER"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MSG_SPLITSCREEN_ADDUSER" => Some(Self::MsgSplitscreenAdduser),
            "MSG_SPLITSCREEN_REMOVEUSER" => Some(Self::MsgSplitscreenRemoveuser),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EQueryCvarValueStatus {
    ValueIntact = 0,
    CvarNotFound = 1,
    NotACvar = 2,
    CvarProtected = 3,
}
impl EQueryCvarValueStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EQueryCvarValueStatus::ValueIntact => "eQueryCvarValueStatus_ValueIntact",
            EQueryCvarValueStatus::CvarNotFound => "eQueryCvarValueStatus_CvarNotFound",
            EQueryCvarValueStatus::NotACvar => "eQueryCvarValueStatus_NotACvar",
            EQueryCvarValueStatus::CvarProtected => "eQueryCvarValueStatus_CvarProtected",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "eQueryCvarValueStatus_ValueIntact" => Some(Self::ValueIntact),
            "eQueryCvarValueStatus_CvarNotFound" => Some(Self::CvarNotFound),
            "eQueryCvarValueStatus_NotACvar" => Some(Self::NotACvar),
            "eQueryCvarValueStatus_CvarProtected" => Some(Self::CvarProtected),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DialogType {
    DialogMsg = 0,
    DialogMenu = 1,
    DialogText = 2,
    DialogEntry = 3,
    DialogAskconnect = 4,
}
impl DialogType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DialogType::DialogMsg => "DIALOG_MSG",
            DialogType::DialogMenu => "DIALOG_MENU",
            DialogType::DialogText => "DIALOG_TEXT",
            DialogType::DialogEntry => "DIALOG_ENTRY",
            DialogType::DialogAskconnect => "DIALOG_ASKCONNECT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DIALOG_MSG" => Some(Self::DialogMsg),
            "DIALOG_MENU" => Some(Self::DialogMenu),
            "DIALOG_TEXT" => Some(Self::DialogText),
            "DIALOG_ENTRY" => Some(Self::DialogEntry),
            "DIALOG_ASKCONNECT" => Some(Self::DialogAskconnect),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SvcMessagesLowFrequency {
    SvcDummy = 600,
}
impl SvcMessagesLowFrequency {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SvcMessagesLowFrequency::SvcDummy => "svc_dummy",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "svc_dummy" => Some(Self::SvcDummy),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BidirectionalMessages {
    BiRebroadcastGameEvent = 16,
    BiRebroadcastSource = 17,
    BiGameEvent = 18,
}
impl BidirectionalMessages {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BidirectionalMessages::BiRebroadcastGameEvent => "bi_RebroadcastGameEvent",
            BidirectionalMessages::BiRebroadcastSource => "bi_RebroadcastSource",
            BidirectionalMessages::BiGameEvent => "bi_GameEvent",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "bi_RebroadcastGameEvent" => Some(Self::BiRebroadcastGameEvent),
            "bi_RebroadcastSource" => Some(Self::BiRebroadcastSource),
            "bi_GameEvent" => Some(Self::BiGameEvent),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BidirectionalMessagesLowFrequency {
    BiRelayInfo = 700,
    BiRelayPacket = 701,
}
impl BidirectionalMessagesLowFrequency {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BidirectionalMessagesLowFrequency::BiRelayInfo => "bi_RelayInfo",
            BidirectionalMessagesLowFrequency::BiRelayPacket => "bi_RelayPacket",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "bi_RelayInfo" => Some(Self::BiRelayInfo),
            "bi_RelayPacket" => Some(Self::BiRelayPacket),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReplayEventTypeT {
    ReplayEventCancel = 0,
    ReplayEventDeath = 1,
    ReplayEventGeneric = 2,
    ReplayEventStuckNeedFullUpdate = 3,
    ReplayEventVictory = 4,
}
impl ReplayEventTypeT {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReplayEventTypeT::ReplayEventCancel => "REPLAY_EVENT_CANCEL",
            ReplayEventTypeT::ReplayEventDeath => "REPLAY_EVENT_DEATH",
            ReplayEventTypeT::ReplayEventGeneric => "REPLAY_EVENT_GENERIC",
            ReplayEventTypeT::ReplayEventStuckNeedFullUpdate => {
                "REPLAY_EVENT_STUCK_NEED_FULL_UPDATE"
            }
            ReplayEventTypeT::ReplayEventVictory => "REPLAY_EVENT_VICTORY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REPLAY_EVENT_CANCEL" => Some(Self::ReplayEventCancel),
            "REPLAY_EVENT_DEATH" => Some(Self::ReplayEventDeath),
            "REPLAY_EVENT_GENERIC" => Some(Self::ReplayEventGeneric),
            "REPLAY_EVENT_STUCK_NEED_FULL_UPDATE" => {
                Some(Self::ReplayEventStuckNeedFullUpdate)
            }
            "REPLAY_EVENT_VICTORY" => Some(Self::ReplayEventVictory),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageAchievementEvent {
    #[prost(uint32, optional, tag = "1")]
    pub achievement: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageCurrentTimescale {
    #[prost(float, optional, tag = "1")]
    pub current: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageShakeDir {
    #[prost(message, optional, tag = "1")]
    pub shake: ::core::option::Option<CUserMessageShake>,
    #[prost(message, optional, tag = "2")]
    pub direction: ::core::option::Option<CMsgVector>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageHudText {
    #[prost(string, optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageTextMsg {
    #[prost(uint32, optional, tag = "1")]
    pub dest: ::core::option::Option<u32>,
    #[prost(string, repeated, tag = "2")]
    pub param: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageGameTitle {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageResetHud {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageSendAudio {
    #[prost(string, optional, tag = "1")]
    pub soundname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub stop: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageRequestState {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageRumble {
    #[prost(int32, optional, tag = "1")]
    pub index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub data: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub flags: ::core::option::Option<i32>,
}
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageItemPickup {
    #[prost(string, optional, tag = "1")]
    pub itemname: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageAmmoDenied {
    #[prost(uint32, optional, tag = "1")]
    pub ammo_id: ::core::option::Option<u32>,
}
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageCreditsMsg {
    #[prost(enumeration = "ERollType", optional, tag = "1", default = "RollNone")]
    pub rolltype: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub logo_length: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CEntityMessagePlayJingle {
    #[prost(message, optional, tag = "1")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CEntityMessageScreenOverlay {
    #[prost(bool, optional, tag = "1")]
    pub start_effect: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CEntityMessageRemoveAllDecals {
    #[prost(bool, optional, tag = "1")]
    pub remove_decals: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CEntityMessagePropagateForce {
    #[prost(message, optional, tag = "1")]
    pub impulse: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "2")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CEntityMessageFixAngle {
    #[prost(bool, optional, tag = "1")]
    pub relative: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub angle: ::core::option::Option<CMsgQAngle>,
    #[prost(message, optional, tag = "3")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageCameraTransition {
    #[prost(uint32, optional, tag = "1")]
    pub camera_type: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
    #[prost(message, optional, tag = "3")]
    pub params_data_driven: ::core::option::Option<
        c_user_message_camera_transition::TransitionDataDriven,
    >,
}
/// Nested message and enum types in `CUserMessageCameraTransition`.
pub mod c_user_message_camera_transition {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMsgParticleManager {
    #[prost(
        enumeration = "ParticleMessage",
        required,
        tag = "1",
        default = "GameParticleManagerEventCreate"
    )]
    pub r#type: i32,
    #[prost(uint32, required, tag = "2")]
    pub index: u32,
    #[prost(message, optional, tag = "3")]
    pub release_particle_index: ::core::option::Option<
        c_user_msg_particle_manager::ReleaseParticleIndex,
    >,
    #[prost(message, optional, tag = "4")]
    pub create_particle: ::core::option::Option<
        c_user_msg_particle_manager::CreateParticle,
    >,
    #[prost(message, optional, tag = "5")]
    pub destroy_particle: ::core::option::Option<
        c_user_msg_particle_manager::DestroyParticle,
    >,
    #[prost(message, optional, tag = "6")]
    pub destroy_particle_involving: ::core::option::Option<
        c_user_msg_particle_manager::DestroyParticleInvolving,
    >,
    #[prost(message, optional, tag = "7")]
    pub update_particle: ::core::option::Option<
        c_user_msg_particle_manager::UpdateParticleObsolete,
    >,
    #[prost(message, optional, tag = "8")]
    pub update_particle_fwd: ::core::option::Option<
        c_user_msg_particle_manager::UpdateParticleFwdObsolete,
    >,
    #[prost(message, optional, tag = "9")]
    pub update_particle_orient: ::core::option::Option<
        c_user_msg_particle_manager::UpdateParticleOrientObsolete,
    >,
    #[prost(message, optional, tag = "10")]
    pub update_particle_fallback: ::core::option::Option<
        c_user_msg_particle_manager::UpdateParticleFallback,
    >,
    #[prost(message, optional, tag = "11")]
    pub update_particle_offset: ::core::option::Option<
        c_user_msg_particle_manager::UpdateParticleOffset,
    >,
    #[prost(message, optional, tag = "12")]
    pub update_particle_ent: ::core::option::Option<
        c_user_msg_particle_manager::UpdateParticleEnt,
    >,
    #[prost(message, optional, tag = "14")]
    pub update_particle_should_draw: ::core::option::Option<
        c_user_msg_particle_manager::UpdateParticleShouldDraw,
    >,
    #[prost(message, optional, tag = "15")]
    pub update_particle_set_frozen: ::core::option::Option<
        c_user_msg_particle_manager::UpdateParticleSetFrozen,
    >,
    #[prost(message, optional, tag = "16")]
    pub change_control_point_attachment: ::core::option::Option<
        c_user_msg_particle_manager::ChangeControlPointAttachment,
    >,
    #[prost(message, optional, tag = "17")]
    pub update_entity_position: ::core::option::Option<
        c_user_msg_particle_manager::UpdateEntityPosition,
    >,
    #[prost(message, optional, tag = "18")]
    pub set_particle_fow_properties: ::core::option::Option<
        c_user_msg_particle_manager::SetParticleFoWProperties,
    >,
    #[prost(message, optional, tag = "19")]
    pub set_particle_text: ::core::option::Option<
        c_user_msg_particle_manager::SetParticleText,
    >,
    #[prost(message, optional, tag = "20")]
    pub set_particle_should_check_fow: ::core::option::Option<
        c_user_msg_particle_manager::SetParticleShouldCheckFoW,
    >,
    #[prost(message, optional, tag = "21")]
    pub set_control_point_model: ::core::option::Option<
        c_user_msg_particle_manager::SetControlPointModel,
    >,
    #[prost(message, optional, tag = "22")]
    pub set_control_point_snapshot: ::core::option::Option<
        c_user_msg_particle_manager::SetControlPointSnapshot,
    >,
    #[prost(message, optional, tag = "23")]
    pub set_texture_attribute: ::core::option::Option<
        c_user_msg_particle_manager::SetTextureAttribute,
    >,
    #[prost(message, optional, tag = "24")]
    pub set_scene_object_generic_flag: ::core::option::Option<
        c_user_msg_particle_manager::SetSceneObjectGenericFlag,
    >,
    #[prost(message, optional, tag = "25")]
    pub set_scene_object_tint_and_desat: ::core::option::Option<
        c_user_msg_particle_manager::SetSceneObjectTintAndDesat,
    >,
    #[prost(message, optional, tag = "26")]
    pub destroy_particle_named: ::core::option::Option<
        c_user_msg_particle_manager::DestroyParticleNamed,
    >,
    #[prost(message, optional, tag = "27")]
    pub particle_skip_to_time: ::core::option::Option<
        c_user_msg_particle_manager::ParticleSkipToTime,
    >,
    #[prost(message, optional, tag = "28")]
    pub particle_can_freeze: ::core::option::Option<
        c_user_msg_particle_manager::ParticleCanFreeze,
    >,
    #[prost(message, optional, tag = "29")]
    pub set_named_value_context: ::core::option::Option<
        c_user_msg_particle_manager::SetParticleNamedValueContext,
    >,
    #[prost(message, optional, tag = "30")]
    pub update_particle_transform: ::core::option::Option<
        c_user_msg_particle_manager::UpdateParticleTransform,
    >,
    #[prost(message, optional, tag = "31")]
    pub particle_freeze_transition_override: ::core::option::Option<
        c_user_msg_particle_manager::ParticleFreezeTransitionOverride,
    >,
    #[prost(message, optional, tag = "32")]
    pub freeze_particle_involving: ::core::option::Option<
        c_user_msg_particle_manager::FreezeParticleInvolving,
    >,
}
/// Nested message and enum types in `CUserMsg_ParticleManager`.
pub mod c_user_msg_particle_manager {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReleaseParticleIndex {}
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
        pub control_point_configuration: ::core::option::Option<
            ::prost::alloc::string::String,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DestroyParticle {
        #[prost(bool, optional, tag = "1")]
        pub destroy_immediately: ::core::option::Option<bool>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DestroyParticleInvolving {
        #[prost(bool, optional, tag = "1")]
        pub destroy_immediately: ::core::option::Option<bool>,
        #[prost(uint32, optional, tag = "3", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
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
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateParticleObsolete {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateParticleFwdObsolete {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub forward: ::core::option::Option<super::CMsgVector>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
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
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
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
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateParticleFallback {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateParticleOffset {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub origin_offset: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "3")]
        pub angle_offset: ::core::option::Option<super::CMsgQAngle>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
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
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateParticleSetFrozen {
        #[prost(bool, optional, tag = "1")]
        pub set_frozen: ::core::option::Option<bool>,
        #[prost(float, optional, tag = "2")]
        pub transition_duration: ::core::option::Option<f32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateParticleShouldDraw {
        #[prost(bool, optional, tag = "1")]
        pub should_draw: ::core::option::Option<bool>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChangeControlPointAttachment {
        #[prost(int32, optional, tag = "1")]
        pub attachment_old: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub attachment_new: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "3", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateEntityPosition {
        #[prost(uint32, optional, tag = "1", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetParticleFoWProperties {
        #[prost(int32, optional, tag = "1")]
        pub fow_control_point: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub fow_control_point2: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "3")]
        pub fow_radius: ::core::option::Option<f32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetParticleShouldCheckFoW {
        #[prost(bool, optional, tag = "1")]
        pub check_fow: ::core::option::Option<bool>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetControlPointModel {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub model_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetControlPointSnapshot {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub snapshot_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetParticleText {
        #[prost(string, optional, tag = "1")]
        pub text: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetTextureAttribute {
        #[prost(string, optional, tag = "1")]
        pub attribute_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub texture_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetSceneObjectGenericFlag {
        #[prost(bool, optional, tag = "1")]
        pub flag_value: ::core::option::Option<bool>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetSceneObjectTintAndDesat {
        #[prost(fixed32, optional, tag = "1")]
        pub tint: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "2")]
        pub desat: ::core::option::Option<f32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ParticleSkipToTime {
        #[prost(float, optional, tag = "1")]
        pub skip_to_time: ::core::option::Option<f32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ParticleCanFreeze {
        #[prost(bool, optional, tag = "1")]
        pub can_freeze: ::core::option::Option<bool>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ParticleFreezeTransitionOverride {
        #[prost(float, optional, tag = "1")]
        pub freeze_transition_override: ::core::option::Option<f32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FreezeParticleInvolving {
        #[prost(bool, optional, tag = "1")]
        pub set_frozen: ::core::option::Option<bool>,
        #[prost(float, optional, tag = "2")]
        pub transition_duration: ::core::option::Option<f32>,
        #[prost(uint32, optional, tag = "3", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetParticleNamedValueContext {
        #[prost(message, repeated, tag = "1")]
        pub float_values: ::prost::alloc::vec::Vec<
            set_particle_named_value_context::FloatContextValue,
        >,
        #[prost(message, repeated, tag = "2")]
        pub vector_values: ::prost::alloc::vec::Vec<
            set_particle_named_value_context::VectorContextValue,
        >,
        #[prost(message, repeated, tag = "3")]
        pub transform_values: ::prost::alloc::vec::Vec<
            set_particle_named_value_context::TransformContextValue,
        >,
        #[prost(message, repeated, tag = "4")]
        pub ehandle_values: ::prost::alloc::vec::Vec<
            set_particle_named_value_context::EHandleContext,
        >,
    }
    /// Nested message and enum types in `SetParticleNamedValueContext`.
    pub mod set_particle_named_value_context {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FloatContextValue {
            #[prost(uint32, optional, tag = "1")]
            pub value_name_hash: ::core::option::Option<u32>,
            #[prost(float, optional, tag = "2")]
            pub value: ::core::option::Option<f32>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct VectorContextValue {
            #[prost(uint32, optional, tag = "1")]
            pub value_name_hash: ::core::option::Option<u32>,
            #[prost(message, optional, tag = "2")]
            pub value: ::core::option::Option<super::super::CMsgVector>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TransformContextValue {
            #[prost(uint32, optional, tag = "1")]
            pub value_name_hash: ::core::option::Option<u32>,
            #[prost(message, optional, tag = "2")]
            pub angles: ::core::option::Option<super::super::CMsgQAngle>,
            #[prost(message, optional, tag = "3")]
            pub translation: ::core::option::Option<super::super::CMsgVector>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct EHandleContext {
            #[prost(uint32, optional, tag = "1")]
            pub value_name_hash: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "2", default = "16777215")]
            pub ent_index: ::core::option::Option<u32>,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMsgHudError {
    #[prost(int32, optional, tag = "1")]
    pub order_id: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMsgCustomGameEvent {
    #[prost(string, optional, tag = "1")]
    pub event_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageHapticsManagerEffect {
    #[prost(int32, optional, tag = "1")]
    pub hand_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub effect_name_hash_code: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "3")]
    pub effect_scale: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageAnimStateGraphState {
    #[prost(int32, optional, tag = "1")]
    pub entity_index: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageCommandQueueState {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub player_slot: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub command_queue_info: ::core::option::Option<
        c_user_message_command_queue_state::CommandQueueInfoT,
    >,
}
/// Nested message and enum types in `CUserMessageCommandQueueState`.
pub mod c_user_message_command_queue_state {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommandQueueInfoT {
        #[prost(uint32, optional, tag = "1")]
        pub commands_queued: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "2")]
        pub command_queue_desired_size: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "3")]
        pub starved_command_ticks: ::core::option::Option<u32>,
        #[prost(int32, optional, tag = "4")]
        pub time_dilation_percent: ::core::option::Option<i32>,
    }
}
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageServerFrameTime {
    #[prost(float, optional, tag = "1")]
    pub frame_time: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageLagCompensationError {
    #[prost(float, optional, tag = "1")]
    pub distance: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageRequestDllStatus {
    #[prost(string, optional, tag = "1")]
    pub dll_action: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub full_report: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
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
    pub itemdetails: ::prost::alloc::vec::Vec<
        c_user_message_util_msg_response::ItemDetail,
    >,
    #[prost(int32, optional, tag = "10")]
    pub itemgroup: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub total_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub total_count2: ::core::option::Option<i32>,
}
/// Nested message and enum types in `CUserMessage_UtilMsg_Response`.
pub mod c_user_message_util_msg_response {
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
}
/// Nested message and enum types in `CUserMessage_DllStatus`.
pub mod c_user_message_dll_status {
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
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageRequestInventory {
    #[prost(int32, optional, tag = "1")]
    pub inventory: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub offset: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub options: ::core::option::Option<i32>,
}
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
    pub inventories: ::prost::alloc::vec::Vec<
        c_user_message_inventory_response::InventoryDetail,
    >,
    #[prost(message, repeated, tag = "10")]
    pub inventories2: ::prost::alloc::vec::Vec<
        c_user_message_inventory_response::InventoryDetail,
    >,
    #[prost(message, repeated, tag = "14")]
    pub inventories3: ::prost::alloc::vec::Vec<
        c_user_message_inventory_response::InventoryDetail,
    >,
    #[prost(int32, optional, tag = "11")]
    pub inv_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub build_version: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub instance: ::core::option::Option<i32>,
}
/// Nested message and enum types in `CUserMessage_Inventory_Response`.
pub mod c_user_message_inventory_response {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageRequestDiagnostic {
    #[prost(message, repeated, tag = "1")]
    pub diagnostics: ::prost::alloc::vec::Vec<
        c_user_message_request_diagnostic::Diagnostic,
    >,
}
/// Nested message and enum types in `CUserMessageRequestDiagnostic`.
pub mod c_user_message_request_diagnostic {
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
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageDiagnosticResponse {
    #[prost(message, repeated, tag = "1")]
    pub diagnostics: ::prost::alloc::vec::Vec<
        c_user_message_diagnostic_response::Diagnostic,
    >,
    #[prost(int32, optional, tag = "2")]
    pub build_version: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub instance: ::core::option::Option<i32>,
}
/// Nested message and enum types in `CUserMessage_Diagnostic_Response`.
pub mod c_user_message_diagnostic_response {
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
    }
}
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    UmMaxBase = 200,
}
impl EBaseUserMessages {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EBaseUserMessages::UmAchievementEvent => "UM_AchievementEvent",
            EBaseUserMessages::UmCloseCaption => "UM_CloseCaption",
            EBaseUserMessages::UmCloseCaptionDirect => "UM_CloseCaptionDirect",
            EBaseUserMessages::UmCurrentTimescale => "UM_CurrentTimescale",
            EBaseUserMessages::UmDesiredTimescale => "UM_DesiredTimescale",
            EBaseUserMessages::UmFade => "UM_Fade",
            EBaseUserMessages::UmGameTitle => "UM_GameTitle",
            EBaseUserMessages::UmHudMsg => "UM_HudMsg",
            EBaseUserMessages::UmHudText => "UM_HudText",
            EBaseUserMessages::UmColoredText => "UM_ColoredText",
            EBaseUserMessages::UmRequestState => "UM_RequestState",
            EBaseUserMessages::UmResetHud => "UM_ResetHUD",
            EBaseUserMessages::UmRumble => "UM_Rumble",
            EBaseUserMessages::UmSayText => "UM_SayText",
            EBaseUserMessages::UmSayText2 => "UM_SayText2",
            EBaseUserMessages::UmSayTextChannel => "UM_SayTextChannel",
            EBaseUserMessages::UmShake => "UM_Shake",
            EBaseUserMessages::UmShakeDir => "UM_ShakeDir",
            EBaseUserMessages::UmTextMsg => "UM_TextMsg",
            EBaseUserMessages::UmScreenTilt => "UM_ScreenTilt",
            EBaseUserMessages::UmVoiceMask => "UM_VoiceMask",
            EBaseUserMessages::UmSendAudio => "UM_SendAudio",
            EBaseUserMessages::UmItemPickup => "UM_ItemPickup",
            EBaseUserMessages::UmAmmoDenied => "UM_AmmoDenied",
            EBaseUserMessages::UmShowMenu => "UM_ShowMenu",
            EBaseUserMessages::UmCreditsMsg => "UM_CreditsMsg",
            EBaseUserMessages::UmCloseCaptionPlaceholder => "UM_CloseCaptionPlaceholder",
            EBaseUserMessages::UmCameraTransition => "UM_CameraTransition",
            EBaseUserMessages::UmAudioParameter => "UM_AudioParameter",
            EBaseUserMessages::UmParticleManager => "UM_ParticleManager",
            EBaseUserMessages::UmHudError => "UM_HudError",
            EBaseUserMessages::UmCustomGameEvent => "UM_CustomGameEvent",
            EBaseUserMessages::UmAnimGraphUpdate => "UM_AnimGraphUpdate",
            EBaseUserMessages::UmHapticsManagerPulse => "UM_HapticsManagerPulse",
            EBaseUserMessages::UmHapticsManagerEffect => "UM_HapticsManagerEffect",
            EBaseUserMessages::UmCommandQueueState => "UM_CommandQueueState",
            EBaseUserMessages::UmUpdateCssClasses => "UM_UpdateCssClasses",
            EBaseUserMessages::UmServerFrameTime => "UM_ServerFrameTime",
            EBaseUserMessages::UmLagCompensationError => "UM_LagCompensationError",
            EBaseUserMessages::UmRequestDllStatus => "UM_RequestDllStatus",
            EBaseUserMessages::UmRequestUtilAction => "UM_RequestUtilAction",
            EBaseUserMessages::UmUtilActionResponse => "UM_UtilActionResponse",
            EBaseUserMessages::UmDllStatusResponse => "UM_DllStatusResponse",
            EBaseUserMessages::UmRequestInventory => "UM_RequestInventory",
            EBaseUserMessages::UmInventoryResponse => "UM_InventoryResponse",
            EBaseUserMessages::UmRequestDiagnostic => "UM_RequestDiagnostic",
            EBaseUserMessages::UmDiagnosticResponse => "UM_DiagnosticResponse",
            EBaseUserMessages::UmExtraUserData => "UM_ExtraUserData",
            EBaseUserMessages::UmMaxBase => "UM_MAX_BASE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UM_AchievementEvent" => Some(Self::UmAchievementEvent),
            "UM_CloseCaption" => Some(Self::UmCloseCaption),
            "UM_CloseCaptionDirect" => Some(Self::UmCloseCaptionDirect),
            "UM_CurrentTimescale" => Some(Self::UmCurrentTimescale),
            "UM_DesiredTimescale" => Some(Self::UmDesiredTimescale),
            "UM_Fade" => Some(Self::UmFade),
            "UM_GameTitle" => Some(Self::UmGameTitle),
            "UM_HudMsg" => Some(Self::UmHudMsg),
            "UM_HudText" => Some(Self::UmHudText),
            "UM_ColoredText" => Some(Self::UmColoredText),
            "UM_RequestState" => Some(Self::UmRequestState),
            "UM_ResetHUD" => Some(Self::UmResetHud),
            "UM_Rumble" => Some(Self::UmRumble),
            "UM_SayText" => Some(Self::UmSayText),
            "UM_SayText2" => Some(Self::UmSayText2),
            "UM_SayTextChannel" => Some(Self::UmSayTextChannel),
            "UM_Shake" => Some(Self::UmShake),
            "UM_ShakeDir" => Some(Self::UmShakeDir),
            "UM_TextMsg" => Some(Self::UmTextMsg),
            "UM_ScreenTilt" => Some(Self::UmScreenTilt),
            "UM_VoiceMask" => Some(Self::UmVoiceMask),
            "UM_SendAudio" => Some(Self::UmSendAudio),
            "UM_ItemPickup" => Some(Self::UmItemPickup),
            "UM_AmmoDenied" => Some(Self::UmAmmoDenied),
            "UM_ShowMenu" => Some(Self::UmShowMenu),
            "UM_CreditsMsg" => Some(Self::UmCreditsMsg),
            "UM_CloseCaptionPlaceholder" => Some(Self::UmCloseCaptionPlaceholder),
            "UM_CameraTransition" => Some(Self::UmCameraTransition),
            "UM_AudioParameter" => Some(Self::UmAudioParameter),
            "UM_ParticleManager" => Some(Self::UmParticleManager),
            "UM_HudError" => Some(Self::UmHudError),
            "UM_CustomGameEvent" => Some(Self::UmCustomGameEvent),
            "UM_AnimGraphUpdate" => Some(Self::UmAnimGraphUpdate),
            "UM_HapticsManagerPulse" => Some(Self::UmHapticsManagerPulse),
            "UM_HapticsManagerEffect" => Some(Self::UmHapticsManagerEffect),
            "UM_CommandQueueState" => Some(Self::UmCommandQueueState),
            "UM_UpdateCssClasses" => Some(Self::UmUpdateCssClasses),
            "UM_ServerFrameTime" => Some(Self::UmServerFrameTime),
            "UM_LagCompensationError" => Some(Self::UmLagCompensationError),
            "UM_RequestDllStatus" => Some(Self::UmRequestDllStatus),
            "UM_RequestUtilAction" => Some(Self::UmRequestUtilAction),
            "UM_UtilActionResponse" => Some(Self::UmUtilActionResponse),
            "UM_DllStatusResponse" => Some(Self::UmDllStatusResponse),
            "UM_RequestInventory" => Some(Self::UmRequestInventory),
            "UM_InventoryResponse" => Some(Self::UmInventoryResponse),
            "UM_RequestDiagnostic" => Some(Self::UmRequestDiagnostic),
            "UM_DiagnosticResponse" => Some(Self::UmDiagnosticResponse),
            "UM_ExtraUserData" => Some(Self::UmExtraUserData),
            "UM_MAX_BASE" => Some(Self::UmMaxBase),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EBaseEntityMessages {
    EmPlayJingle = 136,
    EmScreenOverlay = 137,
    EmRemoveAllDecals = 138,
    EmPropagateForce = 139,
    EmDoSpark = 140,
    EmFixAngle = 141,
}
impl EBaseEntityMessages {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EBaseEntityMessages::EmPlayJingle => "EM_PlayJingle",
            EBaseEntityMessages::EmScreenOverlay => "EM_ScreenOverlay",
            EBaseEntityMessages::EmRemoveAllDecals => "EM_RemoveAllDecals",
            EBaseEntityMessages::EmPropagateForce => "EM_PropagateForce",
            EBaseEntityMessages::EmDoSpark => "EM_DoSpark",
            EBaseEntityMessages::EmFixAngle => "EM_FixAngle",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EM_PlayJingle" => Some(Self::EmPlayJingle),
            "EM_ScreenOverlay" => Some(Self::EmScreenOverlay),
            "EM_RemoveAllDecals" => Some(Self::EmRemoveAllDecals),
            "EM_PropagateForce" => Some(Self::EmPropagateForce),
            "EM_DoSpark" => Some(Self::EmDoSpark),
            "EM_FixAngle" => Some(Self::EmFixAngle),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ERollType {
    RollNone = -1,
    RollStats = 0,
    RollCredits = 1,
    RollLateJoinLogo = 2,
    RollOuttro = 3,
}
impl ERollType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ERollType::RollNone => "ROLL_NONE",
            ERollType::RollStats => "ROLL_STATS",
            ERollType::RollCredits => "ROLL_CREDITS",
            ERollType::RollLateJoinLogo => "ROLL_LATE_JOIN_LOGO",
            ERollType::RollOuttro => "ROLL_OUTTRO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ROLL_NONE" => Some(Self::RollNone),
            "ROLL_STATS" => Some(Self::RollStats),
            "ROLL_CREDITS" => Some(Self::RollCredits),
            "ROLL_LATE_JOIN_LOGO" => Some(Self::RollLateJoinLogo),
            "ROLL_OUTTRO" => Some(Self::RollOuttro),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
impl ParticleMessage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ParticleMessage::GameParticleManagerEventCreate => {
                "GAME_PARTICLE_MANAGER_EVENT_CREATE"
            }
            ParticleMessage::GameParticleManagerEventUpdate => {
                "GAME_PARTICLE_MANAGER_EVENT_UPDATE"
            }
            ParticleMessage::GameParticleManagerEventUpdateForward => {
                "GAME_PARTICLE_MANAGER_EVENT_UPDATE_FORWARD"
            }
            ParticleMessage::GameParticleManagerEventUpdateOrientation => {
                "GAME_PARTICLE_MANAGER_EVENT_UPDATE_ORIENTATION"
            }
            ParticleMessage::GameParticleManagerEventUpdateFallback => {
                "GAME_PARTICLE_MANAGER_EVENT_UPDATE_FALLBACK"
            }
            ParticleMessage::GameParticleManagerEventUpdateEnt => {
                "GAME_PARTICLE_MANAGER_EVENT_UPDATE_ENT"
            }
            ParticleMessage::GameParticleManagerEventUpdateOffset => {
                "GAME_PARTICLE_MANAGER_EVENT_UPDATE_OFFSET"
            }
            ParticleMessage::GameParticleManagerEventDestroy => {
                "GAME_PARTICLE_MANAGER_EVENT_DESTROY"
            }
            ParticleMessage::GameParticleManagerEventDestroyInvolving => {
                "GAME_PARTICLE_MANAGER_EVENT_DESTROY_INVOLVING"
            }
            ParticleMessage::GameParticleManagerEventRelease => {
                "GAME_PARTICLE_MANAGER_EVENT_RELEASE"
            }
            ParticleMessage::GameParticleManagerEventLatency => {
                "GAME_PARTICLE_MANAGER_EVENT_LATENCY"
            }
            ParticleMessage::GameParticleManagerEventShouldDraw => {
                "GAME_PARTICLE_MANAGER_EVENT_SHOULD_DRAW"
            }
            ParticleMessage::GameParticleManagerEventFrozen => {
                "GAME_PARTICLE_MANAGER_EVENT_FROZEN"
            }
            ParticleMessage::GameParticleManagerEventChangeControlPointAttachment => {
                "GAME_PARTICLE_MANAGER_EVENT_CHANGE_CONTROL_POINT_ATTACHMENT"
            }
            ParticleMessage::GameParticleManagerEventUpdateEntityPosition => {
                "GAME_PARTICLE_MANAGER_EVENT_UPDATE_ENTITY_POSITION"
            }
            ParticleMessage::GameParticleManagerEventSetFowProperties => {
                "GAME_PARTICLE_MANAGER_EVENT_SET_FOW_PROPERTIES"
            }
            ParticleMessage::GameParticleManagerEventSetText => {
                "GAME_PARTICLE_MANAGER_EVENT_SET_TEXT"
            }
            ParticleMessage::GameParticleManagerEventSetShouldCheckFow => {
                "GAME_PARTICLE_MANAGER_EVENT_SET_SHOULD_CHECK_FOW"
            }
            ParticleMessage::GameParticleManagerEventSetControlPointModel => {
                "GAME_PARTICLE_MANAGER_EVENT_SET_CONTROL_POINT_MODEL"
            }
            ParticleMessage::GameParticleManagerEventSetControlPointSnapshot => {
                "GAME_PARTICLE_MANAGER_EVENT_SET_CONTROL_POINT_SNAPSHOT"
            }
            ParticleMessage::GameParticleManagerEventSetTextureAttribute => {
                "GAME_PARTICLE_MANAGER_EVENT_SET_TEXTURE_ATTRIBUTE"
            }
            ParticleMessage::GameParticleManagerEventSetSceneObjectGenericFlag => {
                "GAME_PARTICLE_MANAGER_EVENT_SET_SCENE_OBJECT_GENERIC_FLAG"
            }
            ParticleMessage::GameParticleManagerEventSetSceneObjectTintAndDesat => {
                "GAME_PARTICLE_MANAGER_EVENT_SET_SCENE_OBJECT_TINT_AND_DESAT"
            }
            ParticleMessage::GameParticleManagerEventDestroyNamed => {
                "GAME_PARTICLE_MANAGER_EVENT_DESTROY_NAMED"
            }
            ParticleMessage::GameParticleManagerEventSkipToTime => {
                "GAME_PARTICLE_MANAGER_EVENT_SKIP_TO_TIME"
            }
            ParticleMessage::GameParticleManagerEventCanFreeze => {
                "GAME_PARTICLE_MANAGER_EVENT_CAN_FREEZE"
            }
            ParticleMessage::GameParticleManagerEventSetNamedValueContext => {
                "GAME_PARTICLE_MANAGER_EVENT_SET_NAMED_VALUE_CONTEXT"
            }
            ParticleMessage::GameParticleManagerEventUpdateTransform => {
                "GAME_PARTICLE_MANAGER_EVENT_UPDATE_TRANSFORM"
            }
            ParticleMessage::GameParticleManagerEventFreezeTransitionOverride => {
                "GAME_PARTICLE_MANAGER_EVENT_FREEZE_TRANSITION_OVERRIDE"
            }
            ParticleMessage::GameParticleManagerEventFreezeInvolving => {
                "GAME_PARTICLE_MANAGER_EVENT_FREEZE_INVOLVING"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GAME_PARTICLE_MANAGER_EVENT_CREATE" => {
                Some(Self::GameParticleManagerEventCreate)
            }
            "GAME_PARTICLE_MANAGER_EVENT_UPDATE" => {
                Some(Self::GameParticleManagerEventUpdate)
            }
            "GAME_PARTICLE_MANAGER_EVENT_UPDATE_FORWARD" => {
                Some(Self::GameParticleManagerEventUpdateForward)
            }
            "GAME_PARTICLE_MANAGER_EVENT_UPDATE_ORIENTATION" => {
                Some(Self::GameParticleManagerEventUpdateOrientation)
            }
            "GAME_PARTICLE_MANAGER_EVENT_UPDATE_FALLBACK" => {
                Some(Self::GameParticleManagerEventUpdateFallback)
            }
            "GAME_PARTICLE_MANAGER_EVENT_UPDATE_ENT" => {
                Some(Self::GameParticleManagerEventUpdateEnt)
            }
            "GAME_PARTICLE_MANAGER_EVENT_UPDATE_OFFSET" => {
                Some(Self::GameParticleManagerEventUpdateOffset)
            }
            "GAME_PARTICLE_MANAGER_EVENT_DESTROY" => {
                Some(Self::GameParticleManagerEventDestroy)
            }
            "GAME_PARTICLE_MANAGER_EVENT_DESTROY_INVOLVING" => {
                Some(Self::GameParticleManagerEventDestroyInvolving)
            }
            "GAME_PARTICLE_MANAGER_EVENT_RELEASE" => {
                Some(Self::GameParticleManagerEventRelease)
            }
            "GAME_PARTICLE_MANAGER_EVENT_LATENCY" => {
                Some(Self::GameParticleManagerEventLatency)
            }
            "GAME_PARTICLE_MANAGER_EVENT_SHOULD_DRAW" => {
                Some(Self::GameParticleManagerEventShouldDraw)
            }
            "GAME_PARTICLE_MANAGER_EVENT_FROZEN" => {
                Some(Self::GameParticleManagerEventFrozen)
            }
            "GAME_PARTICLE_MANAGER_EVENT_CHANGE_CONTROL_POINT_ATTACHMENT" => {
                Some(Self::GameParticleManagerEventChangeControlPointAttachment)
            }
            "GAME_PARTICLE_MANAGER_EVENT_UPDATE_ENTITY_POSITION" => {
                Some(Self::GameParticleManagerEventUpdateEntityPosition)
            }
            "GAME_PARTICLE_MANAGER_EVENT_SET_FOW_PROPERTIES" => {
                Some(Self::GameParticleManagerEventSetFowProperties)
            }
            "GAME_PARTICLE_MANAGER_EVENT_SET_TEXT" => {
                Some(Self::GameParticleManagerEventSetText)
            }
            "GAME_PARTICLE_MANAGER_EVENT_SET_SHOULD_CHECK_FOW" => {
                Some(Self::GameParticleManagerEventSetShouldCheckFow)
            }
            "GAME_PARTICLE_MANAGER_EVENT_SET_CONTROL_POINT_MODEL" => {
                Some(Self::GameParticleManagerEventSetControlPointModel)
            }
            "GAME_PARTICLE_MANAGER_EVENT_SET_CONTROL_POINT_SNAPSHOT" => {
                Some(Self::GameParticleManagerEventSetControlPointSnapshot)
            }
            "GAME_PARTICLE_MANAGER_EVENT_SET_TEXTURE_ATTRIBUTE" => {
                Some(Self::GameParticleManagerEventSetTextureAttribute)
            }
            "GAME_PARTICLE_MANAGER_EVENT_SET_SCENE_OBJECT_GENERIC_FLAG" => {
                Some(Self::GameParticleManagerEventSetSceneObjectGenericFlag)
            }
            "GAME_PARTICLE_MANAGER_EVENT_SET_SCENE_OBJECT_TINT_AND_DESAT" => {
                Some(Self::GameParticleManagerEventSetSceneObjectTintAndDesat)
            }
            "GAME_PARTICLE_MANAGER_EVENT_DESTROY_NAMED" => {
                Some(Self::GameParticleManagerEventDestroyNamed)
            }
            "GAME_PARTICLE_MANAGER_EVENT_SKIP_TO_TIME" => {
                Some(Self::GameParticleManagerEventSkipToTime)
            }
            "GAME_PARTICLE_MANAGER_EVENT_CAN_FREEZE" => {
                Some(Self::GameParticleManagerEventCanFreeze)
            }
            "GAME_PARTICLE_MANAGER_EVENT_SET_NAMED_VALUE_CONTEXT" => {
                Some(Self::GameParticleManagerEventSetNamedValueContext)
            }
            "GAME_PARTICLE_MANAGER_EVENT_UPDATE_TRANSFORM" => {
                Some(Self::GameParticleManagerEventUpdateTransform)
            }
            "GAME_PARTICLE_MANAGER_EVENT_FREEZE_TRANSITION_OVERRIDE" => {
                Some(Self::GameParticleManagerEventFreezeTransitionOverride)
            }
            "GAME_PARTICLE_MANAGER_EVENT_FREEZE_INVOLVING" => {
                Some(Self::GameParticleManagerEventFreezeInvolving)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EHapticPulseType {
    VrHandHapticPulseLight = 0,
    VrHandHapticPulseMedium = 1,
    VrHandHapticPulseStrong = 2,
}
impl EHapticPulseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EHapticPulseType::VrHandHapticPulseLight => "VR_HAND_HAPTIC_PULSE_LIGHT",
            EHapticPulseType::VrHandHapticPulseMedium => "VR_HAND_HAPTIC_PULSE_MEDIUM",
            EHapticPulseType::VrHandHapticPulseStrong => "VR_HAND_HAPTIC_PULSE_STRONG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VR_HAND_HAPTIC_PULSE_LIGHT" => Some(Self::VrHandHapticPulseLight),
            "VR_HAND_HAPTIC_PULSE_MEDIUM" => Some(Self::VrHandHapticPulseMedium),
            "VR_HAND_HAPTIC_PULSE_STRONG" => Some(Self::VrHandHapticPulseStrong),
            _ => None,
        }
    }
}
