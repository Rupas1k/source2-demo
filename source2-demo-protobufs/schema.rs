//! Runtime lookup for the checked-in protobuf schemas.
use prost_reflect::{DescriptorPool, MessageDescriptor};
use std::collections::HashMap;

/// Namespace of a numeric protobuf identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageFamily {
    /// Outer demo command.
    Command,
    /// Message inside a packet.
    Packet,
}

/// Runtime protobuf names and descriptors for one replay game.
pub struct MessageSchema {
    commands: HashMap<u32, String>,
    packets: HashMap<u32, String>,
    pool: DescriptorPool,
}

impl MessageSchema {
    /// Select descriptors from the replay header's game directory. An unrecognized game uses common definitions.
    pub fn new(game_directory: &str) -> Result<Self, prost_reflect::DescriptorError> {
        let game = game_directory.trim_end_matches(['/', '\\']).rsplit(['/', '\\']).next().unwrap_or("");
        let bytes: &[u8] = match game {
            "dota" => include_bytes!(concat!(env!("OUT_DIR"), "/dota.bin")),
            "citadel" => include_bytes!(concat!(env!("OUT_DIR"), "/citadel.bin")),
            "csgo" | "cs2" => include_bytes!(concat!(env!("OUT_DIR"), "/cs2.bin")),
            _ => include_bytes!(concat!(env!("OUT_DIR"), "/common.bin")),
        };
        let pool = DescriptorPool::decode(bytes)?;
        let mut schema = Self {
            commands: HashMap::new(),
            packets: HashMap::new(),
            pool,
        };
        for (family, prefix, names) in [
            ("EDemoCommands", "DEM_", &["CDemo"][..]),
            ("NET_Messages", "net_", &["CNETMsg_"]),
            ("SVC_Messages", "svc_", &["CSVCMsg_"]),
            ("EBaseUserMessages", "UM_", &["CUserMessage"]),
            ("EBaseGameEvents", "GE_", &["CMsg"]),
            ("EDotaUserMessages", "DOTA_UM_", &["CDOTAUserMsg_"]),
            ("CitadelUserMessageIds", "k_EUserMsg_", &["CCitadelUserMsg_", "CCitadelUserMessage_"]),
            ("ECitadelGameEvents", "GE_", &["CMsg"]),
            ("ECstrike15UserMessages", "CS_UM_", &["CCSUsrMsg_"]),
            ("ECsgoGameEvents", "GE_", &["CMsgTE", "CMsg"]),
        ] {
            let Some(enumeration) = schema.pool.get_enum_by_name(family) else {
                continue;
            };
            for value in enumeration.values() {
                let Some(suffix) = value.name().strip_prefix(prefix) else {
                    continue;
                };
                let suffix = if family == "ECsgoGameEvents" {
                    suffix.trim_end_matches("Id")
                } else {
                    suffix
                };
                let name = names
                    .iter()
                    .map(|p| format!("{p}{suffix}"))
                    .find(|n| schema.pool.get_message_by_name(n).is_some())
                    .unwrap_or_else(|| value.name().to_owned());
                let registry = if family == "EDemoCommands" {
                    &mut schema.commands
                } else {
                    &mut schema.packets
                };
                registry.insert(value.number() as u32, name);
            }
        }
        Ok(schema)
    }

    /// Display name for a command or packet ID. Unknown IDs receive a family-specific label.
    pub fn name(&self, family: MessageFamily, id: u32) -> &str {
        let map = if family == MessageFamily::Command {
            &self.commands
        } else {
            &self.packets
        };
        map.get(&id).map(String::as_str).unwrap_or(if family == MessageFamily::Command {
            "Unknown command"
        } else {
            "Unknown message"
        })
    }

    /// Protobuf descriptor, including nested field metadata, when a mapping exists.
    pub fn descriptor(&self, family: MessageFamily, id: u32) -> Option<MessageDescriptor> {
        let name = if family == MessageFamily::Command && id == 8 {
            "CDemoPacket"
        } else {
            self.name(family, id)
        };
        let name = match name {
            "DOTA_UM_CombatLogDataHLTV" => "CMsgDOTACombatLogEntry",
            other => other,
        };
        self.pool.get_message_by_name(name)
    }
}
