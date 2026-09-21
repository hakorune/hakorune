---
Status: closed__2026-09-21__WarningBaselineRefreshI43__S6CSubstringV9TestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I43
Date: 2026-09-21
Parent: mirbuilder-warning-common-v2-session-test-facade-i0-2026-09-21.md
Implementation permission: selection recorded; execution delegated to MIRBUILDER-WARNING-S6C-SUBSTRING-V9-TEST-FACADE-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I44
---

# MirBuilder warning baseline refresh I43

## Six-line brief

```text
Decision: refresh both warning surfaces after the I42 common-V2 session test
  facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,788/561 and select at most one caller-zero import cohort.
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
| lib | 1,788 | `/tmp/hakorune-warning-i43-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i43-lib-test-20260921.log` |

The selected caller-zero import cohort is the S6C Substring V9 test facade:

* `src/mir/builder/resolved_lowering/common_v2_session/s6c_substring_v9_issuer.rs:8`
  imports `NonZeroU64`, used only by the `#[cfg(test)]` wire helpers.
* `:17` imports `EndAuthorizedTextV1` alongside production lease reject/consume
  types; the concrete owner is used only by the `#[cfg(test)]` adoption helper.

The bounded execution slice gates only these two imports under `#[cfg(test)]`;
V9 issuer validation and runtime lease ownership remain unchanged.

## Closeout evidence

The delegated import-facade slice completed with the fixed gates, sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,786 | `/tmp/hakorune-warning-i0-s6c-substring-v9-test-facade-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-s6c-substring-v9-test-facade-lib-test-20260921.log` |

Both commands exited 0. `NonZeroU64` and `EndAuthorizedTextV1` are now
imported only under `#[cfg(test)]`; V9 issuer and runtime lease owners remain
unchanged. I43 is closed, and I44 is the next design-stop baseline refresh.
