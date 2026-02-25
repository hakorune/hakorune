---
Status: SSOT
Scope: CorePlan “ExitBranch” feature（exit 付きブランチの共通化）
Related:
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/join-explicit-cfg-construction.md
- src/mir/builder/control_flow/plan/REGISTRY.md
- src/mir/builder/control_flow/plan/features/README.md
- src/mir/builder/control_flow/plan/features/exit_if_map.rs
---

# ExitBranch feature (SSOT)

目的: If/BranchN/Loop 内で繰り返し出てくる「ブランチ本体の前処理（prelude）＋ exit（return/break/continue）」を **共通の feature 部品**として切り出し、個別パターンの例外増殖を止める。

この部品は「新しい pattern 追加」ではなく、既存の `exit_if_map`/match/loop 等の **重複を削るためのレゴ化（分解）**。

## Constraints (must)

- no AST rewrite（見かけ等価の式変形・コード移動は禁止）
- release default unchanged（拡張の適用は strict/dev + `HAKO_JOINIR_PLANNER_REQUIRED=1` のみ）
- silent fallback 禁止（判定不能/不明瞭は `Freeze` または `Ok(None)` に倒す。境界は `planfrag-freeze-taxonomy` をSSOT）
- EdgeCFG: 1 block = 1 terminator（`BranchStub`/`EdgeStub` の契約を壊さない）

## Data model (concept)

ExitBranch は「ブランチの最後が exit で終わる」形を 1 箇所で表す。

- `ExitBranchAst`:
  - `prelude`: exit 直前に並ぶ stmt（順序保持）
  - `exit`: `Return(value)` / `Break(depth=1)` / `Continue(depth=1, with_phi_args?)`

ExitBranch は “shape 検出（facts/canon）” をしない。呼び出し側（facts/normalizer）が「ここは exit branch」と分かっている箇所に適用する。

## Allowed prelude (minimal)

最初の受理範囲は conservative で良い（拡張は fixture+fast gate で固定してから）。

- `LocalDecl`（init ありのみ、順序保持）
- `Assign`
- `FunctionCall` / `MethodCall`（stmt としての呼び出し）
- 禁止: prelude 内の control flow（if/loop/return/break/continue）、未初期化 local、評価順が変わる rewrite を要するもの

## Lowering contract (must)

`ExitBranch.lower(prelude, exit, ...) -> Vec<CorePlan>`

- prelude は **そのままの順序**で lower（effects を吐く）
- prelude が bindings を更新する場合、ブランチ内の bindings と `variable_ctx` を同期する
- ブランチを抜けるとき、必要なら `variable_ctx` を restore する（branch-local の汚染を防ぐ）
- exit の lowering は共通化:
  - `Break(depth=1)` は `CoreExitPlan::Break(1)`
  - `Continue(depth=1)` は `ContinueTarget` / carrier merge の契約に従う（phi args が必要な場合はエラー prefix 付きで fail-fast）
  - `Return(value)` は value を lower して `CoreExitPlan::Return(Some(value_id))`

## Integration plan (decomposition-first)

1. 既存の重複を “移設” する（挙動不変）
   - 例: `src/mir/builder/control_flow/plan/features/exit_if_map.rs` の `split_exit_branch` / prelude lowering を `features/exit_branch.rs` に移動
2. 呼び出し側は `ExitBranch` を使う薄いアダプタにする（pattern を増やさない）
3. 次の候補（必要になった時だけ）
   - BranchN/match の各 arm に同じ `ExitBranch` を適用（arm の前処理＋exit を共通化）

Acceptance（green 維持）:
- `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh` PASS
- `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh` PASS（29ae 含む）

