---
Status: Ready
Scope: code（未接続・仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/coreplan-skeleton-feature-model.md
  - docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P4: ExitMap presence → Frag.exits（未接続・仕様不変）

Date: 2025-12-30  
Status: Ready for execution  
Scope: CorePlan direct compose（Pattern1 subset）に “ExitMap presence” を接続し、Frag 語彙に投影する（未配線のまま）

## 目的

- Facts 側で `exit_kinds_present`（presence）が投影できるようになった（Phase 29an）ので、次はこれを
  `Frag.exits`（EdgeCFG ExitMap 語彙）へ投影して、**「pattern を増やさず ExitMap を feature として扱う」** 道筋を固める。
- まずは `direct CoreLoop skeleton`（Phase 29ao P3）で生成する `Frag` に対して、
  `CanonicalLoopFacts.exit_kinds_present` → `Frag.exits` を **未配線（target=None）** で入れる。

## 非目的

- router/legacy の挙動変更（P4 は未接続）
- 実際の break/continue/return の CFG 配線（P4 は “presence の語彙投影” だけ）
- 新 env var / 恒常ログ追加

## 実装ポイント（重要）

### 1) “exits は emit されない” を利用して安全に語彙だけ固定

`PlanLowerer` は `emit_frag()` に `frag.wires` / `frag.branches` しか渡さないため、`frag.exits` は現時点で
実行経路に影響しない。よって、P4 は “語彙固定” のみを安全に先行できる。

### 2) LoopId の扱い（暫定）

`ExitKind::Break/Continue` は `LoopId` を要求するが、Plan 層には LoopId allocator がまだ無い。
P4 では **presence の投影**として `LoopId(0)` を使用する（未接続・未配線なので挙動不変）。

## 実装手順

### Step 1: exitmap presence → Frag.exits のビルダ関数を追加

対象（P3 で追加済みの direct skeleton 実装）:
- `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs`

追加（案）:

- `fn build_exitmap_from_presence(present: &BTreeSet<ExitKindFacts>, from: BasicBlockId) -> BTreeMap<ExitKind, Vec<EdgeStub>>`

実装:
- `present` を走査して `ExitKindFacts::{Return,Break,Continue}` を `ExitKind::{Return,Break(LoopId(0)),Continue(LoopId(0))}` へ写像
- 各 kind に対して `EdgeStub::without_args(from, kind)` を 1 本だけ入れる（target=None のまま）
- それ以外（未対応の kind）は追加しない（P4 の scope 外）

from ブロック:
- `body_bb` を使う（break/continue/return は “ループ本体中” から発生する前提に寄せる）

### Step 2: direct skeleton が生成する Frag に exits を埋める

対象:
- `normalize_loop_skeleton_from_facts(...)` の `Frag { exits: ... }`

変更:
- `exits: BTreeMap::new()` を `exits: build_exitmap_from_presence(&facts.exit_kinds_present, body_bb)` に置換

### Step 3: unit tests（presence の投影を固定）

対象:
- `src/mir/builder/control_flow/plan/composer/mod.rs` の direct skeleton テスト
  - もしくは `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs` に unit test を追加

追加テスト（例）:

1) `exit_kinds_present = ∅` のとき `frag.exits.is_empty()`
2) `exit_kinds_present = {Return}` のとき `frag.exits` に `ExitKind::Return` がある（stub.target=None）
3) `exit_kinds_present = {Break,Continue}` のとき `frag.exits` にそれぞれがある（`LoopId(0)`、stub.target=None）

注: P4 は “語彙投影” のみで、配線/compose はまだ行わない。

### Step 4: docs 更新

更新:
- `docs/development/current/main/phases/phase-29ao/README.md`（P4 完了記録 + Next を P5 に）
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p4): project exitmap presence into frag exits (unconnected)"`

## 次（P5）

P5 では cleanup presence を ExitKind 単位で wire し、`exitkind-cleanup-effect-contract-ssot.md` の契約を code 側へ落とす。
