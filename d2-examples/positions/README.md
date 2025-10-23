# Positions Example

ヒーローの位置とゲーム状態を時系列で記録するサンプルプログラムです。

## 概要

このexampleは、Dota 2のリプレイファイル（.dem）を解析し、各tickにおけるヒーローの位置情報とゲーム状態を`positions.csv`ファイルに出力します。

## 使い方

### ビルド

```bash
cd d2-examples/positions
cargo build --release
```

### 実行

```bash
cargo run --release <リプレイファイルのパス> [--pov=radiant|dire]
```

例：
```bash
# Radiant視点（デフォルト）
cargo run --release ~/replays/match_12345.dem

# Dire視点
cargo run --release ~/replays/match_12345.dem --pov=dire
```

## 出力ファイル

### positions.csv

試合中のすべてのヒーローの位置とゲーム状態を記録したCSVファイルが生成されます。

#### カラム構成

| カラム名 | データ型 | 説明 |
|---------|---------|------|
| `time_s` | float | ゲーム内時刻（秒） |
| `tick` | int | ティック数 |
| `entity_index` | int | エンティティインデックス |
| `team` | int | チーム番号（2=Radiant, 3=Dire） |
| `player_id` | int | プレイヤーID（0-9） |
| `player_name` | string | プレイヤー名 |
| `side` | string | 視点チームからの敵味方（ally/enemy/neutral） |
| `x` | float | X座標 |
| `y` | float | Y座標 |
| `z` | float | Z座標（高さ） |
| `class` | string | エンティティクラス名（ヒーロー名） |
| `hp` | int | 現在の体力 |
| `max_hp` | int | 最大体力 |
| `mana` | float | 現在のマナ |
| `max_mana` | float | 最大マナ |
| `level` | int | ヒーローレベル |
| `gold` | int | 現在の所持ゴールド（Reliable + Unreliable） |
| `xp` | int | 現在の経験値 |
| `life_state` | int | 生存状態（0=生存, 1=死亡, 2=未生成） |
| `is_alive` | int | 生存フラグ（1=生存, 0=死亡/未生成） |

#### サンプル出力

```csv
time_s,tick,entity_index,team,player_id,player_name,side,x,y,z,class,hp,max_hp,mana,max_mana,level,gold,xp,life_state,is_alive
0.033,1,202,2,0,Player1,ally,-6912.000,-6400.000,384.000,CDOTA_Unit_Hero_Axe,625,625,234.0,234.0,1,625,0,0,1
0.033,1,260,3,5,Player2,enemy,6272.000,5952.000,384.000,CDOTA_Unit_Hero_Invoker,560,560,435.0,435.0,1,625,0,0,1
0.067,2,202,2,0,Player1,ally,-6911.234,-6399.456,384.000,CDOTA_Unit_Hero_Axe,625,625,234.0,234.0,1,625,0,0,1
0.067,2,260,3,5,Player2,enemy,6271.789,5951.234,384.000,CDOTA_Unit_Hero_Invoker,560,560,435.0,435.0,1,625,0,0,1
```

## データ活用例

### Python + pandas での分析

```python
import pandas as pd
import matplotlib.pyplot as plt

# CSVファイルを読み込み
df = pd.read_csv('positions.csv')

# 特定のプレイヤーのデータを抽出
player_data = df[df['player_name'] == 'Player1']

# 体力の推移をグラフ化
plt.figure(figsize=(12, 6))
plt.subplot(2, 1, 1)
plt.plot(player_data['time_s'], player_data['hp'])
plt.title('HP over time')
plt.xlabel('Time (s)')
plt.ylabel('HP')

# 位置のヒートマップを作成
plt.subplot(2, 1, 2)
plt.hexbin(player_data['x'], player_data['y'], gridsize=50, cmap='YlOrRd')
plt.title('Position heatmap')
plt.xlabel('X')
plt.ylabel('Y')
plt.colorbar(label='Frequency')

plt.tight_layout()
plt.savefig('player_analysis.png')
```

### ゴールド・経験値の分析

```python
# 時間あたりのゴールド増加率（GPM）を計算
df_sorted = df.sort_values(['player_id', 'time_s'])
df_sorted['gold_diff'] = df_sorted.groupby('player_id')['gold'].diff()
df_sorted['time_diff'] = df_sorted.groupby('player_id')['time_s'].diff()
df_sorted['gpm'] = (df_sorted['gold_diff'] / df_sorted['time_diff']) * 60

# プレイヤーごとの平均GPM
avg_gpm = df_sorted.groupby('player_name')['gpm'].mean()
print(avg_gpm)
```

