---
Status: SSOT
Scope: plan/features helper boundary（join/exit/phi/carrier の“一箇所化”ルール）
Related:
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/exitbranch-ssot.md
- src/mir/builder/control_flow/plan/REGISTRY.md
- src/mir/builder/control_flow/plan/features/README.md
---

# Feature helper boundary (SSOT)

目的: “pattern 固有 ops” に join/exit/phi の直書きが再流入して、例外パターンが積み上がるのを防ぐ。
小箱（lego）として **helper を一箇所に寄せる**ことで、selfhost unblock のサイクル（fixture→fast gate→最小拡張）を「負債化しない」形で継続できるようにする。

## Rule (must)

### 1) ops / pipeline は “部品の適用” だけ

`features/*_ops.rs` / `features/*_pipeline.rs` は次だけを担当する:
- skeleton が用意した slot に feature helper を適用する
- DomainPlan/CorePlan の “材料” を組み立てる（ただし AST 再解析は禁止）

禁止（ops に置かない）:
- PHI（join）の直接挿入
- `BranchStub` / `EdgeStub` の直接組み立て（ExitKind/EdgeArgs の直書き）
- carrier merge（per-edge join / final_values の合流）を手書き
- “exit 付きブランチ（prelude + ExitKind）” の独自実装

### 2) helper は “一箇所化”して再利用する

下記を SSOT helper として扱い、同型の処理を増やさない（増えそうなら移設する）。

- If join / phi:
  - `src/mir/builder/control_flow/plan/features/if_join.rs`
- ExitIf / branch / stubs:
  - `src/mir/builder/control_flow/plan/features/exit_if_map.rs`
- Carrier merge / final_values:
  - `src/mir/builder/control_flow/plan/features/carrier_merge.rs`
- Conditional update join (select/if join):
  - `src/mir/builder/control_flow/plan/features/conditional_update_join.rs`
- ExitBranch（prelude + ExitKind 共通化）:
  - `src/mir/builder/control_flow/plan/parts/stmt.rs` (return prelude SSOT)
  - `src/mir/builder/control_flow/plan/parts/exit.rs` (break/continue/return SSOT)
  - `src/mir/builder/control_flow/plan/features/exit_branch.rs` は互換の薄い委譲のみ
- EdgeCFG stubs (BranchStub/EdgeStub builders):
  - `src/mir/builder/control_flow/plan/features/edgecfg_stubs.rs`
- Carrier variable sets (SSOT entry point):
  - `src/mir/builder/control_flow/plan/features/carriers.rs`
  - SSOT entry points:
    - `collect_from_body`
    - `collect_outer_from_body`
    - `collect_from_recipe_continue_with_return`
    - `collect_from_recipe_continue_only`
  - Suffix convention (private helpers):
    - `_cwr` = ContinueWithReturn
    - `_co` = ContinueOnly
- Loop carrier PHI / bindings / EdgeArgs:
  - `src/mir/builder/control_flow/plan/features/loop_carriers.rs`
  - `with_loop_carriers` (attach carriers to CoreLoopPlan)
  - `build_loop_phi_info` (2-input header PHI - T38/T39/T40)
  - `build_step_join_phi_info` (empty-input step join PHI - T44)
  - `build_preheader_only_phi_info` (1-input header PHI - T46)
  - `build_after_merge_phi_info` (after_bb merge PHI for break/after joins)
  - `build_loop_bindings` (variable→ValueId map - T37a/T37b)
  - `build_expr_carrier_join_args` (T32: 実装移設済み)
  - CorePhiInfo direct constructions should be limited to `loop_carriers.rs`
    （`plan/core.rs` の型定義を除く）
- StepMode setters:
  - `src/mir/builder/control_flow/plan/step_mode.rs`（plan-wide SSOT）
  - `src/mir/builder/control_flow/plan/features/step_mode.rs`（features 側の薄い委譲）
  - `InlineInBody` / `ExtractToStepBb` と `has_explicit_step` の組を helper で固定（ops/pipeline/normalizer/composer/skeleton の直書き禁止）

補足（回帰防止）:
- `conditional_update_join` は `CoreExitPlan::{Break,Continue,ContinueWithPhiArgs}` を直書きしない（`exit_branch` helper を使う）。

## Closeout checklist (features/**)

