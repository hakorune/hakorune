# ADR-002: 明示する所有操作と共通ownership passへの移行

Status: 設計方向を採用。意味契約の一部と新構文・全面切替の実装は未選択。
Date: 2026-10-24
Scope: 次期所有モデルの判断・未決点・移行タスクをここへ集約する。

現在の言語意味論は[ownership.md](../../reference/language/ownership.md)、
運用は[RULES.md](../RULES.md)、実行対象は
[CURRENT_STATE](../current/main/CURRENT_STATE.toml)が所有する。
このADRは現行アプリの完了条件を増やさず、parser対応や性能を主張しない。
Array carrier格納はownership.mdの独立した採用済みtargetとして先に接続する。

## 採用する方向

所有操作を局所的に読めるようにし、単独所有を基本経路にする。
全Boxへの暗黙RC、最終使用位置による暗黙move、推論失敗時のDynamic化は採用しない。

1. `=` は右辺の結果を左辺へ格納する。
2. 所有値を手放す操作は `move`、値の複製は `copy`、同じ対象の共有は `share`。
3. 新しい所有値を返す式は直接渡せる。数値・Boolは暗黙コピー。
4. 引数は既定 `read`。変更を許す引数は `mut`、所有権を受け取る引数は `take`。
5. `return`は所有結果を呼出し元へ渡すのでmove不要。借用値は返さない。
   その他の場所では最終使用でもmoveを明記し、ブロック末尾やbreakにも省略を広げない。
6. 部分moveは禁止し、要素を取り出す操作は契約を持つ `remove` / `replace` に限定する。

```hako
// 次期構文の説明例。今日のparser-liveの実行例ではない。
local p = Point(0, 0)
local q = move p
local r = copy q
arr.push(move r)
arr.push(Point(1, 2))
translate(mut q, 10)
return q
```

裸の `local b = a` は、所有値なら暗黙alias/move/copyにせず診断する方向。
`return q`の省略は所有可能なローカル全体に限り、借用引数やfieldからの部分moveを許可しない。
明示Dynamicは独立した契約として残す。静的検査を基本にすることと、Dynamicの禁止は別である。

## 切替前に確定する境界

| 項目 | 決めること |
| --- | --- |
| record / box | recordの値意味論とboxのidentityを区別。Boxまで一律MVSへ変える決定はしない。 |
| copy | 複製可能な型、Boxの新identity、入れ子の所有値・Shared・弱参照・失敗時の扱い。 |
| share / mut | Sharedの変更手段と同期、read/mutの排他区間、再入・FFI・suspendとの関係。shareだけでは安全性を発行しない。 |
| 構文と借用 | 宣言側read/mut/takeと呼出し側の表記、受け側takeとmove実引数の照合、借用を保存できる場所。 |
| Dynamic carrierのread | 生存中の`SelfContainedDynamicCarrier`を通常callableの`read`入力へ渡す時、callerがcarrierを保持してcall終了後に一度だけEndし、calleeは保持・escape・returnしない。`take`/`share`や追加ownerを発行しない。このsource/ABI/lifecycle接続を明示して検証する。 |
| 型検査 | 推論対象・注釈必須境界・明示Dynamic・診断時点。未解決型を暗黙Dynamicにしない。 |
| 再代入と失敗 | RHSを評価して成功後にcommit、旧値のEnd。moveの消費時点、copy失敗、Fault時の責任を各辺で保持。 |
| 既存互換 | alias、destination-demand移転、anchored borrowed returnを置換する範囲と一回の切替条件。 |
| MIRを使う比較 | 挙動不変の整理では旧MIRを診断資料として比較する。所有規則自体を変える移行では採用済みの新source契約と明示migration例を正解とし、旧MIRの挙動を規範にしない。 |

既存Mapの削除はBool結果である。上のremove/replace方針から、既存APIを所有値返却へ
自動変更しない。APIごとの取り出し契約は移行時に明記する。

## コンパイラの責務

```text
parse / resolve / type + source intent
  -> 同一のsemantic CFGでownership・borrowを解析
  -> scope / Normal / FaultごとのEnd計画
  -> sole lower / MIR
  -> 変換後MIR・ABIの独立検証
  -> 最適化 / LLVM
```

