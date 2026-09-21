---
Status: closed__2026-09-21__WarningBaselineRefreshI19__SelectedUnifiedCallTestImports
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I19
Date: 2026-09-21
Parent: mirbuilder-warning-semantic-admission-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) unified-call test import facade only
NextCard: MIRBUILDER-WARNING-UNIFIED-CALL-TEST-IMPORTS-I0
---

# MirBuilder warning baseline refresh I19

## Six-line brief

```text
Decision: refresh both warning surfaces after the semantic-admission
  test-facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,812/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I19 inventory and decision

The fixed commands completed successfully with the expected baseline:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,812 | `/tmp/hakorune-warning-i19-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i19-lib-test-20260921.log` |

The selected caller-zero cohort is the unified-call post-success test import
facade:

* `src/mir/builder/calls/unified_emitter/post_success.rs:9` —
  `CalleeBoxKind` and `TypeCertainty` are consumed by the local `#[cfg(test)]`
  module only; production post-success code uses `Callee` directly.

The bounded next slice is to gate only these two test imports with
`#[cfg(test)]`; call preparation and publication semantics remain unchanged.
