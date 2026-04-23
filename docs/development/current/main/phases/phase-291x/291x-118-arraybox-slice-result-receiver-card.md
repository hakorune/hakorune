---
Status: Landed
Date: 2026-04-24
Scope: Pin `ArrayBox.slice()` result follow-up calls so the slice result stays on the `ArrayBox` receiver path.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
  - src/mir/builder/types/annotation.rs
  - src/mir/builder/calls/unified_emitter.rs
  - src/tests/mir_corebox_router_unified.rs
---

# ArrayBox Slice Result Receiver Card

## Decision

Close only the existing `ArrayBox.slice()` result-receiver cleanup:

```text
ArrayBox.slice(start, end)
  -> publishes Box("ArrayBox")
  -> follow-up calls on the result use ArrayBox routing
  -> no RuntimeDataBox union receiver fallback for the direct source chain
```

This is not a new Array surface row. The `slice/2` row is already cataloged and
Unified. This card only fixes or pins the receiver publication contract for
direct source follow-up calls.

## Current Facts

- `ArrayBox.slice/2` is already catalog-backed and Unified.
- `src/mir/builder/types/annotation.rs` already publishes
  `MirType::Box("ArrayBox")` for `ArrayBox.slice/2`.
- The remaining documented drift is the direct source chain where a
  `slice()` result can behave like a union receiver and route through
  `RuntimeDataBox`.
- The fix must stay in MIR type/origin publication. Runtime ArrayBox behavior,
  storage, and slice semantics are out of scope.

## Implementation Slice

- add a focused MIR test for `local s = a.slice(...); s.length()`
- assert the `slice` result publishes `Box("ArrayBox")`
- assert the follow-up `length()` call is emitted as `ArrayBox.length`
- assert no `RuntimeDataBox.length` call is emitted for that direct source chain
- if the test fails, propagate the existing `Box("ArrayBox")` publication into
  the receiver origin path without adding new surface rows

## Non-Goals

- do not add new ArrayBox methods
- do not change `slice()` runtime behavior
- do not change generic element typing for `get` / `pop` / `remove`
- do not reopen RuntimeDataBox dispatch policy broadly
- do not reopen phase-137x perf work

## Acceptance

```bash
cargo test -q array_value_slice_result_followup_uses_arraybox_receiver_path --lib
cargo test -q array_value_slice_uses_unified_receiver_arg_shape_and_array_return --lib
bash tools/checks/current_state_pointer_guard.sh
```

## Landing Snapshot

- Added a focused MIR test for direct source
  `local s = a.slice(...); local n = s.length()`.
- The test pins `ArrayBox.slice` as receiver-plus-start-plus-end Unified shape.
- The test pins the `slice` result type as `Box("ArrayBox")`.
- The test pins the follow-up call as `ArrayBox.length` and asserts no
  `RuntimeDataBox.length` fallback appears for that chain.
- No runtime, router, or catalog code change was needed; the existing type
  publication path already satisfied the contract.

## Exit Condition

A direct source `ArrayBox.slice()` result can feed a follow-up CoreBox call
without degrading the receiver family to `RuntimeDataBox`.
