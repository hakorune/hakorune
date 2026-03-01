---
Status: Active
Decision: accepted
Date: 2026-03-01
Scope: RZ-LOADER-min1 として resolver/instance の route contract 解決を `route_resolver` へ集約する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-245-runtime-route-residue-relock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/route_resolver.rs
  - src/runtime/plugin_loader_v2/enabled/method_resolver.rs
  - src/runtime/plugin_loader_v2/enabled/instance_manager.rs
---

# 29cc-248 RZ-LOADER-min1 Route Contract Box Lock

## Purpose

`method_resolver` と `instance_manager` が持つ route 解決の重複責務を減らし、  
resolver 判定源を `route_resolver` へ寄せる。

## Decision

- `route_resolver` に契約箱を追加:
  - `MethodRouteContract { type_id, method_id, returns_result }`
  - `BirthRouteContract { type_id, birth_id, fini_id }`
- `route_resolver` に共通解決関数を追加:
  - `resolve_method_contract()`
  - `resolve_birth_contract()`
- `method_resolver::resolve_method_handle()` は `resolve_method_contract()` を使用。
- `instance_manager` は `BirthRouteContract` と `resolve_birth_contract()` を使用。
- 既定挙動（fail-fast/compat条件）は変更しない。

## Acceptance

- `cargo check --bin hakorune` green
- `tools/checks/dev_gate.sh runtime-exec-zero` green
- mainline route で compat fallback drift が発生しない
