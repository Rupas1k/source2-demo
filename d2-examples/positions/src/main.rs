use source2_demo::prelude::*;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Default)]
struct Positions {
    heroes: BTreeSet<u32>,
    out: Option<BufWriter<File>>,
    initialized: bool,
    // 視点チーム（2: Radiant, 3: Dire）
    pov_team: i32,
}

#[observer]
impl Positions {

    #[on_entity]
    fn on_entity(&mut self, _ctx: &Context, ev: EntityEvents, e: &Entity) -> ObserverResult {
        // ヒーロー/プレイヤブルユニット推定: m_iPlayerID を持つエンティティ
        let has_player_id = e.get_property_by_name("m_iPlayerID").is_ok();
        match ev {
            EntityEvents::Created => {
                if has_player_id {
                    self.heroes.insert(e.index());
                }
            }
            EntityEvents::Updated => {
                if has_player_id {
                    self.heroes.insert(e.index());
                }
            }
            EntityEvents::Deleted => {
                if self.heroes.contains(&e.index()) {
                    self.heroes.remove(&e.index());
                }
            }
        }
        Ok(())
    }

    #[on_tick_end]
    fn on_tick_end(&mut self, ctx: &Context) -> ObserverResult {
        // 初回呼び出し時に CSV を遅延初期化
        if self.out.is_none() {
            let f = File::create("positions.csv")?;
            let mut w = BufWriter::new(f);
            writeln!(
                w,
                "time_s,tick,entity_index,team,player_id,player_name,side,x,y,z,class"
            )?;
            self.out = Some(w);
        }
        let Some(out) = self.out.as_mut() else { return Ok(()) };

        // 初回のみ、既存エンティティからヒーローを収集（プロローグ時点の生成分を拾う）
        if !self.initialized {
            for e in ctx.entities().iter() {
                if e.get_property_by_name("m_iPlayerID").is_ok() {
                    self.heroes.insert(e.index());
                }
            }
            self.initialized = true;
        }

        // Dota2 は 30 tick/秒 相当
        let time_s = (ctx.tick() as f32) / 30.0;
        let tick = ctx.tick();

        // PlayerResource（プレイヤ名取得用）
        let player_resource = ctx.entities().get_by_class_name("CDOTA_PlayerResource").ok();

        for &idx in &self.heroes {
            let e = match ctx.entities().get_by_index(idx as usize) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let team: Option<i32> = e
                .get_property_by_name("m_iTeamNum")
                .ok()
                .and_then(|p| p.try_into().ok());
            let player_id: Option<i32> = e
                .get_property_by_name("m_iPlayerID")
                .ok()
                .and_then(|p| p.try_into().ok());

            // プレイヤ名取得（存在しない/未設定なら空文字）
            let player_name = if let (Some(pr), Some(pid)) = (player_resource, player_id) {
                // try_property! は Option<T> を返す
                if let Some(name) = try_property!(pr, String, "m_vecPlayerData.{:04}.m_iszPlayerName", pid) {
                    name
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            // 位置プロパティのフォールバック（m_vecOrigin → CBodyComponent.m_vecOrigin）
            let x: Option<f32> = e
                .get_property_by_name("m_vecOrigin[0]")
                .ok()
                .and_then(|p| p.try_into().ok());
            let y: Option<f32> = e
                .get_property_by_name("m_vecOrigin[1]")
                .ok()
                .and_then(|p| p.try_into().ok());
            let z: Option<f32> = e
                .get_property_by_name("m_vecOrigin[2]")
                .ok()
                .and_then(|p| p.try_into().ok());

            let (mut x, mut y, mut z) = (x, y, z);

            if x.is_none() && y.is_none() {
                // cell + vec からワールド座標を計算
                let cx: Option<i32> = e
                    .get_property_by_name("CBodyComponent.m_cellX")
                    .ok()
                    .and_then(|p| p.try_into().ok());
                let cy: Option<i32> = e
                    .get_property_by_name("CBodyComponent.m_cellY")
                    .ok()
                    .and_then(|p| p.try_into().ok());
                let cz: Option<i32> = e
                    .get_property_by_name("CBodyComponent.m_cellZ")
                    .ok()
                    .and_then(|p| p.try_into().ok());
                let vx: Option<f32> = e
                    .get_property_by_name("CBodyComponent.m_vecX")
                    .ok()
                    .and_then(|p| p.try_into().ok());
                let vy: Option<f32> = e
                    .get_property_by_name("CBodyComponent.m_vecY")
                    .ok()
                    .and_then(|p| p.try_into().ok());
                let vz: Option<f32> = e
                    .get_property_by_name("CBodyComponent.m_vecZ")
                    .ok()
                    .and_then(|p| p.try_into().ok());

                const CELL_SIZE: f32 = 128.0;
                if let (Some(cx), Some(vx)) = (cx, vx) {
                    x = Some(cx as f32 * CELL_SIZE + vx);
                }
                if let (Some(cy), Some(vy)) = (cy, vy) {
                    y = Some(cy as f32 * CELL_SIZE + vy);
                }
                if let (Some(cz), Some(vz)) = (cz, vz) {
                    z = Some(cz as f32 * CELL_SIZE + vz);
                }
            }

            let class = e.class().name();

            // 敵味方の判定（視点チームに対して）
            let side = match team.unwrap_or(-1) {
                t if t == self.pov_team => "ally",
                2 | 3 => "enemy",
                _ => "neutral",
            };

            writeln!(
                out,
                "{:.3},{},{},{},{},{},{},{:.3},{:.3},{:.3},{}",
                time_s,
                tick,
                idx,
                team.unwrap_or(-1),
                player_id.unwrap_or(-1),
                player_name.replace(',', " "),
                side,
                x.unwrap_or(0.0),
                y.unwrap_or(0.0),
                z.unwrap_or(0.0),
                class
            )?;
        }

        Ok(())
    }

    #[on_stop]
    fn on_stop(&mut self, _ctx: &Context) -> ObserverResult {
        if let Some(out) = self.out.as_mut() {
            out.flush()?;
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

    let positions = parser.register_observer::<Positions>();
    // --pov=2|3|radiant|dire（省略時は 2:Radiant）
    let mut pov_team: i32 = 2;
    for a in &args[2..] {
        if let Some(rest) = a.strip_prefix("--pov=") {
            pov_team = match rest.to_ascii_lowercase().as_str() {
                "2" | "radiant" | "goodguys" => 2,
                "3" | "dire" | "badguys" => 3,
                _ => 2,
            };
        }
    }
    positions.borrow_mut().pov_team = pov_team;

    let start = std::time::Instant::now();
    parser.run_to_end()?;
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
