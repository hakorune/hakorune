---
Status: design_stop__2026-09-22__WarningBaselineRefreshI130__AwaitingNextCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I130
Date: 2026-09-22
Parent: mirbuilder-warning-dynamic-v2-reject-dead-variants-i129-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: prefer MIR-RETIRE-FIRST-OLD-EDGE-R0 only if its caller-zero proof changes
---

# MirBuilder warning baseline refresh I130

## Six-line brief

```text
Decision: refresh warning counts after I129 and select exactly one bounded row;
  keep the LoopBreak old-edge lane parked unless caller-zero changes.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
  warning classification policy, and the existing retirement census.
Non-authority: warning-count guesses, blanket allow, cargo-fix, AST rescans,
  or a new semantic receipt without a named owner.
Fail-fast boundary: a new warning, unclassified red, nonzero old-edge caller,
  or missing absence guard stops selection and records NoSafeSlice.
Smallest next slice: one caller-zero old-edge candidate, or one finite
  production-zero warning item after exact census.
Non-claims: no broad warning cleanup, parser semantic change, VM repair, or
  LegacyCallV0 retirement beyond one explicitly selected slice.
```

## Baseline and retirement boundary

I129 removed four declaration-only Dynamic V2 metadata reject variants. The
post-change baseline is lib **1,690** warnings and lib-test **545** warnings.
The provider-admission focused receipt is **6/6**. The I120 old-edge census is
unchanged: the compatibility registry, ledger-free compatibility branch,
shared `lower_loop_or_freeze_v1`, and parser composite successor still prevent
LoopBreak caller-zero proof. `MIR-RETIRE-FIRST-OLD-EDGE-R0` therefore remains
`NoSafeSlice` unless a new census changes that result.

## Acceptance

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Record both warning counts and the complete selected owner/caller inventory
before selecting the next fast card. Do not delete the parser source-admission
row accessors as a group without a fresh caller census.
