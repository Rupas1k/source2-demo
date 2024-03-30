use crate::proto::{CMsgDotaCombatLogEntry, DotaCombatlogTypes};
use crate::string_table::StringTable;
use crate::Result;
use anyhow::anyhow;

#[derive(Clone)]
pub struct CombatLog<'a> {
    pub(crate) names: &'a StringTable,
    pub(crate) log: CMsgDotaCombatLogEntry,
}

impl<'a> CombatLog<'a> {
    pub fn type_(&self) -> DotaCombatlogTypes {
        self.log.r#type()
    }

    pub fn target_name(&self) -> Result<&str> {
        self.log
            .target_name
            .and_then(|id| {
                self.names
                    .items
                    .get(&id.try_into().unwrap())
                    .map(|name| name.key.as_str())
            })
            .ok_or_else(|| anyhow!("No target name"))
    }

    pub fn target_source_name(&self) -> Result<&str> {
        self.log
            .target_source_name
            .and_then(|id| {
                self.names
                    .items
                    .get(&id.try_into().unwrap())
                    .map(|name| name.key.as_str())
            })
            .ok_or_else(|| anyhow!("No target source name"))
    }

    pub fn attacker_name(&self) -> Result<&str> {
        self.log
            .attacker_name
            .and_then(|id| {
                self.names
                    .items
                    .get(&id.try_into().unwrap())
                    .map(|name| name.key.as_str())
            })
            .ok_or_else(|| anyhow!("No attacker name"))
    }

    pub fn damage_source_name(&self) -> Result<&str> {
        self.log
            .damage_source_name
            .and_then(|id| {
                self.names
                    .items
                    .get(&id.try_into().unwrap())
                    .map(|name| name.key.as_str())
            })
            .ok_or_else(|| anyhow!("No damage source name"))
    }

    pub fn inflictor_name(&self) -> Result<&str> {
        self.log
            .inflictor_name
            .and_then(|id| {
                self.names
                    .items
                    .get(&id.try_into().unwrap())
                    .map(|name| name.key.as_str())
            })
            .ok_or_else(|| anyhow!("No inflictor name"))
    }

    pub fn attacker_illusion(&self) -> Result<bool> {
        self.log
            .is_attacker_illusion
            .ok_or_else(|| anyhow!("No attacker illusion"))
    }

    pub fn attacker_hero(&self) -> Result<bool> {
        self.log
            .is_attacker_hero
            .ok_or_else(|| anyhow!("No attacker hero"))
    }

    pub fn target_illusion(&self) -> Result<bool> {
        self.log
            .is_target_illusion
            .ok_or_else(|| anyhow!("No target illusion"))
    }

    pub fn target_hero(&self) -> Result<bool> {
        self.log
            .is_target_hero
            .ok_or_else(|| anyhow!("No target hero"))
    }

    pub fn visible_radiant(&self) -> Result<bool> {
        self.log
            .is_visible_radiant
            .ok_or_else(|| anyhow!("No visible radiant"))
    }

    pub fn visible_dire(&self) -> Result<bool> {
        self.log
            .is_visible_dire
            .ok_or_else(|| anyhow!("No visible dire"))
    }

    pub fn value(&self) -> Result<u32> {
        self.log.value.ok_or_else(|| anyhow!("No value"))
    }

    pub fn health(&self) -> Result<i32> {
        self.log.health.ok_or_else(|| anyhow!("No health"))
    }

    pub fn timestamp(&self) -> Result<f32> {
        self.log.timestamp.ok_or_else(|| anyhow!("No timestamp"))
    }

    pub fn stun_duration(&self) -> Result<f32> {
        self.log
            .stun_duration
            .ok_or_else(|| anyhow!("No stun duration"))
    }

    pub fn slow_duration(&self) -> Result<f32> {
        self.log
            .slow_duration
            .ok_or_else(|| anyhow!("No slow duration"))
    }

    pub fn ability_toggle_on(&self) -> Result<bool> {
        self.log
            .is_ability_toggle_on
            .ok_or_else(|| anyhow!("No ability toggle on"))
    }

    pub fn ability_toggle_off(&self) -> Result<bool> {
        self.log
            .is_ability_toggle_off
            .ok_or_else(|| anyhow!("No ability toggle off"))
    }

    pub fn ability_level(&self) -> Result<u32> {
        self.log
            .ability_level
            .ok_or_else(|| anyhow!("No ability level"))
    }

    pub fn location_x(&self) -> Result<f32> {
        self.log.location_x.ok_or_else(|| anyhow!("No location x"))
    }

    pub fn location_y(&self) -> Result<f32> {
        self.log.location_y.ok_or_else(|| anyhow!("No location y"))
    }

    pub fn gold_reason(&self) -> Result<u32> {
        self.log
            .gold_reason
            .ok_or_else(|| anyhow!("No gold reason"))
    }

