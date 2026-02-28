# NyRT Core C ABI v0 (Runtime Boundary)

Updated: 2026-02-13

This document defines the runtime-side C ABI lane.

Important boundary:
- Plugin method dispatch is not defined here.
- Plugin dispatch uses TypeBox ABI v2 (`docs/reference/plugin-abi/nyash_abi_v2.md`).

See also:
- `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`
- `docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md`

## 1. Scope

Core C ABI covers:

1. Runtime route entrypoints (bootstrap/load/execute)
2. Runtime verifier/safety gate entrypoints
3. Plugin -> host reverse-call entrypoints
4. Handle lifecycle exports used by lifecycle contract (`borrowed args / owned return`)

## 2. Canonical Headers

### `include/nyrt.h`

Current v0 header provides minimal runtime scaffold:

- `nyrt_init()`
- `nyrt_teardown()`
- `nyrt_load_mir_json(const char* json_text)`
- `nyrt_exec_main(uint64_t module_handle)`
- `nyrt_verify_mir_json(const char* json_text)`
- `nyrt_safety_check_mir_json(const char* json_text)`
- `nyrt_hostcall(...)`

### `include/nyrt_host_api.h`

Host reverse-call ABI for plugins:

- `nyrt_host_call_name(...)`
- `nyrt_host_call_slot(...)` (preferred stable call path)

TLV values are used at this boundary.

## 3. Lifecycle Extension Symbols

Lifecycle-specific handle operations are currently exported from NyRT kernel FFI:

- `nyrt_handle_retain_h(i64) -> i64`
- `nyrt_handle_release_h(i64) -> void`

Implementation reference:
- `crates/nyash_kernel/src/ffi/lifecycle.rs`

Semantic contract reference:
- `docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md`

## 4. Runtime V0 Helper Slice (execution-path-zero)

execution-path-zero cutover では、以下 3 語彙を固定する。

1. `string_len`
2. `array_get_i64`
3. `array_set_i64`

Ownership contract:

1. `args borrowed / return owned` を維持する。
2. 失敗は strict/dev で fail-fast とし、silent fallback を許可しない。

Entry lock:

1. `lang/src/runtime/collections/string_core_box.hako` (`string_len`)
2. `lang/src/runtime/collections/array_core_box.hako` (`array_get_i64`, `array_set_i64`)

Detailed SSOT:

- `docs/development/current/main/phases/phase-29cc/29cc-216-runtime-v0-abi-slice-lock-ssot.md`
- `docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md`

## 5. Compatibility Policy

1. Keep C ABI signatures stable in v0 lane.
2. Breaking changes require new symbol versioning (`*_v1`, etc.).
3. Never move plugin method semantics into Core C ABI; keep them in TypeBox ABI.

## 6. Non-goals

- Defining TypeBox plugin dispatch wire protocol (belongs to TypeBox ABI v2).
- Defining GC algorithm details (only lifecycle boundary contracts are fixed here).
