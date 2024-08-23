use source2_demo::prelude::*;
use source2_demo::proto::DotaCombatlogTypes;

#[derive(Default)]
struct CombatLog;

#[observer]
impl CombatLog {
    #[on_combat_log]
    fn handle_cle(&mut self, _ctx: &Context, combat_log: &CombatLogEntry) -> ObserverResult {
        let time = combat_log.timestamp()?;
        match combat_log.r#type() {
            DotaCombatlogTypes::DotaCombatlogDamage => {
                println!(
                    "{} {} hits {}{} for {} damage ({}->{})",
                    time,
                    combat_log.attacker_name()?,
                    combat_log.target_name()?,
                    combat_log
                        .inflictor_name()
                        .map(|x| format!(" with {x}"))
                        .unwrap_or_default(),
                    combat_log.value()?,
                    combat_log.health()? as u32 + combat_log.value()?,
                    combat_log.health()?
                )
            }
            DotaCombatlogTypes::DotaCombatlogHeal => {
                println!(
                    "{} {}'s {} heals {} for {} health ({}->{})",
                    time,
                    combat_log.attacker_name().unwrap_or("UNKNOWN"),
                    combat_log.inflictor_name().unwrap_or_default(),
                    combat_log.target_name()?,
                    combat_log.value()?,
                    combat_log.health()? as u32 - combat_log.value()?,
                    combat_log.health()?
                )
            }
            DotaCombatlogTypes::DotaCombatlogModifierAdd => {
                println!(
                    "{} {} receives {} buff/debuff from {}",
                    time,
                    combat_log.target_name()?,
                    combat_log.inflictor_name()?,
                    combat_log.attacker_name()?
                );
            }
            DotaCombatlogTypes::DotaCombatlogModifierRemove => {
                println!(
                    "{} {} loses {} buff/debuff",
                    time,
                    combat_log.target_name()?,
                    combat_log.inflictor_name()?
                );
            }
            DotaCombatlogTypes::DotaCombatlogDeath => {
                println!(
                    "{} {} is killed by {}",
                    time,
                    combat_log.target_name()?,
                    combat_log.attacker_name()?
                );
            }
            DotaCombatlogTypes::DotaCombatlogAbility => {
                println!(
                    "{} {} {} ability {} (lvl {}){}{}",
                    time,
                    combat_log.attacker_name()?,
                    if combat_log.is_ability_toggle_on().is_ok()
                        || combat_log.is_ability_toggle_off().is_ok()
                    {
                        "toggles"
                    } else {
                        "casts"
                    },
                    combat_log.inflictor_name()?,
                    combat_log.ability_level()?,
                    if combat_log.is_ability_toggle_on().is_ok() {
                        " on"
                    } else if combat_log.is_ability_toggle_off().is_ok() {
                        " off"
                    } else {
                        ""
                    },
                    if let Ok(target) = combat_log.target_name() {
                        format!(" on {}", target)
                    } else {
                        "".to_string()
                    }
                )
            }
            DotaCombatlogTypes::DotaCombatlogItem => {
                println!(
                    "{} {} uses item {}",
                    time,
                    combat_log.attacker_name()?,
                    combat_log.inflictor_name()?,
                )
            }
            _ => {}
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

    parser.register_observer::<CombatLog>();

    let start = std::time::Instant::now();
    parser.run_to_end()?;
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
