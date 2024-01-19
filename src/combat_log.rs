use protogen::dota_shared_enums::{CMsgDOTACombatLogEntry, DOTA_COMBATLOG_TYPES};
use crate::string_table::StringTable;

pub struct CombatLog<'a> {
    pub(crate) names: &'a StringTable,
    pub(crate) log: CMsgDOTACombatLogEntry
}

impl<'a> CombatLog<'a> {
    pub fn type_(&self) -> DOTA_COMBATLOG_TYPES {
        self.log.type_()
    }

    pub fn target_name(&self) -> Option<&str> {
        if let Some(id) = self.log.target_name {
            if let Some(name) = self.names.items.get(&(id as i32)) {
                return Some(name.key.as_str())
            }
        }
        None
    }

    pub fn target_source_name(&self) -> Option<&str> {
        if let Some(id) = self.log.target_source_name {
            if let Some(name) = self.names.items.get(&(id as i32)) {
                return Some(name.key.as_str())
            }
        }
        None
    }

    pub fn attacker_name(&self) -> Option<&str> {
        if let Some(id) = self.log.attacker_name {
            if let Some(name) = self.names.items.get(&(id as i32)) {
                return Some(name.key.as_str())
            }
        }
        None
    }

    pub fn damage_source_name(&self) -> Option<&str> {
        if let Some(id) = self.log.damage_source_name {
            if let Some(name) = self.names.items.get(&(id as i32)) {
                return Some(name.key.as_str())
            }
        }
        None
    }

    pub fn inflictor_name(&self) -> Option<&str> {
        if let Some(id) = self.log.inflictor_name {
            if let Some(name) = self.names.items.get(&(id as i32)) {
                return Some(name.key.as_str())
            }
        }
        None
    }

    pub fn attacker_illusion(&self) -> Option<bool> {
        self.log.is_attacker_illusion
    }

    pub fn attacker_hero(&self) -> Option<bool> {
        self.log.is_attacker_hero
    }

    pub fn target_illusion(&self) -> Option<bool> {
        self.log.is_target_illusion
    }

    pub fn target_hero(&self) -> Option<bool> {
        self.log.is_target_hero
    }

    pub fn visible_radiant(&self) -> Option<bool> {
        self.log.is_visible_radiant
    }

    pub fn visible_dire(&self) -> Option<bool> {
        self.log.is_visible_dire
    }

    pub fn value(&self) -> Option<u32> {
        self.log.value
    }

    pub fn health(&self) -> Option<i32> {
        self.log.health
    }

    pub fn timestamp(&self) -> Option<f32> {
        self.log.timestamp
    }

    pub fn stun_duration(&self) -> Option<f32> {
        self.log.stun_duration
    }

    pub fn slow_duration(&self) -> Option<f32> {
        self.log.slow_duration
    }

    pub fn ability_toggle_on(&self) -> Option<bool> {
        self.log.is_ability_toggle_on
    }

    pub fn ability_toggle_off(&self) -> Option<bool> {
        self.log.is_ability_toggle_off
    }

    pub fn ability_level(&self) -> Option<u32> {
        self.log.ability_level
    }

    pub fn location_x(&self) -> Option<f32> {
        self.log.location_x
    }

    pub fn location_y(&self) -> Option<f32> {
        self.log.location_y
    }

    pub fn gold_reason(&self) -> Option<u32> {
        self.log.gold_reason
    }

    pub fn timestamp_raw(&self) -> Option<f32> {
        self.log.timestamp_raw
    }

    pub fn modifier_duration(&self) -> Option<f32> {
        self.log.modifier_duration
    }

    pub fn xp_reason(&self) -> Option<u32> {
        self.log.xp_reason
    }

    pub fn last_hits(&self) -> Option<u32> {
        self.log.last_hits
    }

    pub fn attacker_team(&self) -> Option<u32> {
        self.log.attacker_team
    }

    pub fn target_team(&self) -> Option<u32> {
        self.log.target_team
    }

    pub fn obs_wards_placed(&self) -> Option<u32> {
        self.log.obs_wards_placed
    }

    pub fn assist_player0(&self) -> Option<u32> {
        self.log.assist_player0
    }

    pub fn assist_player1(&self) -> Option<u32> {
        self.log.assist_player1
    }

    pub fn assist_player2(&self) -> Option<u32> {
        self.log.assist_player2
    }

    pub fn assist_player3(&self) -> Option<u32> {
        self.log.assist_player3
    }

    pub fn stack_count(&self) -> Option<u32> {
        self.log.stack_count
    }

