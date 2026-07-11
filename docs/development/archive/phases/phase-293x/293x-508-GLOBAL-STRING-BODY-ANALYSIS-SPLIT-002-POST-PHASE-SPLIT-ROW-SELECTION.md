# 293x-508 GLOBAL-STRING-BODY-ANALYSIS-SPLIT-002 Post-Phase-Split Row Selection

Status: landed
Date: 2026-05-17

## Decision

`GLOBAL-STRING-BODY-ANALYSIS-SPLIT-001` closed the first generic string body
analysis phase split.

Select exactly one next cleanup row:

```text
NUMERIC-SUBSTRATE-SPLIT-001:
  split numeric substrate staged exact-numeric vocabulary into owner modules
  without changing numeric semantics or consumers
```

## Why This Row

The remaining large-file cleanup inventory still has
`src/mir/numeric_substrate.rs` as the largest unsplit proof-first module. It
currently owns target/width vocabulary, exact MIR numeric models, conversion
policies, checked arithmetic/compare/shift helpers, legacy inline-storage name
classification, and the full test suite in one file.

This is a BoxShape cleanup. The numeric substrate is intentionally staged and
contains `#[allow(dead_code)]` rows for future exact numeric verifier/backend
work; this row only makes the owner layout easier to navigate.

## Selected Row

```text
row:
  NUMERIC-SUBSTRATE-SPLIT-001
owner:
  src/mir/numeric_substrate.rs
  src/mir/numeric_substrate/
scope:
  introduce a submodule owner layout for target/type vocabulary, exact value
  conversion, checked ops, and tests while preserving public crate-visible APIs
stop_line:
  no exact numeric semantic changes
  no dead-code staging deletion
  no new verifier/backend/runtime behavior
  no allocator/provider behavior
evidence:
  cargo test -q numeric_substrate
  bash tools/checks/current_state_pointer_guard.sh
  tools/checks/dev_gate.sh quick
  git diff --check
```

## Stop Lines

- Do not change numeric type names, signedness/width resolution, value ranges,
  conversion errors, checked-op behavior, or inline storage classification.
- Do not remove staged `#[allow(dead_code)]` declarations as part of this split.
- Do not add exact numeric verifier, MIR JSON schema, VM/backend, allocator, or
  provider behavior.

## Closeout

This row closes when `NUMERIC-SUBSTRATE-SPLIT-001` has a selected current card
with owner, scope, stop lines, and evidence.
