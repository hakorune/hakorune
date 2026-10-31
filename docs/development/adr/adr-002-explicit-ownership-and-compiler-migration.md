# ADR-002: 明示する所有操作と共通ownership passへの移行

Status: 設計方向を採用。2026-10-30に表面契約を決定（[ownership.md §0](../../reference/language/ownership.md)）。
2026-10-31にShared・`sync box`・threadの実装方向を決定。step 1は限定的なlocal `move`のEXEと独立MIR検証、旧local planner退役まで着地。Sharedと一般ABIは未完、切替は未着手。
Date: 2026-10-30（改訂 2026-10-31）
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
2. 所有値を手放す操作は `move`。複製はcopy契約を宣言した型だけ（`copy x`を`.copy()`へ綴り替えるかは保留）。
   共有は `Shared(move x)` / `Shared(新しい値)` で作る。`share x`前置形と`.share()`は作らない。
3. 新しい所有値を返す式は直接渡せる（`cache.put(Image.load(p))`）。数値・Bool・Shared handleは暗黙コピーの値カテゴリで、サイズでは決めない。
   Shared handleをtake引数へ渡すとhandleが一つ増え、`move shared`は増やさず移す。Sharedは当面読み取り専用で、共有変更とthread越えは別契約。
4. 引数は既定 `read`（書かない）。`mut` と `take` は定義側で宣言する。receiverのread/mut借用も宣言から決まり、呼出し側に書かない。
   receiverの消費だけ呼出し側で明示する（`(move image).into_bytes()`）。
5. `return`は所有ローカル全体と新しい所有値に限りmove不要。借用値は、定義側で支持元を宣言した
   `from me` / `from <引数>` の結果だけ返せる（2026-10-30改訂。制約はownership.md §0）。
   その他の場所では最終使用でもmoveを明記し、ブロック末尾やbreakにも省略を広げない。
6. 部分moveは禁止し、要素を取り出す操作は契約を持つ `remove` / `replace` に限定する。

```hako
// 次期構文の説明例。今日のparser-liveの実行例ではない。
local p = Point(0, 0)
local q = move p
local r = copy q
arr.push(move r)
arr.push(Point(1, 2))
translate(q, 10)        // mutは定義側。呼出し側の印は未決（案: 書かない）
return q
```

裸の `local b = a` は、所有値なら暗黙alias/move/copyにせず診断する方向。field読み `local x = me.field` は注釈なしの `from me` 読み借用で、借用結果と同じ検査器が扱う。
`return q`の省略は所有可能なローカル全体に限り、借用引数やfieldからの部分moveを許可しない。
明示Dynamicは独立した契約として残す。静的検査を基本にすることと、Dynamicの禁止は別である。

## 切替前に確定する境界

