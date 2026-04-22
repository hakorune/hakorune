---
Status: Landed
Date: 2026-04-22
Scope: A2b implementation card for moving `array_string_len_window` keep-live reuse from `.inc` analysis to MIR metadata.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
  - docs/development/current/main/phases/phase-292x/292x-94-array-string-len-window-route-card.md
---

# 292x-95: `array_string_len_window` Keep-Live Route Card

## Problem

A2a moved the narrow len-only `array.get(i) -> length` route into MIR
metadata, but `analyze_array_string_len_window_candidate` still owns the
live-source case in C:

```text
array.get(i) -> copy* -> length
                         -> later substring(source, ...)
```

That case must emit both the direct length helper and a slot load for the
still-live source. Keeping the decision in `.inc` leaves C as the route
planner for a common array/string boundary.

## Decision

MIR owns the safe substring reuse proof and emits a keep-live route tag.
`.inc` consumes the tag only to validate fields, materialize the slot, emit
the selected length helper, skip the length instruction, and fail fast on
malformed metadata.

`source_only_insert_mid` and piecewise concat direct-set reuse stay out of
this card. They need their own proof vocabulary because they publish a source
reference for a later store shape rather than merely keeping a later substring
consumer alive.

## Metadata Vocabulary

Route id:

```text
array.string_len.window
```

Mode added by this card:

```text
keep_get_live
```

Proof added by this card:

```text
array_get_len_keep_source_live
```

Required mode-specific fields:

- `keep_get_live: true`
- `source_only_insert_mid: false`
- `effects: ["load.cell", "observe.len", "keep.source.live"]`

Skip rule:

- `len_only`: skip copy chain plus length instruction
- `keep_get_live`: skip only the length instruction; copy chain stays live
  because later source consumers may depend on it

## Detection Shape

Accept:

- entry is `ArrayBox.get(i)` or a `RuntimeDataBox.get(i)` whose root is proven
  ArrayBox
- copy chain from the get result is allowed
- next non-copy instruction is `length` / `len` / `size` on the same value root
- every later source-root use is a `RuntimeDataBox.substring(...)` receiver
- no later source-root use is a generic copy, concat, set, or unknown call

Reject:

- source-only insert-mid / piecewise concat direct-set optimization claims
- source uses that would require `.inc` to rediscover store shape
- helper-name or benchmark-name proof

## Implementation Steps

1. [x] Extend `ArrayStringLenWindowMode` with `KeepGetLive`.
2. [x] Extend `ArrayStringLenWindowProof` with
   `ArrayGetLenKeepSourceLive`.
3. [x] Teach the MIR route planner to classify later safe substring reuse.
4. [x] Emit mode-specific JSON effects and booleans.
5. [x] Teach `.inc` metadata lowering to emit slot-load + length for
   `keep_get_live` without running the C analyzer.
6. [x] Update the live-after-get boundary smoke to require
   `reason=mir_route_metadata`.
7. [x] Leave source-only direct-set fixtures on legacy fallback until A2c.

## Verification

Required before commit:

```bash
cargo test -q array_string_len_window
cargo test -q build_mir_json_root_emits_array_string_len_window_routes
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_len_live_after_get_min.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
git diff --check
```

Focused route proof:

```text
[llvm-route/trace] stage=array_string_len_window result=hit reason=mir_route_metadata ... keep_get_live=1
```

Landed evidence:

```text
benchmarks/bench_kilo_meso_substring_concat_array_set.hako
  -> metadata.array_string_len_window_routes[0].mode = keep_get_live
  -> proof = array_get_len_keep_source_live
  -> skip_instruction_indices = [len_instruction_index]

apps/tests/mir_shape_guard/array_string_len_live_after_get_min_v1.mir.json
  -> phase29ck boundary smoke requires reason=mir_route_metadata,
     keep_get_live=1, and proof=array_get_len_keep_source_live
```
