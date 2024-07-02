use crate::proto::{CMsgDotaCombatLogEntry, DotaCombatlogTypes};
use crate::string_table::StringTable;
use anyhow::{anyhow, Context, Result};

#[derive(Clone)]
pub struct CombatLogEntry<'a> {
    pub(crate) names: &'a StringTable,
    pub(crate) log: CMsgDotaCombatLogEntry,
}

impl<'a> CombatLogEntry<'a> {
    pub fn type_(&self) -> DotaCombatlogTypes {
        self.log.r#type()
    }

    pub fn target_name(&self) -> Result<&str> {
        self.log
            .target_name
            .and_then(|id| {
                self.names
                    .items
                    .get(id as usize)
                    .map(|name| name.key.as_ref())
            })
            .with_context(|| anyhow!("No target name for {:?}", self.type_()))
    }

    pub fn target_source_name(&self) -> Result<&str> {
        self.log
            .target_source_name
            .and_then(|id| {
                self.names
                    .items
                    .get(id as usize)
                    .map(|name| name.key.as_ref())
            })
            .with_context(|| anyhow!("No target source name for {:?}", self.type_()))
    }

    pub fn attacker_name(&self) -> Result<&str> {
        self.log
            .attacker_name
            .and_then(|id| {
                self.names
                    .items
                    .get(id as usize)
                    .map(|name| name.key.as_ref())
            })
            .with_context(|| anyhow!("No attacker name for {:?}", self.type_()))
    }

    pub fn damage_source_name(&self) -> Result<&str> {
        self.log
            .damage_source_name
            .and_then(|id| {
                self.names
                    .items
                    .get(id as usize)
                    .map(|name| name.key.as_ref())
            })
            .with_context(|| anyhow!("No damage source name for {:?}", self.type_()))
    }

    pub fn inflictor_name(&self) -> Result<&str> {
        self.log
            .inflictor_name
            .and_then(|id| {
                self.names
                    .items
                    .get(id as usize)
                    .map(|name| name.key.as_ref())
            })
            .with_context(|| anyhow!("No inflictor name for {:?}", self.type_()))
    }

    pub fn value_name(&self) -> Result<&str> {
        self.log
            .value
            .and_then(|id| {
                self.names
                    .items
                    .get(id as usize)
                    .map(|name| name.key.as_ref())
            })
            .with_context(|| anyhow!("No value name for {:?}", self.type_()))
    }

    pub fn attacker_illusion(&self) -> Result<bool> {
        self.log
            .is_attacker_illusion
            .with_context(|| anyhow!("No attacker illusion for {:?}", self.type_()))
    }

    pub fn attacker_hero(&self) -> Result<bool> {
        self.log
            .is_attacker_hero
            .with_context(|| anyhow!("No attacker hero for {:?}", self.type_()))
    }

    pub fn target_illusion(&self) -> Result<bool> {
        self.log
            .is_target_illusion
            .with_context(|| anyhow!("No target illusion for {:?}", self.type_()))
    }

    pub fn target_hero(&self) -> Result<bool> {
        self.log
            .is_target_hero
            .with_context(|| anyhow!("No target hero for {:?}", self.type_()))
    }

    pub fn visible_radiant(&self) -> Result<bool> {
        self.log
            .is_visible_radiant
            .with_context(|| anyhow!("No visible radiant for {:?}", self.type_()))
    }

    pub fn visible_dire(&self) -> Result<bool> {
        self.log
            .is_visible_dire
            .with_context(|| anyhow!("No visible dire for {:?}", self.type_()))
    }

    pub fn value(&self) -> Result<u32> {
        self.log
            .value
            .with_context(|| anyhow!("No value for {:?}", self.type_()))
    }

    pub fn health(&self) -> Result<i32> {
        self.log
            .health
            .with_context(|| anyhow!("No health for {:?}", self.type_()))
    }

    pub fn timestamp(&self) -> Result<f32> {
        self.log
            .timestamp
            .with_context(|| anyhow!("No timestamp for {:?}", self.type_()))
    }

