---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X27 compat bypass fail-fast 化（暗黙 fallback 入口の遮断）。
Related:
  - docs/development/current/main/phases/phase-29x/29x-52-vm-route-observability-contract-ssot.md
  - src/runner/route_orchestrator.rs
  - src/runner/modes/vm_fallback.rs
  - tools/checks/vm_route_bypass_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_compat_bypass_guard_vm.sh
---

# Phase 29x X27: VM Route Compat Bypass Fail-Fast SSOT

## 0. Goal

strict/dev で暗黙 fallback を 0 に保つため、compat fallback 実行入口を
`route_orchestrator` 経由に限定し、bypass を fail-fast で遮断する。

## 1. Contract

1. `execute_vm_fallback_interpreter` の呼び出し元は
   `src/runner/route_orchestrator.rs` のみ許可。
2. `execute_vm_fallback_interpreter` 入口で
   `enforce_vm_compat_fallback_guard_or_exit("vm-fallback")` を必須化。
3. `NYASH_VM_USE_FALLBACK=1` が無い状態で fallback 実行入口に到達した場合、
   次の fail-fast タグで停止する:
   - `[freeze:contract][vm-route/compat-bypass] route=vm-fallback require=NYASH_VM_USE_FALLBACK=1`

## 2. Guard Ownership Check

`tools/checks/vm_route_bypass_guard.sh` で次を機械判定する。

1. `execute_vm_fallback_interpreter(` の callsite が
   `route_orchestrator.rs` / `vm_fallback.rs` 以外に存在しない
2. `vm_fallback.rs` に guard hook が存在する

## 3. Evidence (X27)

1. `cargo check -q --bin hakorune`
2. `cargo test -q route_orchestrator -- --nocapture`
3. `bash tools/checks/vm_route_bypass_guard.sh`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_compat_bypass_guard_vm.sh`
5. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh`
6. `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`

## 4. Next Step

X28 で verifier gate の入口を一本化し、backend間の verify 重複を排除する。
