use d2_stampede::prelude::*;
use d2_stampede::proto::*;
use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use d2_stampede::try_observers;

#[derive(Default)]
pub struct GameState {
    observers: Vec<Rc<RefCell<dyn GameStateObserver + 'static>>>,
}

impl GameState {
    pub fn register_observer(&mut self, obs: Rc<RefCell<dyn GameStateObserver + 'static>>) {
        self.observers.push(obs)
    }
}

impl Observer for GameState {
    fn on_combat_log(&mut self, ctx: &Context, combat_log: &CombatLog) -> Result<()> {
        if combat_log.type_() == DotaCombatlogTypes::DotaCombatlogGameState {
            let state = DotaGameState::try_from(combat_log.value()? as i32)?;
            try_observers!(self, on_game_state_change(ctx, state))?;
            match state {
                DotaGameState::DotaGamerulesStatePreGame => try_observers!(self, on_pregame(ctx))?,
                DotaGameState::DotaGamerulesStateGameInProgress => {
                    try_observers!(self, on_game_start(ctx))?
                }
                DotaGameState::DotaGamerulesStatePostGame => {
                    try_observers!(self, on_postgame(ctx))?
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[allow(unused_variables)]
pub trait GameStateObserver {
    fn on_game_state_change(&self, ctx: &Context, new_state: DotaGameState) -> Result<()> {
        Ok(())
    }

    fn on_pregame(&self, ctx: &Context) -> Result<()> {
        Ok(())
    }

    fn on_game_start(&self, ctx: &Context) -> Result<()> {
        Ok(())
    }

    fn on_postgame(&self, ctx: &Context) -> Result<()> {
        Ok(())
    }
}
