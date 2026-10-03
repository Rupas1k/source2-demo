//! Combat log entries for Deadlock.

use crate::error::CombatLogError;
use crate::proto::{CMsgCitadelCombatLogEntry, ECitadelCombatLogTypes};
use crate::string_table::StringTable;

/// Wrapper for Deadlock combat log entries.
///
/// # Examples
///
/// ```no_run
/// use source2_demo::prelude::*;
/// use source2_demo::proto::ECitadelCombatLogTypes;
///
/// #[derive(Default)]
/// struct CombatLog;
///
/// #[observer]
/// #[uses_combat_log]
/// impl CombatLog {
///     #[on_combat_log]
///     fn on_combat(&mut self, entry: &CitadelCombatLogEntry) -> ObserverResult {
///         if entry.r#type() == ECitadelCombatLogTypes::KECitadelCombatLogDeath {
///             println!("{} killed {}", entry.attacker_name()?, entry.target_name()?);
///         }
///
///         Ok(())
///     }
/// }
/// ```
#[derive(Clone)]
pub struct CitadelCombatLogEntry<'a> {
    pub(crate) names: &'a StringTable,
    pub(crate) log: CMsgCitadelCombatLogEntry,
}

macro_rules! define_combat_log_getters {
    ($($name:ident: $type:ty),*) => {
        $(
            #[doc = concat!("Returns the `", stringify!($name), "` property from the combat log entry.")]
            pub fn $name(&self) -> Result<$type, CombatLogError> {
                self.log.$name.ok_or_else(|| {
                    CombatLogError::EmptyProperty(stringify!($name).into(), format!("{:?}", self.r#type()))
                })
            }
        )*
    };
}

macro_rules! define_name_getter {
    ($($fn_name:ident: $log_prop:ident),*) => {
        $(
            #[doc = concat!("Returns the name associated with the `", stringify!($log_prop), "` property.")]
            pub fn $fn_name(&self) -> Result<&str, CombatLogError> {
                self.log.$log_prop
                    .and_then(|id| self.names.items.get(id as usize).map(|name| name.key.as_ref()))
                    .ok_or_else(|| {
                        CombatLogError::EmptyName(stringify!($fn_name).into(), format!("{:?}", self.r#type()))
                    })
            }
        )*
    };
}

impl<'a> CitadelCombatLogEntry<'a> {
    /// Returns the Deadlock protobuf event type.
    pub fn r#type(&self) -> ECitadelCombatLogTypes {
        self.log.r#type()
    }

    /// Returns the underlying protobuf message.
    pub fn log(&self) -> &CMsgCitadelCombatLogEntry {
        &self.log
    }

    /// Returns the list of assisting player IDs.
    pub fn assist_players(&self) -> &[i32] {
        self.log.assist_players.as_slice()
    }

    define_name_getter! {
        target_name: target_name,
        target_source_name: target_source_name,
        attacker_name: attacker_name,
        damage_source_name: damage_source_name,
        inflictor_name: inflictor_name,
        value_name: value
    }

    define_combat_log_getters! {
        value: u32,
        is_attacker_hero: bool,
        is_target_hero: bool,
        is_visible_sapphire: bool,
        is_visible_amber: bool,
        health: i32,
        timestamp: f32,
        timestamp_raw: f32,
        attacker_team: u32,
        target_team: u32,
        is_ability_toggle_on: bool,
        is_ability_toggle_off: bool,
        damage_type: u32,
        networth: u32,
        currency_type: u32,
        currency_source: u32,
        location_x: f32,
        location_y: f32,
        ability_upgrade_bits: i32,
        modifier_duration: f32,
        target_objective_id: u32,
        target_is_self: bool,
        target_class: i32,
        is_ultimate_ability: bool,
        uses_charges: bool,
        inflictor_is_stolen_ability: bool,
        spell_generated_attack: bool,
        heal_from_lifesteal: bool,
        regenerated_health: f32,
        will_reincarnate: bool,
        modifier_ability: u32,
        modifier_hidden: bool,
        modifier_elapsed_duration: f32,
        hidden_modifier: bool,
        silence_modifier: bool,
        modifier_purged: bool,
        long_range_kill: bool,
        aura_modifier: bool,
        immobilize_modifier: bool,
        modifier_purge_npc: i32,
        modifier_purge_ability: i32,
        modifier_purged_duration: f32,
        movement_control_modifier: bool,
        powerup_type: i32,
        is_primary_weapon: bool,
        is_headshot: bool,
        ability_name: i32,
        full_refresh: i32,
        hits: i32
    }
}
