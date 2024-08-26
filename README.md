# source2-demo

## Installation

Install for Dota 2 replays:

```toml
[dependencies]
source2-demo = { git = "https://github.com/Rupas1k/source2-demo", features = ["dota"] }
```

Install for Deadlock replays:

```toml
[dependencies]
source2-demo = { git = "https://github.com/Rupas1k/source2-demo", features = ["deadlock"] }
```

## Quick start

Simple program that prints chat messages from Dota 2 replay. It handles CDotaUserMsgChatMessage protobuf message and
prints player name and message text. \
More examples can be found in corresponding folders.

```rust
use source2_demo::prelude::*;
use source2_demo::proto::*;

// Create struct that implements Default trait
#[derive(Default)]
struct Chat;

// Mark impl block with observer attribute
#[observer]
impl Chat {
    #[on_message] // Use on_message attribute to mark protobuf message handler
    fn handle_chat_msg(
        &mut self,
        ctx: &Context,
        chat_msg: CDotaUserMsgChatMessage, // Use any protobuf message as an argument (CDotaUserMsgChatMessage in this case)
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read replay file
    let replay = unsafe { memmap2::Mmap::map(&std::fs::File::open("replay.dem")?)? };

    // Create parser 
    let mut parser = Parser::new(&replay)?;

    // Register observers
    parser.register_observer::<Chat>();

    // Run it!
    parser.run_to_end()?;

    Ok(())
}

```

## Build examples

```shell
git clone https://github.com/Rupas1k/source2-demo
cd source2-demo/dl-examples # cd source2-demo/d2-examples
cargo build --release
```
