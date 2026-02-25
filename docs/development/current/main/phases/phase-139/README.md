# Phase 139: post-if `post_k` Return Lowering Unification

Status: DONE ✅  
Scope: if-only `post_if_post_k.rs` の return lowering を `ReturnValueLowererBox` に統一し、loop/if の出口 SSOT を完成させる。  
Related:
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `docs/development/current/main/phases/phase-138/README.md`
- `src/mir/control_tree/normalized_shadow/post_if_post_k.rs`
- `src/mir/control_tree/normalized_shadow/common/return_value_lowerer_box.rs`

---

## Goal

- if-only `post_k` 内の return を `ReturnValueLowererBox` に委譲し、return の仕様を 1 箇所へ集約する。
- out-of-scope は `Ok(None)` で既存経路へフォールバック（既定挙動不変）。

## Non-Goals

- Assign lowering の共通化（return のみ対象）
- ExprLowerer の一般化（Phase 140）
- Call/MethodCall（Phase 141+）

## Plan

1. `post_if_post_k.rs` の return lowering を `ReturnValueLowererBox::lower_to_value_id()` 呼び出しへ置換（DONE）
2. return の重複ロジック削除（変数 lookup / const emission / add emission）（DONE）
3. fixture + smoke を追加して VM/LLVM EXE parity で固定（DONE）

## Tests

- Fixture:
  - `apps/tests/phase139_if_only_post_k_return_add_min.hako`（expected exit code 4）
- Smokes:
  - `tools/smokes/v2/profiles/integration/apps/phase139_if_only_post_k_return_add_vm.sh`
  - `tools/smokes/v2/profiles/integration/apps/phase139_if_only_post_k_return_add_llvm_exe.sh`
- regressions:
  - Phase 136/137/138（return 系）
  - Phase 97（loop(i<n) フォールバック確認）

## Acceptance

- `post_if_post_k.rs` の return lowering が `ReturnValueLowererBox` に一本化されている
- 既存の挙動を壊さない（dev-only、既定挙動不変）
- VM/LLVM EXE parity を fixture/smoke で固定
