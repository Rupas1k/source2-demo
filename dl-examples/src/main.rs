use source2_demo::prelude::*;
use source2_demo::proto::*;

#[derive(Default)]
struct Chat {
    c: u32,
}

#[observer]
impl Chat {
    #[on_message]
    fn on_chat_message(
        &mut self,
        ctx: &Context,
        msg: &CCitadelUserMsgHeroKilled,
    ) -> ObserverResult {
        self.c += 1;
        if self.c == 20 {
            println!("{}", ctx.entities());
            // println!("{}", ctx.entities.().get_by_index(65)?);
            // println!("{}", ctx.entities().get_by_handle(2555906)?);
            // println!("{}", ctx.entities().get_by_handle(4457702)?);
            // println!("{}", ctx.entities().get_by_class_name("CCitadelGameRulesProxy")?);
            // println!("{}",
            // ctx.entities().get_by_class_name("CCitadelPlayerController")?);
            // println!("{}", ctx.entities().get_by_class_name("CCitadelTeam")?);
            panic!();
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    // let args = std::env::args().collect::<Vec<_>>();
    // let Some(filepath) = args.get(1) else {
    //     eprintln!("Usage: {} <demofile>", args[0]);
    //     return Ok(());
    // };

    let filepath = "/home/artemrupasov/projects/dl-demo/replays/1585491.dem";

    let replay = unsafe { memmap2::Mmap::map(&std::fs::File::open(filepath)?)? };
    let mut parser = Parser::new(&replay)?;

    parser.register_observer::<Chat>();

    let start = std::time::Instant::now();

    parser.run_to_end()?;

    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
