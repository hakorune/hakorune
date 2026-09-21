---
Status: closed__2026-09-22__WarningGenericG0ParametersAccessor__DeletedFromProductionBuild
Task: MIRBUILDER-WARNING-GENERIC-G0-PARAMETERS-ACCESSOR-I131
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i130-2026-09-22.md
Implementation permission: true for restricting the production-zero accessor to test builds and preserving its tests
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I132
---

# Warning cleanup: Generic G0 numeric lease parameter accessor

## Six-line brief

```text
Decision: compile VerifiedGenericNumericFactLeaseG0::parameters() only for
  tests; the stored parameter facts and numeric issuer remain unchanged.
Source authority + canonical issuer: the existing Generic G0 numeric fact
  lease issuer and its private lease fields.
Non-authority: warning text alone, a replacement semantic product, AST data,
  type inference, or a new numeric projection.
Fail-fast boundary: any non-test caller, focused red, changed issue/reject
  result, or new warning cancels the slice.
Smallest next slice: add a test-only cfg boundary to the accessor and move the
  cross-module projection test to its existing source-parameter evidence.
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
tracked current docs found two test references: the owner-local test at
`src/mir/numeric_substrate/generic_g0/tests.rs:47` and the compiler projection
test at `src/mir/compiler/generic_g0_numeric_projection_tests.rs:49`. No
production caller, guard, or external production projection calls the
accessor. The `parameters` field, `literals` accessor, numeric issue taxonomy,
and issuer remain in use and are outside the delete set. The owner-local test
retains the accessor under `cfg(test)`; the cross-module test checks the
already-issued source-parameter inventory instead of requiring a production
accessor.

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
from the non-test build and the field/issuer remain present.

## Non-claims and stop rule

This is a behavior-preserving warning cleanup. If a production caller appears,
stop and return to design_stop; do not widen visibility or add a replacement
production accessor.

## Closeout evidence

The accessor now has `#[cfg(test)]`, so it is absent from the production lib
build while the owner-local test remains intact. The projection test now checks
the existing source-parameter inventory and does not require a production
accessor. `cargo fmt --all -- --check` passed; the focused Generic G0 filter
passed **6/6**; `cargo check --profile quick --lib -j4` passed with lib
**1,689** warnings; `cargo test --profile quick --lib --no-run -j4` passed
with lib-test **545** warnings; and the current-state pointer guard passed.
No semantic issuer, field, numeric issue mapping, test, or fallback was
deleted.
