---
Status: Active
Scope: code（仕様不変、CorePlan互換性の拡張）
Related:
- docs/development/current/main/phases/phase-29am/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
---

# Phase 29am P1: CoreLoopPlan body supports Seq-of-effects (flatten)

Date: 2025-12-29  
Status: Ready for execution  
Scope: `CoreLoopPlan.body` の “Effect-only” 制約を維持したまま、`Seq([Effect...])` を flatten して emit できるようにする

## Why

現状:
- `lower_loop_generalized()` は `loop_plan.body` に `CorePlan::Effect` しか許可していない
- しかし Normalizer/Composer の合成都合で `Seq([Effect...])` が自然に出る（将来の Skeleton+Feature 合成で顕在化しやすい）

ここを先に受理しておくと、CorePlan 合成の自由度が上がる一方で、CFG/Frag を壊さない（branch/exit は body_bb では扱わない）。

## Non-goals

- `CorePlan::If` を `loop_plan.body` に入れる（branch は Frag が SSOT）
- `CorePlan::Exit` を `loop_plan.body` に入れる（exit は ExitMap/Frag が SSOT）
- 既存のルーティング/観測/エラー文字列の変更

## Implementation

### Step 1: lowerer で Seq-of-effects を flatten

Target:
- `src/mir/builder/control_flow/plan/lowerer.rs`

Change:
- `lower_loop_generalized()` の body_bb special handling を拡張する
- 許可する形:
  - `CorePlan::Effect(_)`
  - `CorePlan::Seq([CorePlan::Effect(_), ...])`（入れ子 Seq は再帰 flatten）
- それ以外（If/Exit/Loop）は従来通り error（挙動不変）

### Step 2: unit tests

Target:
- `src/mir/builder/control_flow/plan/lowerer.rs` の tests

Add:
- `CoreLoopPlan.body` に `Seq([Effect...])` を入れて PASS

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29am(p1): flatten seq-of-effects in core loop body"`

