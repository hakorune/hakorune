---
Status: ready__2026-09-22__WarningGenericG0ParametersAccessor__Fast
Task: MIRBUILDER-WARNING-GENERIC-G0-PARAMETERS-ACCESSOR-I131
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i130-2026-09-22.md
Implementation permission: true for deleting the production-zero accessor and updating its owner-local test
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I132
---

# Warning cleanup: Generic G0 numeric lease parameter accessor

## Six-line brief

```text
Decision: delete VerifiedGenericNumericFactLeaseG0::parameters(); the stored
  parameter facts and numeric issuer remain unchanged.
Source authority + canonical issuer: the existing Generic G0 numeric fact
  lease issuer and its private lease fields.
Non-authority: warning text alone, a replacement semantic product, AST data,
  type inference, or a new numeric projection.
Fail-fast boundary: any non-test caller, focused red, changed issue/reject
  result, or new warning cancels the slice.
Smallest next slice: remove exactly one accessor and change its owner-local
  test to inspect the retained field directly.
Non-claims: no numeric contract change, source admission, loop route, backend,
  fallback, old-edge retirement, or warning suppression.
```

## Census and delete set

The selected warning is emitted for
`src/mir/numeric_substrate/generic_g0/mod.rs:67`:

```text
VerifiedGenericNumericFactLeaseG0::parameters
```

Repository-wide search across `src`, `tests`, `tools`, `lang`, `apps`, and
tracked current docs found one reference, the owner-local test at
`src/mir/numeric_substrate/generic_g0/tests.rs:47`. No production caller,
guard, or external projection calls the accessor. The `parameters` field,
`literals` accessor, numeric issue taxonomy, and issuer remain in use and are
outside the delete set. The test keeps its cardinality assertion by reading
the retained private field from the descendant test module.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib mir::numeric_substrate::generic_g0
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

The focused filter must execute nonzero named tests and pass. Record the new
lib/lib-test warning counts without suppression. Confirm the accessor is absent
and the field/issuer remain present.

## Non-claims and stop rule

This is a behavior-preserving warning cleanup. If direct field access violates
the module privacy boundary, or if a production caller appears, stop and return
to design_stop; do not add a replacement accessor or widen visibility.
