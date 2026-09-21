---
Status: closed__2026-09-21__WarningDirectAccumProfileTestImports__Execution
Task: MIRBUILDER-WARNING-DIRECT-ACCUM-PROFILE-TEST-IMPORTS-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i53-2026-09-21.md
Implementation permission: true for the two test-only import groups only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I54
---

# Warning cleanup: DirectAccum profile test imports

## Six-line brief

```text
Decision: gate two unused import groups that are consumed only by the
  DirectAccum profile test helper and effect-plan assertions.
Source authority + canonical issuer: direct_accum_profile.rs owns the plan;
  production demand and effect-plan types remain unconditional.
Non-authority: cargo-fix, wildcard imports, visibility changes, plan logic,
  physical adapter behavior, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the winner and effect-plan import groups
  at direct_accum_profile.rs:12,15-16, then run fixed gates sequentially.
Non-claims: no DirectAccum source admission, Recipe, or physical route change.
```

## Preconditions and acceptance

I53 selected the two unused import groups in
`src/mir/compiler/direct_accum_profile.rs:12,15-16`.
`VerifiedLoopPolicyWinnerV1` is consumed only by the test-only parity helper
`admit_direct_accum_profile_v1`; `DirectAccumBindingEffectEntryV1` and
`DirectAccumBindingEffectRoleV1` are consumed only by the test-only effect-plan
assertions. Production code keeps the demand issuer, reject type, and verified
effect-plan product.

Gate only the two import groups with `#[cfg(test)]`. Do not edit the plan
issuer, production adapter, tests, or physical route.

Expected result: two warning diagnostics are removed, so lib warnings
**1,771 → 1,769** and lib-test warnings remain **561**. Both fixed gates must
exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```


## Closeout evidence

The winner and effect-plan import groups received `#[cfg(test)]`; the
DirectAccum issuer, Recipe, physical adapter, tests, and production route were
unchanged. The fixed gates completed sequentially with exit 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,769 | `/tmp/hakorune-warning-i0-direct-accum-profile-test-imports-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-direct-accum-profile-test-imports-lib-test-20260921.log` |

Both selected warning diagnostics disappeared; the lib-test baseline was
unchanged.
