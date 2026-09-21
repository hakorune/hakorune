---
Status: design_stop__2026-09-22__WarningBaselineRefreshI126__AwaitingNextCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I126
Date: 2026-09-22
Parent: mirbuilder-warning-loopbreak-candidate-accessors-i0-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: prefer MIR-RETIRE-FIRST-OLD-EDGE-R0 only if its caller-zero proof changes
---

# MirBuilder warning baseline refresh I126

## Six-line brief

```text
Decision: refresh the warning surface and select exactly one bounded row;
keep the old-edge lane parked unless a new caller-zero proof exists.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
warning classification policy, and the existing LoopBreak retirement card.
Non-authority: warning-count guesses, blanket allow, cargo-fix, AST rescans,
or a new semantic receipt without a named owner.
Fail-fast boundary: a new warning, unclassified red, nonzero old-edge caller,
or missing absence guard stops selection and records NoSafeSlice.
Smallest next slice: census one caller-zero old-edge candidate, or one
production-zero warning item only when that deletion is proven finite.
Non-claims: no broad warning cleanup, parser semantic change, VM repair, or
LegacyCallV0 retirement beyond one explicitly selected slice.
```

## Baseline and old-edge boundary

I125 deleted five candidate accessors and recorded lib **1,691** and lib-test
**547**. The `projection` accessor remains live through the focused test
caller. The I120 caller census remains binding: compatibility registry,
ledger-free legacy, shared `lower_loop_or_freeze_v1`, and parser composite
successor evidence still prevent old-edge deletion.

I126 is design-only. Refresh both warning counts, classify the selected
diagnostics, and name one finite owner/caller/delete set before implementation.
Do not convert the old-edge `NoSafeSlice` into a source disposition.

## Acceptance

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Record the complete selected owner/caller inventory and keep the next row
bounded to one physical responsibility.
