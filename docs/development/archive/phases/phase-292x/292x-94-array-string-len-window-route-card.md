---
Status: Landed
Date: 2026-04-22
Scope: A2a implementation card for moving the simple array string length window from `.inc` analysis to MIR metadata.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
  - docs/development/current/main/phases/phase-292x/292x-93-array-rmw-window-route-card.md
---

# 292x-94: `array_string_len_window` Route Card

## Problem

`analyze_array_string_len_window_candidate` still reads raw MIR JSON in C to
decide whether `array.get(row).length()` can lower to the direct
`nyash.array.string_len_hi` helper. That keeps `.inc` as a planner for one of
the hot array/string read windows.

## Decision

MIR must own the legality proof for the simple len-only window and emit a
pre-decided route tag. `.inc` may only validate that tag, emit the selected
helper, skip the covered follow-up instructions, or fail fast on malformed
metadata.

Complex reuse modes stay fallback-only in this card:

- `keep_get_live` windows that must also materialize the array slot
- `source_only_insert_mid` / piecewise concat windows that reuse the get source

Those modes need separate metadata fields because they change publication and
slot-load behavior. Keeping them out of A2a avoids copying the whole legacy C
analyzer into a new Rust file.

## Proposed Metadata

Route id:

```text
array.string_len.window
```

Required fields:

- `route_id`
- `block`
- `instruction_index`
- `array_value`
- `index_value`
- `source_value`
- `len_instruction_index`
- `len_value`
- `skip_instruction_indices`
- `mode`
- `proof`
- `emit_symbol`
- `effects`

Initial mode vocabulary:

- `len_only`

Initial proof vocabulary:

- `array_get_len_no_later_source_use`

## Detection Shape

Accept only the narrow route:

- entry is `ArrayBox.get(i)` or a `RuntimeDataBox.get(i)` whose root is proven
  ArrayBox
- copy chain from the get result is allowed
- next non-copy instruction is `length` / `len` / `size` on the same value root
- no uncovered use of the get/carried source after the length instruction
- skip covers only follow-up copy/length instructions; the current get
  instruction is not skipped

Reject:

- unproven `RuntimeDataBox` receivers
- post-len uses of the source value
- source-reuse modes that need a slot load or source publication
- helper-name or benchmark-name proof

## Implementation Steps

1. [x] Extract shared array receiver proof so route planners do not duplicate
   RuntimeDataBox -> ArrayBox provenance rules.
2. [x] Add `src/mir/array_string_len_window_plan.rs`.
   - `ArrayStringLenWindowRoute`
   - `ArrayStringLenWindowMode::LenOnly`
   - `ArrayStringLenWindowProof::ArrayGetLenNoLaterSourceUse`
   - `refresh_function_array_string_len_window_routes(...)`
   - `refresh_module_array_string_len_window_routes(...)`
3. [x] Add `FunctionMetadata.array_string_len_window_routes`.
4. [x] Wire semantic refresh beside `array_rmw_window_routes`.
5. [x] Emit `metadata.array_string_len_window_routes` through MIR JSON.
6. [x] Add `.inc` metadata reader in
   `lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_window.inc`.
7. [x] Prefer metadata in `emit_generic_method_get_by_window_or_policy(...)`.
8. [x] Keep `analyze_array_string_len_window_candidate` as fallback-only for
   non-len-only modes.
9. [ ] After the reuse modes are covered by metadata, remove the C analyzer and
   prune the guard allowlist.

## Verification

Required before commit:

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
cargo test -q array_string_len_window
cargo test -q build_mir_json_root_emits_array_string_len_window_routes
bash tools/build_hako_llvmc_ffi.sh
git diff --check
```

Focused route proof:

```text
array_string_len_window_routes >= 1 for benchmarks/bench_kilo_leaf_array_string_len.hako
[llvm-route/trace] stage=array_string_len_window result=hit reason=mir_route_metadata
```

Landed evidence:

```text
array_string_len_window_routes = 1 for benchmarks/bench_kilo_leaf_array_string_len.hako
[llvm-route/trace] stage=array_string_len_window result=hit reason=mir_route_metadata extra=ii=8 len_ii=11 len_dst=62 source_only_insert_mid=0 keep_get_live=0 skip_n=3 proof=array_get_len_no_later_source_use
```