棚卸しで以下を確認し、逸脱があれば SSOT helper へ移設する。
- `rg -n "CorePlan::Exit\\(CoreExitPlan::" src/mir/builder/control_flow/plan/features`
- `rg -n "CoreExitPlan::(Break|Continue|Return|ContinueWithPhiArgs)" src/mir/builder/control_flow/plan/features`
- `rg -n "BranchStub \\{|EdgeStub \\{" src/mir/builder/control_flow/plan --glob '!**/features/edgecfg_stubs.rs'`
- `rg -n "CorePhiInfo \\{|create_phi_bindings\\(" src/mir/builder/control_flow/plan/features`
- `rg -n "step_mode:\\s*LoopStepMode::|has_explicit_step:\\s*(true|false)" src/mir/builder/control_flow/plan --glob '!**/tests.rs'`
- `rg -n "\\[plan/trace\\].*\\{:\\?\\}" src/mir/builder/control_flow/plan/features`

## Body lowering inventory (SSOT)

重複が2箇所以上の同型のみ steps/ へ移設する。
- `generic_loop_body.rs`: `lower_effect_block`
- `loop_cond_continue_with_return_pipeline.rs`: `lower_continue_with_return_block`
- `loop_cond_continue_only_pipeline.rs`: `lower_continue_only_block`
- `loop_cond_return_in_body_pipeline.rs`: `lower_return_in_body_block`
- `parts/stmt.rs`: `lower_return_prelude_stmt` / `lower_return_prelude_block` (SSOT)
- `exit_map.rs`: `lower_exit_block`
- `loop_cond_break_continue_pipeline.rs`: `lower_loop_cond_block`
- `steps/stmt_block.rs`: `lower_stmt_block` (shared)

### Status (inventory result)

棚卸しの結果、現時点では “同型が2箇所以上” の body-lowering は残っていない。
（`lower_stmt_block` は steps/ に移設済み。残りは pipeline/feature 固有の lowering で、無理に抽象化しない。）

## Promotion trigger (when to refactor)

同型の “join/exit/phi/carrier” ロジックが 2 箇所以上に現れたら、次の順に対処する:
1. 既存 helper へ移設（挙動不変）
2. helper API を 1 個だけ増やす（SSOT と gate を更新）
3. ops 側は呼び出しだけに戻す（薄い adapter）

## Next decompositions (planned)

「最小部品」の密度を上げて、同種の仕事が別モジュールに散るのを防ぐための分解候補。
いずれも “挙動不変の移設” を原則にし、gate green を維持したまま進める。

### 1) `if_join.rs` から loop-carrier を分離

問題: `features/if_join.rs` が IfJoin と loop-carrier（phi/bindings/attach）を同居しやすく、責務境界が曖昧。

対応:
- ✅ `features/loop_carriers.rs` を入口SSOT化済み（PHI/bindings/EdgeArgs）
- ✅ `if_join.rs` は “IfJoin のみ” に収束済み（apply-only）

### 2) `exit_if_map.rs` から EdgeCFG stub builder を分離

問題: `features/exit_if_map.rs` が CorePlan 生成（exit-if）と `BranchStub/EdgeStub` 生成を同居しやすい。

対応:
- `features/edgecfg_stubs.rs`（名称は `edge_stubs.rs` でも可）を新設し、stub builder 群を移設
- `exit_if_map.rs` は “exit-if を CorePlan に落とす” のみに寄せる

### 3) carrier 変数集合（vars/ids）を小部品化

問題: `features/carrier_merge.rs` と `features/conditional_update_join.rs` の間で carrier の “集合（vars/ids）” の入口が分散しやすい。

対応:
- ✅ `features/carriers.rs` を入口SSOT化済み（`collect_from_body` / recipe / outer）
- `carrier_merge` / `conditional_update_join` は carriers を入力として受け取る形に寄せる（必要が出たら）

### 4) `canon/cond` と `lower_bool_expr` の境界を強める

問題: canon と lowering の境界が曖昧だと、Facts 側の if 条件判定ロジックが増殖しやすい。

ルール（強化案）:
- canon は “判定と抽出だけ”（analysis-only view 生成）
- lower は “canon が作った view を実行へ落とすだけ”
- Facts は raw AST に対する独自判定を増やさず、canon/view を参照して分岐する
- `lower_bool_expr` は CondCanon を入力に取れる設計を維持し、raw AST の分岐追加は SSOT に記載する

## Acceptance (green)

- `cargo build --release`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`（=29ae 含む）
