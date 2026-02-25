---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X26 route observability 契約固定（`[vm-route/*]` タグ語彙と理由語彙の統一）。
Related:
  - docs/development/current/main/phases/phase-29x/29x-41-vm-route-observability-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-51-route-orchestration-single-entry-ssot.md
  - src/runner/route_orchestrator.rs
  - src/runner/mod.rs
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh
  - tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh
---

# Phase 29x X26: VM Route Observability Contract SSOT

## 0. Goal

`[vm-route/*]` タグの語彙を固定し、route 理由の観測を機械判定可能にする。
X25 で一本化した orchestrator を観測SSOTとして固定する。

## 1. Allowed Tag Vocabulary

`NYASH_VM_ROUTE_TRACE=1` のとき、許可される route タグは次のみ。

1. Pre-dispatch:
   - `[vm-route/pre-dispatch] backend=<backend> file=<path>`
2. Route select:
   - `[vm-route/select] backend=<backend> lane=<lane> reason=<reason>`

legacy 形式の `"[vm-route] pre-dispatch ..."` は禁止。

## 2. Allowed Select Reasons

`[vm-route/select]` の `reason` 語彙は次のみ。

1. `default`
2. `strict-dev-prefer`
3. `env:NYASH_VM_USE_FALLBACK=1`
4. `backend:vm-hako`

## 3. Ownership

route タグ生成責務は `src/runner/route_orchestrator.rs` のみが持つ。

- `format_vm_route_pre_dispatch`
- `format_vm_route_select`
- `emit_vm_route_pre_dispatch`
- `execute_vm_route`

`dispatch.rs` / `mod.rs` / `selfhost.rs` に
route タグ文字列の直書きを再導入しない。

## 4. Evidence (X26)

1. `cargo check -q --bin hakorune`
2. `cargo test -q route_orchestrator -- --nocapture`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh`
5. `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`

## 5. Next Step

X27 は compat bypass の fail-fast 境界を固定し、
strict/dev で暗黙 fallback が 0 であることを守る。
