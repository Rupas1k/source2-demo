// https://github.com/odota/parser/blob/master/src/main/java/opendota/Parse.java

use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{BufReader, BufWriter, Write};
use std::rc::Rc;

use anyhow::{Result, bail};
use hashbrown::HashMap;
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
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
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
    pub firstblood_claimed: Option<i32>,
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
    pub variant: Option<i32>,
    pub facet_hero_id: Option<i32>,
    pub hero_inventory: Option<Vec<Item>>,
    #[serde(rename = "isNeutralActiveDrop")]
    pub is_neutral_active_drop: Option<bool>,
    #[serde(rename = "isNeutralPassiveDrop")]
    pub is_neutral_passive_drop: Option<bool>,
}

impl Entry {
    pub fn new(time: f32) -> Self {
        Entry { time, ..Default::default() }
    }
}

fn upper_camel_to_screaming_snake(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 8);
    let mut prev: Option<char> = None;
    let mut chars = name.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_ascii_uppercase() {
            let next_is_lower = chars.peek().is_some_and(|next| next.is_ascii_lowercase());
            let needs_separator =
                prev.is_some_and(|prev| prev.is_ascii_lowercase() || prev.is_ascii_digit() || (prev.is_ascii_uppercase() && next_is_lower));

            if needs_separator {
                out.push('_');
            }
            out.push(ch);
        } else {
            out.push(ch.to_ascii_uppercase());
        }
        prev = Some(ch);
    }

    out
}

