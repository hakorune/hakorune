# NyRT Core C ABI v0 (Runtime Boundary)

Updated: 2026-02-13

This document defines the runtime-side C ABI lane.

Important boundary:
- Plugin method dispatch is not defined here.
- Plugin dispatch uses TypeBox ABI v2 (`docs/reference/plugin-abi/nyash_abi_v2.md`).

See also:
- `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`
- `docs/reference/abi/nyrt_host_surface_v0.md`
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

### `include/nyrt_dynamic_v2_lease_v1.h`

The selected Boundary AOT lane uses this versioned C calling-convention
projection for the physical `CheckedCallOutEnd` cutpoint:

- `nyrt_dynamic_v2_lease_consume_end_authorized_v1(uint64_t) -> uint32_t`
- `0`: the existing Rust one-shot lease was consumed;
- `1`: zero/invalid token;
- `2`: unknown or already-consumed token;
- `3`: stale handle identity.

The header owns only fixed-width ABI/status vocabulary. The lease table,
generation check, and handle release remain solely in
`runtime::dynamic_v2_lease`. Boundary lowering treats a non-zero status as a
backend contract failure, not as a semantic Fault or fallback. This is a
projection for the static Boundary lane, not a second lifecycle authority.

### `include/nyrt_dynamic_call_slot_v2.h` and `include/nyrt_dynamic_text_scan_v1.h`

The selected Boundary AOT CheckedCallOut lane uses the versioned CallSlot
wire and TextScan entry declarations. The headers own fixed-width transport,
entry, ABI, and wire vocabulary only:

- `hako.text.scan.substring.v1` has logical arguments `(receiver, start, end)`
  and produces an EndAuthorized host-handle result;
- `hako.text.scan.index_of.v1` has logical arguments `(receiver, needle)` and
  produces an ImmediateI64 result with no lease;
- both entries write semantic Normal/Fault and payload/disposition/lease data
  to `HakoDynamicV2CallOutV1`; the `uint32_t` return is transport status.

The canonical MIR `CheckedCallOut` plan/census owns site identity, CFG
successors, Normal projection, and End chronology. The C physicalizer consumes
that site-id projection and emits direct calls plus local trap paths for
malformed transport, wire, or lease status. It does not choose a provider,
reconstruct a site from block coordinates, or turn a backend contract failure
into semantic Fault/fallback. Link, artifact validation, and live publication
remain later W6 transactions.

## 3. Lifecycle Extension Symbols

Lifecycle-specific handle operations are currently exported from NyRT kernel FFI:

- `nyrt_handle_retain_h(i64) -> i64`
- `nyrt_handle_release_h(i64) -> void`

Implementation reference:
- `crates/nyash_kernel/src/ffi/lifecycle.rs`

Semantic contract reference:
- `docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md`

## 4. Runtime V0 Helper Slice (execution-path-zero)

execution-path-zero cutover では、以下 4 語彙を固定する。

1. `string_len`
2. `array_get_i64`
3. `array_set_i64`
4. `map_size_i64`

Ownership contract:

1. `args borrowed / return owned` を維持する。
2. 失敗は strict/dev で fail-fast とし、silent fallback を許可しない。

Entry lock:

1. `lang/src/runtime/collections/string_core_box.hako` (`string_len`)
2. `lang/src/runtime/collections/array_core_box.hako` (`array_get_i64`, `array_set_i64`)
3. `lang/src/runtime/collections/map_core_box.hako` (`map_size_i64`)

Detailed SSOT:

- `docs/development/current/main/phases/phase-29cc/29cc-216-runtime-v0-abi-slice-lock-ssot.md`
- `docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md`

## 4.1 Host Surface Lock (Step-1)

Core C ABI host-facing symbols are fixed by category in:
- `docs/reference/abi/nyrt_host_surface_v0.md`

Rule:
1. Host layer only provides bridge/bootstrap/lifecycle primitives.
2. Runtime/plugin semantic policy must stay in `.hako` side.

## 5. Compatibility Policy

1. Keep C ABI signatures stable in v0 lane.
2. Breaking changes require new symbol versioning (`*_v1`, etc.).
3. Never move plugin method semantics into Core C ABI; keep them in TypeBox ABI.

## 6. Non-goals

- Defining TypeBox plugin dispatch wire protocol (belongs to TypeBox ABI v2).
- Defining GC algorithm details (only lifecycle boundary contracts are fixed here).
