# Phase 29aj P0: PlannerOutcome（Facts+Plan）SSOT

Date: 2025-12-29  
Status: Ready for execution  
Scope: planner API 拡張（互換維持）＋ single_planner の観測を outcome SSOT に統一  
Goal: facts の直抽出を撤去し、strict/dev 観測が planner outcome の facts に依存する状態へ

## Objective

- single_planner が観測タグのために facts を再スキャンする状態をやめる
- planner が plan が None でも facts を返せるようにし、観測 SSOT を planner に固定する
- 既定挙動・エラー文字列は不変（strict/dev の観測タグのみ対象）

## Non-goals

- Pattern2 LoopBodyLocal を planner 経路で実際に lowering する（P1 以降）
- 新しい env var 追加
- タグ文字列の変更（`[plan/pattern2/promotion_hint:{TrimSeg|DigitPos}]` 維持）

## Target Architecture

- planner API 追加（互換維持）
  - 既存: `build_plan(condition, body) -> Result<Option<DomainPlan>, Freeze>`
  - 追加: `build_plan_with_facts(condition, body) -> Result<PlanBuildOutcome, Freeze>`
- single_planner は `build_plan_with_facts()` を 1 回だけ呼び、観測は outcome.facts 参照のみ

## PlanBuildOutcome（新設）

推奨ファイル:
- `src/mir/builder/control_flow/plan/planner/outcome.rs`

構造（例）:
- `facts: Option<CanonicalLoopFacts>`
- `plan: Option<DomainPlan>`

## Implementation Steps

### Step 1: planner outcome の追加（互換維持）

Files:
- `src/mir/builder/control_flow/plan/planner/mod.rs`
- `src/mir/builder/control_flow/plan/planner/outcome.rs`

やること:
- PlanBuildOutcome を追加
- `build_plan_with_facts()` 実装:
  1. `try_build_loop_facts(condition, body)?`
  2. `canonicalize_loop_facts(facts)` → outcome.facts に格納
  3. `build_plan_from_facts(canonical)` → outcome.plan に格納
- 既存 `build_plan()` は互換用に `build_plan_with_facts().map(|o| o.plan)` へ委譲

注意:
- plan が None でも facts は Some になり得る（観測のために重要）

### Step 2: single_planner を outcome SSOT に統一

Files:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

やること:
- planner 呼び出しを `build_plan_with_facts()` に置換し memoize 維持
- P15 で入った facts 直抽出を撤去
- タグ判定は outcome.facts のみ参照

### Step 3: smoke は現状維持

Files:
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_seg_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ab_pattern2_loopbodylocal_min_vm.sh`

やること:
- 変更なし（タグ必須のまま PASS すること）

### Step 4: docs / CURRENT_TASK 更新

Files:
- `docs/development/current/main/phases/phase-29ai/README.md`（P15 の観測が planner outcome 参照になったこと）
- `docs/development/current/main/phases/phase-29ai/P15-...INSTRUCTIONS.md`（facts直抽出禁止の追記）
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## Acceptance Criteria

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`（154/154 PASS）
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` PASS
- single_planner に facts 直抽出が残っていない

## Commit

- `git add -A && git commit -m "phase29aj(p0): expose planner outcome facts for strict observability"`
