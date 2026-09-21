---
Status: closed__2026-09-21__WarningBaselineRefreshI93__NoSafeOldEdge
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I93
Date: 2026-09-21
Parent: mirbuilder-warning-callable-contract-disposition-syntax-delete-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I94
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

## Old-edge census result and next selection

The proposed old-edge target was re-read against the production graph. The
only direct reference to `route_loop_break_recipe` is the live
`ENTRIES[LoopRouteId::LoopBreakRecipe]` registry entry, and its only internal
composer call is `RecipeComposer::compose_loop_break_recipe`. The registry is
invoked by the compatibility `route_loop` path from `try_cf_loop_joinir`;
non-source raw loop paths still reach that path. The source
`raw_loop_child_entry` instead consumes its source candidate through the
source physical adapter before generic facts, and the composite parser shape
currently stops at `RouteNotFrontSelected`. Therefore the route/composer pair
has a live compatibility caller and is not caller-zero. `MIR-RETIRE-FIRST-OLD-
EDGE-R0` is recorded as `NoSafeSlice` until a named source caller switches and
the live compatibility route has its own successor.

The next bounded warning row is the already classified mixed-cfg local
`loop_phi_materializer.rs:467`. Renaming the loop index to `_index` preserves
the test-only failure-injection comparison while making the non-test build's
intent explicit; no semantic owner, receipt, or production route changes.

## Closeout evidence

The route/composer old-edge remains `NoSafeSlice`: the registry entry is a
live compatibility caller, while the source raw-loop entry already bypasses
it through the source physical adapter. I93's post-accessor refresh recorded
lib **1,711** and lib-test **552**; the mixed-cfg warning was selected as the
next bounded implementation row. No compatibility route or old edge was
deleted.
