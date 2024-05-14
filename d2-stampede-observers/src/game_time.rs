use anyhow::{anyhow, bail, Result};
use d2_stampede::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct GameTime {
    start_time: Option<f32>,

    observers: Vec<Rc<RefCell<dyn GameTimeObserver + 'static>>>,
}

impl GameTime {
    pub fn start_time(&self) -> Result<f32> {
        self.start_time
            .ok_or_else(|| anyhow!("Game has not started yet."))
    }

    pub fn register_observer(&mut self, obs: Rc<RefCell<dyn GameTimeObserver>>) {
        self.observers.push(obs)
    }

    pub fn tick(&self, ctx: &Parser) -> Result<i32> {
        if let Ok(game_rules) = ctx.entities.get_by_class_name("CDOTAGamerulesProxy") {
            let is_paused: bool = game_rules
                .get_property_by_name("m_pGameRules.m_bGamePaused")?
                .try_into()?;
            let time_tick: i32 = match is_paused {
                true => game_rules
                    .get_property_by_name("m_pGameRules.m_nPauseStartTick")?
                    .try_into()?,
                false => ctx.net_tick as i32,
            };
            let paused_ticks: i32 = game_rules
                .get_property_by_name("m_pGameRules.m_nTotalPausedTicks")?
                .try_into()?;
            return Ok(time_tick - paused_ticks);
        }
        bail!("No CDOTAGamerulesProxy.")
    }
}

impl Observer for GameTime {
    fn on_tick_start(&mut self, ctx: &Parser) -> Result<()> {
        if self.start_time.is_none() {
            if let Ok(game_rules) = ctx.entities.get_by_class_name("CDOTAGamerulesProxy") {
                let start_time: f32 = game_rules
                    .get_property_by_name("m_pGameRules.m_flGameStartTime")?
                    .try_into()?;
                if start_time > 0.0 {
                    self.start_time = Some(start_time);
                    self.observers
                        .iter()
                        .try_for_each(|obs| obs.borrow_mut().on_game_started(ctx, start_time))?;
                }
            }
        }
        Ok(())
    }
}

pub trait GameTimeObserver {
    fn on_game_started(&mut self, ctx: &Parser, start_time: f32) -> Result<()> {
        Ok(())
    }
}
