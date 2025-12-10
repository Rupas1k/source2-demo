/*!
I used it to estimate(!) fantasy points for TI 2025
**/

use source2_demo::prelude::*;
use source2_demo_observers::dota::game_time::*;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

const ITEMS_PER_HERO: usize = 25;

// CDOTA_Item_Famango
// CDOTA_Item_GreatFamango
// CDOTA_Item_GreaterFamango

#[derive(Debug)]
enum LotusType {
    Famango,
    GreatFamango,
    GreaterFamango,
}

impl LotusType {
    fn from_class_name(class_name: &str) -> Option<Self> {
        match class_name {
            "CDOTA_Item_Famango" => Some(LotusType::Famango),
            "CDOTA_Item_GreatFamango" => Some(LotusType::GreatFamango),
            "CDOTA_Item_GreaterFamango" => Some(LotusType::GreaterFamango),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum LotusEvents {
    NewHandle {
        item_handle: usize,
        hero_handle: usize,
        amount: usize,
    },
    Charge {
        item_handle: usize,
        hero_handle: usize,
        amount: usize,
    },
}

fn format_time(seconds: f32) -> String {
    let total = seconds.round() as u64;
    let minutes = total / 60;
    let secs = total % 60;

    format!("{:02}:{:02}", minutes, secs)
}

#[derive(Default)]
struct Lotuses {
    game_time: Rc<RefCell<GameTime>>,

    last_sum: u32,
    hero_handles: HashMap<usize, usize>,
    handle_to_name: HashMap<usize, Box<str>>,
    lotus_handles: HashMap<usize, usize>, // item_handle -> amount

    events: VecDeque<LotusEvents>,
}

#[observer]
impl Lotuses {
    #[on_tick_start]
    pub fn tick_start(&mut self, ctx: &Context) -> ObserverResult {
        if !self.game_time.borrow().game_started() {
            return Ok(());
        }

        assert_eq!(
            self.hero_handles.len(),
            10,
            "Expected 10 heroes, found {}",
            self.hero_handles.len()
        );

        let mut current_handles = HashMap::<usize, usize>::default();
        let mut lotus_sum = 0;

        for (&hero_handle, _) in self.hero_handles.iter() {
            let hero = ctx.entities().get_by_handle(hero_handle)?;

            for i in 0..ITEMS_PER_HERO {
                let item_handle: usize = property!(hero, "m_hItems.{:04}", i);

                if item_handle == 16777215 {
                    continue;
                }

                let item = ctx.entities().get_by_handle(item_handle)?;

                if LotusType::from_class_name(item.class().name()).is_some() {
                    let amount: usize = property!(item, "m_iCurrentCharges");

                    if !self.lotus_handles.contains_key(&item_handle) {
                        self.events.push_back(LotusEvents::NewHandle {
                            item_handle,
                            hero_handle,
                            amount,
                        });
                    }

                    if self.lotus_handles.contains_key(&item_handle)
                        && amount > self.lotus_handles[&item_handle]
                    {
                        self.events.push_back(LotusEvents::Charge {
                            item_handle,
                            hero_handle,
                            amount,
                        });
                    }

                    current_handles.insert(item_handle, amount as usize);
                }
            }
        }

        for e in ctx.entities().iter() {
            if let Some(lotus_type) = LotusType::from_class_name(e.class().name()) {
                let item_handle = e.handle() as usize;

                let amount: u32 = property!(e, "m_iCurrentCharges");

                lotus_sum += match lotus_type {
                    LotusType::Famango => amount,
                    LotusType::GreatFamango => amount * 3,
                    LotusType::GreaterFamango => amount * 6,
                };

                current_handles.insert(item_handle, amount as usize);
            }
        }

        if lotus_sum > self.last_sum {
            while let Some(event) = self.events.pop_front() {
                match event {
                    LotusEvents::NewHandle {
                        item_handle,
                        hero_handle,
                        amount,
                    } => {
                        let item = ctx.entities().get_by_handle(item_handle)?.class().name();
                        let hero = ctx.entities().get_by_handle(hero_handle)?.class().name();

                        self.hero_handles
                            .entry(hero_handle)
                            .and_modify(|v| *v = v.saturating_add(1))
                            .or_insert(amount);

                        println!(
                            "{}: {} picked by {} ({}->{})",
                            format_time(self.game_time.borrow().time(ctx)?),
                            item,
                            hero,
                            amount - 1,
                            amount
                        );
                    }
                    LotusEvents::Charge {
                        item_handle,
                        hero_handle,
                        amount,
                    } => {
                        let item = ctx.entities().get_by_handle(item_handle)?.class().name();
                        let hero = ctx.entities().get_by_handle(hero_handle)?.class().name();

                        println!(
                            "{}: {} picked by {} ({}->{})",
                            format_time(self.game_time.borrow().time(ctx)?),
                            item,
                            hero,
                            amount - 1,
                            amount
                        );
                        self.hero_handles
                            .entry(hero_handle)
                            .and_modify(|v| *v = v.saturating_add(1))
                            .or_insert(amount);
                    }
                }
            }
        }

        self.lotus_handles = current_handles;
        self.last_sum = lotus_sum;

        Ok(())
    }
}

impl GameTimeObserver for Lotuses {
    fn on_game_started(&mut self, ctx: &Context, _start_time: f32) -> ObserverResult {
        let pr = ctx.entities().get_by_class_name("CDOTA_PlayerResource")?;

        let mut i = 0;

        for e in ctx.entities().iter() {
            if e.class().name().contains("CDOTAPlayerController") {
                let assigned_hero: usize = property!(e, "m_hAssignedHero");

                if assigned_hero == 16777215 {
                    continue;
                }

                let player_name: String = property!(pr, "m_vecPlayerData.{:04}.m_iszPlayerName", i);

                self.handle_to_name
                    .insert(assigned_hero, player_name.into());

                self.hero_handles.insert(assigned_hero, 0);

                i += 1;
            }
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(filepath) = args.get(1) else {
        eprintln!("Usage: {} <demofile>", args[0]);
        return Ok(());
    };

    let replay = unsafe { memmap2::Mmap::map(&std::fs::File::open(filepath)?)? };
    let mut parser = Parser::new(&replay)?;

    let game_time = parser.register_observer::<GameTime>();
    let lotuses = parser.register_observer::<Lotuses>();

    lotuses.borrow_mut().game_time = game_time.clone();
    game_time.borrow_mut().register_app(lotuses.clone());

    let start = std::time::Instant::now();

    parser.run_to_end()?;

    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