    pub fn timestamp_raw(&self) -> Result<f32> {
        self.log
            .timestamp_raw
            .ok_or_else(|| anyhow!("No timestamp raw"))
    }

    pub fn modifier_duration(&self) -> Result<f32> {
        self.log
            .modifier_duration
            .ok_or_else(|| anyhow!("No modifier duration"))
    }

    pub fn xp_reason(&self) -> Result<u32> {
        self.log.xp_reason.ok_or_else(|| anyhow!("No xp reason"))
    }

    pub fn last_hits(&self) -> Result<u32> {
        self.log.last_hits.ok_or_else(|| anyhow!("No last hits"))
    }

    pub fn attacker_team(&self) -> Result<u32> {
        self.log
            .attacker_team
            .ok_or_else(|| anyhow!("No attacker team"))
    }

    pub fn target_team(&self) -> Result<u32> {
        self.log
            .target_team
            .ok_or_else(|| anyhow!("No target team"))
    }

    pub fn obs_wards_placed(&self) -> Result<u32> {
        self.log
            .obs_wards_placed
            .ok_or_else(|| anyhow!("No obs wards placed"))
    }

    pub fn assist_player0(&self) -> Result<u32> {
        self.log
            .assist_player0
            .ok_or_else(|| anyhow!("No assist player0"))
    }

    pub fn assist_player1(&self) -> Result<u32> {
        self.log
            .assist_player1
            .ok_or_else(|| anyhow!("No assist player1"))
    }

    pub fn assist_player2(&self) -> Result<u32> {
        self.log
            .assist_player2
            .ok_or_else(|| anyhow!("No assist player2"))
    }

    pub fn assist_player3(&self) -> Result<u32> {
        self.log
            .assist_player3
            .ok_or_else(|| anyhow!("No assist player3"))
    }

    pub fn stack_count(&self) -> Result<u32> {
        self.log
            .stack_count
            .ok_or_else(|| anyhow!("No stack count"))
    }

    pub fn hidden_modifier(&self) -> Result<bool> {
        self.log
            .hidden_modifier
            .ok_or_else(|| anyhow!("No hidden modifier"))
    }

    pub fn target_building(&self) -> Result<bool> {
        self.log
            .is_target_building
            .ok_or_else(|| anyhow!("No target building"))
    }

    pub fn neutral_camp_type(&self) -> Result<u32> {
        self.log
            .neutral_camp_type
            .ok_or_else(|| anyhow!("No neutral camp type"))
    }

    pub fn rune_type(&self) -> Result<u32> {
        self.log.rune_type.ok_or_else(|| anyhow!("No rune type"))
    }

    pub fn assist_players(&self) -> &[i32] {
        &self.log.assist_players
    }

    pub fn heal_save(&self) -> Result<bool> {
        self.log.is_heal_save.ok_or_else(|| anyhow!("No heal save"))
    }

    pub fn ultimate_ability(&self) -> Result<bool> {
        self.log
            .is_ultimate_ability
            .ok_or_else(|| anyhow!("No ultimate ability"))
    }

    pub fn attacker_hero_level(&self) -> Result<u32> {
        self.log
            .attacker_hero_level
            .ok_or_else(|| anyhow!("No attacker hero level"))
    }

    pub fn target_hero_level(&self) -> Result<u32> {
        self.log
            .target_hero_level
            .ok_or_else(|| anyhow!("No target hero level"))
    }

    pub fn xpm(&self) -> Result<u32> {
        self.log.xpm.ok_or_else(|| anyhow!("No xpm"))
    }

    pub fn gpm(&self) -> Result<u32> {
        self.log.gpm.ok_or_else(|| anyhow!("No gpm"))
    }

    pub fn event_location(&self) -> Result<u32> {
        self.log
            .event_location
            .ok_or_else(|| anyhow!("No event location"))
    }

    pub fn target_is_self(&self) -> Result<bool> {
        self.log
            .target_is_self
            .ok_or_else(|| anyhow!("No target is self"))
    }

    pub fn damage_type(&self) -> Result<u32> {
        self.log
            .damage_type
            .ok_or_else(|| anyhow!("No damage type"))
    }

    pub fn invisibility_modifier(&self) -> Result<bool> {
        self.log
            .invisibility_modifier
            .ok_or_else(|| anyhow!("No invisibility modifier"))
    }

    pub fn damage_category(&self) -> Result<u32> {
        self.log
            .damage_category
            .ok_or_else(|| anyhow!("No damage category"))
    }

    pub fn networth(&self) -> Result<u32> {
        self.log.networth.ok_or_else(|| anyhow!("No networth"))
    }

    pub fn building_type(&self) -> Result<u32> {
        self.log
            .building_type
            .ok_or_else(|| anyhow!("No building type"))
    }

