---
Status: closed__2026-09-21__WarningBaselineRefreshI34__NormalRootExecutionRejectOwnerFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I34
Date: 2026-09-21
Parent: mirbuilder-warning-normal-root-execution-reject-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) normal-root owner reject re-export; execution closed
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I35
---

# MirBuilder warning baseline refresh I34

## Six-line brief

```text
Decision: refresh both warning surfaces after the I33 normal-root reject
  test facade cohort before selecting another cleanup row.
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
| lib | 1,798 | `/tmp/hakorune-warning-i34-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i34-lib-test-20260921.log` |

The selected caller-zero cohort is the remaining owner-module re-export for
the normal-root reject type:

* `src/mir/builder/normal_root_execution/mod.rs:11` —
  `NormalRootExecutionConsumerRejectV1` is used by the module's test fixture
  and by the already test-gated builder facade; there is no production caller
  of this owner-module re-export.
* `NormalRootExecutionConsumerV1` remains a production consumer and stays in
  the unconditional owner export.

The bounded execution slice adds `#[cfg(test)]` only to the reject-type owner
re-export. The repeated test-facade audit from I33 supplies the same
classifier, type, and counterexample boundaries; no runtime route or source
authority changes.

## Closeout

The owner-module reject export is now test-only while the successful consumer
remains unconditional. The fixed gates completed sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,797 | `/tmp/hakorune-warning-i0-normal-root-execution-reject-owner-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-normal-root-execution-reject-owner-lib-test-20260921.log` |

Both commands exited 0. The normal-root owner and route behavior are
unchanged; I35 is the next design-stop baseline at 1,797/561.
