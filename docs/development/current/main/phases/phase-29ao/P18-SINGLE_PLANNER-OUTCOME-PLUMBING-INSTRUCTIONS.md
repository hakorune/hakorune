---
Status: Ready
Scope: code（仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/phases/phase-29ae/README.md
  - src/mir/builder/control_flow/plan/planner/outcome.rs
---

# Phase 29ao P18: single_planner が planner outcome（facts+plan）を返す（P17の二重planner呼び出し撤去）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 仕様不変。P17 で router に入った “Pattern1 strict/dev shadow adopt” が planner を 2 回呼ぶのを、構造で解消する。

## 目的

- `single_planner` を **planner outcome（facts+plan）SSOT** の窓口にして、router 側の二重実行を撤去する。
- P15/P16 で強化した JoinIR 回帰パック（phase29ae pack）を緑のまま維持する。
- 観測（routeログ / promotion_hint タグ）とエラー文字列（既存）を変えない。

## 非目的

- Pattern1 shadow adopt の “strict/dev only” 条件を変える（今回は構造だけ）
- 新 env var 追加、恒常ログ追加
- Facts/Planner のロジック変更（返し方だけ）

## 現状の問題（P17 の副作用）

`route_loop_pattern()` が
1) `single_planner::try_build_domain_plan(ctx)` の内部で planner を実行し  
2) strict/dev + Pattern1 のときに router で `planner::build_plan_with_facts_ctx(..)` をもう一度実行して facts を取り直す  
という二重実行になっている。

これは SSOT 的に “同じ事実を 2 回作る” ので、段階移行としては望ましくない（構造で直したい）。

## 方針

### API追加（後方互換）

- `single_planner` に新APIを追加する:
  - `try_build_domain_plan_with_outcome(ctx) -> Result<(Option<DomainPlan>, PlanBuildOutcome), String>`
- 既存APIは維持する:
  - `try_build_domain_plan(ctx) -> Result<Option<DomainPlan>, String>`
  - 実装は新APIを呼んで `plan` だけ返す（挙動不変）

### router は新APIを使う

- router は `PlanBuildOutcome` を受け取り、P17 の strict/dev Pattern1 adopt で **outcome.facts** を使う（planner を呼び直さない）。

## 実装手順

### Step 1: single_planner に新APIを追加

Files:
- `src/mir/builder/control_flow/plan/single_planner/mod.rs`
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

やること:
- `rules.rs` に `pub(super) fn try_build_domain_plan_with_outcome(ctx) -> Result<(Option<DomainPlan>, planner::outcome::PlanBuildOutcome), String>` を追加
  - outcome の生成は既存と同じ（PlannerContext + planner::build_plan_with_facts_ctx）
  - promotion_hint の出力位置・条件は現状維持
  - ルール順序と routeログも現状維持
- `mod.rs` で上記を再export:
  - `pub(in crate::mir::builder) fn try_build_domain_plan_with_outcome(ctx) -> Result<(Option<DomainPlan>, PlanBuildOutcome), String>`
- 既存 `try_build_domain_plan` は新APIを呼ぶように変更（planだけ捨てる）

### Step 2: router の P17 二重 planner 呼び出しを撤去

Files:
- `src/mir/builder/control_flow/joinir/patterns/router.rs`

やること:
- `single_planner::try_build_domain_plan(ctx)` を `try_build_domain_plan_with_outcome(ctx)` に置換
- strict/dev + Pattern1 の adopt では `outcome.facts` を使う
- 失敗メッセージは現状維持（P17 で追加した文言を変えない）

### Step 3: テスト（最小）

必須:
- `cargo test --release -p nyash-rust --lib`
- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

任意（strict/dev adopt の手動確認）:
- `HAKO_JOINIR_STRICT=1 ./target/release/hakorune --backend vm apps/tests/loop_min_while.hako`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p18): plumb planner outcome through single_planner"`