    pub fn stun_duration(&self) -> Result<f32> {
        self.log
            .stun_duration
            .with_context(|| anyhow!("No stun duration for {:?}", self.type_()))
    }

    pub fn slow_duration(&self) -> Result<f32> {
        self.log
            .slow_duration
            .with_context(|| anyhow!("No slow duration for {:?}", self.type_()))
    }

    pub fn ability_toggle_on(&self) -> Result<bool> {
        self.log
            .is_ability_toggle_on
            .with_context(|| anyhow!("No ability toggle on for {:?}", self.type_()))
    }

    pub fn ability_toggle_off(&self) -> Result<bool> {
        self.log
            .is_ability_toggle_off
            .with_context(|| anyhow!("No ability toggle off for {:?}", self.type_()))
    }

    pub fn ability_level(&self) -> Result<u32> {
        self.log
            .ability_level
            .with_context(|| anyhow!("No ability level for {:?}", self.type_()))
    }

    pub fn location_x(&self) -> Result<f32> {
        self.log
            .location_x
            .with_context(|| anyhow!("No location x for {:?}", self.type_()))
    }

    pub fn location_y(&self) -> Result<f32> {
        self.log
            .location_y
            .with_context(|| anyhow!("No location y for {:?}", self.type_()))
    }

    pub fn gold_reason(&self) -> Result<u32> {
        self.log
            .gold_reason
            .with_context(|| anyhow!("No gold reason for {:?}", self.type_()))
    }

    pub fn timestamp_raw(&self) -> Result<f32> {
        self.log
            .timestamp_raw
            .with_context(|| anyhow!("No timestamp raw for {:?}", self.type_()))
    }

    pub fn modifier_duration(&self) -> Result<f32> {
        self.log
            .modifier_duration
            .with_context(|| anyhow!("No modifier duration for {:?}", self.type_()))
    }

    pub fn xp_reason(&self) -> Result<u32> {
        self.log
            .xp_reason
            .with_context(|| anyhow!("No xp reason for {:?}", self.type_()))
    }

    pub fn last_hits(&self) -> Result<u32> {
        self.log
            .last_hits
            .with_context(|| anyhow!("No last hits for {:?}", self.type_()))
    }

    pub fn attacker_team(&self) -> Result<u32> {
        self.log
            .attacker_team
            .with_context(|| anyhow!("No attacker team for {:?}", self.type_()))
    }

    pub fn target_team(&self) -> Result<u32> {
        self.log
            .target_team
            .with_context(|| anyhow!("No target team for {:?}", self.type_()))
    }

    pub fn obs_wards_placed(&self) -> Result<u32> {
        self.log
            .obs_wards_placed
            .with_context(|| anyhow!("No obs wards placed for {:?}", self.type_()))
    }

    pub fn assist_player0(&self) -> Result<u32> {
        self.log
            .assist_player0
            .with_context(|| anyhow!("No assist player0 for {:?}", self.type_()))
    }

    pub fn assist_player1(&self) -> Result<u32> {
        self.log
            .assist_player1
            .with_context(|| anyhow!("No assist player1 for {:?}", self.type_()))
    }

    pub fn assist_player2(&self) -> Result<u32> {
        self.log
            .assist_player2
            .with_context(|| anyhow!("No assist player2 for {:?}", self.type_()))
    }

    pub fn assist_player3(&self) -> Result<u32> {
        self.log
            .assist_player3
            .with_context(|| anyhow!("No assist player3 for {:?}", self.type_()))
    }

    pub fn stack_count(&self) -> Result<u32> {
        self.log
            .stack_count
            .with_context(|| anyhow!("No stack count for {:?}", self.type_()))
    }

    pub fn hidden_modifier(&self) -> Result<bool> {
        self.log
            .hidden_modifier
            .with_context(|| anyhow!("No hidden modifier for {:?}", self.type_()))
    }

    pub fn target_building(&self) -> Result<bool> {
        self.log
            .is_target_building
            .with_context(|| anyhow!("No target building for {:?}", self.type_()))
    }

