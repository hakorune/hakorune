# ABI Boundary Matrix (SSOT)

Updated: 2026-02-13

This document fixes ABI ownership boundaries for the current runtime/plugin line.

## 1. Canonical ABI Surfaces

Only two ABIs are canonical:

1. Core C ABI (NyRT line)
2. TypeBox ABI v2 (plugin method dispatch line)

`hako_abi_v1` is not a third canonical ABI. It remains a design-first draft only.

## 2. Boundary Matrix

| Boundary | ABI | Canonical docs/headers | Scope | Status |
| --- | --- | --- | --- | --- |
| Runtime bootstrap/load/exec | Core C ABI | `docs/reference/abi/nyrt_c_abi_v0.md`, `include/nyrt.h` | `init/teardown/load_mir_json/exec_main` | Active |
| Runtime verifier/safety gates | Core C ABI | `docs/reference/abi/nyrt_c_abi_v0.md`, `include/nyrt.h` | `verify_mir_json/safety_check_mir_json` | Active |
| Plugin -> host reverse call | Core C ABI | `include/nyrt_host_api.h`, `docs/development/abi/host_api.md` | host handle call by `name` / stable `slot` | Active |
| Handle lifecycle (retain/release) | Core C ABI | `docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md`, `docs/development/current/main/phases/phase-29x/29x-86-abi-borrowed-owned-conformance-extension-ssot.md`, `crates/nyash_kernel/src/ffi/lifecycle.rs` | `borrowed/owned` contract, strong lifecycle ops + matrix conformance cases | Active |
| Runtime V0 helper slice (`string_len`, `array_get_i64`, `array_set_i64`) | Core C ABI | `docs/development/current/main/phases/phase-29cc/29cc-216-runtime-v0-abi-slice-lock-ssot.md`, `docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md`, `lang/src/runtime/collections/array_core_box.hako` | execution-path-zero 向けの最小語彙固定（Step-1） | Active |
| Plugin Box method dispatch | TypeBox ABI v2 | `docs/reference/plugin-abi/nyash_abi_v2.md`, `include/nyash_abi.h` | per-Box `resolve + invoke_id(TLV)` | Active |
| Basic Box direct C facade (`hako_str_*`, `hako_arr_*`, `hako_map_*`) | `hako_abi_v1` draft | `dist/0.1.0-linux-x86_64/include/hako_abi_v1.h` | design proposal only | Non-canonical |

## 3. Rules

1. Runtime lifecycle/host route changes go to Core C ABI.
2. Plugin method call surface changes go to TypeBox ABI v2.
3. Do not add new production symbols to `hako_abi_v1`.
4. If a facade is needed for migration, generate it from Core C ABI and TypeBox ABI; do not create a new semantic ABI.

## 4. Verification Hints

- `bash tools/checks/abi_lane_guard.sh`
  - Expected: `[abi-lane-guard] ok`
- `rg -n "\\bhako_(str|arr|map)_" src crates lang include -S`
  - Expected: no production implementation bindings in current source tree.
- `rg -n "nyrt_host_call_slot|nyrt_verify_mir_json|nyrt_safety_check_mir_json|nyrt_handle_retain_h|nyash_typebox_" src crates include -S`
  - Expected: Core C ABI and TypeBox ABI call points are present.
- `bash tools/checks/phase29x_abi_borrowed_owned_matrix_guard.sh`
  - Expected: `[abi-borrowed-owned-matrix-guard] ok`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_abi_borrowed_owned_conformance_vm.sh`
  - Expected: X51 precondition + borrowed/owned matrix conformance are replayable in one gate.
- `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
  - Expected: `[runtime-v0-abi-slice-guard] ok`
