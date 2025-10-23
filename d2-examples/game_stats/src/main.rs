use source2_demo::prelude::*;
use source2_demo::proto::DotaCombatlogTypes;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

/// プレイヤーごとの統計情報
#[derive(Debug, Default, Clone)]
struct PlayerStats {
    player_name: String,
    hero_name: String,
    kills: u32,
    deaths: u32,
    assists: u32,
    total_gold: u32,
    total_xp: u32,
    level: u32,
    items_purchased: Vec<String>,
    damage_dealt: u32,
    healing_done: u32,
}

impl PlayerStats {
    fn new(player_name: String, hero_name: String) -> Self {
        Self {
            player_name,
            hero_name,
            ..Default::default()
        }
    }
}

/// 試合全体の統計情報を管理
struct GameStats {
    /// プレイヤーID -> 統計情報
    players: HashMap<String, PlayerStats>,
    /// ビルディング破壊数（Radiant / Dire）
    buildings_destroyed: (u32, u32),
    /// ロシャン討伐数
    roshan_kills: u32,
    /// 試合開始時刻
    match_start_time: Option<f32>,
    /// イベントログCSV出力
    events_csv: Option<BufWriter<File>>,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            players: HashMap::default(),
            buildings_destroyed: (0, 0),
            roshan_kills: 0,
            match_start_time: None,
            events_csv: None,
        }
    }
}

#[observer]
impl GameStats {
    /// CSVファイルを初期化
    fn ensure_csv_initialized(&mut self) -> ObserverResult {
        if self.events_csv.is_none() {
            let f = File::create("game_events.csv")?;
            let mut w = BufWriter::new(f);
            writeln!(
                w,
                "time_s,event_type,player,target,value,details"
            )?;
            self.events_csv = Some(w);
        }
        Ok(())
    }

    /// イベントをCSVに記録
    fn log_event(&mut self, time: f32, event_type: &str, player: &str, target: &str, value: u32, details: &str) -> ObserverResult {
        self.ensure_csv_initialized()?;
        if let Some(csv) = &mut self.events_csv {
            writeln!(
                csv,
                "{:.2},{},{},{},{},{}",
                time,
                event_type,
                player.replace(',', " "),
                target.replace(',', " "),
                value,
                details.replace(',', " ")
            )?;
        }
        Ok(())
    }

