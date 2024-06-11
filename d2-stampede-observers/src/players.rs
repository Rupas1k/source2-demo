use anyhow::Result;
use hashbrown::{HashMap, HashSet};
use std::cell::RefCell;
use std::rc::Rc;

use d2_stampede::prelude::*;
use d2_stampede::proto::*;
use d2_stampede::try_observers;

fn class_to_combat_log(class: &str) -> HashSet<Box<str>> {
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

    let mut set = HashSet::default();
    set.insert(name1.into_boxed_str());
    set.insert(name2.into_boxed_str());

    set
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Player {
    pub id: u64,
    pub team: i32,
    pub slot: i32,
    pub hero: Box<str>,
    pub hero_handle: usize,
}

#[derive(Default)]
pub struct Players {
    observers: Vec<Rc<RefCell<dyn PlayersObserver + 'static>>>,

    pub init: bool,
    pub players: Vec<Rc<Player>>,
    pub steam_id_to_player: HashMap<u64, Rc<Player>>,
    pub hero_to_player: HashMap<Box<str>, Rc<Player>>,
    pre_game_tick: Option<u32>,
}

impl Players {
    fn init(&mut self, ctx: &Context) -> Result<()> {
        if let Ok(pr) = ctx.entities.get_by_class_name("CDOTA_PlayerResource") {
            self.players = vec![];
            self.steam_id_to_player = HashMap::default();
            self.hero_to_player = HashMap::default();
            let mut added = 0;
            while added < 10 {
                let id: u64 = property!(pr, "m_vecPlayerData.{added:04}.m_iPlayerSteamID");
                let team: i32 = property!(pr, "m_vecPlayerData.{added:04}.m_iPlayerTeam");
                let slot: i32 = property!(pr, "m_vecPlayerTeamData.{added:04}.m_iTeamSlot");
                let handle: usize = property!(pr, "m_vecPlayerTeamData.{added:04}.m_hSelectedHero");

                let hero_str = ctx.entities.get_by_handle(handle)?.class().name().into();

                let player = Rc::new(Player {
                    id,
                    team,
                    slot,
                    hero: Box::clone(&hero_str),
                    hero_handle: handle,
                });

                self.players.push(player.clone());
                self.hero_to_player.insert(hero_str.clone(), player.clone());

                for name in class_to_combat_log(&hero_str) {
                    self.hero_to_player.insert(name, player.clone());
                }

                self.steam_id_to_player.insert(id, player.clone());

                added += 1;
            }

            self.init = true;

            try_observers!(self, on_players_init(ctx))?;
        }
        Ok(())
    }
}

impl Observer for Players {
    fn on_tick_start(&mut self, ctx: &Context) -> Result<()> {
        if !self.init
            && self.pre_game_tick.is_some()
            && (self.pre_game_tick.unwrap() + 30) < ctx.tick
        {
            self.init(ctx)?;
        }
        Ok(())
    }

    fn on_combat_log(&mut self, ctx: &Context, combat_log: &CombatLog) -> Result<()> {
        if self.pre_game_tick.is_none()
            && combat_log.type_() == DotaCombatlogTypes::DotaCombatlogGameState
            && combat_log.value()? == DotaGameState::DotaGamerulesStatePreGame as u32
        {
            self.pre_game_tick = Some(ctx.tick);
        }
        Ok(())
    }
}

#[allow(unused_variables)]
pub trait PlayersObserver {
    fn on_players_init(&self, ctx: &Context) -> Result<()> {
        Ok(())
    }
}
