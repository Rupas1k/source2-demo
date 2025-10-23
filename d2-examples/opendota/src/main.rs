// https://github.com/odota/parser/blob/5c3ac03bc5a8fb84a3b4873b788155988e9d1176/src/main/java/opendota/Parse.java
// slightly outdated

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

use hashbrown::HashMap;
use anyhow::{Error, Result, bail};
use serde::{Deserialize, Serialize};

use source2_demo::prelude::*;
use source2_demo::proto::*;

mod game_time;
mod wards;

use crate::game_time::*;
use crate::wards::*;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Entry {
    pub time: f32,
    pub r#type: Option<String>,
    pub team: Option<i32>,
    pub unit: Option<String>,
    pub key: Option<String>,
    pub value: Option<u32>,
    pub slot: Option<i32>,
    pub player_slot: Option<i32>,
    pub player1: Option<i32>,
    pub player2: Option<i32>,
    pub attackername: Option<String>,
    pub targetname: Option<String>,
    pub sourcename: Option<String>,
    pub targetsourcename: Option<String>,
    pub attackerhero: Option<bool>,
    pub targethero: Option<bool>,
    pub attackerillusion: Option<bool>,
    pub targetillusion: Option<bool>,
    pub abilitylevel: Option<u8>,
    pub inflictor: Option<String>,
    pub gold_reason: Option<u32>,
    pub xp_reason: Option<u32>,
    pub valuename: Option<String>,
    pub gold: Option<u32>,
    pub lh: Option<u16>,
    pub xp: Option<u16>,
    pub x: Option<u8>,
    pub y: Option<u8>,
    pub z: Option<u8>,
    pub stuns: Option<f32>,
    pub hero_id: Option<i32>,
    pub itemslot: Option<u8>,
    pub charges: Option<u8>,
    pub secondary_charges: Option<u8>,
    pub life_state: Option<u8>,
    pub level: Option<u8>,
    pub kills: Option<u8>,
    pub deaths: Option<u8>,
    pub assists: Option<u8>,
    pub denies: Option<u8>,
    pub entityleft: Option<bool>,
    pub ehandle: Option<u32>,
    pub obs_placed: Option<u8>,
    pub sen_placed: Option<u8>,
    pub creeps_stacked: Option<u8>,
    pub camps_stacked: Option<u8>,
    pub rune_pickups: Option<u8>,
    pub repicked: Option<bool>,
    pub randomed: Option<bool>,
    pub pred_vict: Option<bool>,
    pub stun_duration: Option<f32>,
    pub slow_duration: Option<f32>,
    pub tracked_death: Option<bool>,
    pub greevils_greed_stack: Option<u8>,
    pub tracked_sourcename: Option<String>,
    pub firstblood_claimed: Option<bool>,
    pub teamfight_participation: Option<f32>,
    pub towers_killed: Option<u8>,
    pub roshans_killed: Option<u8>,
    pub observers_placed: Option<u8>,
    pub draft_order: Option<u8>,
    pub pick: Option<bool>,
    pub draft_active_team: Option<u8>,
    pub draft_extime0: Option<u16>,
    pub draft_extime1: Option<u16>,
    pub networth: Option<u32>,
    pub stage: Option<u8>,
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl Entry {
    pub fn new(time: f32) -> Self {
        Entry { time, ..Default::default() }
    }
}

#[derive(Default)]
struct Item {
    id: String,
    slot: u8,
    num_charges: u8,
    num_secondary_charges: u8,
}

#[derive(Default)]
struct Ability {
    id: String,
    level: u8,
}

#[derive(Default)]
struct App {
    output: Vec<Entry>,
    game_time: Rc<RefCell<GameTime>>,
    next_interval: i32,
    valid_indices: [i32; 10],
    init: bool,
    start_time: f32,
    name_to_slot: HashMap<String, i32>,
    abilities_tracking: HashMap<String, u8>,
    slot_to_players_slot: HashMap<i32, i32>,
    steam_id_to_player_slot: HashMap<u64, i32>,
    cosmetics_map: HashMap<i32, i32>,
    dota_plus_xp_map: HashMap<i32, i32>,
    ping_count: i32,
    draft_order_processed: [bool; 24],
    order: u8,
    is_draft_start_time_processed: bool,
    is_dota_plus_processed: bool,
    post_game: bool,
    is_player_starting_items_written: [bool; 10],
    class_to_combat_log: HashMap<String, String>,
    log_buffer: VecDeque<Entry>,
}

impl App {
    pub fn output(&mut self, mut e: Entry) -> Result<()> {
        if self.start_time == 0.0 {
            self.log_buffer.push_back(e);
        } else {
            e.time = (e.time - self.start_time).floor();
            self.output.push(e);
        }
        Ok(())
    }

    pub fn flush_log_buffer(&mut self) -> Result<()> {
        while let Some(e) = self.log_buffer.pop_front() {
            self.output(e)?
        }
        Ok(())
    }

    pub fn time(&self, ctx: &Context) -> Result<f32> {
        Ok(self.game_time.borrow().tick(ctx)? as f32 / 30.0)
    }

    fn get_player_slot(&self, e: &Entity) -> Result<i32> {
        let player_slot: i32 = if let Some(x) = try_property!(e, "m_iPlayerID") {
            x
        } else if let Some(x) = try_property!(e, "m_nPlayerID") {
            x
        } else {
            bail!("No player ID found.");
        };
        Ok(player_slot >> 1)
    }

    fn get_hero_item(&self, ctx: &Context, hero: &Entity, idx: u8) -> Result<Item> {
        let entity_names = ctx.string_tables().get_by_name("EntityNames")?;
        let item_handle: usize = property!(hero, "m_hItems.{idx:04}");

        if item_handle == 0xFFFFFF {
            bail!("{} slot is empty for {}", idx, hero.class().name())
        }

        let item_entity = ctx.entities().get_by_handle(item_handle)?;
        let item_name = entity_names
            .get_row_by_index(property!(item_entity, "m_pEntity.m_nameStringableIndex"))?
            .key();

        Ok(Item {
            id: item_name.into(),
            slot: idx,
            num_charges: property!(item_entity, "m_iCurrentCharges"),
            num_secondary_charges: property!(item_entity, "m_iSecondaryCharges"),
        })
    }

    fn get_hero_inventory(&mut self, ctx: &Context, hero: &Entity) -> Vec<Item> {
        let mut inventory: Vec<Item> = vec![];
        for i in 0..8 {
            if let Ok(item) = self.get_hero_item(ctx, hero, i) {
                inventory.push(item)
            }
        }
        inventory
    }

    fn get_hero_abilities(&self, ctx: &Context, hero: &Entity) -> Vec<Ability> {
        let mut abilities: Vec<Ability> = vec![];
        for i in 0..32 {
            if let Ok(ability) = self.get_hero_ability(ctx, hero, i) {
                abilities.push(ability)
            }
        }
        abilities
    }

    fn get_hero_ability(&self, ctx: &Context, hero: &Entity, idx: i32) -> Result<Ability> {
        let entity_names = ctx.string_tables().get_by_name("EntityNames")?;
        let ability_handle: usize = property!(hero, "m_hAbilities.{idx:04}");

        if ability_handle == 0xFFFFFF {
            bail!("{} ability doesn't exist for {}", idx, hero.class().name());
        }

        let ability_entity = ctx.entities().get_by_handle(ability_handle)?;
        let ability_name = entity_names
            .get_row_by_index(property!(ability_entity, "m_pEntity.m_nameStringableIndex"))?
            .key();

        Ok(Ability {
            id: ability_name.into(),
            level: property!(ability_entity, "m_iLevel"),
        })
    }
}

#[observer]
impl App {
    #[on_message]
    fn handle_demo_cmd(&mut self, ctx: &Context, file_info: &CDemoFileInfo) -> ObserverResult {
        let mut cosmetics_entry = Entry::new(self.time(ctx)?);
        cosmetics_entry.r#type = "cosmetics".to_string().into();
        cosmetics_entry.key = serde_json::to_string(&self.cosmetics_map)?.into();
        self.output(cosmetics_entry)?;

        let mut dota_plus_entry = Entry::new(self.time(ctx)?);
        dota_plus_entry.r#type = "dotaplus".to_string().into();
        dota_plus_entry.key = serde_json::to_string(&self.dota_plus_xp_map)?.into();
        self.output(dota_plus_entry)?;

        let mut epilogue_entry = Entry::new(self.time(ctx)?);
        epilogue_entry.r#type = "epilogue".to_string().into();
        epilogue_entry.key = serde_json::to_string(&file_info)?.into();
        self.output(epilogue_entry)?;

        self.flush_log_buffer()?;
        Ok(())
    }

    #[on_message]
    fn handle_unit_order(&mut self, ctx: &Context, order: CDotaUserMsgSpectatorPlayerUnitOrders) -> ObserverResult {
        if self.time(ctx).is_err() {
            return Ok(());
        }

        let mut entry = Entry::new(self.time(ctx)?);
        if let Ok(entity) = ctx.entities().get_by_index(order.entindex() as usize) {
            entry.r#type = "actions".to_string().into();
            entry.slot = self.get_player_slot(entity).ok();
            entry.key = order.order_type().to_string().into();
            self.output(entry)?;
        }

        Ok(())
    }

    #[on_message]
    fn handle_ping(&mut self, ctx: &Context, location_ping: CDotaUserMsgLocationPing) -> ObserverResult {
        self.ping_count += 1;
        if self.ping_count > 100000 {
            return Ok(());
        }

        let mut entry = Entry::new(self.time(ctx)?);
        entry.r#type = "pings".to_string().into();
        entry.slot = location_ping.player_id().into();

        self.output(entry)?;

        Ok(())
    }

    #[on_tick_start]
    fn draft(&mut self, ctx: &Context) -> ObserverResult {
        let Ok(grp) = ctx.entities().get_by_class_name("CDOTAGamerulesProxy") else {
            return Ok(());
        };

        let draft_stage: i32 = property!(grp, "m_pGameRules.m_nGameState");
        if draft_stage == 2 {
            if !self.is_draft_start_time_processed {
                let x: u64 = property!(grp, "m_pGameRules.m_iPlayerIDsInControl");
                if x != 0 {
                    let mut entry = Entry::new(self.time(ctx)?);
                    entry.r#type = "draft_start".to_string().into();
                    self.output(entry)?;
                    self.is_draft_start_time_processed = true;
                }
            }

            let mut draft_heroes: [i32; 24] = [0; 24];
            draft_heroes[0] = property!(grp, "m_pGameRules.m_BannedHeroes.0000");
            draft_heroes[1] = property!(grp, "m_pGameRules.m_BannedHeroes.0001");
            draft_heroes[2] = property!(grp, "m_pGameRules.m_BannedHeroes.0002");
            draft_heroes[3] = property!(grp, "m_pGameRules.m_BannedHeroes.0003");
            draft_heroes[4] = property!(grp, "m_pGameRules.m_BannedHeroes.0004");
            draft_heroes[5] = property!(grp, "m_pGameRules.m_BannedHeroes.0005");
            draft_heroes[6] = property!(grp, "m_pGameRules.m_BannedHeroes.0006");
            draft_heroes[7] = property!(grp, "m_pGameRules.m_BannedHeroes.0007");
            draft_heroes[8] = property!(grp, "m_pGameRules.m_BannedHeroes.0008");
            draft_heroes[9] = property!(grp, "m_pGameRules.m_BannedHeroes.0009");
            draft_heroes[9] = property!(grp, "m_pGameRules.m_BannedHeroes.0009");
            draft_heroes[10] = try_property!(grp, "m_pGameRules.m_BannedHeroes.0010").unwrap_or_default();
            draft_heroes[11] = try_property!(grp, "m_pGameRules.m_BannedHeroes.0011").unwrap_or_default();
            draft_heroes[12] = try_property!(grp, "m_pGameRules.m_BannedHeroes.0012").unwrap_or_default();
            draft_heroes[13] = try_property!(grp, "m_pGameRules.m_BannedHeroes.0013").unwrap_or_default();
            draft_heroes[14] = property!(grp, "m_pGameRules.m_SelectedHeroes.0000");
            draft_heroes[15] = property!(grp, "m_pGameRules.m_SelectedHeroes.0001");
            draft_heroes[16] = property!(grp, "m_pGameRules.m_SelectedHeroes.0002");
            draft_heroes[17] = property!(grp, "m_pGameRules.m_SelectedHeroes.0003");
            draft_heroes[18] = property!(grp, "m_pGameRules.m_SelectedHeroes.0004");
            draft_heroes[19] = property!(grp, "m_pGameRules.m_SelectedHeroes.0005");
            draft_heroes[20] = property!(grp, "m_pGameRules.m_SelectedHeroes.0006");
            draft_heroes[21] = property!(grp, "m_pGameRules.m_SelectedHeroes.0007");
            draft_heroes[22] = property!(grp, "m_pGameRules.m_SelectedHeroes.0008");
            draft_heroes[23] = property!(grp, "m_pGameRules.m_SelectedHeroes.0009");

            for (i, &hero_id) in draft_heroes.iter().enumerate() {
                if hero_id > 0 && !self.draft_order_processed[i] {
                    let extime0: f32 = property!(grp, "m_pGameRules.m_fExtraTimeRemaining.0000");
                    let extime1: f32 = property!(grp, "m_pGameRules.m_fExtraTimeRemaining.0001");

                    let mut entry = Entry::new(self.time(ctx)?);
                    entry.r#type = "draft_timings".to_string().into();
                    entry.draft_order = self.order.into();
                    entry.pick = (i >= 14).into();
                    entry.hero_id = draft_heroes[i].into();
                    entry.draft_extime0 = (extime0.round() as u16).into();
                    entry.draft_extime1 = (extime1.round() as u16).into();
                    entry.draft_active_team = try_property!(grp, "m_pGameRules.m_iActiveTeam");
                    self.output(entry)?;

                    self.order += 1;
                    self.draft_order_processed[i] = true;
                }
            }
        }

        if self.next_interval == 0 {
            self.next_interval = self.time(ctx)? as i32;
        }

        Ok(())
    }

    #[on_tick_start]
    fn tick_start(&mut self, ctx: &Context) -> ObserverResult {
        let Ok(pr) = ctx.entities().get_by_class_name("CDOTA_PlayerResource") else {
            return Ok(());
        };

        if !self.init {
            let mut added = 0;
            let mut i = 0;
            let mut waiting_for_draft_players = false;
            let mut player_entries = VecDeque::<Entry>::new();
            while added < 10 && i < 30 {
                let player_team: i32 = property!(pr, "m_vecPlayerData.{i:04}.m_iPlayerTeam");
                let team_slot: i32 = property!(pr, "m_vecPlayerTeamData.{i:04}.m_iTeamSlot");
                let steam_id: u64 = property!(pr, "m_vecPlayerData.{i:04}.m_iPlayerSteamID");
                if player_team == 2 || player_team == 3 {
                    let mut entry = Entry::new(self.time(ctx)?);
                    entry.r#type = "player_slot".to_string().into();
                    entry.key = added.to_string().into();
                    entry.value = ((if player_team == 2 { 0 } else { 128 } + team_slot) as u32).into();

                    self.valid_indices[added as usize] = i;
                    added += 1;
                    self.slot_to_players_slot.insert(added, entry.value.unwrap() as i32);
                    self.steam_id_to_player_slot.insert(steam_id, entry.value.unwrap() as i32);
                    player_entries.push_back(entry);
                }
                if player_team == 14 {
                    waiting_for_draft_players = true;
                    break;
                }
                i += 1;
            }
            if !waiting_for_draft_players {
                while let Some(e) = player_entries.pop_front() {
                    self.output(e)?;
                }
                self.init = true;
            }
        }

        if self.init && !self.post_game && self.time(ctx)? as i32 >= self.next_interval {
            for i in 0..10 {
                let hero_id: i32 = property!(pr, "m_vecPlayerTeamData.{i:04}.m_nSelectedHeroID");
                let hero_handle: usize = property!(pr, "m_vecPlayerTeamData.{i:04}.m_hSelectedHero");
                let player_team: i32 = property!(pr, "m_vecPlayerData.{i:04}.m_iPlayerTeam");
                let team_slot: i32 = property!(pr, "m_vecPlayerTeamData.{i:04}.m_iTeamSlot");

                let data_team = if player_team == 2 {
                    ctx.entities().get_by_class_name("CDOTA_DataRadiant")?
                } else {
                    ctx.entities().get_by_class_name("CDOTA_DataDire")?
                };

                let mut entry = Entry::new(self.time(ctx)?);
                entry.r#type = "interval".to_string().into();
                entry.slot = i.into();
                entry.repicked = try_property!(pr, "m_vecPlayerTeamData.{i:04}.m_bHasRepicked");
                entry.randomed = try_property!(pr, "m_vecPlayerTeamData.{i:04}.m_bHasRandomed");
                entry.pred_vict = try_property!(pr, "m_vecPlayerTeamData.{i:04}.m_bHasPredictedVictory");
                entry.firstblood_claimed = try_property!(pr, "m_vecPlayerTeamData.{i:04}.m_bFirstBloodClaimed");
                entry.teamfight_participation = try_property!(pr, "m_vecPlayerTeamData.{i:04}.m_flTeamFightParticipation");
                entry.level = try_property!(pr, "m_vecPlayerTeamData.{i:04}.m_iLevel");
                entry.kills = try_property!(pr, "m_vecPlayerTeamData.{i:04}.m_iKills");
                entry.deaths = try_property!(pr, "m_vecPlayerTeamData.{i:04}.m_iDeaths");
                entry.assists = try_property!(pr, "m_vecPlayerTeamData.{i:04}.m_iAssists");
                entry.denies = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iDenyCount");
                entry.obs_placed = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iObserverWardsPlaced");
                entry.sen_placed = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iSentryWardsPlaced");
                entry.creeps_stacked = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iCreepsStacked");
                entry.camps_stacked = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iCampsStacked");
                entry.rune_pickups = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iRunePickups");
                entry.towers_killed = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iTowerKills");
                entry.roshans_killed = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iRoshanKills");
                entry.networth = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iNetWorth");
                entry.stage = try_property!(ctx.entities().get_by_class_name("CDOTAGamerulesProxy")?, "m_pGameRules.m_nGameState");

                if team_slot >= 0 {
                    entry.gold = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iTotalEarnedGold");
                    entry.lh = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iLastHitCount");
                    entry.xp = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iTotalEarnedXP");
                    entry.stuns = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_fStuns");

                    if let Ok(hero) = ctx.entities().get_by_handle(hero_handle) {
                        entry.x = try_property!(hero, "CBodyComponent.m_cellX");
                        entry.y = try_property!(hero, "CBodyComponent.m_cellY");
                        entry.unit = hero.class().name().to_string().into();
                        entry.hero_id = hero_id.into();
                        entry.life_state = try_property!(hero, "m_lifeState");
                        if hero_id > 0 {
                            let class = hero.class().name();

                            if !self.class_to_combat_log.contains_key(class) {
                                let name1 = "npc_dota_hero_".to_string() + &class["CDOTA_Unit_Hero_".len()..].to_lowercase();
                                let name2 = "npc_dota_hero".to_string()
                                    + &class["CDOTA_Unit_Hero_".len()..]
                                        .to_lowercase()
                                        .chars()
                                        .map(|c| {
                                            if c.is_ascii_uppercase() {
                                                format!("_{}", c.to_lowercase())
                                            } else {
                                                c.to_string()
                                            }
                                        })
                                        .collect::<String>();
                                self.name_to_slot.insert(name1.clone(), i);
                                self.name_to_slot.insert(name2.clone(), i);
                                self.class_to_combat_log.insert(class.to_string(), name1);
                            }

                            let hero_name = self.class_to_combat_log[class].clone();

                            for ability in self.get_hero_abilities(ctx, hero) {
                                let key = hero_name.clone() + ability.id.as_str();
                                if !self.abilities_tracking.contains_key(&key) || self.abilities_tracking[&key] != ability.level {
                                    let mut entry = Entry::new(self.time(ctx)?);
                                    entry.r#type = "DotaAbilityLevel".to_string().into();
                                    entry.targetname = hero_name.clone().into();
                                    entry.valuename = ability.id.clone().into();
                                    entry.abilitylevel = ability.level.into();
                                    self.abilities_tracking.insert(hero_name.to_owned() + &ability.id, ability.level);
                                    self.output(entry)?;
                                }
                            }

                            if self.time(ctx)? as i32 - self.start_time as i32 == 1 {
                                for item in self.get_hero_inventory(ctx, hero) {
                                    let mut starting_items = Entry::new(self.time(ctx)?);
                                    starting_items.r#type = "StartingItems".to_string().into();
                                    starting_items.targetname = hero_name.clone().into();
                                    starting_items.valuename = item.id.clone().into();
                                    starting_items.slot = entry.slot;
                                    starting_items.value = (if entry.slot.unwrap() < 5 { 0 } else { 123 } + entry.slot.unwrap() as u32).into();
                                    starting_items.itemslot = item.slot.into();
                                    starting_items.charges = item.num_charges.into();
                                    starting_items.secondary_charges = item.num_secondary_charges.into();
                                    self.output(starting_items)?;
                                }
                            }

                            if !self.is_player_starting_items_written[entry.slot.unwrap() as usize] {
                                self.is_player_starting_items_written[entry.slot.unwrap() as usize] = true;
                                for item in self.get_hero_inventory(ctx, hero) {
                                    let mut starting_items = Entry::new(self.time(ctx)?);
                                    starting_items.r#type = "DotaCombatlogPurchase".to_string().into();
                                    starting_items.targetname = hero_name.clone().into();
                                    starting_items.valuename = item.id.clone().into();
                                    starting_items.slot = entry.slot;
                                    starting_items.value = (if entry.slot.unwrap() < 5 { 0 } else { 123 } + entry.slot.unwrap() as u32).into();
                                    starting_items.charges = item.num_charges.into();
                                    self.output(starting_items)?;
                                }
                            }
                        }
                    }
                }
                self.output(entry)?;
            }
            self.next_interval += 1;
        }

        if self.post_game && !self.is_dota_plus_processed {
            for i in 0..10 {
                let xp: i32 = try_property!(pr, "m_vecPlayerTeamData.{i:04}.m_unSelectedHeroBadgeXP").unwrap_or_default();
                let steam_id: u64 = property!(pr, "m_vecPlayerData.{i:04}.m_iPlayerSteamID");
                if let Some(slot) = self.steam_id_to_player_slot.get(&steam_id) {
                    self.dota_plus_xp_map.insert(*slot, xp);
                }
            }
            self.is_dota_plus_processed = true;
        }

        Ok(())
    }

    #[on_entity("CDOTAWearableItem")]
    fn on_entity(&mut self, _ctx: &Context, event: EntityEvents, entity: &Entity) -> Result<()> {
        if event == EntityEvents::Created {
            let account_id: u64 = property!(entity, "m_iAccountID");
            let item_definition_idx: i32 = property!(entity, "m_iItemDefinitionIndex");
            if account_id > 0 {
                let account_id64: u64 = 76561197960265728 + account_id;
                let player_slot: i32 = *self.steam_id_to_player_slot.get(&account_id64).unwrap_or(&0);
                self.cosmetics_map.insert(item_definition_idx, player_slot);
            }
        }
        Ok(())
    }

    #[on_combat_log]
    fn handle_cle(&mut self, _ctx: &Context, cle: &CombatLogEntry) -> Result<()> {
        let time = cle.timestamp()?;
        let mut entry = Entry::new(time);
        entry.r#type = format!("{:?}", cle.r#type()).into();
        entry.attackername = cle.attacker_name().ok().map(|x| x.into());
        entry.targetname = cle.target_name().ok().map(|x| x.into());
        entry.sourcename = cle.damage_source_name().ok().map(|x| x.into());
        entry.targetsourcename = cle.target_source_name().ok().map(|x| x.into());
        entry.inflictor = cle.inflictor_name().ok().map(|x| x.into());
        entry.attackerhero = cle.is_attacker_hero().ok();
        entry.targethero = cle.is_target_hero().ok();
        entry.attackerillusion = cle.is_attacker_illusion().ok();
        entry.value = cle.value().ok();
        entry.stun_duration = cle.stun_duration().ok().filter(|&stun| stun > 0.0);
        entry.slow_duration = cle.slow_duration().ok().filter(|&slow| slow > 0.0);

        if cle.r#type() == DotaCombatlogTypes::DotaCombatlogPurchase {
            entry.valuename = cle.value_name().ok().map(|x| x.into());
        }
        if cle.r#type() == DotaCombatlogTypes::DotaCombatlogGold {
            entry.gold_reason = cle.gold_reason().ok();
        }
        if cle.r#type() == DotaCombatlogTypes::DotaCombatlogXp {
            entry.xp_reason = cle.xp_reason().ok();
        }

        if cle.r#type() == DotaCombatlogTypes::DotaCombatlogGameState && cle.value()? == 6 {
            self.post_game = true;
        }

        if cle.r#type() as u32 <= 19 {
            self.output(entry)?;
        }

        Ok(())
    }

    #[on_message]
    fn on_chat_event(&mut self, ctx: &Context, event: CDotaUserMsgChatEvent) -> Result<()> {
        let mut entry = Entry::new(self.time(ctx)?);
        entry.r#type = format!("{:?}", event.r#type()).into();
        entry.player1 = event.playerid_1().into();
        entry.player2 = event.playerid_2().into();
        entry.value = event.value().into();
        self.output(entry)
    }

    #[on_message]
    fn on_all_chat_message(&mut self, ctx: &Context, event: CDotaUserMsgChatMessage) -> Result<()> {
        let mut entry = Entry::new(self.time(ctx)?);
        entry.r#type = if event.channel_type() == 11 {
            "chat".to_string().into()
        } else {
            event.channel_type().to_string().into()
        };
        entry.slot = event.source_player_id().into();
        entry.key = event.message_text().to_string().into();
        self.output(entry)
    }

    #[on_message]
    fn on_chat_wheel(&mut self, ctx: &Context, event: CDotaUserMsgChatWheel) -> Result<()> {
        let mut entry = Entry::new(self.time(ctx)?);
        entry.r#type = "chatwheel".to_string().into();
        entry.slot = event.player_id().into();
        entry.key = event.chat_message_id().to_string().into();
        self.output(entry)
    }
}

