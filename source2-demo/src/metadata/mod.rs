//! Inspection metadata for replay commands, packet messages and serializer-backed fields.
//! Command ranges address the original file. Packet messages address decompressed packet data.
//! Entity and baseline field types come from the replay serializer and current decoded state.
//! Enable the `reflection` feature for protobuf names and field descriptors.
mod entity;
pub use crate::reader::{
    commands, packet_data, packet_messages, Command, RawPacketMessage as PacketMessage,
};
pub(crate) use entity::inspect_fields;
#[cfg(feature = "entity-field-updates")]
pub use entity::EntityFieldUpdate;
pub use entity::{BaselineEntity, EntityField};
#[cfg(feature = "reflection")]
pub use source2_demo_protobufs::schema::{MessageFamily, MessageSchema};
