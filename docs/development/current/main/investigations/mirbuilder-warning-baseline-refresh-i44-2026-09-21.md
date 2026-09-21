---
Status: closed__2026-09-21__WarningBaselineRefreshI44__DraftSealExitTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I44
Date: 2026-09-21
Parent: mirbuilder-warning-s6c-substring-v9-test-facade-i0-2026-09-21.md
Implementation permission: selection recorded; execution delegated to MIRBUILDER-WARNING-DRAFT-SEAL-EXIT-TEST-FACADE-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I45
---

# MirBuilder warning baseline refresh I44

## Six-line brief

```text
Decision: refresh both warning surfaces after the I43 S6C Substring V9 test
  facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,786/561 and select at most one caller-zero import cohort.
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
| lib | 1,786 | `/tmp/hakorune-warning-i44-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i44-lib-test-20260921.log` |

The selected caller-zero import cohort is the draft-seal exit test facade:

* `src/mir/builder/resolved_lowering/draft_seal.rs:38` re-exports
  `DetachedFunctionExitClaimSetV1` and `MultiSiteExitPreparationErrorV1`.
* The only observed consumer of this facade is
  `resolved_lowering/completion_consumption_tests.rs`; production
  `exit_projection` uses `multi_site_exit` directly.

The bounded execution slice gates this grouped re-export with `#[cfg(test)]`;
draft-seal ownership and exit semantics remain unchanged.

## Closeout evidence

The delegated import-facade slice completed with the fixed gates, sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,785 | `/tmp/hakorune-warning-i0-draft-seal-exit-test-facade-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-draft-seal-exit-test-facade-lib-test-20260921.log` |

Both commands exited 0. The grouped draft-seal exit re-export is now gated by
`#[cfg(test)]`; production exit projection remains unchanged. I44 is closed,
and I45 is the next design-stop baseline refresh.