    pub fn modifier_elapsed_duration(&self) -> Result<f32> {
        self.log
            .modifier_elapsed_duration
            .ok_or_else(|| anyhow!("No modifier elapsed duration"))
    }

    pub fn silence_modifier(&self) -> Result<bool> {
        self.log
            .silence_modifier
            .ok_or_else(|| anyhow!("No silence modifier"))
    }

    pub fn heal_from_lifesteal(&self) -> Result<bool> {
        self.log
            .heal_from_lifesteal
            .ok_or_else(|| anyhow!("No heal from lifesteal"))
    }

    pub fn modifier_purged(&self) -> Result<bool> {
        self.log
            .modifier_purged
            .ok_or_else(|| anyhow!("No modifier purged"))
    }

    pub fn spell_evaded(&self) -> Result<bool> {
        self.log
            .spell_evaded
            .ok_or_else(|| anyhow!("No spell evaded"))
    }

    pub fn motion_controller_modifier(&self) -> Result<bool> {
        self.log
            .motion_controller_modifier
            .ok_or_else(|| anyhow!("No motion controller modifier"))
    }

    pub fn long_range_kill(&self) -> Result<bool> {
        self.log
            .long_range_kill
            .ok_or_else(|| anyhow!("No long range kill"))
    }

    pub fn modifier_purge_ability(&self) -> Result<u32> {
        self.log
            .modifier_purge_ability
            .ok_or_else(|| anyhow!("No modifier purge ability"))
    }

    pub fn modifier_purge_npc(&self) -> Result<u32> {
        self.log
            .modifier_purge_npc
            .ok_or_else(|| anyhow!("No modifier purge npc"))
    }

    pub fn root_modifier(&self) -> Result<bool> {
        self.log
            .root_modifier
            .ok_or_else(|| anyhow!("No root modifier"))
    }

    pub fn total_unit_death_count(&self) -> Result<u32> {
        self.log
            .total_unit_death_count
            .ok_or_else(|| anyhow!("No total unit death count"))
    }

    pub fn aura_modifier(&self) -> Result<bool> {
        self.log
            .aura_modifier
            .ok_or_else(|| anyhow!("No aura modifier"))
    }

    pub fn armor_debuff_modifier(&self) -> Result<bool> {
        self.log
            .armor_debuff_modifier
            .ok_or_else(|| anyhow!("No armor debuff modifier"))
    }

    pub fn no_physical_damage_modifier(&self) -> Result<bool> {
        self.log
            .no_physical_damage_modifier
            .ok_or_else(|| anyhow!("No no physical damage modifier"))
    }

    pub fn modifier_ability(&self) -> Result<u32> {
        self.log
            .modifier_ability
            .ok_or_else(|| anyhow!("No modifier ability"))
    }

    pub fn modifier_hidden(&self) -> Result<bool> {
        self.log
            .modifier_hidden
            .ok_or_else(|| anyhow!("No modifier hidden"))
    }

    pub fn inflictor_is_stolen_ability(&self) -> Result<bool> {
        self.log
            .inflictor_is_stolen_ability
            .ok_or_else(|| anyhow!("No inflictor is stolen ability"))
    }

    pub fn kill_eater_event(&self) -> Result<u32> {
        self.log
            .kill_eater_event
            .ok_or_else(|| anyhow!("No kill eater event"))
    }

    pub fn unit_status_label(&self) -> Result<u32> {
        self.log
            .unit_status_label
            .ok_or_else(|| anyhow!("No unit status label"))
    }

    pub fn spell_generated_attack(&self) -> Result<bool> {
        self.log
            .spell_generated_attack
            .ok_or_else(|| anyhow!("No spell generated attack"))
    }

    pub fn at_night_time(&self) -> Result<bool> {
        self.log
            .at_night_time
            .ok_or_else(|| anyhow!("No at night time"))
    }

    pub fn attacker_has_scepter(&self) -> Result<bool> {
        self.log
            .attacker_has_scepter
            .ok_or_else(|| anyhow!("No attacker has scepter"))
    }

    pub fn neutral_camp_team(&self) -> Result<u32> {
        self.log
            .neutral_camp_team
            .ok_or_else(|| anyhow!("No neutral camp team"))
    }

    pub fn regenerated_health(&self) -> Result<f32> {
        self.log
            .regenerated_health
            .ok_or_else(|| anyhow!("No regenerated health"))
    }

    pub fn will_reincarnate(&self) -> Result<bool> {
        self.log
            .will_reincarnate
            .ok_or_else(|| anyhow!("No will reincarnate"))
    }

    pub fn uses_charges(&self) -> Result<bool> {
        self.log
            .uses_charges
            .ok_or_else(|| anyhow!("No uses charges"))
    }
}
