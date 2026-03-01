---
Status: Active
Decision: accepted
Date: 2026-03-01
Scope: RZ-LOADER-min4 として loader/types/bridge の route 判定を route_resolver 契約へ再利用統一する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-250-rz-loader-min3-compat-ffi-branch-isolation-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/route_resolver.rs
  - src/runtime/plugin_loader_v2/enabled/loader/singletons.rs
  - src/runtime/plugin_loader_v2/enabled/instance_manager.rs
  - src/runtime/plugin_loader_v2/enabled/ffi_bridge.rs
  - src/runtime/plugin_loader_v2/enabled/types.rs
---

# 29cc-251 RZ-LOADER-min4 Loader/Types Route Reuse Lock

## Purpose

`RZ-LOADER-min1..min3` で分離した route 契約を `loader/*` と `types` まで再利用し、
mainline の route 判定源を `route_resolver` へ寄せる。

## Decision

- `route_resolver` に `resolve_birth_contract_for_lib()` を追加し、
  singleton prebirth の `type_id/birth/fini` 解決を単一契約へ固定。
- `loader/singletons.rs`:
  - `resolve_type_info + lib一致検証` の二段判定を撤去。
  - invoke route を `resolve_invoke_route_contract()` で取得して呼び出し。
- `instance_manager.rs`:
  - `build_plugin_box_handle` の `invoke_box_fn` を global lookup ではなく
    `resolve_invoke_route_contract()` 経由へ統一。
- `ffi_bridge.rs`:
  - TLV decode（plugin-handle復元）の `invoke_box_fn` 解決を
    `resolve_invoke_route_contract()` 経由へ統一。
  - metadata 参照を global helper ではなく loader instance 経由へ統一。
- `types.rs`:
  - `make_plugin_box_v2` / `construct_plugin_box` の `invoke_box_fn` 解決を
    global loader + `route_resolver` 契約経由へ統一。

## Acceptance

- `cargo check --bin hakorune` green
- `cargo test route_resolver::tests -- --nocapture` green
- `tools/checks/dev_gate.sh runtime-exec-zero` green
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh` green