### 死亡位置の可視化

```python
# 死亡した位置を抽出
death_positions = df[df['is_alive'] == 0]

# チームごとに色分けして表示
plt.figure(figsize=(10, 10))
for team in [2, 3]:
    team_deaths = death_positions[death_positions['team'] == team]
    label = 'Radiant' if team == 2 else 'Dire'
    color = 'green' if team == 2 else 'red'
    plt.scatter(team_deaths['x'], team_deaths['y'],
                label=label, color=color, alpha=0.5, s=50)

plt.title('Death Positions')
plt.xlabel('X')
plt.ylabel('Y')
plt.legend()
plt.grid(True, alpha=0.3)
plt.savefig('death_positions.png')
```

### 機械学習での活用

```python
# 勝敗予測モデルの特徴量作成
features = []
for player_id in df['player_id'].unique():
    player_df = df[df['player_id'] == player_id]

    # 各プレイヤーの統計を計算
    player_features = {
        'player_id': player_id,
        'avg_hp_percent': (player_df['hp'] / player_df['max_hp']).mean(),
        'avg_mana_percent': (player_df['mana'] / player_df['max_mana']).mean(),
        'final_level': player_df['level'].iloc[-1],
        'final_gold': player_df['gold'].iloc[-1],
        'death_count': (player_df['is_alive'].diff() == -1).sum(),
        'avg_distance_from_base': ((player_df['x']**2 + player_df['y']**2)**0.5).mean()
    }
    features.append(player_features)

features_df = pd.DataFrame(features)
# この特徴量を使って勝敗予測モデルを構築
```

## 高度な活用

### リアルタイム可視化

positions.csvを読み込んで、試合の様子をアニメーションとして再生できます：

```python
import matplotlib.animation as animation

# アニメーション用の関数
def update_frame(frame_time):
    frame_data = df[df['time_s'] == frame_time]
    # ... プロット更新処理

ani = animation.FuncAnimation(fig, update_frame,
                              frames=df['time_s'].unique(),
                              interval=33)  # 30fps
ani.save('replay_animation.mp4')
```

### データベースへのインポート

大量のリプレイを分析する場合は、SQLiteやPostgreSQLにインポートして効率的にクエリできます：

```python
import sqlite3

conn = sqlite3.connect('dota_replays.db')
df.to_sql('positions', conn, if_exists='append', index=False)

# クエリ例：特定時間帯の全プレイヤーの状態
query = """
SELECT player_name, AVG(gold) as avg_gold, AVG(level) as avg_level
FROM positions
WHERE time_s BETWEEN 600 AND 900  -- 10-15分
GROUP BY player_name
ORDER BY avg_gold DESC
"""
result = pd.read_sql(query, conn)
```

## 実装の詳細

このexampleは以下の機能を持っています：

1. **エンティティ追跡**: `#[on_entity]`を使ってプレイヤブルユニットを自動検出
2. **Tick毎の記録**: `#[on_tick_end]`で毎tickの状態をCSVに記録
3. **視点切り替え**: `--pov`オプションでRadiant/Dire視点を切り替え可能
4. **包括的な状態取得**:
   - 位置情報（x, y, z座標）
   - 体力・マナの現在値と最大値
   - レベルと経験値
   - 所持ゴールド（Reliable + Unreliable）
   - 生存状態

## パフォーマンス

- Dota 2は30 tick/秒で動作します
- 60分の試合 = 108,000 tick
- 10人のプレイヤー × 108,000 tick = 約108万行のデータ
- CSVファイルサイズ: 約50-100MB（試合時間による）

## 注意事項

- `positions.csv`は毎回上書きされます
- 長時間の試合では大量のデータが生成されます
- メモリ使用量を抑えるため、データはリアルタイムでファイルに書き込まれます

## 参考資料

- [source2-demo ドキュメント](https://github.com/LaihoE/dota2-demo-rs)
- [Dota 2 座標系について](https://dota2.fandom.com/wiki/Coordinates)
- [エンティティプロパティ一覧](../../entity_202_props.txt)