    pub fn neutral_camp_type(&self) -> Result<u32> {
        self.log
            .neutral_camp_type
            .with_context(|| anyhow!("No neutral camp type for {:?}", self.type_()))
    }

    pub fn rune_type(&self) -> Result<u32> {
        self.log
            .rune_type
            .with_context(|| anyhow!("No rune type for {:?}", self.type_()))
    }

    pub fn assist_players(&self) -> &[i32] {
        &self.log.assist_players
    }

    pub fn heal_save(&self) -> Result<bool> {
        self.log
            .is_heal_save
            .with_context(|| anyhow!("No heal save for {:?}", self.type_()))
    }

    pub fn ultimate_ability(&self) -> Result<bool> {
        self.log
            .is_ultimate_ability
            .with_context(|| anyhow!("No ultimate ability for {:?}", self.type_()))
    }

    pub fn attacker_hero_level(&self) -> Result<u32> {
        self.log
            .attacker_hero_level
            .with_context(|| anyhow!("No attacker hero level for {:?}", self.type_()))
    }

    pub fn target_hero_level(&self) -> Result<u32> {
        self.log
            .target_hero_level
            .with_context(|| anyhow!("No target hero level for {:?}", self.type_()))
    }

    pub fn xpm(&self) -> Result<u32> {
        self.log
            .xpm
            .with_context(|| anyhow!("No xpm for {:?}", self.type_()))
    }

    pub fn gpm(&self) -> Result<u32> {
        self.log
            .gpm
            .with_context(|| anyhow!("No gpm for {:?}", self.type_()))
    }

    pub fn event_location(&self) -> Result<u32> {
        self.log
            .event_location
            .with_context(|| anyhow!("No event location for {:?}", self.type_()))
    }

    pub fn target_is_self(&self) -> Result<bool> {
        self.log
            .target_is_self
            .with_context(|| anyhow!("No target is self for {:?}", self.type_()))
    }

    pub fn damage_type(&self) -> Result<u32> {
        self.log
            .damage_type
            .with_context(|| anyhow!("No damage type for {:?}", self.type_()))
    }

    pub fn invisibility_modifier(&self) -> Result<bool> {
        self.log
            .invisibility_modifier
            .with_context(|| anyhow!("No invisibility modifier for {:?}", self.type_()))
    }

    pub fn damage_category(&self) -> Result<u32> {
        self.log
            .damage_category
            .with_context(|| anyhow!("No damage category for {:?}", self.type_()))
    }

    pub fn networth(&self) -> Result<u32> {
        self.log
            .networth
            .with_context(|| anyhow!("No networth for {:?}", self.type_()))
    }

    pub fn building_type(&self) -> Result<u32> {
        self.log
            .building_type
            .with_context(|| anyhow!("No building type for {:?}", self.type_()))
    }

    pub fn modifier_elapsed_duration(&self) -> Result<f32> {
        self.log
            .modifier_elapsed_duration
            .with_context(|| anyhow!("No modifier elapsed duration for {:?}", self.type_()))
    }

    pub fn silence_modifier(&self) -> Result<bool> {
        self.log
            .silence_modifier
            .with_context(|| anyhow!("No silence modifier for {:?}", self.type_()))
    }

    pub fn heal_from_lifesteal(&self) -> Result<bool> {
        self.log
            .heal_from_lifesteal
            .with_context(|| anyhow!("No heal from lifesteal for {:?}", self.type_()))
    }

    pub fn modifier_purged(&self) -> Result<bool> {
        self.log
            .modifier_purged
            .with_context(|| anyhow!("No modifier purged for {:?}", self.type_()))
    }

    pub fn spell_evaded(&self) -> Result<bool> {
        self.log
            .spell_evaded
            .with_context(|| anyhow!("No spell evaded for {:?}", self.type_()))
    }

    pub fn motion_controller_modifier(&self) -> Result<bool> {
        self.log
            .motion_controller_modifier
            .with_context(|| anyhow!("No motion controller modifier for {:?}", self.type_()))
    }

