use d2_stampede::prelude::*;
use d2_stampede_observers::wards::*;

#[derive(Default)]
struct WardEvents;

impl WardsObserver for WardEvents {
    fn on_ward(
        &mut self,
        ctx: &Context,
        ward_class: WardClass,
        event: WardEvent,
        ward: &Entity,
    ) -> ObserverResult {
        println!(
            "{:06} Ward event: {:?} {:?} {:?}",
            ctx.tick(),
            ward.index(),
            ward_class,
            event
        );
        Ok(())
    }
}

impl Observer for WardEvents {}

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(filepath) = args.get(1) else {
        eprintln!("Usage: {} <demofile>", args[0]);
        return Ok(());
    };

    let replay = unsafe { memmap2::Mmap::map(&std::fs::File::open(filepath)?)? };
    let mut parser = Parser::new(&replay)?;

    parser.register_observer::<WardEvents>();

    let start = std::time::Instant::now();
    parser.run_to_end()?;
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
