use source2_demo::prelude::*;
use source2_demo::proto::*;

#[derive(Default)]
struct Chat;

#[observer]
impl Chat {
    #[on_message]
    fn handle_chat_msg(
        &mut self,
        ctx: &Context,
        chat_msg: CDotaUserMsgChatMessage,
    ) -> ObserverResult {
        if let Ok(pr) = ctx.entities().get_by_class_name("CDOTA_PlayerResource") {
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
