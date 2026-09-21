---
Status: closed__2026-09-21__WarningBaselineRefreshI42__CommonV2SessionTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I42
Date: 2026-09-21
Parent: mirbuilder-warning-merged-source-segment-test-facade-i0-2026-09-21.md
Implementation permission: selection recorded; execution delegated to MIRBUILDER-WARNING-COMMON-V2-SESSION-TEST-FACADE-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I43
---

# MirBuilder warning baseline refresh I42

## Six-line brief

```text
Decision: refresh both warning surfaces after the I41 merged-source segment
  test facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,790/561 and select at most one caller-zero import cohort.
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
| lib | 1,790 | `/tmp/hakorune-warning-i42-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i42-lib-test-20260921.log` |

The selected caller-zero import cohort is the common-V2 session test close
seam:

* `src/mir/builder/resolved_lowering/common_v2_session/mod.rs:15` imports
  `MirBuilder`; production methods in this module use the fully qualified
  `crate::mir::builder::MirBuilder`, while the unqualified import is used only
  by the `#[cfg(test)]` draft-seal close seam.
* `:24` imports `ReadyFunctionDraftSealV1`, which is returned only by that same
  test-only close seam.

The bounded execution slice gates these two imports under `#[cfg(test)]`;
canonical session ownership and physical lowering remain unchanged.

## Closeout evidence

The delegated import-facade slice completed with the fixed gates, sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,788 | `/tmp/hakorune-warning-i0-common-v2-session-test-facade-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-common-v2-session-test-facade-lib-test-20260921.log` |

Both commands exited 0. `MirBuilder` and `ReadyFunctionDraftSealV1` are now
imported only under `#[cfg(test)]`; canonical session and physical owners remain
unchanged. I42 is closed, and I43 is the next design-stop baseline refresh.
