# Phase 141: Impure Extension Contract → P1 Known Intrinsic (incremental)

Status: DONE ✅  
Scope: `NormalizedExprLowererBox` の pure/impure 境界を contract（SSOT）として型で固定し、Call/MethodCall を段階投入できる形へ収束させる。  
Related:
- `docs/development/current/main/design/normalized-expr-lowering.md`
- `docs/development/current/main/phases/phase-140/README.md`
- `src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs`
- `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`

---

## Goal

- ExprLowerer が “pure only” のままでも、impure（Call/MethodCall）導入の拡張点（契約）を SSOT 化する。
- P0 は contract のみ（Call/MethodCall を lowering しない、既定挙動不変）。
- P1 は “既知 intrinsic だけ” を許可し、impure 導入の第一歩を作る（それ以外は既定挙動不変）。

## Non-Goals (P0)

- Call/MethodCall の lowering（ValueId生成、effects、receiver materialization）
- impure式の順序付け（effects ordering）
- 型解決（dispatch/overload）

## SSOT

- Contract: `src/mir/control_tree/normalized_shadow/common/expr_lowering_contract.rs`
  - `ExprLoweringScope::{PureOnly, WithImpure(..)}`
  - Phase 141 P1: `ImpurePolicy::KnownIntrinsicOnly`, `KnownIntrinsic::{Length0}`
  - `OutOfScopeReason`（Call/MethodCall 等の最小理由）
- ExprLowerer API: `src/mir/control_tree/normalized_shadow/common/expr_lowerer_box.rs`
  - `lower_expr_with_scope(scope, ...)` を追加
  - 既存 `lower_expr(...)` は `PureOnly` の thin wrapper（挙動不変）
 - Return lowering entry: `src/mir/control_tree/normalized_shadow/common/return_value_lowerer_box.rs`
   - Phase 141 P1: return lowering は `WithImpure(KnownIntrinsicOnly)` を使う（既知 intrinsic のみ許可）

## Tests

- Unit: `cargo test -p nyash-rust --lib mir::control_tree::normalized_shadow::common`
- Added minimal contract tests:
  - `call_is_out_of_scope_in_pure_only`
  - `methodcall_is_out_of_scope_in_pure_only`
  - `methodcall_length0_is_in_scope_with_known_intrinsic_only`
  - `lower_methodcall_length0_emits_method_call_inst`

## Phase 141 P1: KnownIntrinsicOnly (Length0)

### Scope

- `ExprLoweringScope::WithImpure(ImpurePolicy::KnownIntrinsicOnly)` のときのみ、以下を lowering する:
  - `receiver.length()`（引数 0、receiver は env に存在する Variable のみ）
- それ以外の Call/MethodCall は out-of-scope (`Ok(None)`) のまま（既定挙動不変）。

### Fixture / Smoke

- Fixture: `apps/tests/phase141_p1_if_only_post_k_return_length_min.hako`（expected exit code 3）
- Smoke tests:
  - VM: `tools/smokes/v2/profiles/integration/apps/archive/phase141_p1_if_only_post_k_return_length_vm.sh`
  - LLVM EXE: `tools/smokes/v2/profiles/integration/apps/archive/phase141_p1_if_only_post_k_return_length_llvm_exe.sh`

## Phase 141 P1.5: SSOT Reinforcement (Registry + available_inputs + diagnostics)

### Task A: KnownIntrinsic SSOT (Registry)

- 目的: intrinsic の metadata（method 名/arity/type_hint）を 1 箇所に集約し、文字列直書きの散らばりを止める。
- SSOT:
  - `src/mir/control_tree/normalized_shadow/common/known_intrinsics.rs`
  - `KnownIntrinsicRegistryBox::{lookup,get_spec}`
- 効果:
  - `expr_lowerer_box.rs` の by-name 判定が “registry lookup” に収束

### Task B: available_inputs 3-source merge (Bug fix)

- 目的: suffix 正規化が prefix 側で生成された locals（`local s; s="..."`）を見失わないようにする。
- 変更:
  - `AvailableInputsCollectorBox::collect(builder, captured_env, prefix_variables)` を追加
  - 優先順位: Function params > Prefix variables > CapturedEnv
- Call sites:
  - `src/mir/builder/control_flow/normalization/execute_box.rs`
  - `src/mir/builder/control_flow/plan/policies/policies/normalized_shadow_suffix_router_box.rs`
    - historical path token: `policies/normalized_shadow_suffix_router_box.rs` under the old `joinir/patterns/` lane

### Task C: Diagnostics

- 追加: `OutOfScopeReason::IntrinsicNotWhitelisted`
  - “methodcall だが known intrinsic allowlist 外” を区別できるようにする

## Next

- Phase 141 P2+: 一般 Call/MethodCall（effects + typing を分離して拡張）
