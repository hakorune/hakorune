---
Status: Ready
Scope: Stage-3 の入口設計（Skeleton+Feature → CoreLoopComposer v0 の SSOT 固定、実装は次ステップ）
Related:
- docs/development/current/main/phases/phase-29ao/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/coreloop-exitmap-composition-ssot.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
---

# Phase 29ao P42: Stage-3 design — CoreLoopComposer v0 (Skeleton+Feature)

## 目的

Stage-2（P36–P41）で gate 対象の “planner subset” は release 既定でも `facts → composer → CorePlan` を採用できる状態になった。
しかし現在の composer/normalizer は **pattern ごとの from_facts 変換**が中心で、DomainPlan/PatternFacts の重なりが残っている。

P42 は Stage-3 の入口として、`CorePlan` の骨格を “pattern 列挙” ではなく **Skeleton + Feature の合成**で組み立てるための
`CoreLoopComposer v0` の SSOT を先に固める（docs-first）。

## 非目的（P42ではやらない）

- 実経路の振る舞い変更（release 既定は不変）
- 新しい env var 追加
- pattern subset の拡張（誤マッチ防止を優先）
- CorePlan の語彙拡張（必要なら別Phaseへ）

## 現状の整理（前提）

- `CanonicalLoopFacts` は `LoopFacts + skeleton_kind + exit_usage + presence projections` を持つ（projection は既に SSOT 化済み）。
- `CorePlan` の join 入力は `Frag.block_params + EdgeArgs(layout)` を唯一の入口として PHI に落とす（P9–P14）。
- Gate（SSOT）は `phase29ae_regression_pack_vm.sh` で固定。

## P42 Deliverables（docs-first）

### D1: CoreLoopComposer v0 の責務境界（SSOT）

`CoreLoopComposer v0` は以下だけを行う:

- 入力: `CanonicalLoopFacts`（ただし skeleton_kind=Loop のみ対象）
- 出力: `CorePlan::Loop`（Frag.exits は presence の合成規約に従う）
- 生成: emit/merge を助けるための “再解析” はしない（不足は Facts 側の責務）

### D2: v0 が扱う “定義域”（対象/対象外）を明文化

v0 の対象（最小）:

- `skeleton_kind=Loop`
- `value_join_needed=false`（join 値が必要なケースは v1+）
- cleanup は presence のみ投影（実配線は ExitMap/cleanup SSOT に従う）

v0 の対象外（Freeze/None の扱い方針も書く）:

- non-reducible / multi-entry loop / 解析不能
- unwind/try/yield 等の拡張が必要な構造

### D3: Freeze taxonomy の適用境界

`Ok(None)` / `Err(Freeze)` の境界を、Stage-3 に合わせて固定する。

- `Ok(None)`: そもそも v0 の定義域外（未対象）
- `Err(Freeze::unstructured)`: 構造的に対象っぽいが骨格が一意化できない
- `Err(Freeze::inconsistent)`: Facts が矛盾している
- `Err(Freeze::unsupported)`: 将来の拡張待ち（unwind 等）
- `Err(Freeze::bug)`: SSOT 不変条件違反（実装バグ）

※ release 既定で freeze を増やさない（strict/dev で検出可能にする）方針は維持。

## 次の実装（P43候補、参考）

P42 を固めたあと、最初の実装ターゲットは “最小で意味論不変” のものに限定する:

- P43: `CoreLoopComposer v0` を `composer` に scaffold 追加（未接続、Ok(None) 既定）
- P44: Pattern6 planner subset だけを `CoreLoopComposer v0` で組み立てられるか検討（不足する Facts を洗い出し）

## 受け入れ（P42）

- docs のみ（コード変更なし）でもよい
- `phase-29ao/README.md` / `10-Now.md` / `30-Backlog.md` / `coreplan-migration-roadmap-ssot.md` の Next が P42 を指す
