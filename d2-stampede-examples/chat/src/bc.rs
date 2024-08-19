// use d2_stampede::prelude::*;
// use d2_stampede::proto::*;
//
// #[derive(Default)]
// struct Chat;
//
// impl Observer for Chat {
//     fn on_dota_user_message(
//         &mut self,
//         ctx: &Context,
//         msg_type: EDotaUserMessages,
//         msg: &[u8],
//     ) -> ObserverResult {
//         if msg_type == EDotaUserMessages::DotaUmChatMessage {
//             if let Ok(pr) =
// ctx.entities().get_by_class_name("CDOTA_PlayerResource") {
// let message = CdotaUserMsgChatMessage::decode(msg)?;                 let
// name: String = property!(                     pr,
//                     "m_vecPlayerData.{:04}.m_iszPlayerName",
//                     message.source_player_id()
//                 );
//                 println!("{}: {}", name, message.message_text());
//             }
//         }
//         Ok(())
//     }
// }
//
// fn main() -> std::io::Result<()> {
//     let args = std::env::args().collect::<Vec<_>>();
//     let Some(filepath) = args.get(1) else {
//         eprintln!("Usage: {} <demofile>", args[0]);
//         return Ok(());
//     };
//
//     let Ok(file) = std::fs::File::open(filepath) else {
//         eprintln!("Failed to open file: {}", filepath);
//         return Ok(());
//     };
//
//     let replay = unsafe { memmap2::Mmap::map(&file)? };
//     let Ok(mut parser) = Parser::new(&replay) else {
//         eprintln!("Not a replay");
//         return Ok(());
//     };
//
//     parser.register_observer::<Chat>();
//
//     let start = std::time::Instant::now();
//
//     if let Err(e) = parser.run_to_end() {
//         eprintln!("Parser error: {}", e);
//     };
//
//     // parser.run_to_end()?;
//
//     println!("Elapsed: {:?}", start.elapsed());
//
//     Ok(())
// }

use d2_stampede::prelude::*;
use d2_stampede::proto::*;
use std::time::Instant;

// Create new observer
#[derive(Default)]
struct Chat {
    msg_count: u32,
}

// Implement Observer trait for struct
impl Observer for Chat {
    fn on_dota_user_message(
        &mut self,
        ctx: &Context,
        msg_type: EDotaUserMessages,
        msg: &[u8],
    ) -> ObserverResult {
        if msg_type == EDotaUserMessages::DotaUmChatMessage {
            if let Ok(player_resource) = ctx.entities().get_by_class_name("CDOTA_PlayerResource") {
                let message = CDotaUserMsgChatMessage::decode(msg)?;
                let name: String = property!(
                    player_resource,
                    "m_vecPlayerData.{:04}.m_iszPlayerName",
                    message.source_player_id()
                );
                self.msg_count += 1;
                println!("{}: {}", name, message.message_text());
            }
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    // Read replay file using memmap
    // let replay = unsafe {
    // memmap2::Mmap::map(&std::fs::File::open("replay.dem")?)? };
    let replay = unsafe {
        memmap2::Mmap::map(&std::fs::File::open(
            "../../replays/7391247517_310492352.dem",
        )?)?
    };

    // Or without memmap
    //
    // use std::io::Read;
    // let mut file = std::fs::File::open("replay.dem")?;
    // let mut replay = Vec::new();
    // file.read_to_end(&mut replay)?;

    // Create parser instance.
    let mut parser = Parser::new(&replay)?;

    // Register one or many observers
    let chat = parser.register_observer::<Chat>();

    let x = Instant::now();
    // Parse replay from start to end
    parser.run_to_end()?;

    println!("Elapsed: {:?}", x.elapsed());

    println!("Messages count: {}", chat.borrow_mut().msg_count);

    // Or parse only part of replay
    parser.jump_to_tick(120000)?;
    println!(
        "{}",
        parser
            .context()
            .entities()
            .get_by_class_name("CDOTAGamerulesProxy")?
    );
    // parser.run_to_tick(11000)?;

    // Access context without running
    // parser.jump_to_tick(0)?;
    // parser.run_to_end()?;
    // for t in parser.context().string_tables().iter() {
    //     println!("{} {:?}", t.name(), t.iter().collect::<Vec<_>>().len());
    // }
    // println!("{}", parser.context().tick());
    // println!(
    //     "{}",
    //     parser
    //         .context()
    //         .entities()
    //         .iter()
    //         .filter(|&e| e.class().name().contains("CDOTA_Unit_Hero"))
    //         .count()
    // );
    // println!(
    //     "{}",
    //     parser
    //         .context()
    //         .entities()
    //         .get_by_class_name("CDOTA_Unit_Hero_Phoenix")?
    // );
    // println!(
    //     "{}",
    //     parser
    //         .context()
    //         .entities()
    //         .iter()
    //         .filter(|&x| x.class().name().starts_with("CDOTA_Unit_Hero_Puck"))
    //         .collect::<Vec<_>>()
    // );
    // println!("{}", parser.context().last_full_packet_tick);

    Ok(())
}
