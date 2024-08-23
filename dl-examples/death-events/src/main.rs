use source2_demo::prelude::*;

#[derive(Default)]
struct Chat;

#[observer]
impl Chat {
    fn get_hero_name(
        &self,
        ctx: &Context,
        handle: usize,
    ) -> Result<String, source2_demo::error::ParserError> {
        Ok(ctx
            .string_tables()
            .get_by_name("EntityNames")?
            .get_row_by_index(property!(
                ctx.entities().get_by_handle(handle)?,
                "m_pEntity.m_nameStringableIndex"
            ))?
            .key()
            .to_string())
    }

    #[on_game_event]
    fn x(&mut self, ctx: &Context, msg: &GameEvent) -> ObserverResult {
        if msg.name() == "player_death" {
            let victim: i32 = msg.get_value("userid_pawn")?.try_into()?;
            let attacker: i32 = msg.get_value("attacker_pawn")?.try_into()?;
            println!(
                "{:06}: {} killed {}",
                ctx.tick(),
                self.get_hero_name(ctx, attacker as usize)?,
                self.get_hero_name(ctx, victim as usize)?
            );
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
