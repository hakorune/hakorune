---
Status: closed__2026-09-21__WarningBaselineRefreshI35__RawLegacyExpressionTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I35
Date: 2026-09-21
Parent: mirbuilder-warning-normal-root-execution-reject-owner-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) raw legacy expression re-export; execution closed
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I36
---

# MirBuilder warning baseline refresh I35

## Six-line brief

```text
Decision: refresh both warning surfaces after the I34 normal-root owner
  reject facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,797/561 and select at most one caller-zero import cohort.
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
| lib | 1,797 | `/tmp/hakorune-warning-i35-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i35-lib-test-20260921.log` |

The selected caller-zero cohort is the raw legacy expression test facade:

* `src/mir/builder/recursive_child_lowering.rs:48` —
  `drive_raw_legacy_expression_v1` is used by test fixtures and by the
  logical-shortcircuit test-only entry; no production caller requires this
  parent re-export.
* `drive_raw_legacy_body_v1`, `drive_raw_legacy_statement_v1`, and
  `RawLegacyChildLoweringPortV1` remain unconditional because they are a
  separate legacy-port surface.

The bounded execution slice splits only the expression re-export and adds
`#[cfg(test)]`. Legacy body/statement behavior and production lowering remain
unchanged.

## Closeout

The expression helper re-export is now test-only while body/statement helpers
remain unconditional. The fixed gates completed sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,796 | `/tmp/hakorune-warning-i0-raw-legacy-expression-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-raw-legacy-expression-lib-test-20260921.log` |

Both commands exited 0. Legacy body/statement and production lowering remain
unchanged; I36 is the next design-stop baseline at 1,796/561.
