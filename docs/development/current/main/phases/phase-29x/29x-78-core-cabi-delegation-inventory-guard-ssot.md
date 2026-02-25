---
Status: Active
Decision: provisional
Date: 2026-02-13
Scope: X51 の Core C ABI delegation inventory を固定し、非canonical owner への混入を guard で fail-fast する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-56-thin-rust-core-cabi-min-surface-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - tools/checks/phase29x_core_cabi_delegation_allowlist.txt
  - tools/checks/phase29x_core_cabi_delegation_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_core_cabi_delegation_guard_vm.sh
---

# 29x-78: Core C ABI Delegation Inventory Guard (SSOT)

## 0. Goal

- Core C ABI minimal symbols の owner を canonical 2ファイルに固定する。
- 非canonical owner（別src/include）への混入を guard で fail-fast 検出する。

## 1. Canonical delegation owner

Allowlist SSOT:
- `include/nyrt.h`
- `src/abi/nyrt_shim.rs`

Symbols:
1. `nyrt_load_mir_json`
2. `nyrt_exec_main`
3. `nyrt_verify_mir_json`
4. `nyrt_safety_check_mir_json`
5. `nyrt_handle_retain_h`
6. `nyrt_handle_release_h`

## 2. Contract

1. 上記 6 symbol は `src/include` 内で allowlist owner 以外へ現れてはならない。
2. `abi_lane_guard` と `nyrt_core_cabi_surface_guard` を前提チェックとして通す。
3. 逸脱時は guard が即 fail-fast する（silent pass しない）。

## 3. Evidence command

- `bash tools/checks/phase29x_core_cabi_delegation_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_core_cabi_delegation_guard_vm.sh`

