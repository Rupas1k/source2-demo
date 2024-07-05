use crate::proto::{CMsgDotaCombatLogEntry, DotaCombatlogTypes};
use crate::string_table::StringTable;
use anyhow::{anyhow, Context, Result};

#[derive(Clone)]
pub struct CombatLogEntry<'a> {
    pub(crate) names: &'a StringTable,
    pub(crate) log: CMsgDotaCombatLogEntry,
}

#[derive(thiserror::Error, Debug)]
pub enum CombatLogError {
    #[error("No {0} for {1}")]
    EmptyProperty(String, String),
    #[error("No {0} for {1}")]
    EmptyName(String, String),
}

impl<'a> CombatLogEntry<'a> {
    pub fn type_(&self) -> DotaCombatlogTypes {
        self.log.r#type()
    }

    pub fn target_name(&self) -> Result<&str, CombatLogError> {
        self.log
            .target_name
            .and_then(|id| {
                self.names
                    .items
                    .get(id as usize)
                    .map(|name| name.key.as_ref())
            })
            .ok_or_else(|| {
                CombatLogError::EmptyName("target_name".into(), format!("{:?}", self.type_()))
            })
    }

    pub fn target_source_name(&self) -> Result<&str, CombatLogError> {
        self.log
            .target_source_name
            .and_then(|id| {
                self.names
                    .items
                    .get(id as usize)
                    .map(|name| name.key.as_ref())
            })
            .ok_or_else(|| {
                CombatLogError::EmptyName(
                    "target_source_name".into(),
                    format!("{:?}", self.type_()),
                )
            })
    }

    pub fn attacker_name(&self) -> Result<&str, CombatLogError> {
        self.log
            .attacker_name
            .and_then(|id| {
                self.names
                    .items
                    .get(id as usize)
                    .map(|name| name.key.as_ref())
            })
            .ok_or_else(|| {
                CombatLogError::EmptyName("attacker_name".into(), format!("{:?}", self.type_()))
            })
    }

    pub fn damage_source_name(&self) -> Result<&str, CombatLogError> {
        self.log
            .damage_source_name
            .and_then(|id| {
                self.names
                    .items
                    .get(id as usize)
                    .map(|name| name.key.as_ref())
            })
            .ok_or_else(|| {
                CombatLogError::EmptyName(
                    "damage_source_name".into(),
                    format!("{:?}", self.type_()),
                )
            })
    }

    pub fn inflictor_name(&self) -> Result<&str, CombatLogError> {
        self.log
            .inflictor_name
            .and_then(|id| {
                self.names
                    .items
                    .get(id as usize)
                    .map(|name| name.key.as_ref())
            })
            .ok_or_else(|| {
                CombatLogError::EmptyName("inflictor_name".into(), format!("{:?}", self.type_()))
            })
    }

    pub fn value_name(&self) -> Result<&str, CombatLogError> {
        self.log
            .value
            .and_then(|id| {
                self.names
                    .items
                    .get(id as usize)
                    .map(|name| name.key.as_ref())
            })
            .ok_or_else(|| {
                CombatLogError::EmptyName("value_name".into(), format!("{:?}", self.type_()))
            })
    }