| 項目 | 決めること |
| --- | --- |
| record / box | recordの値意味論とboxのidentityを区別。Boxまで一律MVSへ変える決定はしない。 |
| copy | 決定: copy契約を宣言した型だけ。残り: Boxの新identity、入れ子の所有値・Shared・弱参照・失敗時の扱い、`.copy()`への綴り。 |
| Shared / mut | 決定: `Shared<通常Box>`は読み取り専用（readメソッドとfield読みは可、mut/take/field代入は不可）。Sharedだけでは安全性を発行しない。表現は**Shared専用の非原子RC**を物理表現を隠すopaque境界の内側に置く（Unique Boxにheaderを要求しない、Rust `Arc`を境界に出さない）。操作契約: 構築成功＝handle 1でpayloadの所有・End義務を一度だけ取り込む／暗黙複製・take通常渡し＝+1／`move shared`＝0／read・借用＝0（支持元Homeに依存）／handleのEnd＝−1、`1→0`だけがpayload終了へ進む（wrap・0からの復活・Fault後のEnd再試行は禁止）。独立MIR検証には型付きの複製・移転・解放、借用の支持元、実CFG辺との対応を残す（生のinc/decやHome記録だけの検証にしない）。cycle: 構築中の自己参照・可変alias経由の再接続・証明不能な所有循環は公開前に拒否、weakは後送り（strong＝0以降のupgradeは失敗）。残り: header配置・inline化、weak、公開型名。 |
| 共有変更 / sync box | 決定: 基本は単独owner＋必要時`mut`借用、外へ渡すのはID・snapshot・独立所有結果。共有変更は既存`sync box`（[boundary-model](../../reference/concurrency/boundary-model.md)）へ集約し、`Shared<sync box>`は宣言された同期メソッドへの入場だけを許して区間内でのみ排他的な`me`を得る。区間内はsuspend・待ち・task dispatch・event pump・暗黙yield（現`safepoint_and_poll`のscheduler pollを含む）をしない。cleanup/fini（暗黙Endと呼び先を含む）から新規入場しない。`me.helper()`は同じreceiverの生存guardを借りる証拠で許可し、callback・別handle経由は内部呼出しと認定しない。外部再入はFault、再入可能ロックは採らない。同一thread版は別objectへの入れ子を許し同一objectはFault（boundary-modelの現行拒否を切替前に明示改訂）。guardは公開値にせず全終了辺で一度だけ退場。Faultで退出した入場済み区間をpoison（解錠は必ず、最初のFaultを保持、`Result::Err`では付けない）。区間内の効果（suspend・待ち・task実行・取得lock順位・escape上限）は関数契約に持ち、独立verifierが実CFGと呼出しグラフ（挿入End/fini含む）から再集計。未知Dynamic/FFIは区間内で拒否。残り: 条件待ち`wait`（区間を一度終える操作）、poison回復API。 |
| thread | 決定: `move`は所有移転で、越境許可は別（既存`HakoSend`/`HakoShare`/`ThreadRoot` descriptorを検査へ育てる。Dynamicは既定で越境不可）。非原子Sharedと越境用Sharedは型・ABIで別物にし、build設定で同じ型の意味を切り替えない。非原子Sharedを含むtaskはthread固定。thread版`sync box`は型/lock class単位の厳密な昇順＋try取得のみ（guard保持中は待たない）。同一thread版で受理した入れ子をthread版で自動受理しない。チャネルは送信→queue→受信の所有commitと、失敗・cancel・未受信終了のcleanup担当を一意にする。atomic RCは中身のthread安全を意味しない。残り: atomicのmemory ordering、順位付きblocking取得、公平性。整理: thread gateの`thread_registry_gc_roots_enabled`と`safepoint_and_poll`の`gc.safepoint()`は、GCなし方針の「workerが保持する所有・handleと終了責任」要件へ置き換える（名前だけで検査を省かない）。 |
| 構文と借用 | 決定: 宣言側read（省略）/mut/take、呼出し側は所有を失う所だけ`move`、借用結果は`from`宣言付きでlocal・分岐・read引数まで（field/global保存・escape closure・保持FFI・suspend越えは不可）。未決: 通常引数の呼出し側`mut`印（案: 書かない）、§3のcomposite anchored resultsとfield保存制限の関係。 |
| Dynamic carrierのread | 生存中の`SelfContainedDynamicCarrier`を通常callableの`read`入力へ渡す時、callerがcarrierを保持してcall終了後に一度だけEndし、calleeは保持・escape・returnしない。`take`/`share`や追加ownerを発行しない。このsource/ABI/lifecycle接続を明示して検証する。 |
| 型検査 | 推論対象・注釈必須境界・明示Dynamic・診断時点。未解決型を暗黙Dynamicにしない。 |
| 再代入と失敗 | RHSを評価して成功後にcommit、旧値のEnd。moveの消費時点、copy失敗、Fault時の責任を各辺で保持。 未決: take実引数の後の引数評価がFaultした時（案: 所有はcallerに残り、既存Normal/Fault cleanup規則に従う）。 |
| 既存互換 | 決定（案A）: 通常Boxの意味（現行SharedV1）は切替まで変えず、切替はselfhostコードを書く前に一度だけ。切替seriesで「take引数へ`move`なしで所有Boxを渡す」をエラーにし、固定suite 11アプリの該当箇所（container格納 約6箇所、bare alias行 上限17）を同seriesで直して11/11を保つ。新旧間のsilent fallbackなし。anchored borrowed returnは`from`結果へ置換。 |
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
| 済 | 2026-10-30の決定をownership.md §0とこのADRへ記録 | docsのみ。コンパイル速度（Text End台帳の`SourceLoopCleanupPlanV2`統合、サイトの整数ID化）は別に続ける。 |
| 1 | `move x`の受理と移転後使用の拒否、Shared型の追加 | 既存アプリの入力・挙動は不変。同じ入力で既存SSAを本番consumerへ接続し、未接続passを作らない（consumerはpublished検証側。退役予定のVM interpreterに置かない）。`move`は新MIR命令を足さずbindingの付け替えで表す。SharedはopaqueなShared cell（カウント＋受理済みpayloadの所有・End義務）で上の操作契約をEXEまで一つ実装する（意味の模型検証だけでは完了にしない）。暫定cellの退役条件は、直接配置版が同じ契約を満たした時点。 |
| 2 | 一つのpayload型をsource→ownership検査→End→EXEまで直接メモリ配置で接続（fieldはruntime object-store呼出しでなくload/store） | 規模を上げたbinary-treesのbenchmarkで測定。Sharedは read/moveのみ（RC 0）、同一objectの複製・非最終解放、多数objectへの複製・回収、最終解放と子の連鎖Endを分けて測り、RC単体比較とEXE全体比較を分ける（現host-handle retainはArc複製＋handle割当を含む）。copy後の独立性、mut競合拒否、takeの消費時点、Normal/Faultの責任、宣言なしborrowed returnの拒否。copyの残る境界はこの型で紙上移行して確定。Dynamic carrierのreadはcallerがEnd責任を維持。置換した同一source/ABI専用edge・adapterは同seriesで退役。 |
| 3 | 切替（selfhostコードより前） | 上の「既存互換」の条件。Endの分岐/loop/再代入/Invokeへの拡張（終了順、一度だけの消費、必要時だけdrop flag、caller-zeroの専有HomeRelease等だけ削除）は切替までに閉じる。切替前に同一threadの`sync box`を一例だけsource→独立MIR検証→EXEまで接続（複数ownerからの更新・参照、再入拒否、Fault cleanupとpoison、区間内task非実行）し、証拠を既存smokeへ統合する。 |
| 後続 | selfhostは明示moveで書く。Box/container/Sharedへ同じ契約を広げる | 型別identity・slot commit・read支持・Endを各既存ownerで閉じる。未選択機能を現行finite goalへ追加しない。 |
| thread導入 | Send/Share/affinityの実検査、送受信の所有commit、task終了責任 | その後にatomic Sharedとthread版`sync box`を開く。生存中のcellに非原子/原子アクセスを混在させず、表現を変える時はlayout/profileの版と再コンパイルを要求する。 |

数値のみのPointがTrivialなら、それだけでは所有移転やEndの証拠にならない。Trivial分類を
確定し、構文/value copyのcanaryと所有payloadを持つ代表入力を区別する。
record/boxの基本区別は[既存SSOT](../current/main/design/record-box-two-surface-one-substrate-ssot.md)と
[types.md](../../reference/language/types.md)で確定済み。準備ではその区別を再設計せず、copy対象の決定へ使う。
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

暗黙copy/move、最終使用での省略、全Boxの暗黙RCは、局所的な意味とコストを曖昧にする。`share x`前置形、`.share()`、サイズで決める暗黙copy、本体からの借用結果推論も同じ理由で採用しない。
source `copy`を既存CopyOwnedへ単純対応させる案は、値の複製と共有を混同する。
毎stopのfixture/ADR作成や時間だけでの保留は採用せず、現行の1アプリ単位のRULESを維持する。