    /// 戦闘ログを処理
    #[on_combat_log]
    fn handle_combat_log(&mut self, ctx: &Context, log: &CombatLogEntry) -> ObserverResult {
        let time = log.timestamp()?;

        match log.r#type() {
            // 経験値獲得
            DotaCombatlogTypes::DotaCombatlogXp => {
                let target_name = log.target_name()?;
                let xp_amount = log.value()?;
                let reason = log.xp_reason();

                println!(
                    "[{:.2}s] {} gained {} XP (reason: {:?})",
                    time, target_name, xp_amount, reason
                );

                // CSVに記録
                self.log_event(time, "XP_GAIN", target_name, "", xp_amount, &format!("reason: {:?}", reason))?;

                // プレイヤー統計を更新
                if let Some(stats) = self.players.get_mut(target_name) {
                    stats.total_xp += xp_amount;
                } else {
                    let mut new_stats = PlayerStats::new(target_name.to_string(), "Unknown".to_string());
                    new_stats.total_xp = xp_amount;
                    self.players.insert(target_name.to_string(), new_stats);
                }
            }

            // ゴールド獲得
            DotaCombatlogTypes::DotaCombatlogGold => {
                let target_name = log.target_name()?;
                let gold_amount = log.value()?;
                let reason = log.gold_reason();

                println!(
                    "[{:.2}s] {} gained {} gold (reason: {:?})",
                    time, target_name, gold_amount, reason
                );

                // CSVに記録
                self.log_event(time, "GOLD_GAIN", target_name, "", gold_amount, &format!("reason: {:?}", reason))?;

                // プレイヤー統計を更新
                if let Some(stats) = self.players.get_mut(target_name) {
                    stats.total_gold += gold_amount;
                } else {
                    let mut new_stats = PlayerStats::new(target_name.to_string(), "Unknown".to_string());
                    new_stats.total_gold = gold_amount;
                    self.players.insert(target_name.to_string(), new_stats);
                }
            }

            // キル/デス
            DotaCombatlogTypes::DotaCombatlogDeath => {
                let victim = log.target_name()?;
                let killer = log.attacker_name().unwrap_or("Unknown");

                println!(
                    "[{:.2}s] KILL: {} killed by {}",
                    time, victim, killer
                );

                // CSVに記録
                let mut details = String::new();
                if victim.contains("roshan") {
                    details.push_str("ROSHAN_KILL");
                } else if victim.contains("tower") || victim.contains("barrack") || victim.contains("fort") {
                    details.push_str("BUILDING_KILL");
                }
                self.log_event(time, "DEATH", killer, victim, 0, &details)?;

                // 被キル側の統計を更新
                self.players
                    .entry(victim.to_string())
                    .or_insert_with(|| PlayerStats::new(victim.to_string(), "Unknown".to_string()))
                    .deaths += 1;

                // キル側の統計を更新
                if killer != "Unknown" {
                    self.players
                        .entry(killer.to_string())
                        .or_insert_with(|| PlayerStats::new(killer.to_string(), "Unknown".to_string()))
                        .kills += 1;
                }

                // ロシャン討伐チェック
                if victim.contains("roshan") {
                    self.roshan_kills += 1;
                    println!("  >>> ROSHAN KILL #{}", self.roshan_kills);
                }

                // ビルディング破壊チェック
                if victim.contains("tower") || victim.contains("barrack") || victim.contains("fort") {
                    if victim.contains("goodguys") || victim.contains("radiant") {
                        self.buildings_destroyed.0 += 1;
                        println!("  >>> Radiant building destroyed");
                    } else if victim.contains("badguys") || victim.contains("dire") {
                        self.buildings_destroyed.1 += 1;
                        println!("  >>> Dire building destroyed");
                    }
                }
            }

            // ダメージ
            DotaCombatlogTypes::DotaCombatlogDamage => {
                let attacker = log.attacker_name()?;
                let damage = log.value()?;

                // プレイヤー統計を更新
                if let Some(stats) = self.players.get_mut(attacker) {
                    stats.damage_dealt += damage;
                }
            }

            // 回復
            DotaCombatlogTypes::DotaCombatlogHeal => {
                let healer = log.attacker_name().unwrap_or("Unknown");
                let heal_amount = log.value()?;

                // プレイヤー統計を更新
                if healer != "Unknown" {
                    if let Some(stats) = self.players.get_mut(healer) {
                        stats.healing_done += heal_amount;
                    }
                }
            }

            // アイテム購入
            DotaCombatlogTypes::DotaCombatlogPurchase => {
                let buyer = log.attacker_name()?;
                let item_name = log.target_name()?;

                println!(
                    "[{:.2}s] {} purchased {}",
                    time, buyer, item_name
                );

                // CSVに記録
                self.log_event(time, "ITEM_PURCHASE", buyer, item_name, 0, "")?;

                // プレイヤー統計を更新
                if let Some(stats) = self.players.get_mut(buyer) {
                    stats.items_purchased.push(item_name.to_string());
                }
            }

            // ヒーローレベルアップ
            DotaCombatlogTypes::DotaCombatlogHeroLevelup => {
                let hero = log.target_name()?;
                let level = log.value()?;

                println!(
                    "[{:.2}s] {} leveled up to {}",
                    time, hero, level
                );

                // CSVに記録
                self.log_event(time, "LEVEL_UP", hero, "", level, "")?;
            }

            // バイバック
            DotaCombatlogTypes::DotaCombatlogBuyback => {
                let player = log.attacker_name()?;
                println!("[{:.2}s] {} bought back!", time, player);

                // CSVに記録
                self.log_event(time, "BUYBACK", player, "", 0, "")?;
            }

            // ビルディング破壊
            DotaCombatlogTypes::DotaCombatlogTeamBuildingKill => {
                let attacker = log.attacker_name().unwrap_or("Unknown");
                let building = log.target_name()?;
                println!(
                    "[{:.2}s] BUILDING DESTROYED: {} destroyed by {}",
                    time, building, attacker
                );

                // CSVに記録
                self.log_event(time, "BUILDING_DESTROYED", attacker, building, 0, "")?;
            }

            // マルチキル
            DotaCombatlogTypes::DotaCombatlogMultikill => {
                let player = log.attacker_name()?;
                let kills = log.value()?;
                println!(
                    "[{:.2}s] {} got a MULTIKILL ({} kills)!",
                    time, player, kills
                );

                // CSVに記録
                self.log_event(time, "MULTIKILL", player, "", kills, "")?;
            }

            // キルストリーク
            DotaCombatlogTypes::DotaCombatlogKillstreak => {
                let player = log.attacker_name()?;
                let streak = log.value()?;
                println!(
                    "[{:.2}s] {} is on a KILLING SPREE ({} streak)!",
                    time, player, streak
                );

                // CSVに記録
                self.log_event(time, "KILLSTREAK", player, "", streak, "")?;
            }

            // ファーストブラッド
            DotaCombatlogTypes::DotaCombatlogFirstBlood => {
                let killer = log.attacker_name()?;
                let victim = log.target_name()?;
                println!(
                    "[{:.2}s] FIRST BLOOD! {} killed {}",
                    time, killer, victim
                );

                // CSVに記録
                self.log_event(time, "FIRST_BLOOD", killer, victim, 0, "")?;
            }

            // ルーン取得
            DotaCombatlogTypes::DotaCombatlogPickupRune => {
                let player = log.attacker_name()?;
                let rune_type = log.value()?;
                println!(
                    "[{:.2}s] {} picked up rune (type: {})",
                    time, player, rune_type
                );

                // CSVに記録
                self.log_event(time, "RUNE_PICKUP", player, "", rune_type, "")?;
            }

            // イージス取得
            DotaCombatlogTypes::DotaCombatlogAegisTaken => {
                let player = log.attacker_name()?;
                println!("[{:.2}s] {} picked up the AEGIS!", time, player);

                // CSVに記録
                self.log_event(time, "AEGIS_TAKEN", player, "", 0, "")?;
            }

            _ => {}
        }