impl GameTimeObserver for App {
    fn on_game_started(&mut self, _ctx: &Context, start_time: f32) -> Result<()> {
        self.start_time = start_time;
        self.flush_log_buffer()
    }
}

impl WardsObserver for App {
    fn on_ward(&mut self, ctx: &Context, ward_class: WardClass, event: WardEvent, ward: &Entity) -> Result<()> {
        let mut entry = Entry::new(self.time(ctx)?);

        let is_obs = ward_class == WardClass::Observer;
        let x: u8 = property!(ward, "CBodyComponent.m_cellX");
        let y: u8 = property!(ward, "CBodyComponent.m_cellY");
        let z: u8 = property!(ward, "CBodyComponent.m_cellZ");

        entry.r#type = (if is_obs { "obs".to_string() } else { "sen".to_string() } + if event != WardEvent::Placed { "_left" } else { "" }).into();
        entry.entityleft = (event != WardEvent::Placed).into();
        entry.ehandle = ward.handle().into();
        entry.x = x.into();
        entry.y = y.into();
        entry.z = z.into();

        let owner_handle: usize = property!(ward, "m_hOwnerEntity");
        if let Ok(owner) = ctx.entities().get_by_handle(owner_handle) {
            entry.slot = self.get_player_slot(owner)?.into();
        }

        match event {
            WardEvent::Placed | WardEvent::Expired => self.output(entry),
            WardEvent::Killed(killer) => {
                entry.attackername = killer.to_string().into();
                self.output(entry)
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(filepath) = args.get(1) else {
        eprintln!("Usage: {} <demofile>", args[0]);
        return Ok(());
    };

    let replay = unsafe { memmap2::Mmap::map(&std::fs::File::open(filepath)?)? };

    let start = std::time::Instant::now();

    let mut parser = Parser::new(&replay)?;

    let game_time = parser.register_observer::<GameTime>();
    let wards = parser.register_observer::<Wards>();
    let app = parser.register_observer::<App>();

    app.borrow_mut().game_time = game_time.clone();

    game_time.borrow_mut().register_observer(app.clone());
    wards.borrow_mut().register_observer(app.clone());

    parser.run_to_end()?;

    for e in &app.borrow().output {
        println!("{}", e);
    }
    println!("Total entries: {}", app.borrow().output.len());
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
