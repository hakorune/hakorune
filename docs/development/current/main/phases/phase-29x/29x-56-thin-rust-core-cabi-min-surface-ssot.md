---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X30 thin-rust Core C ABI 最小面固定（route / verify / safety / lifecycle）。
Related:
  - docs/development/current/main/phases/phase-29x/29x-50-thin-rust-boundary-lock-ssot.md
  - docs/reference/abi/nyrt_c_abi_v0.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - include/nyrt.h
  - src/abi/nyrt_shim.rs
  - tools/checks/nyrt_core_cabi_surface_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_core_cabi_surface_guard_vm.sh
---

# Phase 29x X30: Thin-Rust Core C ABI Minimal Surface SSOT

## 0. Goal

X24-X29 で一本化した route/verifier/safety 責務を、
Core C ABI 側でも最小面として固定する。
header / shim / docs の三点同期を必須化し、語彙ズレを fail-fast で検知する。

## 1. Canonical Minimal Surface

`include/nyrt.h` / `src/abi/nyrt_shim.rs` / `docs/reference/abi/nyrt_c_abi_v0.md`
で次の symbol を一致させる。

Route:
1. `nyrt_load_mir_json(const char*)`
2. `nyrt_exec_main(uint64_t)`

Verifier/Safety:
3. `nyrt_verify_mir_json(const char*)`
4. `nyrt_safety_check_mir_json(const char*)`

Lifecycle:
5. `nyrt_handle_retain_h(int64_t)`
6. `nyrt_handle_release_h(int64_t)`

## 2. Contract

1. 6 symbol は header/shim/doc の3面で一致する。
2. lifecycle zero-handle contract は shim test で固定する。
3. verifier/safety gate symbol は null input を reject（non-zero rc）する。

## 3. Guard

`tools/checks/nyrt_core_cabi_surface_guard.sh` で
header/shim/doc の symbol 同期を機械判定する。

## 4. Evidence (X30)

1. `cargo check -q --bin hakorune`
2. `cargo test -q nyrt_shim -- --nocapture`
3. `bash tools/checks/nyrt_core_cabi_surface_guard.sh`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29x_core_cabi_surface_guard_vm.sh`
5. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_safety_gate_single_entry_vm.sh`
6. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_verifier_gate_single_entry_vm.sh`

## 5. Next Step

X31 で thin-rust gate pack（X24-X30）の統合 smoke/evidence を固定する。
