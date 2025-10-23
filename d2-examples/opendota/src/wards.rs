use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

use hashbrown::HashMap;

use source2_demo::prelude::*;
use source2_demo::proto::DotaCombatlogTypes;

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
    fn from_target_name(value: &str) -> Option<Self> {
        match value {
            "npc_dota_observer_wards" => Some(WardClass::Observer),
            "npc_dota_sentry_wards" => Some(WardClass::Sentry),
            _ => None,
        }
    }

    fn from_class_name(value: &str) -> Option<Self> {
        match value {
            "CDOTA_NPC_Observer_Ward" => Some(WardClass::Observer),
            "CDOTA_NPC_Observer_Ward_TrueSight" => Some(WardClass::Sentry),
            _ => None,
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
#[observer]
impl Wards {
    #[on_tick_end]
    fn on_tick_end(&mut self, ctx: &Context) -> ObserverResult {
        while let Some(ev) = self.pending_events.pop_front() {
            let old_state = *self.current_life_state.get(&ev.entity_idx).unwrap_or(&2);
            let new_state = ev.life_state;

            let ward_class = WardClass::from_class_name(ctx.entities().get_by_index(ev.entity_idx as usize)?.class().name()).unwrap();

            let event = |event: WardEvent| -> ObserverResult {
                self.observers.iter().try_for_each(|obs| {
                    obs.borrow_mut()
                        .on_ward(ctx, ward_class, event.clone(), ctx.entities().get_by_index(ev.entity_idx as usize)?)
                })
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

    #[on_entity]
    fn on_entity(&mut self, ctx: &Context, event: EntityEvents, entity: &Entity) -> ObserverResult {
        let is_ward = WardClass::from_class_name(entity.class().name()).is_some();

        if !is_ward {
            return Ok(());
        }

        match event {
            EntityEvents::Created => {
                self.current_life_state.insert(entity.index(), property!(entity, "m_lifeState"));
            }
            EntityEvents::Updated => {
                self.pending_events.push_back(PendingEvent {
                    entity_idx: entity.index(),
                    life_state: property!(entity, "m_lifeState"),
                });
            }
            EntityEvents::Deleted => {
                self.current_life_state.remove(&entity.index());
            }
        }

        Ok(())
    }

    #[on_combat_log]
    fn on_combat_log(&mut self, ctx: &Context, combat_log: &CombatLogEntry) -> ObserverResult {
        let Ok(target_name) = combat_log.target_name() else {
            return Ok(());
        };

        let ward_class = WardClass::from_target_name(target_name);
        if ward_class.is_none() {
            return Ok(());
        }

        if combat_log.r#type() == DotaCombatlogTypes::DotaCombatlogDeath {
            let source_name = combat_log.damage_source_name();
            let attacker_name = combat_log.attacker_name();
            if let (Ok(killer), Ok(attacker)) = (source_name, attacker_name) {
                if WardClass::from_target_name(attacker).is_some() {
                    return Ok(());
                }
                self.killers
                    .get_mut(&WardClass::from_target_name(combat_log.target_name()?).unwrap())
                    .unwrap()
                    .push_back(killer.into());
            } else {
                self.killers
                    .get_mut(&WardClass::from_target_name(combat_log.target_name()?).unwrap())
                    .unwrap()
                    .push_back(combat_log.damage_source_name()?.into());
            }
        }
        Ok(())
    }

    #[on_stop]
    fn on_stop(&mut self, ctx: &Context) -> ObserverResult {
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
    fn on_ward(&mut self, ctx: &Context, ward_class: WardClass, event: WardEvent, ward: &Entity) -> ObserverResult {
        Ok(())
    }
}
