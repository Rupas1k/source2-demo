#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CGcSystemMsgGetAccountDetails {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub appid: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CGcSystemMsgGetAccountDetailsResponse {
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CIpLocationInfo {
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CGcMsgGetIpLocationResponse {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<CIpLocationInfo>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EgcPlatform {
    KEGcPlatformNone = 0,
    KEGcPlatformPc = 1,
    KEGcPlatformMac = 2,
    KEGcPlatformLinux = 3,
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
pub enum EProtoDebugVisiblity {
    KEProtoDebugVisibilityAlways = 0,
    KEProtoDebugVisibilityServer = 70,
    KEProtoDebugVisibilityValveServer = 80,
    KEProtoDebugVisibilityGc = 90,
    KEProtoDebugVisibilityNever = 100,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EProtoExecutionSite {
    KEProtoExecutionSiteUnknown = 0,
    KEProtoExecutionSiteSteamClient = 3,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnDataSourceDescObject {
    #[prost(message, repeated, tag = "1")]
    pub elements: ::prost::alloc::vec::Vec<CMsgSteamLearnDataSourceDescElement>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnDataObject {
    #[prost(message, repeated, tag = "1")]
    pub elements: ::prost::alloc::vec::Vec<CMsgSteamLearnDataElement>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnData {
    #[prost(uint32, optional, tag = "1")]
    pub data_source_id: ::core::option::Option<u32>,
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "3")]
    pub data_object: ::core::option::Option<CMsgSteamLearnDataObject>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnDataList {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<CMsgSteamLearnData>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnRegisterDataSourceRequest {
    #[prost(string, optional, tag = "1")]
    pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub data_source: ::core::option::Option<CMsgSteamLearnDataSource>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnRegisterDataSourceResponse {
    #[prost(
        enumeration = "ESteammLearnRegisterDataSourceResult",
        optional,
        tag = "1",
        default = "SteamlearnRegisterDataSourceResultError"
    )]
    pub result: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub data_source: ::core::option::Option<CMsgSteamLearnDataSource>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnCacheDataRequest {
    #[prost(string, optional, tag = "1")]
    pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<CMsgSteamLearnData>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSteamLearnCacheDataResponse {
    #[prost(enumeration = "ESteamLearnCacheDataResult", optional, tag = "1", default = "SteamlearnCacheDataError")]
    pub cache_data_result: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSteamLearnSnapshotProjectResponse {
    #[prost(
        enumeration = "ESteamLearnSnapshotProjectResult",
        optional,
        tag = "1",
        default = "SteamlearnSnapshotProjectError"
    )]
    pub snapshot_result: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnBatchOperationRequest {
    #[prost(message, repeated, tag = "1")]
    pub cache_data_requests: ::prost::alloc::vec::Vec<CMsgSteamLearnCacheDataRequest>,
    #[prost(message, repeated, tag = "2")]
    pub snapshot_requests: ::prost::alloc::vec::Vec<CMsgSteamLearnSnapshotProjectRequest>,
    #[prost(message, repeated, tag = "3")]
    pub inference_requests: ::prost::alloc::vec::Vec<CMsgSteamLearnInferenceRequest>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnBatchOperationResponse {
    #[prost(message, repeated, tag = "1")]
    pub cache_data_responses: ::prost::alloc::vec::Vec<CMsgSteamLearnCacheDataResponse>,
    #[prost(message, repeated, tag = "2")]
    pub snapshot_responses: ::prost::alloc::vec::Vec<CMsgSteamLearnSnapshotProjectResponse>,
    #[prost(message, repeated, tag = "3")]
    pub inference_responses: ::prost::alloc::vec::Vec<CMsgSteamLearnInferenceResponse>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
pub mod c_msg_steam_learn_access_tokens {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CacheDataAccessToken {
        #[prost(uint32, optional, tag = "1")]
        pub data_source_id: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SnapshotProjectAccessToken {
        #[prost(uint32, optional, tag = "1")]
        pub project_id: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct InferenceAccessToken {
        #[prost(uint32, optional, tag = "1")]
        pub project_id: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub access_token: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSteamLearnGetAccessTokensRequest {
    #[prost(uint32, optional, tag = "1")]
    pub appid: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnGetAccessTokensResponse {
    #[prost(
        enumeration = "ESteamLearnGetAccessTokensResult",
        optional,
        tag = "1",
        default = "SteamlearnGetAccessTokensError"
    )]
    pub result: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub access_tokens: ::core::option::Option<CMsgSteamLearnAccessTokens>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgInferenceIterateBeamSearch {
    #[prost(uint32, optional, tag = "1")]
    pub beam_length: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub beam_width: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "3")]
    pub item_decay: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "4")]
    pub next_item_count: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "5")]
    pub item_scalars: ::prost::alloc::vec::Vec<c_msg_inference_iterate_beam_search::CustomItemScalar>,
    #[prost(uint32, optional, tag = "7")]
    pub item_sequence_end: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "8")]
    pub item_sequence_end_threshold: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "9")]
    pub repeat_multiplier: ::core::option::Option<f32>,
}
pub mod c_msg_inference_iterate_beam_search {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct CustomItemScalar {
        #[prost(uint32, optional, tag = "1")]
        pub item: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "2")]
        pub scale: ::core::option::Option<f32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[prost(uint64, repeated, packed = "false", tag = "8")]
    pub keys: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, optional, tag = "9")]
    pub named_inference: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "13")]
    pub iterate_beam_search: ::core::option::Option<CMsgInferenceIterateBeamSearch>,
    #[prost(uint32, optional, tag = "14")]
    pub debug_spew: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSteamLearnInferenceMetadataBackendRequest {
    #[prost(uint32, optional, tag = "1")]
    pub project_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub fetch_id: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnInferenceMetadataResponse {
    #[prost(
        enumeration = "ESteamLearnInferenceMetadataResult",
        optional,
        tag = "1",
        default = "SteamlearnInferenceMetadataError"
    )]
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
pub mod c_msg_steam_learn_inference_metadata_response {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct RowRange {
        #[prost(uint64, optional, tag = "1")]
        pub min_row: ::core::option::Option<u64>,
        #[prost(uint64, optional, tag = "2")]
        pub max_row: ::core::option::Option<u64>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct Range {
        #[prost(string, optional, tag = "1")]
        pub data_element_path: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(float, optional, tag = "2")]
        pub min_value: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "3")]
        pub max_value: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct StdDev {
        #[prost(string, optional, tag = "1")]
        pub data_element_path: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(float, optional, tag = "2")]
        pub mean: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "3")]
        pub std_dev: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct CompactTable {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub map_values: ::prost::alloc::vec::Vec<compact_table::MapValuesEntry>,
        #[prost(message, repeated, tag = "3")]
        pub map_mappings: ::prost::alloc::vec::Vec<compact_table::MapMappingsEntry>,
    }
    pub mod compact_table {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct Entry {
            #[prost(uint32, optional, tag = "1")]
            pub value: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "2")]
            pub mapping: ::core::option::Option<u32>,
            #[prost(uint64, optional, tag = "3")]
            pub count: ::core::option::Option<u64>,
        }
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct MapValuesEntry {
            #[prost(uint32, optional, tag = "1")]
            pub key: ::core::option::Option<u32>,
            #[prost(message, optional, tag = "2")]
            pub value: ::core::option::Option<Entry>,
        }
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct MapMappingsEntry {
            #[prost(uint32, optional, tag = "1")]
            pub key: ::core::option::Option<u32>,
            #[prost(message, optional, tag = "2")]
            pub value: ::core::option::Option<Entry>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
        #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct Entry {
            #[prost(uint32, repeated, packed = "false", tag = "1")]
            pub values: ::prost::alloc::vec::Vec<u32>,
            #[prost(uint32, optional, tag = "2")]
            pub crc: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "3")]
            pub count: ::core::option::Option<u32>,
        }
        #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct MapValuesEntry {
            #[prost(uint32, optional, tag = "1")]
            pub key: ::core::option::Option<u32>,
            #[prost(message, optional, tag = "2")]
            pub value: ::core::option::Option<Entry>,
        }
        #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct MapMappingsEntry {
            #[prost(string, optional, tag = "1")]
            pub key: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(message, optional, tag = "2")]
            pub value: ::core::option::Option<Entry>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct KMeans {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub clusters: ::prost::alloc::vec::Vec<k_means::Cluster>,
    }
    pub mod k_means {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct AppInfoEntry {
        #[prost(uint32, optional, tag = "1")]
        pub key: ::core::option::Option<u32>,
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<AppInfo>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnInferenceBackendResponse {
    #[prost(message, repeated, tag = "1")]
    pub outputs: ::prost::alloc::vec::Vec<c_msg_steam_learn_inference_backend_response::Output>,
}
pub mod c_msg_steam_learn_inference_backend_response {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct Sequence {
        #[prost(float, repeated, packed = "false", tag = "1")]
        pub value: ::prost::alloc::vec::Vec<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RegressionOutput {
        #[prost(float, optional, tag = "2")]
        pub value: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct NamedInferenceOutput {
        #[prost(float, repeated, packed = "false", tag = "3")]
        pub value: ::prost::alloc::vec::Vec<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct BinaryCrossEntropyOutput {
        #[prost(float, optional, tag = "1")]
        pub value: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct MutliBinaryCrossEntropyOutput {
        #[prost(float, repeated, packed = "false", tag = "1")]
        pub weight: ::prost::alloc::vec::Vec<f32>,
        #[prost(float, repeated, packed = "false", tag = "2")]
        pub value: ::prost::alloc::vec::Vec<f32>,
        #[prost(message, repeated, tag = "3")]
        pub value_sequence: ::prost::alloc::vec::Vec<Sequence>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct CategoricalCrossEntropyOutput {
        #[prost(float, repeated, packed = "false", tag = "1")]
        pub weight: ::prost::alloc::vec::Vec<f32>,
        #[prost(float, repeated, packed = "false", tag = "2")]
        pub value: ::prost::alloc::vec::Vec<f32>,
        #[prost(message, repeated, tag = "3")]
        pub value_sequence: ::prost::alloc::vec::Vec<Sequence>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct Output {
        #[prost(oneof = "output::ResponseType", tags = "1, 2, 3, 4, 5")]
        pub response_type: ::core::option::Option<output::ResponseType>,
    }
    pub mod output {
        #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Oneof)]
        pub enum ResponseType {
            #[prost(message, tag = "1")]
            BinaryCrossentropy(super::BinaryCrossEntropyOutput),
            #[prost(message, tag = "2")]
            CategoricalCrossentropy(super::CategoricalCrossEntropyOutput),
            #[prost(message, tag = "3")]
            MultiBinaryCrossentropy(super::MutliBinaryCrossEntropyOutput),
            #[prost(message, tag = "4")]
            Regression(super::RegressionOutput),
            #[prost(message, tag = "5")]
            NamedInference(super::NamedInferenceOutput),
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnInferenceResponse {
    #[prost(enumeration = "ESteamLearnInferenceResult", optional, tag = "1", default = "SteamlearnInferenceError")]
    pub inference_result: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "2")]
    pub backend_response: ::core::option::Option<CMsgSteamLearnInferenceBackendResponse>,
    #[prost(uint64, repeated, packed = "false", tag = "3")]
    pub keys: ::prost::alloc::vec::Vec<u64>,
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
pub enum ESteamLearnGetAccessTokensResult {
    SteamlearnGetAccessTokensError = 0,
    SteamlearnGetAccessTokensSuccess = 1,
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
    SteamlearnInferenceErrorUnsuppliedDataFail = 10,
    SteamlearnInferenceErrorUnsuppliedDataNoKeys = 11,
    SteamlearnInferenceDisabled = 12,
    SteamlearnInferenceErrorNoOutput = 13,
    SteamlearnInferenceErrorInvalidNamedInference = 14,
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSteamLearnServerInfo {
    #[prost(message, optional, tag = "4")]
    pub access_tokens: ::core::option::Option<CMsgSteamLearnAccessTokens>,
    #[prost(message, repeated, tag = "5")]
    pub project_infos: ::prost::alloc::vec::Vec<c_msg_steam_learn_server_info::ProjectInfo>,
}
pub mod c_msg_steam_learn_server_info {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcAssertJobData {
    #[prost(string, optional, tag = "1")]
    pub message_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub message_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcConCommand {
    #[prost(string, optional, tag = "1")]
    pub command: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSdoAssert {
    #[prost(int32, optional, tag = "1")]
    pub sdo_type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<c_msg_sdo_assert::Request>,
}
pub mod c_msg_sdo_assert {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Request {
        #[prost(uint64, repeated, packed = "false", tag = "1")]
        pub key: ::prost::alloc::vec::Vec<u64>,
        #[prost(string, optional, tag = "2")]
        pub requesting_job: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSoidOwner {
    #[prost(uint32, optional, tag = "1")]
    pub r#type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
pub mod c_msg_so_multiple_objects {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SingleObject {
        #[prost(int32, optional, tag = "1")]
        pub type_id: ::core::option::Option<i32>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub object_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
pub mod c_msg_so_cache_subscribed {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SubscribedType {
        #[prost(int32, optional, tag = "1")]
        pub type_id: ::core::option::Option<i32>,
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub object_data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSoCacheUnsubscribed {
    #[prost(message, optional, tag = "2")]
    pub owner_soid: ::core::option::Option<CMsgSoidOwner>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSoCacheSubscriptionRefresh {
    #[prost(message, optional, tag = "2")]
    pub owner_soid: ::core::option::Option<CMsgSoidOwner>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSoCacheVersion {
    #[prost(fixed64, optional, tag = "1")]
    pub version: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcMultiplexMessage {
    #[prost(uint32, optional, tag = "1")]
    pub msgtype: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub payload: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(fixed64, repeated, packed = "false", tag = "3")]
    pub steamids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcSubGcStarting {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub dir_index: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CGcToGcMsgMasterAck {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub dir_index: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub machine_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub process_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub directory: ::prost::alloc::vec::Vec<cgc_to_gc_msg_master_ack::Process>,
}
pub mod cgc_to_gc_msg_master_ack {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Process {
        #[prost(int32, optional, tag = "1", default = "-1")]
        pub dir_index: ::core::option::Option<i32>,
        #[prost(uint32, repeated, packed = "false", tag = "2")]
        pub type_instances: ::prost::alloc::vec::Vec<u32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CGcToGcMsgMasterAckResponse {
    #[prost(int32, optional, tag = "1", default = "2")]
    pub eresult: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcUniverseStartup {
    #[prost(bool, optional, tag = "1")]
    pub is_initial_startup: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcUniverseStartupResponse {
    #[prost(int32, optional, tag = "1")]
    pub eresult: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CGcToGcMsgMasterStartupComplete {
    #[prost(message, repeated, tag = "1")]
    pub gc_info: ::prost::alloc::vec::Vec<cgc_to_gc_msg_master_startup_complete::GcInfo>,
}
pub mod cgc_to_gc_msg_master_startup_complete {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct GcInfo {
        #[prost(int32, optional, tag = "1", default = "-1")]
        pub dir_index: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub machine_name: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CGcToGcMsgRouted {
    #[prost(uint32, optional, tag = "1")]
    pub msg_type: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub sender_id: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub net_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CGcToGcMsgRoutedReply {
    #[prost(uint32, optional, tag = "1")]
    pub msg_type: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub net_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcUpdateSubGcSessionInfo {
    #[prost(message, repeated, tag = "1")]
    pub updates: ::prost::alloc::vec::Vec<c_msg_gc_update_sub_gc_session_info::CMsgUpdate>,
}
pub mod c_msg_gc_update_sub_gc_session_info {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CMsgUpdate {
        #[prost(fixed64, optional, tag = "1")]
        pub steamid: ::core::option::Option<u64>,
        #[prost(fixed32, optional, tag = "2")]
        pub ip: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "3")]
        pub trusted: ::core::option::Option<bool>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcRequestSubGcSessionInfo {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
pub mod c_msg_client_welcome {
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
pub mod c_msg_gc_to_gcso_cache_subscribe {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CMsgHaveVersions {
        #[prost(uint32, optional, tag = "1")]
        pub service_id: ::core::option::Option<u32>,
        #[prost(uint64, optional, tag = "2")]
        pub version: ::core::option::Option<u64>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcsoCacheUnsubscribe {
    #[prost(fixed64, optional, tag = "1")]
    pub subscriber: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub unsubscribe_from_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub unsubscribe_from_type: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcClientPing {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcForwardAccountDetails {
    #[prost(fixed64, optional, tag = "1")]
    pub steamid: ::core::option::Option<u64>,
    #[prost(message, optional, tag = "2")]
    pub account_details: ::core::option::Option<CGcSystemMsgGetAccountDetailsResponse>,
    #[prost(uint32, optional, tag = "3")]
    pub age_seconds: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcLoadSessionSoCache {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub forward_account_details: ::core::option::Option<CMsgGcToGcForwardAccountDetails>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcLoadSessionSoCacheResponse {}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcUpdateSessionStats {
    #[prost(uint32, optional, tag = "1")]
    pub user_sessions: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub server_sessions: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "3")]
    pub in_logon_surge: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToClientRequestDropped {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CWorkshopPopulateItemDescriptionsRequest {
    #[prost(uint32, optional, tag = "1")]
    pub appid: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub languages: ::prost::alloc::vec::Vec<c_workshop_populate_item_descriptions_request::ItemDescriptionsLanguageBlock>,
}
pub mod c_workshop_populate_item_descriptions_request {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SingleItemDescription {
        #[prost(uint32, optional, tag = "1")]
        pub gameitemid: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub item_description: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct ItemDescriptionsLanguageBlock {
        #[prost(string, optional, tag = "1")]
        pub language: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub descriptions: ::prost::alloc::vec::Vec<SingleItemDescription>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CWorkshopGetContributorsRequest {
    #[prost(uint32, optional, tag = "1")]
    pub appid: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub gameitemid: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CWorkshopGetContributorsResponse {
    #[prost(fixed64, repeated, packed = "false", tag = "1")]
    pub contributors: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
pub mod c_workshop_set_item_payment_rules_request {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct WorkshopDirectPaymentRule {
        #[prost(uint64, optional, tag = "1")]
        pub workshop_file_id: ::core::option::Option<u64>,
        #[prost(string, optional, tag = "2")]
        pub rule_description: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct PartnerItemPaymentRule {
        #[prost(uint32, optional, tag = "1")]
        pub account_id: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "2")]
        pub revenue_percentage: ::core::option::Option<f32>,
        #[prost(string, optional, tag = "3")]
        pub rule_description: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CWorkshopSetItemPaymentRulesResponse {
    #[prost(string, repeated, tag = "1")]
    pub validation_errors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CCommunityGetClanAnnouncementsResponse {
    #[prost(uint32, optional, tag = "1")]
    pub maxchars: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "2")]
    pub strip_html: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "3")]
    pub announcements: ::prost::alloc::vec::Vec<CCommunityClanAnnouncementInfo>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSerializedSoCache {
    #[prost(uint32, optional, tag = "1")]
    pub file_version: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub caches: ::prost::alloc::vec::Vec<c_msg_serialized_so_cache::Cache>,
    #[prost(uint32, optional, tag = "3")]
    pub gc_socache_file_version: ::core::option::Option<u32>,
}
pub mod c_msg_serialized_so_cache {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct TypeCache {
        #[prost(uint32, optional, tag = "1")]
        pub r#type: ::core::option::Option<u32>,
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub objects: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        #[prost(uint32, optional, tag = "3")]
        pub service_id: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct Version {
            #[prost(uint32, optional, tag = "1")]
            pub service: ::core::option::Option<u32>,
            #[prost(uint64, optional, tag = "2")]
            pub version: ::core::option::Option<u64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToClientPollConvarRequest {
    #[prost(string, optional, tag = "1")]
    pub convar_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "2")]
    pub poll_id: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToClientPollConvarResponse {
    #[prost(uint32, optional, tag = "1")]
    pub poll_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub convar_value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CGcMsgCompressedMsgToClient {
    #[prost(uint32, optional, tag = "1")]
    pub msg_id: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub compressed_msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
    #[prost(bool, optional, tag = "6")]
    pub trusted_servers_only: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcMasterSubscribeToCacheResponse {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcMasterSubscribeToCacheAsync {
    #[prost(message, optional, tag = "1")]
    pub subscribe_msg: ::core::option::Option<CMsgGcToGcMasterSubscribeToCache>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcMasterDestroyCache {
    #[prost(uint32, optional, tag = "1")]
    pub soid_type: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag = "2")]
    pub soid_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ESourceEngine {
    KEseSource1 = 0,
    KEseSource2 = 1,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PartnerAccountType {
    PartnerNone = 0,
    PartnerPerfectWorld = 1,
    PartnerInvalid = 3,
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CGcStorePurchaseInitLineItem {
    #[prost(uint32, optional, tag = "1")]
    pub item_def_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub quantity: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub cost_in_local_currency: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub purchase_type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "5")]
    pub source_reference_id: ::core::option::Option<u64>,
    #[prost(int32, optional, tag = "6")]
    pub price_index: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcStorePurchaseInit {
    #[prost(string, optional, tag = "1")]
    pub country: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub language: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub currency: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub line_items: ::prost::alloc::vec::Vec<CGcStorePurchaseInitLineItem>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcStorePurchaseInitResponse {
    #[prost(int32, optional, tag = "1")]
    pub result: ::core::option::Option<i32>,
    #[prost(uint64, optional, tag = "2")]
    pub txn_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgClientPingData {
    #[prost(fixed32, repeated, tag = "4")]
    pub relay_codes: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "5")]
    pub relay_pings: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "8")]
    pub region_codes: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "9")]
    pub region_pings: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub region_ping_failed_bitmask: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgInviteToParty {
    #[prost(fixed64, optional, tag = "1")]
    pub steam_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub client_version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub team_id: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "4")]
    pub as_coach: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "5")]
    pub ping_data: ::core::option::Option<CMsgClientPingData>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgInviteToLobby {
    #[prost(fixed64, optional, tag = "1")]
    pub steam_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub client_version: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgInvitationCreated {
    #[prost(uint64, optional, tag = "1")]
    pub group_id: ::core::option::Option<u64>,
    #[prost(fixed64, optional, tag = "2")]
    pub steam_id: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "3")]
    pub user_offline: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgPartyInviteResponse {
    #[prost(uint64, optional, tag = "1")]
    pub party_id: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "2")]
    pub accept: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "3")]
    pub client_version: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "8")]
    pub ping_data: ::core::option::Option<CMsgClientPingData>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgLobbyInviteResponse {
    #[prost(fixed64, optional, tag = "1")]
    pub lobby_id: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "2")]
    pub accept: ::core::option::Option<bool>,
    #[prost(uint32, optional, tag = "3")]
    pub client_version: ::core::option::Option<u32>,
    #[prost(fixed64, optional, tag = "6")]
    pub custom_game_crc: ::core::option::Option<u64>,
    #[prost(fixed32, optional, tag = "7")]
    pub custom_game_timestamp: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgKickFromParty {
    #[prost(fixed64, optional, tag = "1")]
    pub steam_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgLeaveParty {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgCustomGameInstallStatus {
    #[prost(
        enumeration = "ECustomGameInstallStatus",
        optional,
        tag = "1",
        default = "KECustomGameInstallStatusUnknown"
    )]
    pub status: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(fixed32, optional, tag = "3")]
    pub latest_timestamp_from_steam: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgServerAvailable {
    #[prost(message, optional, tag = "1")]
    pub custom_game_install_status: ::core::option::Option<CMsgCustomGameInstallStatus>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgLanServerAvailable {
    #[prost(fixed64, optional, tag = "1")]
    pub lobby_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoEconGameAccountClient {
    #[prost(uint32, optional, tag = "1", default = "0")]
    pub additional_backpack_slots: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "2", default = "false")]
    pub trial_account: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3", default = "true")]
    pub eligible_for_online_play: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub need_to_choose_most_helpful_friend: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub in_coaches_list: ::core::option::Option<bool>,
    #[prost(fixed32, optional, tag = "6")]
    pub trade_ban_expiration: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "7")]
    pub duel_ban_expiration: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "9", default = "false")]
    pub made_first_purchase: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgApplyStrangePart {
    #[prost(uint64, optional, tag = "1")]
    pub strange_part_item_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub item_item_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgApplyPennantUpgrade {
    #[prost(uint64, optional, tag = "1")]
    pub upgrade_item_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub pennant_item_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgApplyEggEssence {
    #[prost(uint64, optional, tag = "1")]
    pub essence_item_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub egg_item_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoEconItemAttribute {
    #[prost(uint32, optional, tag = "1", default = "65535")]
    pub def_index: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub value: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub value_bytes: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSoEconItemEquipped {
    #[prost(uint32, optional, tag = "1")]
    pub new_class: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub new_slot: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSoEconItem {
    #[prost(uint64, optional, tag = "1")]
    pub id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub inventory: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub def_index: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5", default = "1")]
    pub quantity: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6", default = "1")]
    pub level: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7", default = "4")]
    pub quality: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8", default = "0")]
    pub flags: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9", default = "0")]
    pub origin: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "12")]
    pub attribute: ::prost::alloc::vec::Vec<CSoEconItemAttribute>,
    #[prost(message, optional, boxed, tag = "13")]
    pub interior_item: ::core::option::Option<::prost::alloc::boxed::Box<CSoEconItem>>,
    #[prost(uint32, optional, tag = "15", default = "0")]
    pub style: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "16")]
    pub original_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "18")]
    pub equipped_state: ::prost::alloc::vec::Vec<CSoEconItemEquipped>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSortItems {
    #[prost(uint32, optional, tag = "1")]
    pub sort_type: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgItemAcknowledged {
    #[prost(uint32, optional, tag = "1")]
    pub account_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub inventory: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub def_index: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub quality: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub rarity: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub origin: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSetItemPositions {
    #[prost(message, repeated, tag = "1")]
    pub item_positions: ::prost::alloc::vec::Vec<c_msg_set_item_positions::ItemPosition>,
}
pub mod c_msg_set_item_positions {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ItemPosition {
        #[prost(uint64, optional, tag = "1")]
        pub item_id: ::core::option::Option<u64>,
        #[prost(uint32, optional, tag = "2")]
        pub position: ::core::option::Option<u32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcStorePurchaseCancel {
    #[prost(uint64, optional, tag = "1")]
    pub txn_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcStorePurchaseCancelResponse {
    #[prost(uint32, optional, tag = "1")]
    pub result: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcStorePurchaseFinalize {
    #[prost(uint64, optional, tag = "1")]
    pub txn_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcStorePurchaseFinalizeResponse {
    #[prost(uint32, optional, tag = "1")]
    pub result: ::core::option::Option<u32>,
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub item_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcBannedWordListUpdated {
    #[prost(uint32, optional, tag = "1")]
    pub group_id: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcDirtySdoCache {
    #[prost(uint32, optional, tag = "1")]
    pub sdo_type: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "2")]
    pub key_uint64: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSdoNoMemcached {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcUpdateSqlKeyValue {
    #[prost(string, optional, tag = "1")]
    pub key_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcServerVersionUpdated {
    #[prost(uint32, optional, tag = "1")]
    pub server_version: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcClientVersionUpdated {
    #[prost(uint32, optional, tag = "1")]
    pub client_version: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcWebApiAccountChanged {}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgExtractGems {
    #[prost(uint64, optional, tag = "1")]
    pub tool_item_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub item_item_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3", default = "65535")]
    pub item_socket_id: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgExtractGemsResponse {
    #[prost(uint64, optional, tag = "1")]
    pub item_id: ::core::option::Option<u64>,
    #[prost(
        enumeration = "c_msg_extract_gems_response::EExtractGems",
        optional,
        tag = "2",
        default = "KExtractGemsSucceeded"
    )]
    pub response: ::core::option::Option<i32>,
}
pub mod c_msg_extract_gems_response {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EExtractGems {
        KExtractGemsSucceeded = 0,
        KExtractGemsFailedToolIsInvalid = 1,
        KExtractGemsFailedItemIsInvalid = 2,
        KExtractGemsFailedToolCannotRemoveGem = 3,
        KExtractGemsFailedFailedToRemoveGem = 4,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgAddSocket {
    #[prost(uint64, optional, tag = "1")]
    pub tool_item_id: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub item_item_id: ::core::option::Option<u64>,
    #[prost(bool, optional, tag = "3")]
    pub unusual: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgAddSocketResponse {
    #[prost(uint64, optional, tag = "1")]
    pub item_id: ::core::option::Option<u64>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub updated_socket_index: ::prost::alloc::vec::Vec<u32>,
    #[prost(
        enumeration = "c_msg_add_socket_response::EAddSocket",
        optional,
        tag = "3",
        default = "KAddSocketSucceeded"
    )]
    pub response: ::core::option::Option<i32>,
}
pub mod c_msg_add_socket_response {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EAddSocket {
        KAddSocketSucceeded = 0,
        KAddSocketFailedToolIsInvalid = 1,
        KAddSocketFailedItemCannotBeSocketed = 2,
        KAddSocketFailedFailedToAddSocket = 3,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgAddItemToSocketData {
    #[prost(uint64, optional, tag = "1")]
    pub gem_item_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2", default = "65535")]
    pub socket_index: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgAddItemToSocket {
    #[prost(uint64, optional, tag = "1")]
    pub item_item_id: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "2")]
    pub gems_to_socket: ::prost::alloc::vec::Vec<CMsgAddItemToSocketData>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgAddItemToSocketResponse {
    #[prost(uint64, optional, tag = "1")]
    pub item_item_id: ::core::option::Option<u64>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub updated_socket_index: ::prost::alloc::vec::Vec<u32>,
    #[prost(
        enumeration = "c_msg_add_item_to_socket_response::EAddGem",
        optional,
        tag = "3",
        default = "KAddGemSucceeded"
    )]
    pub response: ::core::option::Option<i32>,
}
pub mod c_msg_add_item_to_socket_response {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EAddGem {
        KAddGemSucceeded = 0,
        KAddGemFailedGemIsInvalid = 1,
        KAddGemFailedItemIsInvalid = 2,
        KAddGemFailedFailedToAddGem = 3,
        KAddGemFailedInvalidGemTypeForSocket = 4,
        KAddGemFailedInvalidGemTypeForHero = 5,
        KAddGemFailedInvalidGemTypeForSlot = 6,
        KAddGemFailedSocketContainsUnremovableGem = 7,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgResetStrangeGemCount {
    #[prost(uint64, optional, tag = "1")]
    pub item_item_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2", default = "65535")]
    pub socket_index: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgResetStrangeGemCountResponse {
    #[prost(
        enumeration = "c_msg_reset_strange_gem_count_response::EResetGem",
        optional,
        tag = "1",
        default = "KResetGemSucceeded"
    )]
    pub response: ::core::option::Option<i32>,
}
pub mod c_msg_reset_strange_gem_count_response {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EResetGem {
        KResetGemSucceeded = 0,
        KResetGemFailedFailedToResetGem = 1,
        KResetGemFailedItemIsInvalid = 2,
        KResetGemFailedInvalidSocketId = 3,
        KResetGemFailedSocketCannotBeReset = 4,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToClientPollFileRequest {
    #[prost(string, optional, tag = "1")]
    pub file_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "2")]
    pub client_version: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub poll_id: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToClientPollFileResponse {
    #[prost(uint32, optional, tag = "1")]
    pub poll_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub file_size: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub file_crc: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcPerformManualOp {
    #[prost(uint64, optional, tag = "1")]
    pub op_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub group_code: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcPerformManualOpCompleted {
    #[prost(bool, optional, tag = "1")]
    pub success: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub source_gc: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToGcReloadServerRegionSettings {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcAdditionalWelcomeMsgList {
    #[prost(message, repeated, tag = "1")]
    pub welcome_messages: ::prost::alloc::vec::Vec<CExtraMsgBlock>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgApplyRemoteConVars {
    #[prost(message, repeated, tag = "1")]
    pub con_vars: ::prost::alloc::vec::Vec<c_msg_apply_remote_con_vars::ConVar>,
}
pub mod c_msg_apply_remote_con_vars {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ConVar {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "3")]
        pub version_min: ::core::option::Option<u32>,
        #[prost(uint32, optional, tag = "4")]
        pub version_max: ::core::option::Option<u32>,
        #[prost(enumeration = "super::EgcPlatform", optional, tag = "5", default = "KEGcPlatformNone")]
        pub platform: ::core::option::Option<i32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcToClientApplyRemoteConVars {
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<CMsgApplyRemoteConVars>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcToServerApplyRemoteConVars {
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<CMsgApplyRemoteConVars>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgClientToGcIntegrityStatus {
    #[prost(string, optional, tag = "1")]
    pub report: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub secure_allowed: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "3")]
    pub diagnostics: ::prost::alloc::vec::Vec<c_msg_client_to_gc_integrity_status::Keyvalue>,
}
pub mod c_msg_client_to_gc_integrity_status {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Keyvalue {
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgClientToGcAggregateMetrics {
    #[prost(message, repeated, tag = "1")]
    pub metrics: ::prost::alloc::vec::Vec<c_msg_client_to_gc_aggregate_metrics::SingleMetric>,
}
pub mod c_msg_client_to_gc_aggregate_metrics {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SingleMetric {
        #[prost(string, optional, tag = "1")]
        pub metric_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "2")]
        pub metric_count: ::core::option::Option<u32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgGcToClientAggregateMetricsBackoff {
    #[prost(float, optional, tag = "1")]
    pub upload_rate_modifier: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgGcToServerSteamLearnAccessTokensChanged {
    #[prost(message, optional, tag = "1")]
    pub access_tokens: ::core::option::Option<CMsgSteamLearnAccessTokens>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgGcToServerSteamLearnUseHttp {
    #[prost(bool, optional, tag = "1")]
    pub use_http: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EgcBaseMsg {
    KEMsgGcInviteToParty = 4501,
    KEMsgGcInvitationCreated = 4502,
    KEMsgGcPartyInviteResponse = 4503,
    KEMsgGcKickFromParty = 4504,
    KEMsgGcLeaveParty = 4505,
    KEMsgGcServerAvailable = 4506,
    KEMsgGcClientConnectToServer = 4507,
    KEMsgGcGameServerInfo = 4508,
    KEMsgGclanServerAvailable = 4511,
    KEMsgGcInviteToLobby = 4512,
    KEMsgGcLobbyInviteResponse = 4513,
    KEMsgGcToClientPollFileRequest = 4514,
    KEMsgGcToClientPollFileResponse = 4515,
    KEMsgGcToGcPerformManualOp = 4516,
    KEMsgGcToGcPerformManualOpCompleted = 4517,
    KEMsgGcToGcReloadServerRegionSettings = 4518,
    KEMsgGcAdditionalWelcomeMsgList = 4519,
    KEMsgGcToClientApplyRemoteConVars = 4520,
    KEMsgGcToServerApplyRemoteConVars = 4521,
    KEMsgClientToGcIntegrityStatus = 4522,
    KEMsgClientToGcAggregateMetrics = 4523,
    KEMsgGcToClientAggregateMetricsBackoff = 4524,
    KEMsgGcToServerSteamLearnAccessTokensChanged = 4525,
    KEMsgGcToServerSteamLearnUseHttp = 4526,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECustomGameInstallStatus {
    KECustomGameInstallStatusUnknown = 0,
    KECustomGameInstallStatusReady = 1,
    KECustomGameInstallStatusBusy = 2,
    KECustomGameInstallStatusFailedGeneric = 101,
    KECustomGameInstallStatusFailedInternalError = 102,
    KECustomGameInstallStatusRequestedTimestampTooOld = 103,
    KECustomGameInstallStatusRequestedTimestampTooNew = 104,
    KECustomGameInstallStatusCrcMismatch = 105,
    KECustomGameInstallStatusFailedSteam = 106,
    KECustomGameInstallStatusFailedCanceled = 107,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoFileHeader {
    #[prost(string, required, tag = "1")]
    pub demo_file_stamp: ::prost::alloc::string::String,
    #[prost(int32, optional, tag = "2")]
    pub patch_version: ::core::option::Option<i32>,
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CGameInfo {
    #[prost(message, optional, tag = "4")]
    pub dota: ::core::option::Option<c_game_info::CDotaGameInfo>,
    #[prost(message, optional, tag = "5")]
    pub cs: ::core::option::Option<c_game_info::CCsGameInfo>,
}
pub mod c_game_info {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
        #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct CPlayerInfo {
            #[prost(string, optional, tag = "1")]
            pub hero_name: ::core::option::Option<::prost::alloc::string::String>,
            #[prost(bytes = "vec", optional, tag = "2")]
            pub player_name: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
            #[prost(bool, optional, tag = "3")]
            pub is_fake_client: ::core::option::Option<bool>,
            #[prost(uint64, optional, tag = "4")]
            pub steamid: ::core::option::Option<u64>,
            #[prost(int32, optional, tag = "5")]
            pub game_team: ::core::option::Option<i32>,
        }
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct CHeroSelectEvent {
            #[prost(bool, optional, tag = "1")]
            pub is_pick: ::core::option::Option<bool>,
            #[prost(uint32, optional, tag = "2")]
            pub team: ::core::option::Option<u32>,
            #[prost(int32, optional, tag = "3")]
            pub hero_id: ::core::option::Option<i32>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CCsGameInfo {
        #[prost(int32, repeated, packed = "false", tag = "1")]
        pub round_start_ticks: ::prost::alloc::vec::Vec<i32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoPacket {
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CDemoFullPacket {
    #[prost(message, optional, tag = "1")]
    pub string_table: ::core::option::Option<CDemoStringTables>,
    #[prost(message, optional, tag = "2")]
    pub packet: ::core::option::Option<CDemoPacket>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoSyncTick {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoConsoleCmd {
    #[prost(string, optional, tag = "1")]
    pub cmdstring: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoSendTables {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CDemoClassInfo {
    #[prost(message, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<c_demo_class_info::ClassT>,
}
pub mod c_demo_class_info {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ClassT {
        #[prost(int32, optional, tag = "1")]
        pub class_id: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub network_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "3")]
        pub table_name: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoCustomData {
    #[prost(int32, optional, tag = "1")]
    pub callback_index: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoCustomDataCallbacks {
    #[prost(string, repeated, tag = "1")]
    pub save_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoAnimationHeader {
    #[prost(sint32, optional, tag = "1")]
    pub entity_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub tick: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CDemoStringTables {
    #[prost(message, repeated, tag = "1")]
    pub tables: ::prost::alloc::vec::Vec<c_demo_string_tables::TableT>,
}
pub mod c_demo_string_tables {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ItemsT {
        #[prost(string, optional, tag = "1")]
        pub str: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoStop {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoUserCmd {
    #[prost(int32, optional, tag = "1")]
    pub cmd_number: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoSpawnGroups {
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub msgs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoSpawnGroupsHltvBroadcast {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CDemoRecovery {
    #[prost(message, optional, tag = "1")]
    pub initial_spawn_group: ::core::option::Option<c_demo_recovery::DemoInitialSpawnGroupEntry>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub spawn_group_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
pub mod c_demo_recovery {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct DemoInitialSpawnGroupEntry {
        #[prost(uint32, optional, tag = "1")]
        pub spawngrouphandle: ::core::option::Option<u32>,
        #[prost(bool, optional, tag = "2")]
        pub was_created: ::core::option::Option<bool>,
    }
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
    DemRecovery = 18,
    DemMax = 19,
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
    NetworkDisconnectKickedVacnetabnormalbehavior = 163,
    NetworkDisconnectKickedInsecureclient = 164,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgVector2D {
    #[prost(float, optional, tag = "1")]
    pub x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgQAngle {
    #[prost(float, optional, tag = "1")]
    pub x: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub z: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgTransform {
    #[prost(message, optional, tag = "1")]
    pub position: ::core::option::Option<CMsgVector>,
    #[prost(float, optional, tag = "2")]
    pub scale: ::core::option::Option<f32>,
    #[prost(message, optional, tag = "3")]
    pub orientation: ::core::option::Option<CMsgQuaternion>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CEntityMsg {
    #[prost(uint32, optional, tag = "1", default = "16777215")]
    pub target_entity: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgCVars {
    #[prost(message, repeated, tag = "1")]
    pub cvars: ::prost::alloc::vec::Vec<c_msg_c_vars::CVar>,
}
pub mod c_msg_c_vars {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CVar {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CNetMsgNop {}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CNetMsgSplitScreenUser {
    #[prost(int32, optional, tag = "1")]
    pub slot: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CNetMsgTick {
    #[prost(uint32, optional, tag = "1")]
    pub tick: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub host_computationtime: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub host_computationtime_std_deviation: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7")]
    pub legacy_host_loss: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8")]
    pub host_unfiltered_frametime: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub hltv_replay_flags: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10")]
    pub expected_long_tick: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "11")]
    pub expected_long_tick_reason: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "12")]
    pub host_frame_dropped_pct_x10: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "13")]
    pub host_frame_irregular_arrival_pct_x10: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CNetMsgStringCmd {
    #[prost(string, optional, tag = "1")]
    pub command: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "2")]
    pub prediction_sync: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CNetMsgSetConVar {
    #[prost(message, optional, tag = "1")]
    pub convars: ::core::option::Option<CMsgCVars>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgGameEvent {
    #[prost(string, optional, tag = "1")]
    pub event_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "2")]
    pub eventid: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "3")]
    pub keys: ::prost::alloc::vec::Vec<csvc_msg_game_event::KeyT>,
}
pub mod csvc_msg_game_event {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgListGameEvents {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<csvc_msg_list_game_events::EventT>,
}
pub mod csvc_msg_list_game_events {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct EventT {
        #[prost(int32, optional, tag = "1")]
        pub tick: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub event: ::core::option::Option<super::CSvcMsgGameEvent>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CNetMsgSpawnGroupManifestUpdate {
    #[prost(uint32, optional, tag = "1")]
    pub spawngrouphandle: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub spawngroupmanifest: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bool, optional, tag = "3")]
    pub manifestincomplete: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CNetMsgSpawnGroupSetCreationTick {
    #[prost(uint32, optional, tag = "1")]
    pub spawngrouphandle: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2")]
    pub tickcount: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub creationsequence: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CNetMsgSpawnGroupUnload {
    #[prost(uint32, optional, tag = "1")]
    pub spawngrouphandle: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub flags: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "3")]
    pub tickcount: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CNetMsgSpawnGroupLoadCompleted {
    #[prost(uint32, optional, tag = "1")]
    pub spawngrouphandle: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgVDebugGameSessionIdEvent {
    #[prost(int32, optional, tag = "1")]
    pub clientid: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub gamesessionid: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgPlaceDecalEvent {
    #[prost(message, optional, tag = "1")]
    pub position: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "2")]
    pub normal: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "3")]
    pub saxis: ::core::option::Option<CMsgVector>,
    #[prost(int32, optional, tag = "4")]
    pub boneindex: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub triangleindex: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "5")]
    pub flags: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "6")]
    pub color: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "7")]
    pub random_seed: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "8")]
    pub decal_group_name: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "9")]
    pub size_override: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "10", default = "16777215")]
    pub entityhandle: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "11")]
    pub material_id: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "12")]
    pub sequence_name: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "14")]
    pub position_objectspace: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "15")]
    pub normal_objectspace: ::core::option::Option<CMsgVector>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgClearWorldDecalsEvent {
    #[prost(uint32, optional, tag = "1")]
    pub flagstoclear: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgClearEntityDecalsEvent {
    #[prost(uint32, optional, tag = "1")]
    pub flagstoclear: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgClearDecalsForEntityEvent {
    #[prost(uint32, optional, tag = "1")]
    pub flagstoclear: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2", default = "16777215")]
    pub entityhandle: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSource1LegacyGameEventList {
    #[prost(message, repeated, tag = "1")]
    pub descriptors: ::prost::alloc::vec::Vec<c_msg_source1_legacy_game_event_list::DescriptorT>,
}
pub mod c_msg_source1_legacy_game_event_list {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct KeyT {
        #[prost(int32, optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct DescriptorT {
        #[prost(int32, optional, tag = "1")]
        pub eventid: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "3")]
        pub keys: ::prost::alloc::vec::Vec<KeyT>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSource1LegacyListenEvents {
    #[prost(int32, optional, tag = "1")]
    pub playerslot: ::core::option::Option<i32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub eventarraybits: ::prost::alloc::vec::Vec<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSosStopSoundEvent {
    #[prost(int32, optional, tag = "1")]
    pub soundevent_guid: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSosStopSoundEventHash {
    #[prost(fixed32, optional, tag = "1")]
    pub soundevent_hash: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub source_entity_index: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSosSetSoundEventParams {
    #[prost(int32, optional, tag = "1")]
    pub soundevent_guid: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub packed_params: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSosSetLibraryStackFields {
    #[prost(fixed32, optional, tag = "1")]
    pub stack_hash: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub packed_fields: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CMsgClothStiffenAnimEvent {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_entity_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub vertex_set_hash: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "3")]
    pub intensity: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub length: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "5")]
    pub speed_in: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "6")]
    pub speed_out: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgClothEffectAnimEvent {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub source_entity_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub effect_name_hash: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub operation: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub flags: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub tags: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub pte: ::core::option::Option<CMsgVector>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EBaseGameEvents {
    GeVDebugGameSessionIdEvent = 200,
    GePlaceDecalEvent = 201,
    GeClearWorldDecalsEvent = 202,
    GeClearEntityDecalsEvent = 203,
    GeClearDecalsForEntityEvent = 204,
    GeSource1LegacyGameEventList = 205,
    GeSource1LegacyListenEvents = 206,
    GeSource1LegacyGameEvent = 207,
    GeSosStartSoundEvent = 208,
    GeSosStopSoundEvent = 209,
    GeSosSetSoundEventParams = 210,
    GeSosSetLibraryStackFields = 211,
    GeSosStopSoundEventHash = 212,
    GeClothStiffenAnimEvent = 213,
    GeClothEffectAnimEvent = 214,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
    #[prost(uint32, optional, tag = "51")]
    pub backbuffer_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "52")]
    pub backbuffer_height: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSource2VProfLiteReportItem {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "2")]
    pub active_samples: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub active_samples_1secmax: ::core::option::Option<u32>,
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
    #[prost(uint32, optional, tag = "31")]
    pub usec_1secmax_avg_active: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "32")]
    pub usec_1secmax_p50_active: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "33")]
    pub usec_1secmax_p95_active: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "34")]
    pub usec_1secmax_p99_active: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "41")]
    pub usec_1secmax_avg_all: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "42")]
    pub usec_1secmax_p50_all: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "43")]
    pub usec_1secmax_p95_all: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "44")]
    pub usec_1secmax_p99_all: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSource2VProfLiteReport {
    #[prost(message, optional, tag = "1")]
    pub total: ::core::option::Option<CMsgSource2VProfLiteReportItem>,
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<CMsgSource2VProfLiteReportItem>,
    #[prost(uint32, optional, tag = "3")]
    pub discarded_frames: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgSource2NetworkFlowQuality {
    #[prost(uint32, optional, tag = "1")]
    pub duration: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "5")]
    pub bytes_total: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "6")]
    pub bytes_total_reliable: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "7")]
    pub bytes_total_voice: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "10")]
    pub bytes_sec_p95: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub bytes_sec_p99: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "20")]
    pub enginemsgs_total: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "21")]
    pub enginemsgs_sec_p95: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "22")]
    pub enginemsgs_sec_p99: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "30")]
    pub netframes_total: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "31")]
    pub netframes_dropped: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "32")]
    pub netframes_outoforder: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "34")]
    pub netframes_size_exceeds_mtu: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "35")]
    pub netframes_size_p95: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "36")]
    pub netframes_size_p99: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "40")]
    pub ticks_total: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "41")]
    pub ticks_good: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "42")]
    pub ticks_good_almost_late: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "43")]
    pub ticks_fixed_dropped: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "44")]
    pub ticks_fixed_late: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "45")]
    pub ticks_bad_dropped: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "46")]
    pub ticks_bad_late: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "47")]
    pub ticks_bad_other: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "50")]
    pub tick_missrate_samples_total: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "51")]
    pub tick_missrate_samples_perfect: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "52")]
    pub tick_missrate_samples_perfectnet: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "53")]
    pub tick_missratenet_p75_x10: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "54")]
    pub tick_missratenet_p95_x10: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "55")]
    pub tick_missratenet_p99_x10: ::core::option::Option<u32>,
    #[prost(sint32, optional, tag = "61")]
    pub recvmargin_p1: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag = "62")]
    pub recvmargin_p5: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag = "63")]
    pub recvmargin_p25: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag = "64")]
    pub recvmargin_p50: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag = "65")]
    pub recvmargin_p75: ::core::option::Option<i32>,
    #[prost(sint32, optional, tag = "66")]
    pub recvmargin_p95: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "70")]
    pub netframe_jitter_p50: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "71")]
    pub netframe_jitter_p99: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "72")]
    pub interval_peakjitter_p50: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "73")]
    pub interval_peakjitter_p95: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "74")]
    pub packet_misdelivery_rate_p50_x4: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "75")]
    pub packet_misdelivery_rate_p95_x4: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "80")]
    pub net_ping_p5: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "81")]
    pub net_ping_p50: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "82")]
    pub net_ping_p95: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSource2PerfIntervalSample {
    #[prost(float, optional, tag = "1")]
    pub frame_time_max_ms: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub frame_time_avg_ms: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub frame_time_min_ms: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "4")]
    pub frame_count: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "5")]
    pub frame_time_total_ms: ::core::option::Option<f32>,
    #[prost(message, repeated, tag = "6")]
    pub tags: ::prost::alloc::vec::Vec<c_msg_source2_perf_interval_sample::Tag>,
}
pub mod c_msg_source2_perf_interval_sample {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Tag {
        #[prost(string, optional, tag = "1")]
        pub tag: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(uint32, optional, tag = "2")]
        pub max_value: ::core::option::Option<u32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSource2MetricsMatchPerfSummaryNotification {
    #[prost(uint32, optional, tag = "1")]
    pub appid: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub game_mode: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub server_build_id: ::core::option::Option<u32>,
    #[prost(fixed32, optional, tag = "4")]
    pub server_popid: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "10")]
    pub server_profile: ::core::option::Option<CMsgSource2VProfLiteReport>,
    #[prost(message, repeated, tag = "11")]
    pub clients: ::prost::alloc::vec::Vec<c_source2_metrics_match_perf_summary_notification::Client>,
    #[prost(string, optional, tag = "20")]
    pub map: ::core::option::Option<::prost::alloc::string::String>,
}
pub mod c_source2_metrics_match_perf_summary_notification {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct Client {
        #[prost(message, optional, tag = "1")]
        pub system_specs: ::core::option::Option<super::CMsgSource2SystemSpecs>,
        #[prost(message, optional, tag = "2")]
        pub profile: ::core::option::Option<super::CMsgSource2VProfLiteReport>,
        #[prost(uint32, optional, tag = "3")]
        pub build_id: ::core::option::Option<u32>,
        #[prost(message, optional, tag = "4")]
        pub downstream_flow: ::core::option::Option<super::CMsgSource2NetworkFlowQuality>,
        #[prost(message, optional, tag = "5")]
        pub upstream_flow: ::core::option::Option<super::CMsgSource2NetworkFlowQuality>,
        #[prost(fixed64, optional, tag = "10")]
        pub steamid: ::core::option::Option<u64>,
        #[prost(message, repeated, tag = "11")]
        pub perf_samples: ::prost::alloc::vec::Vec<super::CMsgSource2PerfIntervalSample>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CMsgSource2PlayStatsPackedRecordList {
    #[prost(string, optional, tag = "1")]
    pub record_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub field_defs: ::prost::alloc::vec::Vec<c_msg_source2_play_stats_packed_record_list::FieldDef>,
    #[prost(uint32, optional, tag = "3")]
    pub record_count: ::core::option::Option<u32>,
    #[prost(uint64, repeated, tag = "4")]
    pub uint64_vals: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint32, repeated, tag = "5")]
    pub uint32_vals: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "6")]
    pub uint16_vals: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "7")]
    pub uint8_vals: ::prost::alloc::vec::Vec<u32>,
    #[prost(int64, repeated, tag = "8")]
    pub int64_vals: ::prost::alloc::vec::Vec<i64>,
    #[prost(int32, repeated, tag = "9")]
    pub int32_vals: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag = "10")]
    pub int16_vals: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag = "11")]
    pub int8_vals: ::prost::alloc::vec::Vec<i32>,
    #[prost(double, repeated, tag = "12")]
    pub float64_vals: ::prost::alloc::vec::Vec<f64>,
    #[prost(float, repeated, tag = "13")]
    pub float32_vals: ::prost::alloc::vec::Vec<f32>,
    #[prost(bool, repeated, tag = "14")]
    pub bool_vals: ::prost::alloc::vec::Vec<bool>,
    #[prost(string, repeated, tag = "15")]
    pub string_vals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "16")]
    pub low_cardinality_string_vals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(fixed32, repeated, tag = "17")]
    pub utcdatetime_vals: ::prost::alloc::vec::Vec<u32>,
    #[prost(fixed64, repeated, tag = "18")]
    pub steamidtrustbucket_vals: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag = "19")]
    pub trustbucket_vals: ::prost::alloc::vec::Vec<c_msg_source2_play_stats_packed_record_list::SteamIdList>,
    #[prost(uint64, repeated, tag = "20")]
    pub steamid_vals: ::prost::alloc::vec::Vec<u64>,
}
pub mod c_msg_source2_play_stats_packed_record_list {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct FieldDef {
        #[prost(string, optional, tag = "1")]
        pub field_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(
            enumeration = "super::ESource2PlayStatsFieldType",
            optional,
            tag = "2",
            default = "Source2PlayStatsInvalid"
        )]
        pub field_type: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SteamIdList {
        #[prost(fixed64, repeated, tag = "1")]
        pub steamid: ::prost::alloc::vec::Vec<u64>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSource2MetricsRecordPlayStatsNotification {
    #[prost(message, repeated, tag = "1")]
    pub record_types: ::prost::alloc::vec::Vec<CMsgSource2PlayStatsPackedRecordList>,
    #[prost(uint32, optional, tag = "2")]
    pub appid: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSource2MetricsFetchMapDataRequest {
    #[prost(uint32, optional, tag = "1")]
    pub appid: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub map_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub game_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub game_mode: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "5")]
    pub param: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "6")]
    pub time_span: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSource2MetricsFetchMapDataResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<c_source2_metrics_fetch_map_data_response::MapData>,
}
pub mod c_source2_metrics_fetch_map_data_response {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct MapData {
        #[prost(string, optional, tag = "1")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub r#type: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "3")]
        pub data: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageUserSentBugBug {
    #[prost(string, optional, tag = "1")]
    pub command_line: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub autoexec_cfg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub system_specs: ::core::option::Option<CMsgSource2SystemSpecs>,
    #[prost(uint32, optional, tag = "4")]
    pub build_id: ::core::option::Option<u32>,
    #[prost(int32, optional, tag = "5")]
    pub osversion: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub command_logs: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "7")]
    pub bugbug_no: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ESource2PlayStatsFieldType {
    Source2PlayStatsInvalid = 0,
    Source2PlayStatsUInt64 = 1,
    Source2PlayStatsUInt32 = 2,
    Source2PlayStatsUInt16 = 3,
    Source2PlayStatsUInt8 = 4,
    Source2PlayStatsInt64 = 5,
    Source2PlayStatsInt32 = 6,
    Source2PlayStatsInt16 = 7,
    Source2PlayStatsInt8 = 8,
    Source2PlayStatsFloat64 = 9,
    Source2PlayStatsFloat32 = 10,
    Source2PlayStatsBool = 11,
    Source2PlayStatsString = 12,
    Source2PlayStatsLowCardinalityString = 13,
    Source2PlayStatsUtcDateTime = 14,
    Source2PlayStatsSteamIdTrustBucket = 15,
    Source2PlayStatsSteamIdTrustBucketMin = 16,
    Source2PlayStatsSteamId = 17,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CclcMsgMove {
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "4")]
    pub last_command_number: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgVoiceData {
    #[prost(message, optional, tag = "1")]
    pub audio: ::core::option::Option<CMsgVoiceAudio>,
    #[prost(fixed64, optional, tag = "2")]
    pub xuid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "3")]
    pub tick: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CclcMsgBaselineAck {
    #[prost(int32, optional, tag = "1")]
    pub baseline_tick: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub baseline_nr: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CclcMsgListenEvents {
    #[prost(fixed32, repeated, packed = "false", tag = "1")]
    pub event_mask: ::prost::alloc::vec::Vec<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CclcMsgLoadingProgress {
    #[prost(int32, optional, tag = "1")]
    pub progress: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CclcMsgSplitPlayerConnect {
    #[prost(string, optional, tag = "1")]
    pub playername: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CclcMsgSplitPlayerDisconnect {
    #[prost(int32, optional, tag = "1")]
    pub slot: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CclcMsgServerStatus {
    #[prost(bool, optional, tag = "1")]
    pub simplified: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CclcMsgRequestPause {
    #[prost(enumeration = "RequestPauseT", optional, tag = "1", default = "RpPause")]
    pub pause_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub pause_group: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CclcMsgCmdKeyValues {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CclcMsgRconServerDetails {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CclcMsgDiagnostic {
    #[prost(message, optional, tag = "1")]
    pub system_specs: ::core::option::Option<CMsgSource2SystemSpecs>,
    #[prost(message, optional, tag = "2")]
    pub vprof_report: ::core::option::Option<CMsgSource2VProfLiteReport>,
    #[prost(message, optional, tag = "3")]
    pub downstream_flow: ::core::option::Option<CMsgSource2NetworkFlowQuality>,
    #[prost(message, optional, tag = "4")]
    pub upstream_flow: ::core::option::Option<CMsgSource2NetworkFlowQuality>,
    #[prost(message, repeated, tag = "5")]
    pub perf_samples: ::prost::alloc::vec::Vec<CMsgSource2PerfIntervalSample>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgClassInfo {
    #[prost(bool, optional, tag = "1")]
    pub create_on_client: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "2")]
    pub classes: ::prost::alloc::vec::Vec<csvc_msg_class_info::ClassT>,
}
pub mod csvc_msg_class_info {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ClassT {
        #[prost(int32, optional, tag = "1")]
        pub class_id: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "3")]
        pub class_name: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgSetPause {
    #[prost(bool, optional, tag = "1")]
    pub paused: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgVoiceInit {
    #[prost(int32, optional, tag = "1")]
    pub quality: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub codec: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3", default = "0")]
    pub version: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgPrint {
    #[prost(string, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgSounds {
    #[prost(bool, optional, tag = "1")]
    pub reliable_sound: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "2")]
    pub sounds: ::prost::alloc::vec::Vec<csvc_msg_sounds::SounddataT>,
}
pub mod csvc_msg_sounds {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgPrefetch {
    #[prost(int32, optional, tag = "1")]
    pub sound_index: ::core::option::Option<i32>,
    #[prost(enumeration = "PrefetchType", optional, tag = "2", default = "PftSound")]
    pub resource_type: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgSetView {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub entity_index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub slot: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgFixAngle {
    #[prost(bool, optional, tag = "1")]
    pub relative: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub angle: ::core::option::Option<CMsgQAngle>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CSvcMsgCrosshairAngle {
    #[prost(message, optional, tag = "1")]
    pub angle: ::core::option::Option<CMsgQAngle>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgSplitScreen {
    #[prost(enumeration = "ESplitScreenMessageType", optional, tag = "1", default = "MsgSplitscreenAdduser")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub slot: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3", default = "-1")]
    pub player_index: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgGetCvarValue {
    #[prost(int32, optional, tag = "1")]
    pub cookie: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub cvar_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgMenu {
    #[prost(int32, optional, tag = "1")]
    pub dialog_type: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub menu_key_values: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgUserMessage {
    #[prost(int32, optional, tag = "1")]
    pub msg_type: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub msg_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "3")]
    pub passthrough: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgGameEventList {
    #[prost(message, repeated, tag = "1")]
    pub descriptors: ::prost::alloc::vec::Vec<csvc_msg_game_event_list::DescriptorT>,
}
pub mod csvc_msg_game_event_list {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct KeyT {
        #[prost(int32, optional, tag = "1")]
        pub r#type: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct DescriptorT {
        #[prost(int32, optional, tag = "1")]
        pub eventid: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "3")]
        pub keys: ::prost::alloc::vec::Vec<KeyT>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    pub has_pvs_vis_bits_deprecated: ::core::option::Option<u32>,
    #[prost(sint32, repeated, tag = "22")]
    pub cmd_recv_status: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, optional, tag = "19")]
    pub non_transmitted_entities: ::core::option::Option<csvc_msg_packet_entities::NonTransmittedEntitiesT>,
    #[prost(uint32, optional, tag = "20")]
    pub cq_starved_command_ticks: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "21")]
    pub cq_discarded_command_ticks: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "23")]
    pub outofpvs_entity_updates: ::core::option::Option<csvc_msg_packet_entities::OutofpvsEntityUpdatesT>,
    #[prost(bytes = "vec", optional, tag = "999")]
    pub dev_padding: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
pub mod csvc_msg_packet_entities {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct AlternateBaselineT {
        #[prost(int32, optional, tag = "1")]
        pub entity_index: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub baseline_index: ::core::option::Option<i32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct NonTransmittedEntitiesT {
        #[prost(int32, optional, tag = "1")]
        pub header_count: ::core::option::Option<i32>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct OutofpvsEntityUpdatesT {
        #[prost(int32, optional, tag = "1")]
        pub count: ::core::option::Option<i32>,
        #[prost(bytes = "vec", optional, tag = "2")]
        pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgTempEntities {
    #[prost(bool, optional, tag = "1")]
    pub reliable: ::core::option::Option<bool>,
    #[prost(int32, optional, tag = "2")]
    pub num_entries: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub entity_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgUpdateStringTable {
    #[prost(int32, optional, tag = "1")]
    pub table_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub num_changed_entries: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub string_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgVoiceData {
    #[prost(message, optional, tag = "1")]
    pub audio: ::core::option::Option<CMsgVoiceAudio>,
    #[prost(int32, optional, tag = "2", default = "-1")]
    pub client_deprecated: ::core::option::Option<i32>,
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
    #[prost(int32, optional, tag = "8", default = "-1")]
    pub entity: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgPacketReliable {
    #[prost(int32, optional, tag = "1")]
    pub tick: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub messagessize: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "3")]
    pub state: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgServerSteamId {
    #[prost(uint64, optional, tag = "1")]
    pub steam_id: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgCmdKeyValues {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgRconServerDetails {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub token: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "2")]
    pub details: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CMsgIpcAddress {
    #[prost(fixed64, optional, tag = "1")]
    pub computer_guid: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "2")]
    pub process_id: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgPeerList {
    #[prost(message, repeated, tag = "1")]
    pub peer: ::prost::alloc::vec::Vec<CMsgServerPeer>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgClearAllStringTables {
    #[prost(string, optional, tag = "1")]
    pub mapname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub create_tables_skipped: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct PolymorphicFieldT {
        #[prost(int32, optional, tag = "1")]
        pub polymorphic_field_serializer_name_sym: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub polymorphic_field_serializer_version: ::core::option::Option<i32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ProtoFlattenedSerializerT {
    #[prost(int32, optional, tag = "1")]
    pub serializer_name_sym: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub serializer_version: ::core::option::Option<i32>,
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub fields_index: ::prost::alloc::vec::Vec<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgFlattenedSerializer {
    #[prost(message, repeated, tag = "1")]
    pub serializers: ::prost::alloc::vec::Vec<ProtoFlattenedSerializerT>,
    #[prost(string, repeated, tag = "2")]
    pub symbols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub fields: ::prost::alloc::vec::Vec<ProtoFlattenedSerializerFieldT>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgStopSound {
    #[prost(fixed32, optional, tag = "1")]
    pub guid: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CBidirMsgRebroadcastSource {
    #[prost(int32, optional, tag = "1")]
    pub eventsource: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CBidirMsgPredictionEvent {
    #[prost(uint32, optional, tag = "1")]
    pub event_id: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub event_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag = "3")]
    pub sync_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub sync_val_uint32: ::core::option::Option<u32>,
}
pub mod c_bidir_msg_prediction_event {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ESyncType {
        StTick = 0,
        StUserCmdNum = 1,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Port {
        #[prost(int32, optional, tag = "1")]
        pub port: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgBroadcastCommand {
    #[prost(string, optional, tag = "1")]
    pub cmd: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgHltvFixupOperatorStatus {
    #[prost(uint32, optional, tag = "1")]
    pub mode: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub override_operator_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
    #[prost(bytes = "vec", optional, tag = "6")]
    pub delta_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CSvcMsgUserCommands {
    #[prost(message, repeated, tag = "1")]
    pub commands: ::prost::alloc::vec::Vec<CMsgServerUserCmd>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CSvcMsgNextMsgPredicted {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub predicted_by_player_slot: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub message_type_id: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClcMessages {
    ClcClientInfo = 20,
    ClcMove = 21,
    ClcVoiceData = 22,
    ClcBaselineAck = 23,
    ClcRespondCvarValue = 25,
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
    SvcNextMsgPredicted = 77,
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
    BiGameEventDeprecated = 18,
    BiPredictionEvent = 19,
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageAchievementEvent {
    #[prost(uint32, optional, tag = "1")]
    pub achievement: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageCurrentTimescale {
    #[prost(float, optional, tag = "1")]
    pub current: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageShakeDir {
    #[prost(message, optional, tag = "1")]
    pub shake: ::core::option::Option<CUserMessageShake>,
    #[prost(message, optional, tag = "2")]
    pub direction: ::core::option::Option<CMsgVector>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageSayText {
    #[prost(int32, optional, tag = "1", default = "-1")]
    pub playerindex: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub chat: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageHudText {
    #[prost(string, optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageTextMsg {
    #[prost(uint32, optional, tag = "1")]
    pub dest: ::core::option::Option<u32>,
    #[prost(string, repeated, tag = "2")]
    pub param: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageGameTitle {}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageResetHud {}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageSendAudio {
    #[prost(string, optional, tag = "1")]
    pub soundname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub stop: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageVoiceMask {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub gamerules_masks: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub ban_masks: ::prost::alloc::vec::Vec<u32>,
    #[prost(bool, optional, tag = "3")]
    pub mod_enable: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageRequestState {}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageRumble {
    #[prost(int32, optional, tag = "1")]
    pub index: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub data: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub flags: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageSayTextChannel {
    #[prost(int32, optional, tag = "1")]
    pub player: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub channel: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageItemPickup {
    #[prost(string, optional, tag = "1")]
    pub itemname: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageAmmoDenied {
    #[prost(uint32, optional, tag = "1")]
    pub ammo_id: ::core::option::Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageCreditsMsg {
    #[prost(enumeration = "ERollType", optional, tag = "1", default = "RollNone")]
    pub rolltype: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "2")]
    pub logo_length: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CEntityMessagePlayJingle {
    #[prost(message, optional, tag = "1")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CEntityMessageScreenOverlay {
    #[prost(bool, optional, tag = "1")]
    pub start_effect: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CEntityMessagePropagateForce {
    #[prost(message, optional, tag = "1")]
    pub impulse: ::core::option::Option<CMsgVector>,
    #[prost(message, optional, tag = "2")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CEntityMessageFixAngle {
    #[prost(bool, optional, tag = "1")]
    pub relative: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "2")]
    pub angle: ::core::option::Option<CMsgQAngle>,
    #[prost(message, optional, tag = "3")]
    pub entity_msg: ::core::option::Option<CEntityMsg>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageCameraTransition {
    #[prost(uint32, optional, tag = "1")]
    pub camera_type: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "2")]
    pub duration: ::core::option::Option<f32>,
    #[prost(message, optional, tag = "3")]
    pub params_data_driven: ::core::option::Option<c_user_message_camera_transition::TransitionDataDriven>,
}
pub mod c_user_message_camera_transition {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct TransitionDataDriven {
        #[prost(string, optional, tag = "1")]
        pub filename: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, optional, tag = "2", default = "-1")]
        pub attach_ent_index: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "3")]
        pub duration: ::core::option::Option<f32>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CUserMsgParticleManager {
    #[prost(enumeration = "ParticleMessage", optional, tag = "1", default = "GameParticleManagerEventCreate")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub index: ::core::option::Option<u32>,
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
    #[prost(message, optional, tag = "39")]
    pub add_fan: ::core::option::Option<c_user_msg_particle_manager::AddFan>,
    #[prost(message, optional, tag = "40")]
    pub update_fan: ::core::option::Option<c_user_msg_particle_manager::UpdateFan>,
    #[prost(message, optional, tag = "41")]
    pub set_particle_cluster_growth: ::core::option::Option<c_user_msg_particle_manager::SetParticleClusterGrowth>,
    #[prost(message, optional, tag = "42")]
    pub remove_fan: ::core::option::Option<c_user_msg_particle_manager::RemoveFan>,
    #[prost(message, optional, tag = "43")]
    pub create_smoke_grid: ::core::option::Option<c_user_msg_particle_manager::CreateSmokeGrid>,
    #[prost(message, optional, tag = "44")]
    pub set_override_texture: ::core::option::Option<c_user_msg_particle_manager::SetOverrideTexture>,
}
pub mod c_user_msg_particle_manager {
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ReleaseParticleIndex {}
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleObsolete {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleFwdObsolete {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub forward: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleFallback {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleOffset {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(message, optional, tag = "2")]
        pub origin_offset: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "3")]
        pub angle_offset: ::core::option::Option<super::CMsgQAngle>,
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
        #[prost(bool, optional, tag = "6")]
        pub include_wearables: ::core::option::Option<bool>,
        #[prost(message, optional, tag = "7")]
        pub offset_position: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "8")]
        pub offset_angles: ::core::option::Option<super::CMsgQAngle>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateParticleSetFrozen {
        #[prost(bool, optional, tag = "1")]
        pub set_frozen: ::core::option::Option<bool>,
        #[prost(float, optional, tag = "2")]
        pub transition_duration: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct UpdateParticleShouldDraw {
        #[prost(bool, optional, tag = "1")]
        pub should_draw: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ChangeControlPointAttachment {
        #[prost(int32, optional, tag = "1")]
        pub attachment_old: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub attachment_new: ::core::option::Option<i32>,
        #[prost(uint32, optional, tag = "3", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateEntityPosition {
        #[prost(uint32, optional, tag = "1", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
        #[prost(message, optional, tag = "2")]
        pub position: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SetParticleFoWProperties {
        #[prost(int32, optional, tag = "1")]
        pub fow_control_point: ::core::option::Option<i32>,
        #[prost(int32, optional, tag = "2")]
        pub fow_control_point2: ::core::option::Option<i32>,
        #[prost(float, optional, tag = "3")]
        pub fow_radius: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SetParticleShouldCheckFoW {
        #[prost(bool, optional, tag = "1")]
        pub check_fow: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SetControlPointModel {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub model_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SetControlPointSnapshot {
        #[prost(int32, optional, tag = "1")]
        pub control_point: ::core::option::Option<i32>,
        #[prost(string, optional, tag = "2")]
        pub snapshot_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SetParticleText {
        #[prost(string, optional, tag = "1")]
        pub text: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bool, optional, tag = "2")]
        pub localize: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SetTextureAttribute {
        #[prost(string, optional, tag = "1")]
        pub attribute_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(string, optional, tag = "2")]
        pub texture_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SetOverrideTexture {
        #[prost(string, optional, tag = "1")]
        pub texture_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SetSceneObjectGenericFlag {
        #[prost(bool, optional, tag = "1")]
        pub flag_value: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SetSceneObjectTintAndDesat {
        #[prost(fixed32, optional, tag = "1")]
        pub tint: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "2")]
        pub desat: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ParticleSkipToTime {
        #[prost(float, optional, tag = "1")]
        pub skip_to_time: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ParticleCanFreeze {
        #[prost(bool, optional, tag = "1")]
        pub can_freeze: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ParticleFreezeTransitionOverride {
        #[prost(float, optional, tag = "1")]
        pub freeze_transition_override: ::core::option::Option<f32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct FreezeParticleInvolving {
        #[prost(bool, optional, tag = "1")]
        pub set_frozen: ::core::option::Option<bool>,
        #[prost(float, optional, tag = "2")]
        pub transition_duration: ::core::option::Option<f32>,
        #[prost(uint32, optional, tag = "3", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddModellistOverrideElement {
        #[prost(string, optional, tag = "1")]
        pub model_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(float, optional, tag = "2")]
        pub spawn_probability: ::core::option::Option<f32>,
        #[prost(uint32, optional, tag = "3")]
        pub groupid: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ClearModellistOverride {
        #[prost(uint32, optional, tag = "1")]
        pub groupid: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
        pub struct FloatContextValue {
            #[prost(uint32, optional, tag = "1")]
            pub value_name_hash: ::core::option::Option<u32>,
            #[prost(float, optional, tag = "2")]
            pub value: ::core::option::Option<f32>,
        }
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
        pub struct VectorContextValue {
            #[prost(uint32, optional, tag = "1")]
            pub value_name_hash: ::core::option::Option<u32>,
            #[prost(message, optional, tag = "2")]
            pub value: ::core::option::Option<super::super::CMsgVector>,
        }
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
        pub struct TransformContextValue {
            #[prost(uint32, optional, tag = "1")]
            pub value_name_hash: ::core::option::Option<u32>,
            #[prost(message, optional, tag = "2")]
            pub angles: ::core::option::Option<super::super::CMsgQAngle>,
            #[prost(message, optional, tag = "3")]
            pub translation: ::core::option::Option<super::super::CMsgVector>,
        }
        #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
        pub struct EHandleContext {
            #[prost(uint32, optional, tag = "1")]
            pub value_name_hash: ::core::option::Option<u32>,
            #[prost(uint32, optional, tag = "2", default = "16777215")]
            pub ent_index: ::core::option::Option<u32>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CreatePhysicsSim {
        #[prost(string, optional, tag = "1")]
        pub prop_group_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bool, optional, tag = "2")]
        pub use_high_quality_simulation: ::core::option::Option<bool>,
        #[prost(uint32, optional, tag = "3")]
        pub max_particle_count: ::core::option::Option<u32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct DestroyPhysicsSim {}
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CreateSmokeGrid {
        #[prost(string, optional, tag = "1")]
        pub vdata_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SetVData {
        #[prost(string, optional, tag = "1")]
        pub vdata_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SetMaterialOverride {
        #[prost(string, optional, tag = "1")]
        pub material_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bool, optional, tag = "2")]
        pub include_children: ::core::option::Option<bool>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddFan {
        #[prost(bool, optional, tag = "1")]
        pub active: ::core::option::Option<bool>,
        #[prost(message, optional, tag = "2")]
        pub bounds_mins: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "3")]
        pub bounds_maxs: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "4")]
        pub fan_origin: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "5")]
        pub fan_origin_offset: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "6")]
        pub fan_direction: ::core::option::Option<super::CMsgVector>,
        #[prost(float, optional, tag = "7")]
        pub force: ::core::option::Option<f32>,
        #[prost(string, optional, tag = "8")]
        pub fan_force_curve: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bool, optional, tag = "9")]
        pub falloff: ::core::option::Option<bool>,
        #[prost(bool, optional, tag = "10")]
        pub pull_towards_point: ::core::option::Option<bool>,
        #[prost(float, optional, tag = "11")]
        pub curve_min_dist: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "12")]
        pub curve_max_dist: ::core::option::Option<f32>,
        #[prost(uint32, optional, tag = "13")]
        pub fan_type: ::core::option::Option<u32>,
        #[prost(float, optional, tag = "14")]
        pub cone_start_radius: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "15")]
        pub cone_end_radius: ::core::option::Option<f32>,
        #[prost(float, optional, tag = "16")]
        pub cone_length: ::core::option::Option<f32>,
        #[prost(uint32, optional, tag = "17", default = "16777215")]
        pub entity_handle: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "18")]
        pub attachment_name: ::core::option::Option<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct UpdateFan {
        #[prost(bool, optional, tag = "1")]
        pub active: ::core::option::Option<bool>,
        #[prost(message, optional, tag = "2")]
        pub fan_origin: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "3")]
        pub fan_origin_offset: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "4")]
        pub fan_direction: ::core::option::Option<super::CMsgVector>,
        #[prost(float, optional, tag = "7")]
        pub fan_ramp_ratio: ::core::option::Option<f32>,
        #[prost(message, optional, tag = "5")]
        pub bounds_mins: ::core::option::Option<super::CMsgVector>,
        #[prost(message, optional, tag = "6")]
        pub bounds_maxs: ::core::option::Option<super::CMsgVector>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct RemoveFan {}
    #[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SetParticleClusterGrowth {
        #[prost(float, optional, tag = "1")]
        pub duration: ::core::option::Option<f32>,
        #[prost(message, optional, tag = "2")]
        pub origin: ::core::option::Option<super::CMsgVector>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMsgHudError {
    #[prost(int32, optional, tag = "1")]
    pub order_id: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMsgCustomGameEvent {
    #[prost(string, optional, tag = "1")]
    pub event_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageHapticsManagerEffect {
    #[prost(int32, optional, tag = "1")]
    pub hand_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub effect_name_hash_code: ::core::option::Option<u32>,
    #[prost(float, optional, tag = "3")]
    pub effect_scale: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageAnimStateGraphState {
    #[prost(int32, optional, tag = "1")]
    pub entity_index: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageUpdateCssClasses {
    #[prost(int32, optional, tag = "1")]
    pub target_world_panel: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub css_classes: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub is_add: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageServerFrameTime {
    #[prost(float, optional, tag = "1")]
    pub frame_time: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct CUserMessageLagCompensationError {
    #[prost(float, optional, tag = "1")]
    pub distance: ::core::option::Option<f32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageRequestDllStatus {
    #[prost(string, optional, tag = "1")]
    pub dll_action: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub full_report: ::core::option::Option<bool>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageRequestInventory {
    #[prost(int32, optional, tag = "1")]
    pub inventory: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub offset: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub options: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CUserMessageRequestDiagnostic {
    #[prost(message, repeated, tag = "1")]
    pub diagnostics: ::prost::alloc::vec::Vec<c_user_message_request_diagnostic::Diagnostic>,
}
pub mod c_user_message_request_diagnostic {
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
    #[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Criteria {
        #[prost(uint32, optional, tag = "1")]
        pub name_symbol: ::core::option::Option<u32>,
        #[prost(string, optional, tag = "2")]
        pub value: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
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
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CUserMessageUsageReport {
    #[prost(string, optional, tag = "1")]
    pub usage: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EBaseUserMessages {
    UmAchievementEvent = 101,
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
    UmUserSentBugBug = 167,
    UmUsageReport = 168,
    UmMaxBase = 200,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EBaseEntityMessages {
    EmPlayJingle = 136,
    EmScreenOverlay = 137,
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
    GameParticleManagerEventAddFan = 36,
    GameParticleManagerEventUpdateFan = 37,
    GameParticleManagerEventSetClusterGrowth = 38,
    GameParticleManagerEventRemoveFan = 39,
    GameParticleManagerEventCreateSmokeGrid = 40,
    GameParticleManagerEventSetOverrideTexture = 41,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EHapticPulseType {
    VrHandHapticPulseLight = 0,
    VrHandHapticPulseMedium = 1,
    VrHandHapticPulseStrong = 2,
}
