use source2_demo::prelude::*;
use source2_demo::proto::*;

#[derive(Default)]
struct Chat {
    c: CSvcMsgGameEventList,
}

#[observer]
impl Chat {
    #[on_message]
    fn x(&mut self, ctx: &Context, msg: &CSvcMsgGameEventList) -> ObserverResult {
        self.c = msg.clone();
        println!("{:?}", self.c);
        // panic!();
        Ok(())
    }
    // #[on_message]
    // fn on_chat_message(&mut self, ctx: &Context, msg: &CSvcMsgGameEvent) ->
    // ObserverResult {     let name = self
    //         .c
    //         .descriptors
    //         .iter()
    //         .find(|x| x.eventid == msg.eventid)
    //         .unwrap()
    //         .name();
    //     println!("{}", name);
    //     if name == "player_used_ability" {
    //         // println!("{:?}", msg);
    //         // println!("{}", ctx.string_tables());
    //         // panic!();
    //     }
    //
    //     // if msg.eventid == 165 {
    //     //     println!("{:?}", msg);
    //     // }
    //     Ok(())
    // }
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
