use source2_demo::prelude::*;
use std::collections::HashMap;
use std::io::Write;

struct LifeState {
    current_life_state: HashMap<u32, i32>,
    output: Box<dyn Write>,
}

impl Default for LifeState {
    fn default() -> Self {
        LifeState {
            current_life_state: HashMap::default(),
            output: Box::new(std::io::stdout()),
        }
    }
}

#[observer]
impl LifeState {
    #[on_entity]
    fn handle_entities(&mut self, ctx: &Context, entity: &Entity) -> ObserverResult {
        if let Ok(life_state) = entity.get_property_by_name("m_lifeState") {
            let new_state: i32 = life_state.try_into()?;
            let old_state: i32 = *self.current_life_state.get(&entity.index()).unwrap_or(&2);
            if old_state != new_state {
                match new_state {
                    0 => {
                        writeln!(
                            self.output,
                            "{:06}: {} at index {} has spawned",
                            ctx.tick(),
                            entity.class().name(),
                            entity.index()
                        )?;
                        self.current_life_state.insert(entity.index(), new_state);
                    }
                    1 => {
                        writeln!(
                            self.output,
                            "{:06}: {} at index {} has died",
                            ctx.tick(),
                            entity.class().name(),
                            entity.index()
                        )?;
                        self.current_life_state.insert(entity.index(), new_state);
                    }
                    2 => {
                        self.current_life_state.remove(&entity.index());
                    }
                    _ => unreachable!(),
                }
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

    parser.register_observer::<LifeState>();

    let start = std::time::Instant::now();
    parser.run_to_end()?;
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
