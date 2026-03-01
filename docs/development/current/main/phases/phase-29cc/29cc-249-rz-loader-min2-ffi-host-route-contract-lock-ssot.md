---
Status: Active
Decision: accepted
Date: 2026-03-01
Scope: RZ-LOADER-min2 として ffi_bridge / host_bridge の invoke route 解決を route_resolver 契約箱経由へ統一する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-248-rz-loader-min1-route-contract-box-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/route_resolver.rs
  - src/runtime/plugin_loader_v2/enabled/ffi_bridge.rs
  - src/runtime/plugin_loader_v2/enabled/instance_manager.rs
  - src/runtime/plugin_loader_v2/enabled/host_bridge.rs
---

# 29cc-249 RZ-LOADER-min2 FFI/Host Route Contract Lock

## Purpose

`ffi_bridge` / `instance_manager` が個別に持っていた invoke route 解決を縮退し、  
`route_resolver` の契約箱で判定源を統一する。

## Decision

- `route_resolver` に `InvokeRouteContract` を追加:
  - `invoke_box_fn`
  - `invoke_shim_fn`
- `resolve_invoke_route_contract(loader, type_id)` を追加。
- `ffi_bridge::invoke_instance_method()` は:
  - `resolve_method_contract()` で type/method/lib を一括解決
  - `resolve_invoke_route_contract()` で invoke 経路を取得
  - `host_bridge::invoke_alloc_with_route()` 呼び出しへ統一
- `instance_manager` の birth invoke も `resolve_invoke_route_contract()` を使用。
- fail-fast/compat 挙動は変更しない。

## Acceptance

- `cargo check --bin hakorune` green
- `tools/checks/dev_gate.sh runtime-exec-zero` green
- `phase29y_no_compat_mainline_vm.sh` green
