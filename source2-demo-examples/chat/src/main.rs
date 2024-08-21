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
        // println!("{:?}", self.c);
        // panic!();
        Ok(())
    }
    #[on_message]
    fn y(&mut self, ctx: &Context, msg: &CSvcMsgGameEvent) -> ObserverResult {
        let name = self
            .c
            .descriptors
            .iter()
            .find(|x| x.eventid == msg.eventid)
            .unwrap()
            .name();
        // println!("{}", name);
        // if name == "dota_game_state_change" {
        //     println!("{}", ctx.string_tables().get_by_name("CombatLogNames")?);
        //     println!("{:?}", msg);
        //     panic!();
        // }

        // if msg.eventid == 165 {
        //     println!("{:?}", msg);
        // }
        Ok(())
    }
    #[on_message]
    fn handle_chat_msg(
        &mut self,
        ctx: &Context,
        chat_msg: CDotaUserMsgChatMessage,
    ) -> ObserverResult {
        if let Ok(pr) = ctx.entities().get_by_class_name("CDOTA_PlayerResource") {
            // println!(
            //     "{}",
            //     ctx.entities().get_by_class_name("CDOTAPlayerController")?
            // );
            let name: String = property!(
                pr,
                "m_vecPlayerData.{:04}.m_iszPlayerName",
                chat_msg.source_player_id()
            );
            println!("{}: {}", name, chat_msg.message_text());
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

    parser.register_observer::<Chat>();

    let start = std::time::Instant::now();

    parser.run_to_end()?;

    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
