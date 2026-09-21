---
Status: closed__2026-09-21__WarningBaselineRefreshI33__NormalRootExecutionRejectTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I33
Date: 2026-09-21
Parent: mirbuilder-warning-module-declaration-shell-prepare-error-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) normal-root reject re-export; execution closed
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I34
---

# MirBuilder warning baseline refresh I33

## Six-line brief

```text
Decision: refresh both warning surfaces after the I32 declaration-shell
  prepare-error test facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,798/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## Repeated test-facade pattern audit

I30, I31, and I32 each closed one parent re-export whose consumers were
`#[cfg(test)]` only. The audit of this repeated pattern is now explicit:

* classifier arm: only a parent re-export with zero production callers may be
  gated; a neighboring export with a production caller must remain visible;
* transferred/opaque subtree: these exports move no runtime value or policy,
  only an intra-crate type name into test modules;
* type requirement: test modules retain the same `pub(in crate::mir)` path;
* counterexample: any production `crate::mir::builder::*` caller rejects the
  row and returns it to design stop.

The I33 warning surface has one selected row satisfying that audit:

* `src/mir/builder.rs:190` — `NormalRootExecutionConsumerRejectV1` has only
  test consumers (`normal_callable_semantic_package` test code and the
  normal-root execution tests). `NormalRootExecutionConsumerV1` remains a
  production consumer and must stay in the unconditional export.

The bounded execution slice splits this import and adds `#[cfg(test)]` only to
the reject-type re-export. No runtime route or source authority changes.

## Closeout

The reject-type export is now test-only while the successful consumer remains
unconditional. The fixed gates completed sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,797 | `/tmp/hakorune-warning-i0-normal-root-execution-reject-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-normal-root-execution-reject-lib-test-20260921.log` |

Both commands exited 0. The normal-root execution owner and route behavior are
unchanged; I34 is the next design-stop baseline at 1,797/561.
