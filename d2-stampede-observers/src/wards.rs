use anyhow::bail;
use d2_stampede::prelude::*;
use d2_stampede::proto::DotaCombatlogTypes;
use d2_stampede::try_observers;
use hashbrown::HashMap;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WardClass {
    Observer,
    Sentry,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum WardEvent {
    Placed,
    Killed(Box<str>),
    Expired,
}

impl WardClass {
    fn from_target_name(value: &str) -> d2_stampede::Result<Self> {
        match value {
            "npc_dota_observer_wards" => Ok(WardClass::Observer),
            "npc_dota_sentry_wards" => Ok(WardClass::Sentry),
            _ => bail!(""),
        }
    }

    fn from_class_name(value: &str) -> d2_stampede::Result<Self> {
        match value {
            "CDOTA_NPC_Observer_Ward" => Ok(WardClass::Observer),
            "CDOTA_NPC_Observer_Ward_TrueSight" => Ok(WardClass::Sentry),
            _ => bail!(""),
        }
    }
}

#[derive(Debug)]
struct PendingEvent {
    entity_idx: u32,
    life_state: i32,
}

pub struct Wards {
    pending_events: VecDeque<PendingEvent>,
    current_life_state: HashMap<u32, i32>,
    killers: HashMap<WardClass, VecDeque<Box<str>>>,

    observers: Vec<Rc<RefCell<dyn WardsObserver + 'static>>>,
}

impl Wards {
    pub fn register_observer<T: WardsObserver + 'static>(&mut self, obs: Rc<RefCell<T>>) {
        self.observers.push(obs as Rc<RefCell<dyn WardsObserver>>)
    }
}

impl Default for Wards {
    fn default() -> Self {
        let mut killers = HashMap::default();
        killers.insert(WardClass::Observer, VecDeque::new());
        killers.insert(WardClass::Sentry, VecDeque::new());

        Wards {
            pending_events: Default::default(),
            current_life_state: HashMap::default(),
            killers,
            observers: Vec::new(),
        }
    }
}

#[allow(unused_variables)]
impl Observer for Wards {
    fn on_tick_end(&mut self, ctx: &Context) -> d2_stampede::Result<()> {
        while let Some(ev) = self.pending_events.pop_front() {
            let old_state = *self.current_life_state.get(&ev.entity_idx).unwrap_or(&2);
            let new_state = ev.life_state;

            let ward_class = WardClass::from_class_name(
                ctx.entities
                    .get_by_index(ev.entity_idx as usize)?
                    .class()
                    .name(),
            )?;

            let event = |event: WardEvent| -> d2_stampede::Result<()> {
                try_observers!(
                    self,
                    on_ward(
                        ctx,
                        ward_class,
                        event.clone(),
                        ctx.entities.get_by_index(ev.entity_idx as usize)?
                    )
                )
            };

            if old_state != new_state {
                self.current_life_state.insert(ev.entity_idx, new_state);
                // created
                if new_state == 0 {
                    event(WardEvent::Placed)?;
                }
                // killed / expired
                if new_state == 1 {
                    if let Some(killer) = self.killers.get_mut(&ward_class).unwrap().pop_front() {
                        event(WardEvent::Killed(killer.clone()))?;
                    } else {
                        event(WardEvent::Expired)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn on_entity(
        &mut self,
        ctx: &Context,
        event: EntityEvents,
        entity: &Entity,
    ) -> d2_stampede::Result<()> {
        if event == EntityEvents::Created
            && WardClass::from_class_name(entity.class().name()).is_ok()
        {
            if let Ok(life_state) = entity.get_property_by_name("m_lifeState") {
                self.current_life_state.remove(&entity.index());
                self.pending_events.push_back(PendingEvent {
                    entity_idx: entity.index(),
                    life_state: life_state.try_into()?,
                });
            }
        }
        if event == EntityEvents::Updated && self.current_life_state.contains_key(&entity.index()) {
            if let Ok(life_state) = entity.get_property_by_name("m_lifeState") {
                self.pending_events.push_back(PendingEvent {
                    entity_idx: entity.index(),
                    life_state: life_state.try_into()?,
                });
            }
        }
        if event == EntityEvents::Deleted && self.current_life_state.contains_key(&entity.index()) {
            self.current_life_state.remove(&entity.index());
        }
        Ok(())
    }

    fn on_combat_log(&mut self, ctx: &Context, combat_log: &CombatLog) -> d2_stampede::Result<()> {
        if combat_log.type_() == DotaCombatlogTypes::DotaCombatlogDeath
            && combat_log.target_name().is_ok()
            && WardClass::from_target_name(combat_log.target_name()?).is_ok()
        {
            if let (Ok(killer), Ok(attacker)) =
                (combat_log.damage_source_name(), combat_log.attacker_name())
            {
                if WardClass::from_target_name(attacker).is_err() {
                    self.killers
                        .get_mut(&WardClass::from_target_name(combat_log.target_name()?)?)
                        .unwrap()
                        .push_back(killer.into());
                }
            } else {
                self.killers
                    .get_mut(&WardClass::from_target_name(combat_log.target_name()?)?)
                    .unwrap()
                    .push_back(combat_log.damage_source_name()?.into());
            }
        }
        Ok(())
    }

    fn epilogue(&mut self, ctx: &Context) -> d2_stampede::Result<()> {
        self.current_life_state.iter().for_each(|state| {
            self.pending_events.push_back(PendingEvent {
                entity_idx: *state.0,
                life_state: 1,
            })
        });
        self.on_tick_end(ctx)
    }
}

#[allow(unused_variables)]
pub trait WardsObserver {
    fn on_ward(
        &mut self,
        ctx: &Context,
        ward_class: WardClass,
        event: WardEvent,
        ward: &Entity,
    ) -> d2_stampede::Result<()> {
        Ok(())
    }
}
