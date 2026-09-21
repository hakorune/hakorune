---
Status: closed__2026-09-21__WarningBaselineRefreshI29__FunctionOwnerIdTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I29
Date: 2026-09-21
Parent: mirbuilder-warning-binding-ref-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) FunctionOwnerIdV1 import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I30
---

# MirBuilder warning baseline refresh I29

## Six-line brief

```text
Decision: refresh both warning surfaces after the I28 BindingRef facade cohort
  before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,802/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## Selection evidence

The fixed commands completed sequentially at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,802 | `/tmp/hakorune-warning-i29-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i29-lib-test-20260921.log` |

The selected caller-zero cohort is the scalar operand recipe test facade:

* `src/mir/builder/normal_script_direct_static_join_handoff/scalar_operand_recipe.rs:15` —
  the unqualified `FunctionOwnerIdV1` is consumed only by the
  `#[cfg(test)]` `from_parts_for_test` helper. Production fields and accessors
  use the fully qualified type path.

The bounded execution slice moves only `FunctionOwnerIdV1` into a test-only
import. Scalar recipe construction, physical input, and direct-static lowering
remain unchanged. Any production consumer, compile failure, changed test
warning, or count mismatch returns the row to design stop.

## Closeout evidence

`FunctionOwnerIdV1` now resolves only in a `#[cfg(test)]` import; production
fields and accessors retain their qualified paths. The fixed commands were run
sequentially and both exited 0:

| command | result | evidence |
| --- | --- | --- |
| `cargo check --profile quick --lib -j4` | lib warnings **1,801** | `/tmp/hakorune-warning-i0-function-owner-id-lib-20260921.log` |
| `cargo test --profile quick --lib --no-run -j4` | lib-test warnings **561**, test executable built | `/tmp/hakorune-warning-i0-function-owner-id-lib-test-20260921.log` |

The only source change is the import grouping in
`scalar_operand_recipe.rs`; recipe construction, physical input, and lowering
are unchanged.
