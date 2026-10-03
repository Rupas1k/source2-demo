use std::collections::HashSet;

use hashbrown::HashMap;

use crate::Entry;

#[derive(Default)]
pub struct CombatLogAnnotations {
    tracked: HashMap<String, String>,
    greed_learned: bool,
    last_hits: HashSet<i32>,
}

impl CombatLogAnnotations {
    pub fn annotate(&mut self, entry: &mut Entry, name_to_slot: &HashMap<String, i32>) {
        let kind = entry.r#type.as_deref().unwrap_or_default();
        let target = entry.targetname.as_deref().unwrap_or_default();
        let attacker = entry.attackername.as_deref().unwrap_or_default();
        let inflictor = entry.inflictor.as_deref().unwrap_or_default();

        if inflictor == "modifier_bounty_hunter_track" {
            match kind {
                "DOTA_COMBATLOG_MODIFIER_ADD" => {
                    self.tracked.insert(target.into(), attacker.into());
                }
                "DOTA_COMBATLOG_MODIFIER_REMOVE" => {
                    self.tracked.remove(target);
                }
                _ => {}
            }
        }

        if kind == "DOTA_COMBATLOG_DEATH" {
            if let Some(source) = self.tracked.get(target) {
                entry.tracked_death = Some(true);
                entry.tracked_sourcename = Some(source.clone());
            }
        }

        if attacker != "npc_dota_hero_alchemist" || entry.attackerillusion.unwrap_or_default() {
            return;
        }

        if kind == "DOTA_COMBATLOG_MODIFIER_ADD" && inflictor == "modifier_alchemist_goblins_greed" {
            self.greed_learned = true;
        }

        if !self.greed_learned || kind != "DOTA_COMBATLOG_DEATH" {
            return;
        }

        let Some(slot) = name_to_slot.get(attacker) else {
            return;
        };
        let allied_creeps = if *slot < 5 { "goodguys" } else { "badguys" };
        if target.contains(allied_creeps) {
            return;
        }

        let time = entry.time as i32;
        self.last_hits.retain(|last_hit| *last_hit + 40 >= time);
        entry.greevils_greed_stack = Some(self.last_hits.len() as u32);
        self.last_hits.insert(time);
    }
}
