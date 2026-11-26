//! Inspection metadata for replay commands, packet messages and serializer-backed fields.
//! Command ranges address the original file. Packet messages address decompressed packet data.
//! Entity and baseline field types come from the replay serializer and current decoded state.
mod entity;
pub use crate::reader::{
    commands, packet_data, packet_messages, Command, RawPacketMessage as PacketMessage,
};
pub(crate) use entity::inspect_fields;
pub use entity::{BaselineEntity, EntityField};
