# Phase 29ai P13: Memoize Facts in single_planner（SSOT, 仕様不変）

Date: 2025-12-29  
Status: Ready for execution  
Scope: `single_planner` 内で Facts/Planner 呼び出しを 1 回に集約し、Pattern6/7/2 の planner-first で二重スキャンをなくす  
Goal: “入口は一本” を保ったまま、Facts/Planner の実行コストと揺れを抑えて SSOT を強化する

## Objective

現状 `src/mir/builder/control_flow/plan/single_planner/rules.rs` は Pattern6/7/2 の各 rule で
`planner::build_plan(ctx.condition, ctx.body)` を個別に呼び出している。

P13 では `try_build_domain_plan()` の先頭で planner を 1 回だけ実行して結果を memoize し、各 rule ではその結果を参照する。
これにより:
- Facts 抽出が 1 回に収束（LoopBodyLocal facts のような scan を二重実行しない）
- 観測差分を増やさず（planner `Ok(None)` でログ無し）に、planner-first の骨格を強くできる

## Non-goals

- planner の適用範囲拡張（機能追加）
- Freeze を実行経路で増やす（P13 は `Ok(None)` 運用を維持）
- 既存ログ/エラー文字列の変更
- ルール順序の変更（PLAN_EXTRACTORS SSOT）

## Implementation Steps

### Step 1: planner を 1 回だけ呼ぶ

ファイル:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

変更:
- `try_build_domain_plan()` 冒頭で以下を 1 回だけ実行し、`planner_opt: Option<DomainPlan>` を得る:
  - `let planner_opt = planner::build_plan(ctx.condition, ctx.body).map_err(|f| f.to_string())?;`

注意:
- `planner_opt` が `Some` の場合でも、各 rule で “採用できる型” 以外なら採用しない（現状の挙動を維持）。

### Step 2: Pattern6/7/2 は memoized planner 結果を見る

方針:
- Pattern6:
  - `planner_opt` が `Some(DomainPlan::ScanWithInit(_))` のときだけ採用
  - それ以外は legacy Pattern6 extractor
- Pattern7:
  - `planner_opt` が `Some(DomainPlan::SplitScan(_))` のときだけ採用
  - それ以外は legacy Pattern7 extractor
- Pattern2:
  - `planner_opt` が `Some(DomainPlan::Pattern2Break(_))` のときだけ採用
  - それ以外は legacy Pattern2 extractor

### Step 3: 既存の debug ログ規約を維持

現状の方針を維持:
- planner `Ok(None)` では新規ログを出さない
- 採用時は既存の pattern 名ログを維持

### Step 4: SSOT verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Notes

P13 は「構造的に SSOT を強くする」ための最小整備で、挙動は変えない。
次に Pattern2 LoopBodyLocal promotion を Plan 系へ吸収する（P14+）ときの土台になる。

