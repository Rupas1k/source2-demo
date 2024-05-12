use d2_stampede::prelude::*;
use nohash_hasher::IntMap;

#[derive(Default)]
struct LifeStateObserver {
    current_life_state: IntMap<i32, i32>,
}

impl Observer for LifeStateObserver {
    fn on_entity(
        &mut self,
        ctx: &Parser,
        event: EntityEvent,
        entity: &Entity,
    ) -> d2_stampede::Result<()> {
        if EntityEvent::Created == event || EntityEvent::Updated == event {
            if let Ok(life_state) = entity.get_property_by_name("m_lifeState") {
                let new_state: i32 = life_state.try_into()?;
                let old_state: i32 = *self.current_life_state.get(&entity.index()).unwrap_or(&2);
                if old_state != new_state {
                    match new_state {
                        0 => {
                            println!(
                                "{:06}: {} at index {} has spawned",
                                ctx.tick,
                                entity.class().name(),
                                entity.index()
                            );
                            self.current_life_state.insert(entity.index(), new_state);
                        }
                        1 => {
                            println!(
                                "{:06}: {} at index {} has died",
                                ctx.tick,
                                entity.class().name(),
                                entity.index()
                            );
                            self.current_life_state.insert(entity.index(), new_state);
                        }
                        2 => {
                            self.current_life_state.remove(&entity.index());
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        if EntityEvent::Deleted == event {
            self.current_life_state.remove(&entity.index());
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(filepath) = args.get(1) else {
        eprintln!("Usage: {} <demofile>", args[0]);
        return Ok(());
    };

    let Ok(file) = std::fs::File::open(filepath) else {
        eprintln!("Failed to open file: {}", filepath);
        return Ok(());
    };

    let mmap = unsafe { memmap2::Mmap::map(&file)? };
    let mut parser = Parser::new(&mmap);

    parser.register_observer::<LifeStateObserver>();

    #[cfg(feature = "bench")]
    let start = std::time::Instant::now();

    if let Err(e) = parser.run() {
        eprintln!("Parser error: {:?}", e);
    };

    #[cfg(feature = "bench")]
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
