use d2_stampede::prelude::*;
use d2_stampede::proto::DotaCombatlogTypes;

#[derive(Default)]
struct CombatLogObserver;

impl Observer for CombatLogObserver {
    fn on_combat_log(&mut self, _ctx: &Parser, combat_log: &CombatLog) -> d2_stampede::Result<()> {
        let time = combat_log.timestamp()?;
        match combat_log.type_() {
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
                    if combat_log.ability_toggle_on().is_ok()
                        || combat_log.ability_toggle_off().is_ok()
                    {
                        "toggles"
                    } else {
                        "casts"
                    },
                    combat_log.inflictor_name()?,
                    combat_log.ability_level()?,
                    if combat_log.ability_toggle_on().is_ok() {
                        " on"
                    } else if combat_log.ability_toggle_off().is_ok() {
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

    parser.register_observer::<CombatLogObserver>();

    #[cfg(feature = "bench")]
    let start = std::time::Instant::now();

    if let Err(e) = parser.run() {
        eprintln!("Parser error: {:?}", e);
    };

    #[cfg(feature = "bench")]
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}