fn debug_name_to_screaming_snake(value: impl std::fmt::Debug) -> String {
    upper_camel_to_screaming_snake(&format!("{value:?}"))
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Item {
    id: String,
    slot: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_charges: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_secondary_charges: Option<u8>,
}

#[derive(Default)]
struct Ability {
    id: String,
    level: u8,
}

#[derive(Default)]
struct App {
    output: Option<BufWriter<std::io::Stdout>>,
    game_time: Rc<RefCell<GameTime>>,
    time: i32,
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
    was_paused: bool,
    pause_start_time: i32,
    pause_start_game_time: i32,
}

impl App {
    pub fn output(&mut self, mut e: Entry) -> ObserverResult {
        if self.start_time == 0.0 {
            self.log_buffer.push_back(e);
        } else {
            e.time = (e.time - self.start_time).floor();
            let output = self.output.as_mut().ok_or_else(|| anyhow::anyhow!("output writer not initialized"))?;
            serde_json::to_writer(&mut *output, &e)?;
            output.write_all(b"\n")?;
        }
        Ok(())
    }

    pub fn flush_log_buffer(&mut self) -> ObserverResult {
        while let Some(e) = self.log_buffer.pop_front() {
            self.output(e)?
        }
        Ok(())
    }

    pub fn flush_writer(&mut self) -> ObserverResult {
        if let Some(output) = self.output.as_mut() {
            output.flush()?;
        }
        Ok(())
    }

    pub fn time(&self, _ctx: &Context) -> Result<f32> {
        Ok(self.time as f32)
    }

    fn update_time(&mut self, ctx: &Context) {
        if let Ok(tick) = self.game_time.borrow().tick(ctx) {
            self.time = (tick as f32 / 30.0).round() as i32;
        }
    }

    fn precise_location(cell: u8, vec: Option<f32>) -> f32 {
        (cell as f32 * 128.0 + vec.unwrap_or_default()) / 128.0
    }

    fn get_player_slot(&self, e: &Entity) -> Result<i32> {
        let player_slot: i32 = if let Some(x) = try_property!(e, "m_nPlayerID") {
            x
        } else if let Some(x) = try_property!(e, "m_iPlayerID") {
            x
        } else if let Some(x) = try_property!(e, "m_iPlayerOwnerID") {
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
        let item_name_idx: usize = try_property!(item_entity, "m_pEntity.m_nameStringTableIndex")
            .or_else(|| try_property!(item_entity, "m_pEntity.m_nameStringableIndex"))
            .ok_or_else(|| anyhow::anyhow!("No item name string table index for {}", item_entity.class().name()))?;
        let item_name = entity_names.get_row(item_name_idx)?.key();
        let num_charges = property!(item_entity, u8, "m_iCurrentCharges");
        let num_secondary_charges = property!(item_entity, u8, "m_iSecondaryCharges");

        Ok(Item {
            id: item_name.into(),
            slot: idx,
            num_charges: (num_charges != 0).then_some(num_charges),
            num_secondary_charges: (num_secondary_charges != 0).then_some(num_secondary_charges),
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
        let ability_handle: usize = try_property!(hero, "m_hAbilities.{idx:04}")
            .or_else(|| try_property!(hero, "m_vecAbilities.{idx:04}"))
            .unwrap_or(0xFFFFFF);

        if ability_handle == 0xFFFFFF {
            bail!("{} ability doesn't exist for {}", idx, hero.class().name());
        }

        let ability_entity = ctx.entities().get_by_handle(ability_handle)?;
        let ability_name_idx: usize = try_property!(ability_entity, "m_pEntity.m_nameStringTableIndex")
            .or_else(|| try_property!(ability_entity, "m_pEntity.m_nameStringableIndex"))
            .ok_or_else(|| anyhow::anyhow!("No ability name string table index for {}", ability_entity.class().name()))?;
        let ability_name = entity_names.get_row(ability_name_idx)?.key();

        Ok(Ability {
            id: ability_name.into(),
            level: property!(ability_entity, "m_iLevel"),
        })
    }
}

#[observer]
#[uses_entities]
#[uses_combat_log]
impl App {
    #[on_message]
    fn handle_demo_cmd(&mut self, ctx: &Context, file_info: CDemoFileInfo) -> ObserverResult {
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
        if self.ping_count > 10000 {
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
        self.update_time(ctx);

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
        self.update_time(ctx);

        let Ok(pr) = ctx.entities().get_by_class_name("CDOTA_PlayerResource") else {
            return Ok(());
        };

        if let Ok(game_rules) = ctx.entities().get_by_class_name("CDOTAGamerulesProxy") {
            let is_paused: bool = property!(game_rules, "m_pGameRules.m_bGamePaused");
            let time_tick: i32 = if is_paused {
                property!(game_rules, "m_pGameRules.m_nPauseStartTick")
            } else {
                ctx.net_tick() as i32
            };

            if is_paused && !self.was_paused {
                self.pause_start_time = time_tick;
                self.pause_start_game_time = self.time(ctx)? as i32;
                self.was_paused = true;
            } else if !is_paused && self.was_paused {
                let pause_duration = ((time_tick - self.pause_start_time) as f32 / 30.0).round() as u32;
                if pause_duration > 0 {
                    let mut entry = Entry::new(self.pause_start_game_time as f32);
                    entry.r#type = "game_paused".to_string().into();
                    entry.key = "pause_duration".to_string().into();
                    entry.value = pause_duration.into();
                    self.output(entry)?;
                }
                self.was_paused = false;
            }
        }

        if !self.init {
            let mut added = 0;
            let mut i = 0;
            let mut waiting_for_draft_players = false;
            let mut player_entries = VecDeque::<Entry>::new();
            while added < 10 && i < 30 {
                let Some(player_team) = try_property!(pr, i32, "m_vecPlayerData.{i:04}.m_iPlayerTeam") else {
                    i += 1;
                    continue;
                };
                let Some(team_slot) = try_property!(pr, i32, "m_vecPlayerTeamData.{i:04}.m_iTeamSlot") else {
                    i += 1;
                    continue;
                };
                let Some(steam_id) = try_property!(pr, u64, "m_vecPlayerData.{i:04}.m_iPlayerSteamID") else {
                    i += 1;
                    continue;
                };
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
                let valid_index = self.valid_indices[i as usize];
                let hero_id: i32 = property!(pr, "m_vecPlayerTeamData.{valid_index:04}.m_nSelectedHeroID");
                let hero_handle: usize = property!(pr, "m_vecPlayerTeamData.{valid_index:04}.m_hSelectedHero");
                let player_team: i32 = property!(pr, "m_vecPlayerData.{valid_index:04}.m_iPlayerTeam");
                let team_slot: i32 = property!(pr, "m_vecPlayerTeamData.{valid_index:04}.m_iTeamSlot");
                let mut variant: Option<i32> = try_property!(pr, "m_vecPlayerTeamData.{valid_index:04}.m_nSelectedHeroVariant");
                let mut facet_hero_id = None;

                let data_team = if player_team == 2 {
                    ctx.entities().get_by_class_name("CDOTA_DataRadiant")?
                } else {
                    ctx.entities().get_by_class_name("CDOTA_DataDire")?
                };

                let mut entry = Entry::new(self.time(ctx)?);
                entry.r#type = "interval".to_string().into();
                entry.slot = i.into();
                entry.repicked = try_property!(pr, "m_vecPlayerTeamData.{valid_index:04}.m_bHasRepicked");
                entry.randomed = try_property!(pr, "m_vecPlayerTeamData.{valid_index:04}.m_bHasRandomed");
                entry.pred_vict = try_property!(pr, "m_vecPlayerTeamData.{valid_index:04}.m_bHasPredictedVictory");
                entry.firstblood_claimed = try_property!(pr, "m_vecPlayerTeamData.{valid_index:04}.m_iFirstBloodClaimed");
                entry.teamfight_participation =
                    try_property!(pr, f32, "m_vecPlayerTeamData.{valid_index:04}.m_flTeamFightParticipation").filter(|x| x.is_finite());
                entry.level = try_property!(pr, "m_vecPlayerTeamData.{valid_index:04}.m_iLevel");
                entry.kills = try_property!(pr, "m_vecPlayerTeamData.{valid_index:04}.m_iKills");
                entry.deaths = try_property!(pr, "m_vecPlayerTeamData.{valid_index:04}.m_iDeaths");
                entry.assists = try_property!(pr, "m_vecPlayerTeamData.{valid_index:04}.m_iAssists");
                entry.denies = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iDenyCount");
                entry.obs_placed = try_property!(data_team, "m_vecDataTeam.{team_slot:04}.m_iObserverWardsPlaced");
                entry.observers_placed = entry.obs_placed;
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
                        let cell_x = try_property!(hero, u8, "CBodyComponent.m_cellX");
                        let cell_y = try_property!(hero, u8, "CBodyComponent.m_cellY");
                        let vec_x = try_property!(hero, f32, "CBodyComponent.m_vecX");
                        let vec_y = try_property!(hero, f32, "CBodyComponent.m_vecY");
                        if let (Some(x), Some(y)) = (cell_x, cell_y) {
                            entry.x = Some(Self::precise_location(x, vec_x));
                            entry.y = Some(Self::precise_location(y, vec_y));
                        }
                        if let Some(facet_key) = try_property!(hero, u64, "m_iHeroFacetKey") {
                            facet_hero_id = ((facet_key >> 32) as i32).into();
                            variant = ((facet_key & 0xFF) as i32).into();
                        }
                        entry.unit = hero.class().name().to_string().into();
                        entry.hero_id = hero_id.into();
                        entry.variant = variant;
                        entry.facet_hero_id = facet_hero_id;
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
                                    entry.r#type = upper_camel_to_screaming_snake("DotaAbilityLevel").into();
                                    entry.targetname = hero_name.clone().into();
                                    entry.valuename = ability.id.clone().into();
                                    entry.abilitylevel = ability.level.into();
                                    self.abilities_tracking.insert(hero_name.to_owned() + &ability.id, ability.level);
                                    self.output(entry)?;
                                }
                            }

                            let hero_inventory = self.get_hero_inventory(ctx, hero);
                            if self.time(ctx)? as i32 - self.start_time as i32 == 1 {
                                for item in &hero_inventory {
                                    let mut starting_items = Entry::new(self.time(ctx)?);
                                    starting_items.r#type = "STARTING_ITEM".to_string().into();
                                    starting_items.targetname = hero_name.clone().into();
                                    starting_items.valuename = item.id.clone().into();
                                    starting_items.slot = entry.slot;
                                    starting_items.value = (if entry.slot.unwrap() < 5 { 0 } else { 123 } + entry.slot.unwrap() as u32).into();
                                    starting_items.itemslot = item.slot.into();
                                    starting_items.charges = item.num_charges;
                                    starting_items.secondary_charges = item.num_secondary_charges;
                                    self.output(starting_items)?;
                                }
                            }

                            if !self.is_player_starting_items_written[entry.slot.unwrap() as usize] {
                                self.is_player_starting_items_written[entry.slot.unwrap() as usize] = true;
                                for item in &hero_inventory {
                                    let mut starting_items = Entry::new(self.time(ctx)?);
                                    starting_items.r#type = "DOTA_COMBATLOG_PURCHASE".to_string().into();
                                    starting_items.targetname = hero_name.clone().into();
                                    starting_items.valuename = item.id.clone().into();
                                    starting_items.slot = entry.slot;
                                    starting_items.value = (if entry.slot.unwrap() < 5 { 0 } else { 123 } + entry.slot.unwrap() as u32).into();
                                    starting_items.charges = item.num_charges;
                                    self.output(starting_items)?;
                                }
                            }
                            entry.hero_inventory = hero_inventory.into();
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

    #[on_entity]
    fn on_entity(&mut self, ctx: &Context, event: EntityEvents, entity: &Entity) -> ObserverResult {
        if event == EntityEvents::Created {
            let entity_name = entity.class().name();
            if entity_name == "CDOTAWearableItem" {
                let account_id: u64 = property!(entity, "m_iAccountID");
                let item_definition_idx: i32 = property!(entity, "m_iItemDefinitionIndex");
                if account_id > 0 {
                    let account_id64: u64 = 76561197960265728 + account_id;
                    let player_slot: i32 = *self.steam_id_to_player_slot.get(&account_id64).unwrap_or(&0);
                    self.cosmetics_map.insert(item_definition_idx, player_slot);
                }
            } else if entity_name.starts_with("CDOTA_Item_Tier") && entity_name.ends_with("Token") {
                let mut entry = Entry::new(self.time(ctx)?);
                entry.r#type = "neutral_token".to_string().into();
                entry.slot = self.get_player_slot(entity).ok();
                entry.key = entity_name.strip_prefix("CDOTA_Item_").unwrap_or(entity_name).to_string().into();
                self.output(entry)?;
            } else if entity_name.starts_with("CDOTA_Item_") {
                let is_neutral_active_drop = try_property!(entity, bool, "m_bIsNeutralActiveDrop");
                let is_neutral_passive_drop = try_property!(entity, bool, "m_bIsNeutralPassiveDrop");
                let neutral_drop_team = try_property!(entity, i32, "m_nNeutralDropTeam").unwrap_or_default();
                if neutral_drop_team != 0 && (is_neutral_active_drop.unwrap_or_default() || is_neutral_passive_drop.unwrap_or_default()) {
                    let mut entry = Entry::new(self.time(ctx)?);
                    entry.r#type = "neutral_item_history".to_string().into();
                    entry.slot = self.get_player_slot(entity).ok();
                    entry.key = entity_name.strip_prefix("CDOTA_Item_").unwrap_or(entity_name).to_string().into();
                    entry.is_neutral_active_drop = is_neutral_active_drop;
                    entry.is_neutral_passive_drop = is_neutral_passive_drop;
                    self.output(entry)?;
                }
            }
        }
        Ok(())
    }

    #[on_combat_log]
    fn handle_cle(&mut self, cle: &CombatLogEntry) -> ObserverResult {
        let time = cle.timestamp()?.round();
        self.time = time as i32;
        let mut entry = Entry::new(time);
        entry.r#type = debug_name_to_screaming_snake(cle.r#type()).into();
        entry.attackername = cle.attacker_name().unwrap_or("dota_unknown").to_string().into();
        entry.targetname = cle.target_name().unwrap_or("dota_unknown").to_string().into();
        entry.sourcename = cle.damage_source_name().unwrap_or("dota_unknown").to_string().into();
        entry.targetsourcename = cle.target_source_name().unwrap_or("dota_unknown").to_string().into();
        entry.inflictor = cle.inflictor_name().unwrap_or("dota_unknown").to_string().into();
        entry.attackerhero = cle.is_attacker_hero().unwrap_or(false).into();
        entry.targethero = cle.is_target_hero().unwrap_or(false).into();
        entry.attackerillusion = cle.is_attacker_illusion().unwrap_or(false).into();
        entry.targetillusion = cle.is_target_illusion().unwrap_or(false).into();
        entry.value = cle.value().unwrap_or_default().into();
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

        if (cle.r#type() as i32) <= (DotaCombatlogTypes::DotaCombatlogFirstBlood as i32) {
            self.output(entry)?;
        }

        Ok(())
    }

    #[on_message]
    fn on_chat_event(&mut self, ctx: &Context, event: CDotaUserMsgChatEvent) -> ObserverResult {
        let mut entry = Entry::new(self.time(ctx)?);
        entry.r#type = debug_name_to_screaming_snake(event.r#type()).into();
        entry.player1 = event.playerid_1().into();
        entry.player2 = event.playerid_2().into();
        entry.value = event.value().into();
        self.output(entry)
    }

    #[on_message]
    fn on_all_chat_message(&mut self, ctx: &Context, event: CDotaUserMsgChatMessage) -> ObserverResult {
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
    fn on_chat_wheel(&mut self, ctx: &Context, event: CDotaUserMsgChatWheel) -> ObserverResult {
        let mut entry = Entry::new(self.time(ctx)?);
        entry.r#type = "chatwheel".to_string().into();
        entry.slot = event.player_id().into();
        entry.key = event.chat_message_id().to_string().into();
        self.output(entry)
    }
}

impl GameTimeObserver for App {
    fn on_game_started(&mut self, _ctx: &Context, start_time: f32) -> ObserverResult {
        self.start_time = start_time.round();
        self.flush_log_buffer()
    }
}

impl WardsObserver for App {
    fn on_ward(&mut self, ctx: &Context, ward_class: WardClass, event: WardEvent, ward: &Entity) -> ObserverResult {
        let mut entry = Entry::new(self.time(ctx)?);

        let is_obs = ward_class == WardClass::Observer;
        let x: u8 = property!(ward, "CBodyComponent.m_cellX");
        let y: u8 = property!(ward, "CBodyComponent.m_cellY");
        let z: u8 = property!(ward, "CBodyComponent.m_cellZ");
        let vec_x = try_property!(ward, f32, "CBodyComponent.m_vecX");
        let vec_y = try_property!(ward, f32, "CBodyComponent.m_vecY");
        let vec_z = try_property!(ward, f32, "CBodyComponent.m_vecZ");

        entry.r#type = (if is_obs { "obs".to_string() } else { "sen".to_string() } + if event != WardEvent::Placed { "_left" } else { "" }).into();
        entry.entityleft = (event != WardEvent::Placed).into();
        entry.ehandle = ward.handle().into();
        entry.x = Some(Self::precise_location(x, vec_x));
        entry.y = Some(Self::precise_location(y, vec_y));
        entry.z = Some(Self::precise_location(z, vec_z));

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

    let start = std::time::Instant::now();

    let input = BufReader::new(std::fs::File::open(filepath)?);
    let mut parser = Parser::from_reader(input)?;

    let game_time = parser.register_observer::<GameTime>();
    let wards = parser.register_observer::<Wards>();
    let app = parser.register_observer::<App>();

    app.borrow_mut().game_time = game_time.clone();
    app.borrow_mut().output = Some(BufWriter::new(std::io::stdout()));

    game_time.borrow_mut().register_observer(app.clone());
    wards.borrow_mut().register_observer(app.clone());

    parser.run_to_end()?;
    app.borrow_mut().flush_writer()?;
    eprintln!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
