# Phase 29ai P0: LoopFacts SSOT + Single Planner skeleton — Instructions

Status: Historical reference (implemented)
Scope: Plan/Frag 導線の SSOT 化（docs-first, 仕様不変）

## Goal

Plan/Frag の入口を「numbered route label で分岐」から「Facts（事実）をSSOTとして構築→単一PlannerでPlan化」へ寄せる。

- Historical skeleton API: `build_plan(...) -> Result<Option<Plan>, Freeze>`
- Current live surface: `single_planner::try_build_outcome(...) -> Result<PlanBuildOutcome, String>`

Historical note:
- P0 は `Plan` / `PlanKind` / `loop_facts.rs` / `planner/build.rs` という 2025 skeleton-era vocabulary で書かれているよ。
- current runtime では `plan/facts/{loop_builder.rs,loop_types.rs}` と `planner/{mod.rs,outcome.rs}` が live lane だよ。

## Non-goals

- 仕様変更（挙動変更）
- 大規模な既存置換（このP0は骨格とSSOTを作るだけ）
- fixture/smoke の新設（既存の quick / regression pack で担保）

## Target Architecture (SSOT)

```
src/mir/builder/control_flow/plan/
  facts/                # 解析のみ（AST/CFG から Facts）
    mod.rs
    loop_builder.rs     # current LoopFacts collection entrypoint
    loop_types.rs       # current LoopFacts type surface
    scan_shapes.rs      # step/cond の形（enum）
  normalize/             # Facts の表現ゆれを正規化（純変換）
    mod.rs
    canonicalize.rs
  planner/               # Facts → Plan（純変換、分岐はここに閉じ込める）
    mod.rs
    outcome.rs          # current PlanBuildOutcome surface
    helpers.rs          # planner helpers
  emit/                  # Plan → Frag（生成のみ）
    mod.rs
```

## API Contract

### Facts layer

- `try_build_loop_facts(...) -> Result<Option<LoopFacts>, Freeze>`
  - `Ok(Some(facts))`: facts が揃った
  - `Ok(None)`: 対象外（既存経路へ）
  - `Err(freeze_tag)`: 契約違反（Fail-Fast）

### Planner layer

- execution-time skeleton note: `build_plan_from_facts(...) -> Result<Plan, Freeze>`
- current runtime note: planner outcome は `PlanBuildOutcome` / recipe contract を返す

## Implementation Steps (P0)

1) `facts/` と `planner/` の骨格を追加（未使用でもOK）
2) “入口SSOT” の関数だけ用意（中身は `Ok(None)` でも良い）
3) 既存の route implementation は触らない（P0は並走の足場だけ）
4) docs を更新（Phase 29ai README に P0 入口/目的を明記）

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- 既存挙動は不変（回帰パック/quick が緑）
- Facts/Planner の SSOT 骨格ができ、次Pで “route absorption” の受け口が明確