    pub fn long_range_kill(&self) -> Result<bool> {
        self.log
            .long_range_kill
            .with_context(|| anyhow!("No long range kill for {:?}", self.type_()))
    }

    pub fn modifier_purge_ability(&self) -> Result<u32> {
        self.log
            .modifier_purge_ability
            .with_context(|| anyhow!("No modifier purge ability for {:?}", self.type_()))
    }

    pub fn modifier_purge_npc(&self) -> Result<u32> {
        self.log
            .modifier_purge_npc
            .with_context(|| anyhow!("No modifier purge npc for {:?}", self.type_()))
    }

    pub fn root_modifier(&self) -> Result<bool> {
        self.log
            .root_modifier
            .with_context(|| anyhow!("No root modifier for {:?}", self.type_()))
    }

    pub fn total_unit_death_count(&self) -> Result<u32> {
        self.log
            .total_unit_death_count
            .with_context(|| anyhow!("No total unit death count for {:?}", self.type_()))
    }

    pub fn aura_modifier(&self) -> Result<bool> {
        self.log
            .aura_modifier
            .with_context(|| anyhow!("No aura modifier for {:?}", self.type_()))
    }

    pub fn armor_debuff_modifier(&self) -> Result<bool> {
        self.log
            .armor_debuff_modifier
            .with_context(|| anyhow!("No armor debuff modifier for {:?}", self.type_()))
    }

    pub fn no_physical_damage_modifier(&self) -> Result<bool> {
        self.log
            .no_physical_damage_modifier
            .with_context(|| anyhow!("No no physical damage modifier for {:?}", self.type_()))
    }

    pub fn modifier_ability(&self) -> Result<u32> {
        self.log
            .modifier_ability
            .with_context(|| anyhow!("No modifier ability for {:?}", self.type_()))
    }

    pub fn modifier_hidden(&self) -> Result<bool> {
        self.log
            .modifier_hidden
            .with_context(|| anyhow!("No modifier hidden for {:?}", self.type_()))
    }

    pub fn inflictor_is_stolen_ability(&self) -> Result<bool> {
        self.log
            .inflictor_is_stolen_ability
            .with_context(|| anyhow!("No inflictor is stolen ability for {:?}", self.type_()))
    }

    pub fn kill_eater_event(&self) -> Result<u32> {
        self.log
            .kill_eater_event
            .with_context(|| anyhow!("No kill eater event for {:?}", self.type_()))
    }

    pub fn unit_status_label(&self) -> Result<u32> {
        self.log
            .unit_status_label
            .with_context(|| anyhow!("No unit status label for {:?}", self.type_()))
    }

    pub fn spell_generated_attack(&self) -> Result<bool> {
        self.log
            .spell_generated_attack
            .with_context(|| anyhow!("No spell generated attack for {:?}", self.type_()))
    }

    pub fn at_night_time(&self) -> Result<bool> {
        self.log
            .at_night_time
            .with_context(|| anyhow!("No at night time for {:?}", self.type_()))
    }

    pub fn attacker_has_scepter(&self) -> Result<bool> {
        self.log
            .attacker_has_scepter
            .with_context(|| anyhow!("No attacker has scepter for {:?}", self.type_()))
    }

    pub fn neutral_camp_team(&self) -> Result<u32> {
        self.log
            .neutral_camp_team
            .with_context(|| anyhow!("No neutral camp team for {:?}", self.type_()))
    }

    pub fn regenerated_health(&self) -> Result<f32> {
        self.log
            .regenerated_health
            .with_context(|| anyhow!("No regenerated health for {:?}", self.type_()))
    }

    pub fn will_reincarnate(&self) -> Result<bool> {
        self.log
            .will_reincarnate
            .with_context(|| anyhow!("No will reincarnate for {:?}", self.type_()))
    }

    pub fn uses_charges(&self) -> Result<bool> {
        self.log
            .uses_charges
            .with_context(|| anyhow!("No uses charges for {:?}", self.type_()))
    }
}
