---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の A3-min4 として、PluginBoxMetadata を shim固定依存から route-aware 形へ縮退する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-228-runtime-a3-min3-globals-errors-extern-failfast-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/types.rs
  - src/runtime/plugin_loader_v2/enabled/loader/metadata.rs
  - src/runtime/plugin_loader_v2/stub.rs
---

# 29cc-229 Runtime A3-min4 PluginBoxMetadata Route-Aware Lock

## Purpose

`PluginBoxMetadata` が shim invoke 関数ポインタを保持している設計をやめ、`BoxInvokeFn` を軸に route-aware なメタデータへ寄せる。

## Fixed Contract

1. `PluginBoxMetadata` は `invoke_fn: InvokeFn` を持たず、`invoke_box_fn: Option<BoxInvokeFn>` を持つ。
2. `metadata_for_type_id` は `invoke_box_fn` を `box_invoke_fn_for_type_id` で解決して格納する。
3. fail-fast 既定で unresolved route は `None` のまま保持し、上位の fail-fast 経路へ委譲する。
4. `enabled/stub` の型定義は同一形に同期する。

## Acceptance

1. `cargo check --bin hakorune` green
2. `tools/checks/dev_gate.sh runtime-exec-zero` green
3. `phase29y_no_compat_mainline_vm.sh` green

## Not in this lock

1. A3完了判定（loader/types/globals/errors/extern 全体closeout）は次境界で扱う
2. source削除（no-delete-first を維持）
