# Game Stats Example

包括的な試合データを取得・分析するためのサンプルプログラムです。

## 概要

このexampleは、Dota 2のリプレイファイル（.dem）を解析し、以下のような試合の詳細なデータを取得・表示します：

### 取得できるデータ

#### 戦闘ログから取得
- **経験値（XP）**: プレイヤーごとの経験値獲得量と獲得理由
- **ゴールド（Gold）**: プレイヤーごとのゴールド獲得量と獲得理由
- **キル/デス**: プレイヤーのキル数とデス数
- **ダメージ**: プレイヤーが与えた総ダメージ量
- **回復**: プレイヤーが行った総回復量
- **アイテム購入**: 購入したアイテムのリスト
- **ヒーローレベルアップ**: レベルアップのタイミング
- **バイバック**: バイバックの使用
- **ビルディング破壊**: タワーやバラックの破壊
- **マルチキル**: マルチキルの達成
- **キルストリーク**: キルストリークの記録
- **ファーストブラッド**: ファーストブラッドの獲得
- **ルーン取得**: ルーンの取得
- **イージス取得**: イージスの取得
- **ロシャン討伐**: ロシャンのキル数

#### エンティティから取得
- **プレイヤーレベル**: 各プレイヤーの現在レベル
- **ヒーロー名**: 使用しているヒーロー

## 使い方

### ビルド

```bash
cd d2-examples/game_stats
cargo build --release
```

### 実行

```bash
cargo run --release <リプレイファイルのパス>
```

例：
```bash
cargo run --release ~/replays/match_12345.dem
```

## 出力例

### リアルタイム出力
プログラムは試合の進行に合わせて、以下のようなイベントをリアルタイムで表示します：

```
[45.23s] npc_dota_hero_axe gained 125 XP (reason: Some(4))
[45.50s] npc_dota_hero_lina gained 250 gold (reason: Some(1))
[67.80s] KILL: npc_dota_hero_pudge killed by npc_dota_hero_invoker
[120.45s] npc_dota_hero_antimage purchased item_power_treads
[180.90s] FIRST BLOOD! npc_dota_hero_phantom_assassin killed npc_dota_hero_crystal_maiden
[250.15s] npc_dota_hero_storm_spirit is on a KILLING SPREE (3 streak)!
[300.00s] BUILDING DESTROYED: npc_dota_badguys_tower1_mid destroyed by npc_dota_hero_templar_assassin
[450.30s] npc_dota_hero_ursa picked up the AEGIS!
  >>> ROSHAN KILL #1
```

### サマリー出力
試合終了後、以下のような統計情報のサマリーが表示されます：

```
================================================================================
MATCH STATISTICS SUMMARY
================================================================================

--- Building Destruction ---
Radiant buildings destroyed: 8
Dire buildings destroyed: 11
Roshan kills: 2

--- Player Statistics ---

#1 npc_dota_hero_antimage
  Hero: CDOTA_Unit_Hero_Antimage
  K/D/A: 15/3/8
  Level: 25
  Total Gold: 35420
  Total XP: 42150
  Damage Dealt: 45280
  Healing Done: 1250
  Items Purchased: 12
    - item_quelling_blade, item_tango, item_power_treads, item_battle_fury, ...

#2 npc_dota_hero_invoker
  Hero: CDOTA_Unit_Hero_Invoker
  K/D/A: 12/5/18
  Level: 24
  Total Gold: 32150
  Total XP: 38900
  Damage Dealt: 38920
  Healing Done: 3450
  Items Purchased: 14
    - item_clarity, item_boots, item_aghanims_scepter, ...

...
```

## データ構造

### PlayerStats
各プレイヤーの統計情報を保持する構造体：

```rust
struct PlayerStats {
    player_name: String,    // プレイヤー名
    hero_name: String,      // ヒーロー名
    kills: u32,             // キル数
    deaths: u32,            // デス数
    assists: u32,           // アシスト数
    total_gold: u32,        // 総獲得ゴールド
    total_xp: u32,          // 総獲得経験値
    level: u32,             // レベル
    items_purchased: Vec<String>,  // 購入したアイテムのリスト
    damage_dealt: u32,      // 与えたダメージの合計
    healing_done: u32,      // 行った回復の合計
}
```

## 実装の詳細

このexampleは以下の2つのObserverを使用しています：

1. **戦闘ログObserver (`#[on_combat_log]`)**:
   - 戦闘に関するすべてのイベントを処理
   - XP、ゴールド、ダメージ、回復などのデータを収集

2. **エンティティObserver (`#[on_entity]`)**:
   - ヒーローのエンティティを監視
   - レベルやヒーロー名などの情報を取得

## カスタマイズ

このexampleは拡張可能な設計になっています。以下のようなカスタマイズが可能です：

- 特定のプレイヤーやヒーローのみを追跡
- データをCSVやJSONファイルに出力
- リアルタイムグラフ表示
- より詳細な統計分析（GPM、XPM、KDA ratioなど）
- ウェブダッシュボードとの連携

## 参考資料

- [source2-demo ドキュメント](https://github.com/LaihoE/dota2-demo-rs)
- [Dota 2 プロトコルバッファ定義](../../source2-demo-protobufs/)
- [戦闘ログタイプ一覧](../../source2-demo-protobufs/dota.rs)
