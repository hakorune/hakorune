# Phase 29ai P13: Memoize Facts in single_planner（historical instruction SSOT, 仕様不変）

Date: 2025-12-29
Status: Historical reference (implemented)
Scope: `single_planner` 内で Facts/Planner 呼び出しを 1 回に集約し、scan_with_init / split_scan / loop_break の planner-first で二重スキャンをなくした slice
Goal: “入口は一本” を保ったまま、Facts/Planner の実行コストと揺れを抑えて SSOT を強化する

Historical note:
- P13 の step bullets は 2025-12 時点の `planner::build_plan(...)` / `DomainPlan` transition vocabulary を残しているよ。
- current runtime では `build_plan_with_facts_ctx(...) -> PlanBuildOutcome` と route-first label が live surface だよ。

## Objective

現状 `src/mir/builder/control_flow/plan/single_planner/rules.rs` は scan_with_init / split_scan / loop_break の各 rule で
`planner::build_plan(ctx.condition, ctx.body)` を個別に呼び出している。

P13 では `try_build_outcome()` の先頭で planner を 1 回だけ実行して結果を memoize し、各 rule ではその結果を参照する。
これにより:
- Facts 抽出が 1 回に収束（LoopBodyLocal facts のような scan を二重実行しない）
- 観測差分を増やさず（planner `Ok(None)` でログ無し）に、planner-first の骨格を強くできる

## Non-goals

- planner の適用範囲拡張（機能追加）
- Freeze を実行経路で増やす（P13 は `Ok(None)` 運用を維持）
- 既存ログ/エラー文字列の変更
- ルール順序の変更（historical numbered-route order SSOT）

## Implementation Steps

### Step 1: planner を 1 回だけ呼ぶ

ファイル:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

変更:
- `try_build_outcome()` 冒頭で以下を 1 回だけ実行し、`PlanBuildOutcome` を得る:
  - `let outcome = planner::build_plan_with_facts_ctx(&planner_ctx, ctx.condition, ctx.body).map_err(|f| f.to_string())?;`

注意:
- `planner_opt` が `Some` の場合でも、各 rule で “採用できる型” 以外なら採用しない（現状の挙動を維持）。

### Step 2: scan_with_init / split_scan / loop_break は memoized planner 結果を見る

方針:
- scan_with_init:
  - `planner_opt` が `Some(DomainPlan::ScanWithInit(_))` のときだけ採用
  - それ以外は legacy scan_with_init extractor
- split_scan:
  - `planner_opt` が `Some(DomainPlan::SplitScan(_))` のときだけ採用
  - それ以外は legacy split_scan extractor
- loop_break:
  - `planner_opt` が `Some(DomainPlan::Pattern2Break(_))` のときだけ採用（historical payload token）
  - それ以外は legacy loop_break extractor

### Step 3: 既存の debug ログ規約を維持

現状の方針を維持:
- planner `Ok(None)` では新規ログを出さない
- 採用時は既存の route label / compatibility log を維持

### Step 4: SSOT verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Notes

P13 は「構造的に SSOT を強くする」ための最小整備で、挙動は変えない。
次に loop_break body-local promotion を Plan 系へ吸収する（P14+）ときの土台になる。
