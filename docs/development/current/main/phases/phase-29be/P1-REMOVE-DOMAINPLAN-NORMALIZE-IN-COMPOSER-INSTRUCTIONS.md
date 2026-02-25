---
Status: Ready
Scope: code + docs
---

# Phase 29be P1: Remove `PlanNormalizer::normalize(domain_plan)` from gate composition paths

## Goal

Phase 29be P0 の inventory に基づき、gate 内で `DomainPlan → PlanNormalizer::normalize(domain_plan)` を踏む経路を削減する。

対象は `Pattern2/3/5` の `value_join_needed=false` 経路で、現状 `coreloop_single_entry.rs` が
`PlanNormalizer::normalize(domain_plan.clone(), ctx)` を呼んでいる部分。

## Non-goals

- subset 拡張（facts の新規抽出など）
- 新しい env var の追加
- release 既定の恒常ログ/エラー文字列変更

## Target

Inventory (P0):
- `src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs`
  - Pattern2/3/5 の `value_join_needed=false` が `PlanNormalizer::normalize(domain_plan)` に依存

## Steps

### Step 1: composer を “typed plan entry” に切替

`src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs`

- `try_compose_core_loop_for_domain_plan(...)` の以下を置換:
  - `DomainPlan::Pattern2Break(_)` かつ `!facts.value_join_needed`
  - `DomainPlan::Pattern3IfPhi(_)` かつ `!facts.value_join_needed`
  - `DomainPlan::Pattern5InfiniteEarlyExit(_)` かつ `!facts.value_join_needed`

置換方針:

- `PlanNormalizer::normalize(builder, domain_plan.clone(), ctx)` を廃止し、
  各 pattern の typed entry（例: `PlanNormalizer::normalize_pattern2_break(...)`）で CorePlan を生成する。
- 引数の `Pattern*Plan` は `DomainPlan` 内に保持されている型をそのまま使う（clone）。

狙い:

- “DomainPlan を持つこと” ではなく “domain_plan による generic normalize を踏むこと” を gate から消す。
- 既存の normalizer 実装をそのまま使うことで意味論を変えない（構造的置換）。

### Step 2: Inventory 更新（SSOT）

`docs/development/current/main/phases/phase-29be/README.md`

- Inventory (P0) の composer 行を更新:
  - `PlanNormalizer::normalize(domain_plan)` が gate 内で残っていないこと
  - 置換後は typed entry を使っていること

### Step 3: Now/Roadmap 更新

- `docs/development/current/main/10-Now.md`: Next を P2 へ
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`: Next を P1→P2 に同期

## Verification

- `rg -n \"PlanNormalizer::normalize\\(\" src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs`
  - Pattern2/3/5 の non-join 経路でヒットしないこと
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A`
- `git commit -m \"phase29be(p1): remove domain_plan normalize from composer\"`

