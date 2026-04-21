---
Status: Active
Date: 2026-04-22
Scope: First implementation card for moving array RMW window route legality from `.inc` to MIR metadata.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
---

# 292x-93: `array_rmw_window` Route Card

## Problem

`analyze_array_rmw_window_candidate` currently reads raw MIR JSON in C to decide
whether a method-get window can lower to a fused array RMW helper. That makes
`.inc` a planner.

## Decision

MIR must decide the route and emit a pre-decided tag. `.inc` may only consume
that tag, validate the fields it needs, emit the selected helper, and mark the
covered instructions skipped.

## Proposed Metadata

Route id:

```text
array.rmw_add1.window
```

Required fields:

- `block`
- `instruction_index`
- `skip_instruction_indices`
- `proof`
- `emit_symbol`
- `operands`
- `array_value`
- `index_value`
- `input_value`
- `const_value`
- `result_value`
- `set_instruction_index`

Initial proof vocabulary:

- `array_get_string_edit_array_set_trailing_len`
- `array_get_add1_set_same_slot`

The exact field names may follow the surrounding MIR JSON metadata style, but
the ownership rule must not change: MIR owns legality, `.inc` owns emission.

## Implementation Steps

1. Add `src/mir/array_rmw_window_plan.rs`.
   - `ArrayRmwWindowRoute`
   - `ArrayRmwWindowProof::ArrayGetAdd1SetSameSlot`
   - `refresh_function_array_rmw_window_routes(...)`
   - `refresh_module_array_rmw_window_routes(...)`
2. Add `FunctionMetadata.array_rmw_window_routes`.
3. Wire semantic refresh after value/string facts and before array-text route consumers.
4. Emit `metadata.array_rmw_window_routes` through MIR JSON.
5. Add `.inc` metadata reader, likely
   `lang/c-abi/shims/hako_llvmc_ffi_array_rmw_window_metadata.inc`.
6. Prefer metadata in `emit_generic_method_get_by_window_or_policy(...)`.
7. Keep `analyze_array_rmw_window_candidate` as fallback-only for the first
   implementation commit.
8. Add or update focused smoke to prove metadata-first route selection.
9. After coverage is stable, remove the C analyzer and prune the guard allowlist.

## Detection Shape

Accept only the narrow route:

- entry is `ArrayBox.get(i)` or a `RuntimeDataBox.get(i)` whose root is proven ArrayBox
- copy chain from the get result is allowed
- `const 1`
- `Add(carried, const1)` or reversed operand order
- same receiver / same index `set(index, add_result)`
- no uncovered use of the get/carried value after the set
- skip covers only follow-up instructions; the current get instruction is not skipped

Reject:

- non-`1` constants
- unproven `RuntimeDataBox` receivers
- post-set live uses of the get value
- helper-name or benchmark-name proof

## Verification

Required before commit:

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
git diff --check
```

Implementation commit should additionally run the focused route smoke selected
from the current `.inc` lowering path and `tools/checks/dev_gate.sh quick`
before deleting fallback code.

Targeted implementation checks:

```bash
cargo test -p nyash_rust array_rmw_window
cargo test -p nyash_rust mir_json_emit
```
