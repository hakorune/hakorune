---
Status: design_stop__2026-09-21__WarningBaselineRefreshI93__SelectNextCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I93
Date: 2026-09-21
Parent: mirbuilder-warning-callable-contract-disposition-syntax-delete-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIR-RETIRE-FIRST-OLD-EDGE-R0 or one explicitly justified caller-zero warning row
---

# MirBuilder warning baseline refresh I93

## Six-line brief

```text
Decision: refresh the post-delete warning surface and choose one bounded row;
prefer the first caller-zero old-edge deletion when its census is complete.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
warning classification policy, and the LoopBreak retirement card's delete set.
Non-authority: warning-count guesses, blanket allow, cargo-fix, or a new
semantic receipt without a named owner.
Fail-fast boundary: a new warning, nonzero caller, unclassified red, or
missing old-edge guard stops selection and records NoSafeSlice.
Smallest next slice: census one old-edge deletion candidate, or one warning
facade only when its caller-zero proof is complete.
Non-claims: no broad warning cleanup, parser semantic change, VM repair, or
LegacyCallV0 retirement beyond the selected single edge.
```

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record lib/lib-test warning counts, classify every selected diagnostic, and
keep production callers such as `admitted_registry::branch_count` in their
owner rows. Before any implementation, perform the old-edge caller census.
The current physical audit classifies `route_loop_break_recipe` as a
compatibility route, not a source consumer; a source caller must first be
switched and the exact edge must then prove caller-zero before deletion is
selected.

## Deletion-lane instruction

The warning cleanup has now demonstrated its first real source deletion in
I93. The next worker/card selection must explicitly consider the bounded
`MIR-RETIRE-FIRST-OLD-EDGE-R0` lane: caller census, source-caller switch if
needed, one physical deletion, absence guard, and build/focused/guard receipt.
It must not expand to `lower_loop_or_freeze_v1`, `LegacyCallV0`, or the live
compatibility route. If the census is not caller-zero or the physical owner
is not sole, record `NoSafeSlice` and retain the warning cohort as baseline
debt.
