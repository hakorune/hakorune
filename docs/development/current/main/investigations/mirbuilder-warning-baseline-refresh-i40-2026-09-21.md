---
Status: closed__2026-09-21__WarningBaselineRefreshI40__ValueIdTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I40
Date: 2026-09-21
Parent: mirbuilder-warning-generic-loop-extract-observe-test-facade-i0-2026-09-21.md
Implementation permission: selection recorded; execution delegated to MIRBUILDER-WARNING-VALUEID-TEST-FACADE-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I41
---

# MirBuilder warning baseline refresh I40

## Six-line brief

```text
Decision: refresh both warning surfaces after the I39 generic-loop extract
  observe test facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,792/561 and select at most one caller-zero import cohort.
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

The fixed gates completed sequentially at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,792 | `/tmp/hakorune-warning-i40-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i40-lib-test-20260921.log` |

The selected caller-zero import cohort is the `ValueId` test-only import:

* `src/mir/builder/resolved_lowering/common_v2_s6c_textref_entry_bridge.rs:11`
  imports `BasicBlockId` and `ValueId` together. `BasicBlockId` is used by the
  production bridge; `ValueId` is referenced only by the `#[cfg(test)]` helper
  rows at lines 261, 262, 326, and 327.
* The parent module's production code has no `ValueId` use. Splitting the import
  and gating only `ValueId` under `#[cfg(test)]` preserves the test helper and
  removes one lib-only unused-import warning.

The bounded execution slice changes only this import boundary; bridge semantics,
physical lane ownership, and test cases remain unchanged.

## Closeout evidence

The delegated import-facade slice completed with the fixed gates, sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,791 | `/tmp/hakorune-warning-i0-valueid-test-facade-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-valueid-test-facade-lib-test-20260921.log` |

Both commands exited 0. `ValueId` is now imported only under `#[cfg(test)]`;
production `BasicBlockId` and bridge ownership remain unchanged. I40 is closed,
and I41 is the next design-stop baseline refresh.
