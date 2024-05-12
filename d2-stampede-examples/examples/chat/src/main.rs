use d2_stampede::prelude::*;
use d2_stampede::proto::*;

#[derive(Default)]
struct ChatObserver;

impl Observer for ChatObserver {
    fn on_base_user_message(
        &mut self,
        _ctx: &Parser,
        msg_type: EBaseUserMessages,
        msg: &[u8],
    ) -> d2_stampede::Result<()> {
        if msg_type == EBaseUserMessages::UmSayText2 {
            let message = CUserMessageSayText2::decode(msg)?;
            println!("{}: {}", message.param1(), message.param2());
        }
        Ok(())
    }

    fn on_dota_user_message(
        &mut self,
        ctx: &Parser,
        msg_type: EDotaUserMessages,
        msg: &[u8],
    ) -> d2_stampede::Result<()> {
        if msg_type == EDotaUserMessages::DotaUmChatMessage {
            if let Ok(pr) = ctx.entities.get_by_class_name("CDOTA_PlayerResource") {
                let message = CdotaUserMsgChatMessage::decode(msg)?;
                let name: String = pr
                    .get_property_by_name(&format!(
                        "m_vecPlayerData.{:04}.m_iszPlayerName1",
                        message.source_player_id()
                    ))?
                    .try_into()?;
                println!("{}: {}", name, message.message_text());
            }
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
    parser.register_observer::<ChatObserver>();

    #[cfg(feature = "bench")]
    let start = std::time::Instant::now();

    if let Err(e) = parser.run() {
        eprintln!("Parser error: {:?}", e);
    };

    #[cfg(feature = "bench")]
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
