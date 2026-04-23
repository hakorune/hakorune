---
Status: Landed
Date: 2026-04-24
Scope: Publish `ArrayBox.get` / `pop` / `remove` element result types when the receiver carries a known `Array<T>` MIR type.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
---

# ArrayBox Element Result Publication Card

## Decision

Promote only the typed element-result publication slice:

```text
ArrayBox.get(index)    -> T when receiver is Array<T>
ArrayBox.pop()         -> T when receiver is Array<T>
ArrayBox.remove(index) -> T when receiver is Array<T>
```

If the receiver is only known as `Box("ArrayBox")`, mixed, empty, or otherwise
data-dependent, the MIR result stays `Unknown`.

This keeps the existing runtime and router contracts intact. The new truth is
not a global "ArrayBox.get always returns T" claim; it is a local MIR
publication rule that consumes existing receiver type facts.

## Preconditions

- `ArrayBox.get`, `pop`, and `remove` already route through the catalog-backed
  Unified value path.
- `src/boxes/array/surface_catalog.rs` marks those rows as value-returning.
- `src/mir/builder/types/annotation.rs` intentionally returns `None` for
  those rows unless a receiver-local element type is known.
- `MirType::Array(Box<MirType>)` already exists as the MIR carrier for typed
  arrays.

## Implementation Slice

- add a small Array element publication helper in the MIR builder type layer
- record homogeneous local ArrayBox element facts from:
  - array literals
  - `ArrayBox.push(value)`
  - `ArrayBox.set(index, value)`
  - `ArrayBox.insert(index, value)`
- annotate `get` / `pop` / `remove` call results from receiver
  `MirType::Array(T)`
- preserve `Unknown` for mixed or untyped receivers
- add focused MIR tests for typed and mixed ArrayBox element-result behavior

## Non-Goals

- do not change ArrayBox router policy
- do not change runtime `ArrayBox` storage or VM dispatch
- do not introduce public generic syntax
- do not change `MapBox.get(existing-key)` typing
- do not treat unknown or mixed arrays as typed
- do not reopen phase-137x perf work

## Acceptance

```bash
cargo test -q array_value_get_uses_unified_receiver_arg_shape_and_element_return --lib
cargo test -q array_value_pop_uses_unified_receiver_arg_shape_and_element_return --lib
cargo test -q array_value_remove_uses_unified_receiver_arg_shape_and_element_return --lib
cargo test -q array_value_mixed_element_results_stay_unknown --lib
cargo test -q corebox_surface_aliases_use_catalog_return_type --lib
bash tools/checks/current_state_pointer_guard.sh
```

## Landing Snapshot

- `ArrayBox.push/set/insert` now record homogeneous receiver-local element facts
  before LocalSSA receiver materialization.
- `ArrayBox.get/pop/remove` publish `T` only when the finalized receiver carries
  `MirType::Array(T)`.
- Mixed or untyped receivers keep the previous `Unknown` result contract.

## Exit Condition

Homogeneous local ArrayBox values publish `get` / `pop` / `remove` as the known
element type, while mixed or untyped ArrayBox receivers keep the previous
`Unknown` result contract.
