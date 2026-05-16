# 293x-509 NUMERIC-SUBSTRATE-SPLIT-001

Status: selected current
Date: 2026-05-17

## Decision

`NUMERIC-SUBSTRATE-SPLIT-001` is a BoxShape cleanup for the staged numeric
substrate. It splits `src/mir/numeric_substrate.rs` into smaller owner modules
without changing exact numeric semantics, verifier behavior, runtime behavior,
or backend behavior.

## Scope

- Keep `src/mir/numeric_substrate.rs` as the public crate-visible facade.
- Add a submodule directory under `src/mir/numeric_substrate/`.
- Move coherent groups behind the facade:
  - target/width/type-name vocabulary
  - exact numeric MIR value/signature models and conversion helpers
  - checked arithmetic/compare/shift policies
  - test owner if needed to keep production files thin
- Preserve all existing crate-visible function/type names unless a local
  re-export is needed to keep consumers stable.

## Stop Lines

- Do not change accepted numeric type names or pointer-width resolution.
- Do not change exact numeric value ranges, conversion errors, checked
  arithmetic, compare, shift behavior, or inline i64 storage classification.
- Do not remove staged `#[allow(dead_code)]` rows; their future-row comments are
  part of the staging contract.
- Do not add exact numeric verifier/CorePlan/backend/runtime behavior.
- Do not touch allocator/provider activation, hooks, host allocator replacement,
  or `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `NUM.1` | Add the numeric substrate submodule owner layout. | facade still compiles. | no API rename |
| `NUM.2` | Move target/type vocabulary behind the facade. | focused tests pass. | no type-name behavior change |
| `NUM.3` | Move exact value/conversion and checked-op helpers. | focused tests pass. | no semantic change |
| `NUM.4` | Move or contain tests to keep production files navigable. | focused tests pass. | no test weakening |
| `NUM.5` | Verify and close out. | required evidence is green. | no adjacent cleanup |

## Required Evidence

```text
cargo test -q numeric_substrate
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Closeout

This row closes when the numeric substrate has a smaller owner layout and all
existing numeric substrate behavior remains unchanged.