    pub fn attacker_illusion(&self) -> Result<bool, CombatLogError> {
        self.log.is_attacker_illusion.ok_or_else(|| {
            CombatLogError::EmptyProperty("attacker_illusion".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn attacker_hero(&self) -> Result<bool, CombatLogError> {
        self.log.is_attacker_hero.ok_or_else(|| {
            CombatLogError::EmptyProperty("attacker_hero".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn target_illusion(&self) -> Result<bool, CombatLogError> {
        self.log.is_target_illusion.ok_or_else(|| {
            CombatLogError::EmptyProperty("target_illusion".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn target_hero(&self) -> Result<bool, CombatLogError> {
        self.log.is_target_hero.ok_or_else(|| {
            CombatLogError::EmptyProperty("target_hero".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn visible_radiant(&self) -> Result<bool, CombatLogError> {
        self.log.is_visible_radiant.ok_or_else(|| {
            CombatLogError::EmptyProperty("visible_radiant".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn visible_dire(&self) -> Result<bool, CombatLogError> {
        self.log.is_visible_dire.ok_or_else(|| {
            CombatLogError::EmptyProperty("visible_dire".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn value(&self) -> Result<u32, CombatLogError> {
        self.log.value.ok_or_else(|| {
            CombatLogError::EmptyProperty("value".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn health(&self) -> Result<i32, CombatLogError> {
        self.log.health.ok_or_else(|| {
            CombatLogError::EmptyProperty("health".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn timestamp(&self) -> Result<f32, CombatLogError> {
        self.log.timestamp.ok_or_else(|| {
            CombatLogError::EmptyProperty("timestamp".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn stun_duration(&self) -> Result<f32, CombatLogError> {
        self.log.stun_duration.ok_or_else(|| {
            CombatLogError::EmptyProperty("stun_duration".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn slow_duration(&self) -> Result<f32, CombatLogError> {
        self.log.slow_duration.ok_or_else(|| {
            CombatLogError::EmptyProperty("slow_duration".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn ability_toggle_on(&self) -> Result<bool, CombatLogError> {
        self.log.is_ability_toggle_on.ok_or_else(|| {
            CombatLogError::EmptyProperty("ability_toggle_on".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn ability_toggle_off(&self) -> Result<bool, CombatLogError> {
        self.log.is_ability_toggle_off.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "ability_toggle_off".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn ability_level(&self) -> Result<u32, CombatLogError> {
        self.log.ability_level.ok_or_else(|| {
            CombatLogError::EmptyProperty("ability_level".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn location_x(&self) -> Result<f32, CombatLogError> {
        self.log.location_x.ok_or_else(|| {
            CombatLogError::EmptyProperty("location_x".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn location_y(&self) -> Result<f32, CombatLogError> {
        self.log.location_y.ok_or_else(|| {
            CombatLogError::EmptyProperty("location_y".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn gold_reason(&self) -> Result<u32, CombatLogError> {
        self.log.gold_reason.ok_or_else(|| {
            CombatLogError::EmptyProperty("gold_reason".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn timestamp_raw(&self) -> Result<f32, CombatLogError> {
        self.log.timestamp_raw.ok_or_else(|| {
            CombatLogError::EmptyProperty("timestamp_raw".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn modifier_duration(&self) -> Result<f32, CombatLogError> {
        self.log.modifier_duration.ok_or_else(|| {
            CombatLogError::EmptyProperty("modifier_duration".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn xp_reason(&self) -> Result<u32, CombatLogError> {
        self.log.xp_reason.ok_or_else(|| {
            CombatLogError::EmptyProperty("xp_reason".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn last_hits(&self) -> Result<u32, CombatLogError> {
        self.log.last_hits.ok_or_else(|| {
            CombatLogError::EmptyProperty("last_hits".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn attacker_team(&self) -> Result<u32, CombatLogError> {
        self.log.attacker_team.ok_or_else(|| {
            CombatLogError::EmptyProperty("attacker_team".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn target_team(&self) -> Result<u32, CombatLogError> {
        self.log.target_team.ok_or_else(|| {
            CombatLogError::EmptyProperty("target_team".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn obs_wards_placed(&self) -> Result<u32, CombatLogError> {
        self.log.obs_wards_placed.ok_or_else(|| {
            CombatLogError::EmptyProperty("obs_wards_placed".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn assist_player0(&self) -> Result<u32, CombatLogError> {
        self.log.assist_player0.ok_or_else(|| {
            CombatLogError::EmptyProperty("assist_player0".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn assist_player1(&self) -> Result<u32, CombatLogError> {
        self.log.assist_player1.ok_or_else(|| {
            CombatLogError::EmptyProperty("assist_player1".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn assist_player2(&self) -> Result<u32, CombatLogError> {
        self.log.assist_player2.ok_or_else(|| {
            CombatLogError::EmptyProperty("assist_player2".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn assist_player3(&self) -> Result<u32, CombatLogError> {
        self.log.assist_player3.ok_or_else(|| {
            CombatLogError::EmptyProperty("assist_player3".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn stack_count(&self) -> Result<u32, CombatLogError> {
        self.log.stack_count.ok_or_else(|| {
            CombatLogError::EmptyProperty("stack_count".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn hidden_modifier(&self) -> Result<bool, CombatLogError> {
        self.log.hidden_modifier.ok_or_else(|| {
            CombatLogError::EmptyProperty("hidden_modifier".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn target_building(&self) -> Result<bool, CombatLogError> {
        self.log.is_target_building.ok_or_else(|| {
            CombatLogError::EmptyProperty("target_building".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn neutral_camp_type(&self) -> Result<u32, CombatLogError> {
        self.log.neutral_camp_type.ok_or_else(|| {
            CombatLogError::EmptyProperty("neutral_camp_type".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn rune_type(&self) -> Result<u32, CombatLogError> {
        self.log.rune_type.ok_or_else(|| {
            CombatLogError::EmptyProperty("rune_type".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn assist_players(&self) -> &[i32] {
        &self.log.assist_players
    }

    pub fn heal_save(&self) -> Result<bool, CombatLogError> {
        self.log.is_heal_save.ok_or_else(|| {
            CombatLogError::EmptyProperty("heal_save".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn ultimate_ability(&self) -> Result<bool, CombatLogError> {
        self.log.is_ultimate_ability.ok_or_else(|| {
            CombatLogError::EmptyProperty("ultimate_ability".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn attacker_hero_level(&self) -> Result<u32, CombatLogError> {
        self.log.attacker_hero_level.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "attacker_hero_level".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn target_hero_level(&self) -> Result<u32, CombatLogError> {
        self.log.target_hero_level.ok_or_else(|| {
            CombatLogError::EmptyProperty("target_hero_level".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn xpm(&self) -> Result<u32, CombatLogError> {
        self.log.xpm.ok_or_else(|| {
            CombatLogError::EmptyProperty("xpm".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn gpm(&self) -> Result<u32, CombatLogError> {
        self.log.gpm.ok_or_else(|| {
            CombatLogError::EmptyProperty("gpm".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn event_location(&self) -> Result<u32, CombatLogError> {
        self.log.event_location.ok_or_else(|| {
            CombatLogError::EmptyProperty("event_location".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn target_is_self(&self) -> Result<bool, CombatLogError> {
        self.log.target_is_self.ok_or_else(|| {
            CombatLogError::EmptyProperty("target_is_self".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn damage_type(&self) -> Result<u32, CombatLogError> {
        self.log.damage_type.ok_or_else(|| {
            CombatLogError::EmptyProperty("damage_type".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn invisibility_modifier(&self) -> Result<bool, CombatLogError> {
        self.log.invisibility_modifier.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "invisibility_modifier".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn damage_category(&self) -> Result<u32, CombatLogError> {
        self.log.damage_category.ok_or_else(|| {
            CombatLogError::EmptyProperty("damage_category".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn networth(&self) -> Result<u32, CombatLogError> {
        self.log.networth.ok_or_else(|| {
            CombatLogError::EmptyProperty("networth".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn building_type(&self) -> Result<u32, CombatLogError> {
        self.log.building_type.ok_or_else(|| {
            CombatLogError::EmptyProperty("building_type".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn modifier_elapsed_duration(&self) -> Result<f32, CombatLogError> {
        self.log.modifier_elapsed_duration.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "modifier_elapsed_duration".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn silence_modifier(&self) -> Result<bool, CombatLogError> {
        self.log.silence_modifier.ok_or_else(|| {
            CombatLogError::EmptyProperty("silence_modifier".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn heal_from_lifesteal(&self) -> Result<bool, CombatLogError> {
        self.log.heal_from_lifesteal.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "heal_from_lifesteal".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn modifier_purged(&self) -> Result<bool, CombatLogError> {
        self.log.modifier_purged.ok_or_else(|| {
            CombatLogError::EmptyProperty("modifier_purged".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn spell_evaded(&self) -> Result<bool, CombatLogError> {
        self.log.spell_evaded.ok_or_else(|| {
            CombatLogError::EmptyProperty("spell_evaded".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn motion_controller_modifier(&self) -> Result<bool, CombatLogError> {
        self.log.motion_controller_modifier.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "motion_controller_modifier".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn long_range_kill(&self) -> Result<bool, CombatLogError> {
        self.log.long_range_kill.ok_or_else(|| {
            CombatLogError::EmptyProperty("long_range_kill".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn modifier_purge_ability(&self) -> Result<u32, CombatLogError> {
        self.log.modifier_purge_ability.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "modifier_purge_ability".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn modifier_purge_npc(&self) -> Result<u32, CombatLogError> {
        self.log.modifier_purge_npc.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "modifier_purge_npc".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn root_modifier(&self) -> Result<bool, CombatLogError> {
        self.log.root_modifier.ok_or_else(|| {
            CombatLogError::EmptyProperty("root_modifier".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn total_unit_death_count(&self) -> Result<u32, CombatLogError> {
        self.log.total_unit_death_count.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "total_unit_death_count".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn aura_modifier(&self) -> Result<bool, CombatLogError> {
        self.log.aura_modifier.ok_or_else(|| {
            CombatLogError::EmptyProperty("aura_modifier".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn armor_debuff_modifier(&self) -> Result<bool, CombatLogError> {
        self.log.armor_debuff_modifier.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "armor_debuff_modifier".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn no_physical_damage_modifier(&self) -> Result<bool, CombatLogError> {
        self.log.no_physical_damage_modifier.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "no_physical_damage_modifier".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn modifier_ability(&self) -> Result<u32, CombatLogError> {
        self.log.modifier_ability.ok_or_else(|| {
            CombatLogError::EmptyProperty("modifier_ability".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn modifier_hidden(&self) -> Result<bool, CombatLogError> {
        self.log.modifier_hidden.ok_or_else(|| {
            CombatLogError::EmptyProperty("modifier_hidden".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn inflictor_is_stolen_ability(&self) -> Result<bool, CombatLogError> {
        self.log.inflictor_is_stolen_ability.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "inflictor_is_stolen_ability".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn kill_eater_event(&self) -> Result<u32, CombatLogError> {
        self.log.kill_eater_event.ok_or_else(|| {
            CombatLogError::EmptyProperty("kill_eater_event".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn unit_status_label(&self) -> Result<u32, CombatLogError> {
        self.log.unit_status_label.ok_or_else(|| {
            CombatLogError::EmptyProperty("unit_status_label".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn spell_generated_attack(&self) -> Result<bool, CombatLogError> {
        self.log.spell_generated_attack.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "spell_generated_attack".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn at_night_time(&self) -> Result<bool, CombatLogError> {
        self.log.at_night_time.ok_or_else(|| {
            CombatLogError::EmptyProperty("at_night_time".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn attacker_has_scepter(&self) -> Result<bool, CombatLogError> {
        self.log.attacker_has_scepter.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "attacker_has_scepter".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn neutral_camp_team(&self) -> Result<u32, CombatLogError> {
        self.log.neutral_camp_team.ok_or_else(|| {
            CombatLogError::EmptyProperty("neutral_camp_team".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn regenerated_health(&self) -> Result<f32, CombatLogError> {
        self.log.regenerated_health.ok_or_else(|| {
            CombatLogError::EmptyProperty(
                "regenerated_health".into(),
                format!("{:?}", self.type_()),
            )
        })
    }

    pub fn will_reincarnate(&self) -> Result<bool, CombatLogError> {
        self.log.will_reincarnate.ok_or_else(|| {
            CombatLogError::EmptyProperty("will_reincarnate".into(), format!("{:?}", self.type_()))
        })
    }

    pub fn uses_charges(&self) -> Result<bool, CombatLogError> {
        self.log.uses_charges.ok_or_else(|| {
            CombatLogError::EmptyProperty("uses_charges".into(), format!("{:?}", self.type_()))
        })
    }
}
