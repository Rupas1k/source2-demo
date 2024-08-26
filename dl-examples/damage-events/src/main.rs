use source2_demo::prelude::*;
use source2_demo::proto::*;

#[derive(Default)]
struct DamageEvents;

#[observer]
impl DamageEvents {
    fn get_entity_name(
        &self,
        ctx: &Context,
        index: usize,
    ) -> Result<String, source2_demo::error::ParserError> {
        Ok(ctx
            .string_tables()
            .get_by_name("EntityNames")?
            .get_row_by_index(property!(
                ctx.entities().get_by_index(index)?,
                "m_pEntity.m_nameStringableIndex"
            ))?
            .key()
            .to_string())
    }

    #[on_message]
    fn damage(&mut self, ctx: &Context, msg: CCitadelUserMessageDamage) -> ObserverResult {
        // ???
        if msg.r#type() == 0 {
            return Ok(());
        }

        let victim_class = ctx
            .entities()
            .get_by_handle(msg.entindex_victim() as usize)?
            .class()
            .name();

        let attacker_class = ctx
            .entities()
            .get_by_handle(msg.entindex_attacker() as usize)?
            .class()
            .name();

        if victim_class != "CCitadelPlayerPawn" || attacker_class != "CCitadelPlayerPawn" {
            return Ok(());
        }

        println!(
            "{:06}: {} damaged {} for {}{} {} ({}/{})",
            ctx.tick(),
            self.get_entity_name(ctx, msg.entindex_attacker() as usize)?,
            self.get_entity_name(ctx, msg.entindex_victim() as usize)?,
            msg.damage(),
            if msg.damage_absorbed() > 0 {
                format!(
                    "({} absorbed ({}/{}))",
                    msg.damage_absorbed(),
                    msg.victim_shield_new(),
                    msg.victim_shield_max()
                )
            } else {
                "".to_string()
            },
            if msg.entindex_ability() >= 0 && msg.entindex_ability() <= 16384 {
                format!(
                    "with {}",
                    self.get_entity_name(ctx, msg.entindex_ability() as usize)?
                )
            } else {
                "".to_string()
            },
            msg.victim_health_new(),
            msg.victim_health_max(),
        );

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

    parser.register_observer::<DamageEvents>();

    let start = std::time::Instant::now();
    parser.run_to_end()?;
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
