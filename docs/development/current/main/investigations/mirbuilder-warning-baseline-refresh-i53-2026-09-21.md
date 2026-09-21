---
Status: closed__2026-09-21__WarningBaselineRefreshI53__DirectAccumProfileTestImports
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I53
Date: 2026-09-21
Parent: mirbuilder-warning-callable-semantic-batch-facade-i0-2026-09-21.md
Implementation permission: selection recorded; execution completed by MIRBUILDER-WARNING-DIRECT-ACCUM-PROFILE-TEST-IMPORTS-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I54
---

# MirBuilder warning baseline refresh I53

## Six-line brief

```text
Decision: refresh both warning surfaces after the I52 callable semantic batch
  facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,771/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each, sequentially. Record
lint, file:line, owner, and production/test/compat/generated role, then select
one cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

The previous fixed baseline is lib **1,771** and lib-test **561** after I52.


## Selection evidence

The fixed gates completed sequentially at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,771 | `/tmp/hakorune-warning-i53-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i53-lib-test-20260921.log` |

The selected caller-zero cohort is two test-only import groups in the single
owner `src/mir/compiler/direct_accum_profile.rs:12,15-16`:

* `VerifiedLoopPolicyWinnerV1` is used only by the `#[cfg(test)]`
  `admit_direct_accum_profile_v1` helper and its tests.
* `DirectAccumBindingEffectEntryV1` and `DirectAccumBindingEffectRoleV1` are
  used only by the `#[cfg(test)]` effect-plan assertions.

The production issuer retains `issue_selected_loop_recipe_demand_v1`,
`SelectedLoopDemandRejectV1`, and `VerifiedDirectAccumBindingEffectPlanV1`;
the physical adapter and production DirectAccum plan are unchanged.

The bounded execution slice gates only these two import groups under
`#[cfg(test)]`.

## Closeout evidence

The two selected import groups were gated under `#[cfg(test)]`. Both fixed
gates exited 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,769 | `/tmp/hakorune-warning-i0-direct-accum-profile-test-imports-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-direct-accum-profile-test-imports-lib-test-20260921.log` |

I53 closes with no DirectAccum issuer, Recipe, physical adapter, or production
route change.