        Ok(())
    }

    /// エンティティの状態変化を処理（ヒーローのレベルなど）
    #[on_entity("CDOTA_Unit_Hero_.*")]
    fn handle_hero(&mut self, ctx: &Context, ev: EntityEvents, entity: &Entity) -> ObserverResult {
        // ヒーロー名を取得
        let hero_name = entity.class().name();

        // レベルを取得
        if let Ok(level_prop) = entity.get_property_by_name("m_iCurrentLevel") {
            if let Ok(level) = level_prop.try_into() as Result<i32, _> {
                // プレイヤーIDを取得
                if let Ok(player_id_prop) = entity.get_property_by_name("m_iPlayerID") {
                    if let Ok(player_id) = player_id_prop.try_into() as Result<i32, _> {
                        let player_key = format!("player_{}", player_id);

                        if let Some(stats) = self.players.get_mut(&player_key) {
                            if stats.level != level as u32 {
                                println!(
                                    "[Tick {}] {} leveled up to {}",
                                    ctx.tick(),
                                    hero_name,
                                    level
                                );
                                stats.level = level as u32;
                                stats.hero_name = hero_name.to_string();
                            }
                        } else {
                            let mut new_stats = PlayerStats::new(player_key.clone(), hero_name.to_string());
                            new_stats.level = level as u32;
                            self.players.insert(player_key, new_stats);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// CSVファイルをフラッシュ
    fn flush_csv(&mut self) -> ObserverResult {
        if let Some(csv) = &mut self.events_csv {
            csv.flush()?;
        }
        Ok(())
    }

    /// プレイヤー統計をCSVファイルに出力
    fn export_player_stats_csv(&self) -> ObserverResult {
        let f = File::create("player_stats.csv")?;
        let mut w = BufWriter::new(f);

        // ヘッダー
        writeln!(
            w,
            "player_name,hero_name,kills,deaths,assists,level,total_gold,total_xp,damage_dealt,healing_done,items_purchased_count,items_list"
        )?;

        // プレイヤーをゴールド順にソート
        let mut players: Vec<_> = self.players.values().collect();
        players.sort_by(|a, b| b.total_gold.cmp(&a.total_gold));

        // データ行
        for stats in players {
            writeln!(
                w,
                "{},{},{},{},{},{},{},{},{},{},{},\"{}\"",
                stats.player_name.replace(',', " "),
                stats.hero_name.replace(',', " "),
                stats.kills,
                stats.deaths,
                stats.assists,
                stats.level,
                stats.total_gold,
                stats.total_xp,
                stats.damage_dealt,
                stats.healing_done,
                stats.items_purchased.len(),
                stats.items_purchased.join("; ")
            )?;
        }

        w.flush()?;
        Ok(())
    }

    /// 試合サマリーをCSVファイルに出力
    fn export_match_summary_csv(&self) -> ObserverResult {
        let f = File::create("match_summary.csv")?;
        let mut w = BufWriter::new(f);

        writeln!(w, "metric,value")?;
        writeln!(w, "radiant_buildings_destroyed,{}", self.buildings_destroyed.0)?;
        writeln!(w, "dire_buildings_destroyed,{}", self.buildings_destroyed.1)?;
        writeln!(w, "roshan_kills,{}", self.roshan_kills)?;
        writeln!(w, "total_players,{}", self.players.len())?;

        w.flush()?;
        Ok(())
    }

    /// 試合終了時に統計情報を表示
    fn print_summary(&self) {
        println!("\n");
        println!("=".repeat(80));
        println!("MATCH STATISTICS SUMMARY");
        println!("=".repeat(80));

        println!("\n--- Building Destruction ---");
        println!("Radiant buildings destroyed: {}", self.buildings_destroyed.0);
        println!("Dire buildings destroyed: {}", self.buildings_destroyed.1);
        println!("Roshan kills: {}", self.roshan_kills);

        println!("\n--- Player Statistics ---");
        let mut players: Vec<_> = self.players.values().collect();
        players.sort_by(|a, b| b.total_gold.cmp(&a.total_gold));

        for (i, stats) in players.iter().enumerate() {
            println!("\n#{} {}", i + 1, stats.player_name);
            println!("  Hero: {}", stats.hero_name);
            println!("  K/D/A: {}/{}/{}", stats.kills, stats.deaths, stats.assists);
            println!("  Level: {}", stats.level);
            println!("  Total Gold: {}", stats.total_gold);
            println!("  Total XP: {}", stats.total_xp);
            println!("  Damage Dealt: {}", stats.damage_dealt);
            println!("  Healing Done: {}", stats.healing_done);
            println!("  Items Purchased: {}", stats.items_purchased.len());
            if !stats.items_purchased.is_empty() {
                println!("    - {}", stats.items_purchased.join(", "));
            }
        }

        println!("\n{}", "=".repeat(80));
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

    let mut game_stats = GameStats::default();
    parser.register_observer_from_ref(&mut game_stats);

    println!("Parsing replay file: {}", filepath);
    println!("This may take a moment...\n");

    let start = std::time::Instant::now();
    parser.run_to_end()?;
    let elapsed = start.elapsed();

    // CSVファイルをフラッシュ
    game_stats.flush_csv()?;

    // 統計情報のサマリーを表示
    game_stats.print_summary();

    // CSV出力
    println!("\nExporting data to CSV files...");
    game_stats.export_player_stats_csv()?;
    game_stats.export_match_summary_csv()?;

    println!("  - game_events.csv: All game events log");
    println!("  - player_stats.csv: Player statistics summary");
    println!("  - match_summary.csv: Match summary");

    println!("\nParsing completed in {:.2?}", elapsed);

    Ok(())
}
