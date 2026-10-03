use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{Result, bail};

use source2_demo::prelude::*;

#[derive(Default)]
pub struct GameTime {
    start_time: Option<f32>,
    observers: Vec<Rc<RefCell<dyn GameTimeObserver + 'static>>>,
}

impl GameTime {
    pub fn register_observer<T: GameTimeObserver + 'static>(&mut self, obs: Rc<RefCell<T>>) {
        self.observers.push(obs)
    }

    #[inline(always)]
    pub fn tick(&self, ctx: &Context) -> Result<i32> {
        let Ok(game_rules) = ctx.entities().get_by_class_name("CDOTAGamerulesProxy") else {
            bail!("No CDOTAGamerulesProxy.")
        };

        if let Some(time) = try_property!(game_rules, f32, "m_pGameRules.m_fGameTime") {
            return Ok((time * 30.0).round() as i32);
        }

        let is_paused: bool = property!(game_rules, "m_pGameRules.m_bGamePaused");

        let time_tick: i32 = match is_paused {
            true => property!(game_rules, "m_pGameRules.m_nPauseStartTick"),
            false => ctx.net_tick() as i32,
        };

        let paused_ticks: i32 = property!(game_rules, "m_pGameRules.m_nTotalPausedTicks");

        Ok(time_tick - paused_ticks)
    }
}

#[observer]
#[uses_entities]
impl GameTime {
    #[on_tick_start]
    fn on_tick_start(&mut self, ctx: &Context) -> ObserverResult {
        if self.start_time.is_some() {
            return Ok(());
        }

        let Ok(game_rules) = ctx.entities().get_by_class_name("CDOTAGamerulesProxy") else {
            return Ok(());
        };

        let start_time: f32 = property!(game_rules, "m_pGameRules.m_flGameStartTime");

        if start_time > 0.0 {
            self.start_time = Some(start_time);

            self.observers
                .iter()
                .try_for_each(|obs| obs.borrow_mut().on_game_started(ctx, start_time))?;
        }

        Ok(())
    }
}

#[allow(unused_variables)]
pub trait GameTimeObserver {
    fn on_game_started(&mut self, ctx: &Context, start_time: f32) -> ObserverResult {
        Ok(())
    }
}
