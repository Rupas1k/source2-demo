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

## 出力形式

このプログラムは、2種類の出力を提供します：

1. **標準出力**: リアルタイムイベントと試合終了時のサマリー
2. **CSVファイル**: データ分析用の構造化データ（3ファイル）

### CSVファイル出力

プログラム実行後、以下の3つのCSVファイルが生成されます：

#### 1. `game_events.csv`
試合中のすべてのイベントを時系列で記録したログファイル。

**カラム構成:**
- `time_s`: イベント発生時刻（秒）
- `event_type`: イベントタイプ（XP_GAIN, GOLD_GAIN, DEATH, ITEM_PURCHASE など）
- `player`: プレイヤー名（イベントを起こしたプレイヤー）
- `target`: ターゲット名（イベントの対象）
- `value`: 数値（XP量、ゴールド量、レベルなど）
- `details`: 追加情報（獲得理由など）

**イベントタイプ一覧:**
- `XP_GAIN`: 経験値獲得
- `GOLD_GAIN`: ゴールド獲得
- `DEATH`: キル/デス
- `ITEM_PURCHASE`: アイテム購入
- `LEVEL_UP`: レベルアップ
- `BUYBACK`: バイバック
- `BUILDING_DESTROYED`: ビルディング破壊
- `MULTIKILL`: マルチキル
- `KILLSTREAK`: キルストリーク
- `FIRST_BLOOD`: ファーストブラッド
- `RUNE_PICKUP`: ルーン取得
- `AEGIS_TAKEN`: イージス取得

#### 2. `player_stats.csv`
各プレイヤーの統計情報をまとめたファイル（ゴールド順にソート済み）。

**カラム構成:**
- `player_name`: プレイヤー名
- `hero_name`: ヒーロー名
- `kills`: キル数
- `deaths`: デス数
- `assists`: アシスト数
- `level`: 最終レベル
- `total_gold`: 総獲得ゴールド
- `total_xp`: 総獲得経験値
- `damage_dealt`: 与えたダメージの合計
- `healing_done`: 行った回復の合計
- `items_purchased_count`: 購入したアイテム数
- `items_list`: 購入したアイテムのリスト（セミコロン区切り）

#### 3. `match_summary.csv`
試合全体のサマリー情報。

**カラム構成:**
- `metric`: 指標名
- `value`: 値

**指標一覧:**
- `radiant_buildings_destroyed`: Radiantビルディング破壊数
- `dire_buildings_destroyed`: Direビルディング破壊数
- `roshan_kills`: ロシャン討伐数
- `total_players`: プレイヤー総数

### CSV活用例

CSVファイルは、Excel、Google Sheets、Pythonのpandas、Rのデータフレームなど、様々なツールで分析できます：

```python
# Pythonでの分析例
import pandas as pd

# イベントログを読み込み
events = pd.read_csv('game_events.csv')

# ゴールド獲得の時系列グラフを作成
gold_events = events[events['event_type'] == 'GOLD_GAIN']
gold_per_player = gold_events.groupby(['player', 'time_s'])['value'].sum()

# プレイヤー統計を読み込み
stats = pd.read_csv('player_stats.csv')

# KDA ratioを計算
stats['kda'] = (stats['kills'] + stats['assists']) / stats['deaths'].replace(0, 1)

# GPM（Gold Per Minute）を計算
max_time = events['time_s'].max() / 60
stats['gpm'] = stats['total_gold'] / max_time
```

## 出力例

### リアルタイム出力（標準出力）
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

### サマリー出力（標準出力）
試合終了後、以下のような統計情報のサマリーが表示され、CSVファイルが生成されます：

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

================================================================================

Exporting data to CSV files...
  - game_events.csv: All game events log
  - player_stats.csv: Player statistics summary
  - match_summary.csv: Match summary

Parsing completed in 2.35s
```

### CSVファイル出力例

#### game_events.csv
```csv
time_s,event_type,player,target,value,details
45.23,XP_GAIN,npc_dota_hero_axe,,125,reason: Some(4)
45.50,GOLD_GAIN,npc_dota_hero_lina,,250,reason: Some(1)
67.80,DEATH,npc_dota_hero_invoker,npc_dota_hero_pudge,0,
120.45,ITEM_PURCHASE,npc_dota_hero_antimage,item_power_treads,0,
180.90,FIRST_BLOOD,npc_dota_hero_phantom_assassin,npc_dota_hero_crystal_maiden,0,
250.15,KILLSTREAK,npc_dota_hero_storm_spirit,,3,
```

#### player_stats.csv
```csv
player_name,hero_name,kills,deaths,assists,level,total_gold,total_xp,damage_dealt,healing_done,items_purchased_count,items_list
npc_dota_hero_antimage,CDOTA_Unit_Hero_Antimage,15,3,8,25,35420,42150,45280,1250,12,"item_quelling_blade; item_tango; item_power_treads; item_battle_fury"
npc_dota_hero_invoker,CDOTA_Unit_Hero_Invoker,12,5,18,24,32150,38900,38920,3450,14,"item_clarity; item_boots; item_aghanims_scepter"
```

#### match_summary.csv
```csv
metric,value
radiant_buildings_destroyed,8
dire_buildings_destroyed,11
roshan_kills,2
total_players,10
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
- 追加のCSVファイル出力（tick単位の詳細データなど）
- JSONやその他の形式でのエクスポート
- リアルタイムグラフ表示
- より詳細な統計分析（GPM、XPM、KDA ratioなど）
- ウェブダッシュボードとの連携

### CSV出力の活用

生成されたCSVファイルは、Excelやデータ分析ツールで簡単に開けます。以下のような活用例があります：

- **タイムライン分析**: `game_events.csv`を使って、試合の流れを時系列で可視化
- **プレイヤー比較**: `player_stats.csv`を使って、プレイヤー間の統計を比較
- **機械学習**: CSVデータを使って勝敗予測モデルを構築
- **レポート生成**: 統計データから自動的にPDFレポートを生成

## 参考資料

- [source2-demo ドキュメント](https://github.com/LaihoE/dota2-demo-rs)
- [Dota 2 プロトコルバッファ定義](../../source2-demo-protobufs/)
- [戦闘ログタイプ一覧](../../source2-demo-protobufs/dota.rs)
