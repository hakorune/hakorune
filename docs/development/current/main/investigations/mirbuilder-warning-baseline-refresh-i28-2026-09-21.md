---
Status: closed__2026-09-21__WarningBaselineRefreshI28__BindingRefTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I28
Date: 2026-09-21
Parent: mirbuilder-warning-scalar-operand-recipe-argument-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) BindingRefV1 import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I29
---

# MirBuilder warning baseline refresh I28

## Six-line brief

```text
Decision: refresh both warning surfaces after the I27 scalar-argument facade
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,803/561 and select at most one caller-zero import cohort.
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
| lib | 1,803 | `/tmp/hakorune-warning-i28-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i28-lib-test-20260921.log` |

The selected caller-zero cohort is the Script semantic-source test facade:

* `src/mir/builder/normal_script_semantic_source.rs:20` —
  `BindingRefV1` is used only by the `#[cfg(test)]`
  `outbox_materializations()` accessor. The other types in its import group
  remain production inputs and must stay unconditional.

The bounded execution slice moves only `BindingRefV1` into the existing
test-only resolved-semantics import. The Script source product, resolver
relations, and production lowering remain unchanged. Any production consumer,
compile failure, changed test warning, or count mismatch returns the row to
design stop.

## Closeout evidence

`BindingRefV1` now resolves only in the existing `#[cfg(test)]` import group;
the other resolved-semantics imports remain unconditional. The fixed commands
were run sequentially and both exited 0:

| command | result | evidence |
| --- | --- | --- |
| `cargo check --profile quick --lib -j4` | lib warnings **1,802** | `/tmp/hakorune-warning-i0-binding-ref-lib-20260921.log` |
| `cargo test --profile quick --lib --no-run -j4` | lib-test warnings **561**, test executable built | `/tmp/hakorune-warning-i0-binding-ref-lib-test-20260921.log` |

The only source change is the import grouping in
`normal_script_semantic_source.rs`; the Script source product, resolver
relations, and production lowering are unchanged.
