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

    let replay = unsafe { memmap2::Mmap::map(&file)? };
    let Ok(mut parser) = Parser::new(&replay) else {
        eprintln!("Not a replay");
        return Ok(());
    };

    let wards = parser.register_observer::<Wards>();
    let app = parser.register_observer::<WardEvents>();
    wards.borrow_mut().register_observer(app);

    let start = std::time::Instant::now();

    if let Err(e) = parser.run_to_end() {
        eprintln!("Parser error: {:?}", e);
    };

    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
