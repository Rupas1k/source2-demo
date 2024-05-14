use d2_stampede::prelude::*;
use d2_stampede::proto::DotaCombatlogTypes;
use nohash_hasher::IntMap;
use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WardClasses {
    Observer,
    Sentry,
}

#[derive(Debug, Eq, PartialEq)]
pub enum WardEvents {
    Placed,
    Killed(Box<str>),
    Expired,
}

impl WardClasses {
    fn from_target_name(value: &str) -> d2_stampede::Result<Self> {
        match value {
            "npc_dota_observer_wards" => Ok(WardClasses::Observer),
            "npc_dota_sentry_wards" => Ok(WardClasses::Sentry),
            _ => unreachable!(),
        }
    }

    fn from_class_name(value: &str) -> d2_stampede::Result<Self> {
        match value {
            "CDOTA_NPC_Observer_Ward" => Ok(WardClasses::Observer),
            "CDOTA_NPC_Observer_Ward_TrueSight" => Ok(WardClasses::Sentry),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct PendingEvent {
    entity_idx: i32,
    life_state: i32,
}

pub struct Wards {
    pending_events: VecDeque<PendingEvent>,
    current_life_state: IntMap<i32, i32>,
    killers: FxHashMap<WardClasses, VecDeque<Box<str>>>,

    observers: Vec<Rc<RefCell<dyn WardsObserver>>>,
}

impl Wards {
    pub fn new() -> Self {
        let mut killers = FxHashMap::default();
        killers.insert(WardClasses::Observer, VecDeque::new());
        killers.insert(WardClasses::Sentry, VecDeque::new());

        Wards {
            pending_events: Default::default(),
            current_life_state: IntMap::default(),
            killers,
            observers: Vec::new(),
        }
    }

    pub fn register_observer<T: WardsObserver + 'static>(&mut self, obs: Rc<RefCell<T>>) {
        self.observers.push(obs as Rc<RefCell<dyn WardsObserver>>)
    }
}

impl Default for Wards {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused_variables)]
impl Observer for Wards {
    fn on_tick_end(&mut self, ctx: &Parser) -> d2_stampede::Result<()> {
        while let Some(ev) = self.pending_events.pop_front() {
            let old_state = *self.current_life_state.get(&ev.entity_idx).unwrap_or(&2);
            let new_state = ev.life_state;

            let ward_class = WardClasses::from_class_name(
                ctx.entities.get_by_index(&ev.entity_idx)?.class().name(),
            )?;

            if old_state != new_state {
                self.current_life_state.insert(ev.entity_idx, new_state);
                // created
                if new_state == 0 {
                    self.observers.iter_mut().try_for_each(|obs| {
                        obs.borrow_mut().on_ward(
                            ctx,
                            ward_class,
                            WardEvents::Placed,
                            ctx.entities.get_by_index(&ev.entity_idx)?,
                        )
                    })?
                }
                // killed / expired
                if new_state == 1 {
                    if let Some(killer) = self.killers.get_mut(&ward_class).unwrap().pop_front() {
                        self.observers.iter_mut().try_for_each(|obs| {
                            obs.borrow_mut().on_ward(
                                ctx,
                                ward_class,
                                WardEvents::Killed(Box::clone(&killer)),
                                ctx.entities.get_by_index(&ev.entity_idx)?,
                            )
                        })?
                    } else {
                        self.observers.iter_mut().try_for_each(|obs| {
                            obs.borrow_mut().on_ward(
                                ctx,
                                ward_class,
                                WardEvents::Expired,
                                ctx.entities.get_by_index(&ev.entity_idx)?,
                            )
                        })?
                    }
                }
            }
        }
        Ok(())
    }

    fn on_entity(
        &mut self,
        ctx: &Parser,
        event: EntityEvent,
        entity: &Entity,
    ) -> d2_stampede::Result<()> {
        if event == EntityEvent::Created
            && WardClasses::from_class_name(entity.class().name()).is_ok()
        {
            if let Ok(life_state) = entity.get_property_by_name("m_lifeState") {
                self.current_life_state.remove(&entity.index());
                self.pending_events.push_back(PendingEvent {
                    entity_idx: entity.index(),
                    life_state: life_state.try_into()?,
                });
            }
        }
        if event == EntityEvent::Updated && self.current_life_state.contains_key(&entity.index()) {
            if let Ok(life_state) = entity.get_property_by_name("m_lifeState") {
                self.pending_events.push_back(PendingEvent {
                    entity_idx: entity.index(),
                    life_state: life_state.try_into()?,
                });
            }
        }
        if event == EntityEvent::Deleted && self.current_life_state.contains_key(&entity.index()) {
            self.current_life_state.remove(&entity.index());
        }
        Ok(())
    }

    fn on_combat_log(&mut self, ctx: &Parser, combat_log: &CombatLog) -> d2_stampede::Result<()> {
        if combat_log.type_() == DotaCombatlogTypes::DotaCombatlogDeath
            && combat_log.target_name().is_ok()
            && WardClasses::from_target_name(combat_log.target_name()?).is_ok()
        {
            if let (Ok(killer), Ok(attacker)) =
                (combat_log.damage_source_name(), combat_log.attacker_name())
            {
                if WardClasses::from_target_name(attacker).is_err() {
                    self.killers
                        .get_mut(&WardClasses::from_target_name(combat_log.target_name()?)?)
                        .unwrap()
                        .push_back(killer.into());
                }
            } else {
                self.killers
                    .get_mut(&WardClasses::from_target_name(combat_log.target_name()?)?)
                    .unwrap()
                    .push_back(combat_log.damage_source_name()?.into());
            }
        }
        Ok(())
    }

    fn epilogue(&mut self, ctx: &Parser) -> d2_stampede::Result<()> {
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
        ctx: &Parser,
        ward_class: WardClasses,
        event: WardEvents,
        ward: &Entity,
    ) -> d2_stampede::Result<()> {
        Ok(())
    }
}