MirBuilderはキーワードだけでなく、正確なPlace・Binding・型・引数契約・評価順・
Normal/Fault辺を保持する。unknownをTrivialにする既定値で新モデルを通さない。
検査は実際に獲得された所有責任について、全経路で移転またはEndがちょうど一回と証明する。
End挿入前は未消費を破棄義務として計画し、挿入後の検証で実際の消費を再確認する。
maybe状態はCFGの必要性がある場合だけdrop flagにし、loop世代とFault前後を区別する。

既存[Ownership SSA](../../../src/mir/ownership_ssa/README.md)はPHI辺移転・二重消費・
終了時の未処理を検査する狭い土台。本番callerはなく、managed-call ABIと一般の借用検査は未対応。
既存[local planner](../../../src/mir/builder/resolved_lowering/ownership/README.md)も未接続で、
field/index/capture・一般Placeは対象外。借用の暗黙コピー処理は新規則へそのまま流用しない。
現在のMIR `CopyOwned`は同じobjectの独立owner tokenを作る命令で、source `copy`の複製ではない。
`DestroyOwned`の共通化には、型別終了処理・Fault・順序を残す。KeepAliveの生存支持や
ReleaseStrongの既存callerを名前変更だけで削除しない。
LLVM noaliasはmutの綴りから発行せず、実際の排他区間と到達可能なaliasを証明する。

## 移行タスクと完了条件

| 順 | 選択・作業 | 完了証拠と置換対象 |
| --- | --- | --- |
| 現在 | APP-json-streamのArray carrier source・commit・Endを接続 | 既存EXE smokeの全契約と出力/exit。新所有構文へ入力を書換えない。 |
| 準備 | 数本の既存.hakoを紙上移行し、上の境界を決着 | record、Box、Array/Mapを含む少数例でmove/copy/share数・必要コピー・借用困難点を比較。copyの意味を決める前に単なる語数で評価しない。 |
| 1 | 代表recordのmove/returnをsource→ownership検査→End→EXEへ接続 | 移転後使用とborrowed returnを拒否。同じ入力で既存SSAを本番consumerへ接続し、未接続passを作らない。置換した同一source専用edgeは検証後同seriesで退役。 |
| 2 | 同じ型のcopyとread/mut/take callをEXEまで接続 | copy後の独立性、mut競合拒否、takeの消費時点、Normal/Faultの責任。Dynamic carrierのreadはcallerがEnd責任を維持し、calleeへownerを渡さない。旧の同一source/ABI専用adapterを切替と同じseriesで退役。 |
| 3 | 同じ所有型のEndを分岐/loop/再代入/Invokeへ拡張 | Return/Faultでの終了順と一度だけの消費。必要時だけdrop flag。意味が一致しcaller-zeroとなった専有HomeRelease等だけ削除。 |
| 後続 | Box/container/Sharedへ同じ契約を広げる | 型別identity・slot commit・read支持・Endを各既存ownerで閉じる。未選択機能を現行finite goalへ追加しない。 |

数値のみのPointがTrivialなら、それだけでは所有移転やEndの証拠にならない。Trivial分類を
確定し、構文/value copyのcanaryと所有payloadを持つ代表入力を区別する。
新規test/checker/cardは原則増やさず、追加が必要ならRULES §6に従う。
実装選択は既存parked checkpointに従う。現行アプリを未完のまま全面改修へ切り替えない。
4〜5万行削減・Rust相当の速度・期間は未計測であり、達成条件や見積もりには使わない。

## Recipe生成の整理

既存Recipe ownerでlogical keyの採番と元sourceとの対応を同時に構築し、semantic sealを共通化する。
BuilderはValueId/BasicBlockIdや意味を新規推測しない。別の手書きsource対応表を増やさない。
同一不変入力の意味検証は再利用し、lower後MIR/ABIの独立検証は保持する。
fixture専用の固定キー・定数制約とproductionの制約を区別してから置換する。
この整理は現在のアプリの共通ownerを直す機会に限定し、別の全面改修campaignを開始しない。

## 不採用と理由

暗黙copy/move、最終使用での省略、全Boxの暗黙RCは、局所的な意味とコストを曖昧にする。
source `copy`を既存CopyOwnedへ単純対応させる案は、値の複製と共有を混同する。
毎stopのfixture/ADR作成や時間だけでの保留は採用せず、現行の1アプリ単位のRULESを維持する。