    pub fn hidden_modifier(&self) -> Option<bool> {
        self.log.hidden_modifier
    }

    pub fn target_building(&self) -> Option<bool> {
        self.log.is_target_building
    }

    pub fn neutral_camp_type(&self) -> Option<u32> {
        self.log.neutral_camp_type
    }

    pub fn rune_type(&self) -> Option<u32> {
        self.log.rune_type
    }

    pub fn assist_players(&self) -> &Vec<i32> {
        &self.log.assist_players
    }

    pub fn heal_save(&self) -> Option<bool> {
        self.log.is_heal_save
    }

    pub fn ultimate_ability(&self) -> Option<bool> {
        self.log.is_ultimate_ability
    }

    pub fn attacker_hero_level(&self) -> Option<u32> {
        self.log.attacker_hero_level
    }

    pub fn target_hero_level(&self) -> Option<u32> {
        self.log.target_hero_level
    }

    pub fn xpm(&self) -> Option<u32> {
        self.log.xpm
    }

    pub fn gpm(&self) -> Option<u32> {
        self.log.gpm
    }

    pub fn event_location(&self) -> Option<u32> {
        self.log.event_location
    }

    pub fn target_is_self(&self) -> Option<bool> {
        self.log.target_is_self
    }

    pub fn damage_type(&self) -> Option<u32> {
        self.log.damage_type
    }

    pub fn invisibility_modifier(&self) -> Option<bool> {
        self.log.invisibility_modifier
    }

    pub fn damage_category(&self) -> Option<u32> {
        self.log.damage_category
    }

    pub fn networth(&self) -> Option<u32> {
        self.log.networth
    }

    pub fn building_type(&self) -> Option<u32> {
        self.log.building_type
    }

    pub fn modifier_elapsed_duration(&self) -> Option<f32> {
        self.log.modifier_elapsed_duration
    }

    pub fn silence_modifier(&self) -> Option<bool> {
        self.log.silence_modifier
    }

    pub fn heal_from_lifesteal(&self) -> Option<bool> {
        self.log.heal_from_lifesteal
    }

    pub fn modifier_purged(&self) -> Option<bool> {
        self.log.modifier_purged
    }

    pub fn spell_evaded(&self) -> Option<bool> {
        self.log.spell_evaded
    }

    pub fn motion_controller_modifier(&self) -> Option<bool> {
        self.log.motion_controller_modifier
    }

    pub fn long_range_kill(&self) -> Option<bool> {
        self.log.long_range_kill
    }

    pub fn modifier_purge_ability(&self) -> Option<u32> {
        self.log.modifier_purge_ability
    }

    pub fn modifier_purge_npc(&self) -> Option<u32> {
        self.log.modifier_purge_npc
    }

    pub fn root_modifier(&self) -> Option<bool> {
        self.log.root_modifier
    }

    pub fn total_unit_death_count(&self) -> Option<u32> {
        self.log.total_unit_death_count
    }

    pub fn aura_modifier(&self) -> Option<bool> {
        self.log.aura_modifier
    }

    pub fn armor_debuff_modifier(&self) -> Option<bool> {
        self.log.armor_debuff_modifier
    }

    pub fn no_physical_damage_modifier(&self) -> Option<bool> {
        self.log.no_physical_damage_modifier
    }

    pub fn modifier_ability(&self) -> Option<u32> {
        self.log.modifier_ability
    }

    pub fn modifier_hidden(&self) -> Option<bool> {
        self.log.modifier_hidden
    }

    pub fn inflictor_is_stolen_ability(&self) -> Option<bool> {
        self.log.inflictor_is_stolen_ability
    }

    pub fn kill_eater_event(&self) -> Option<u32> {
        self.log.kill_eater_event
    }

    pub fn unit_status_label(&self) -> Option<u32> {
        self.log.unit_status_label
    }

    pub fn spell_generated_attack(&self) -> Option<bool> {
        self.log.spell_generated_attack
    }

    pub fn at_night_time(&self) -> Option<bool> {
        self.log.at_night_time
    }

    pub fn attacker_has_scepter(&self) -> Option<bool> {
        self.log.attacker_has_scepter
    }

    pub fn neutral_camp_team(&self) -> Option<u32> {
        self.log.neutral_camp_team
    }

    pub fn regenerated_health(&self) -> Option<f32> {
        self.log.regenerated_health
    }

    pub fn will_reincarnate(&self) -> Option<bool> {
        self.log.will_reincarnate
    }

    pub fn uses_charges(&self) -> Option<bool> {
        self.log.uses_charges
    }
}