# d2-stampede

Dota 2 replay parser written in Rust.

### About

TODO: Complete this readme

### Quick Start

```rust
use d2_stampede::prelude::*;
use d2_stampede::proto::*;

// Create struct that implements Observer and Default traits
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
    // Read replay file
    let replay = unsafe { memmap2::Mmap::map(&std::fs::File::open("replay.dem")?)? };

    // Create parser 
    let mut parser = Parser::new(&replay)?;

    // Register observers
    parser.register_observer::<Chat>();

    // Parse replay from start to end
    parser.run_to_end()?;

    // Or parse only parts of replay
    parser.jump_to_tick(10000)?;
    parser.run_to_tick(11000)?;

    Ok(())
}

```

### Examples

[d2-stampede-examples](https://github.com/Rupas1k/d2-stampede/tree/master/d2-stampede-examples)

#### Download and Build

```shell
git clone https://github.com/Rupas1k/d2-stampede.git
cd d2-stampede
cargo build --release
```

[odota-rust](https://github.com/rupas1k/odota-rust) - copy of [OpenDota parser](https://github.com/odota/parser) in
Rust \
[d2wm-parser](https://github.com/rupas1k/d2wm-parser) - wards parser with bindings for Python