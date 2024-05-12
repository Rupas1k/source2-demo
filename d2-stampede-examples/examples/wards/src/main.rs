use crate::wards::{WardClasses, WardEvents, Wards, WardsObserver};
use d2_stampede::prelude::*;

mod wards;

#[derive(Default)]
struct MyObs;

impl WardsObserver for MyObs {
    fn on_ward(
        &mut self,
        ctx: &Parser,
        ward_class: WardClasses,
        event: WardEvents,
        ward: &Entity,
    ) -> d2_stampede::Result<()> {
        println!(
            "{:06} Ward event: {:?} {:?} {:?}",
            ctx.tick,
            ward.index(),
            ward_class,
            event
        );
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

    let wards = parser.register_observer::<Wards>();
    wards.borrow_mut().register_observer::<MyObs>();

    #[cfg(feature = "bench")]
    let start = std::time::Instant::now();

    if let Err(e) = parser.run() {
        eprintln!("Parser error: {:?}", e);
    };

    #[cfg(feature = "bench")]
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
