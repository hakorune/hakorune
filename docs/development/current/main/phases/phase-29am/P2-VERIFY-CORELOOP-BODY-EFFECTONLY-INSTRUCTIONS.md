---
Status: Active
Scope: code（fail-fastの前倒し、仕様不変）
Related:
- docs/development/current/main/phases/phase-29am/README.md
- docs/development/current/main/design/effect-classification-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
---

# Phase 29am P2: Verify CoreLoopPlan.body is Effect-only (Seq flatten allowed)

Date: 2025-12-29  
Status: Ready for execution  
Scope: `CoreLoopPlan.body` の許可語彙を verifier で fail-fast（仕様不変）

## Objective

- `CoreLoopPlan.body` は “body_bb に emit できるものだけ” を許可する
- 許可語彙は最小（Effect-only）。ただし P1 により `Seq([Effect...])` は flatten して扱えるため許可
- `If/Exit/Loop` が body に混入した場合、lowerer の実行時エラーではなく **PlanVerifier の局所エラー**で落とす

## Non-goals

- 既存のルーティング/観測の変更
- `If/Exit` を body で扱う実装追加（branch/exit は Frag/ExitMap がSSOT）
- 新 env var / 恒常ログ追加

## Implementation

### Step 1: PlanVerifier で Loop.body の語彙制約を追加

Target:
- `src/mir/builder/control_flow/plan/verifier.rs`

Rule:
- Loop.body の各 item は
  - `CorePlan::Effect(_)` または
  - `CorePlan::Seq([Effect...])`（入れ子 Seq は再帰で検査）
  のみ許可
- それ以外が出たら Err（新しい invariant tag を追加してよい）

### Step 2: unit tests

Target:
- `src/mir/builder/control_flow/plan/verifier.rs` の tests

Add (minimum):
- body に `Seq([Effect, Seq([Effect])])` が入っても verify OK
- body に `If` または `Exit` が入ったら verify Err

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29am(p2): verify core loop body effect-only"`

