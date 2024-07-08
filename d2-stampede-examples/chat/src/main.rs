use d2_stampede::prelude::*;
use d2_stampede::proto::*;

#[derive(Default)]
struct Chat;

impl Observer for Chat {
    fn on_dota_user_message(
        &mut self,
        ctx: &Context,
        msg_type: EDotaUserMessages,
        msg: &[u8],
    ) -> ObserverResult {
        if msg_type == EDotaUserMessages::DotaUmChatMessage {
            if let Ok(pr) = ctx.entities().get_by_class_name("CDOTA_PlayerResource") {
                let message = CDotaUserMsgChatMessage::decode(msg)?;
                let name: String = property!(
                    pr,
                    "m_vecPlayerData.{:04}.m_iszPlayerName",
                    message.source_player_id()
                );
                println!("{}: {}", name, message.message_text());
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

    parser.register_observer::<Chat>();

    let start = std::time::Instant::now();
    parser.run_to_end()?;
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
