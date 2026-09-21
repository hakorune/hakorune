---
Status: closed__2026-09-22__WarningBaselineRefreshI130__SelectedI131
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I130
Date: 2026-09-22
Parent: mirbuilder-warning-dynamic-v2-reject-dead-variants-i129-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-GENERIC-G0-PARAMETERS-ACCESSOR-I131
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

## Owner selection request

The owner has requested that the next selected slice prefer the deletion lane
`MIR-RETIRE-FIRST-OLD-EDGE-R0`: one exact source-side LoopBreak old-edge
caller, with caller census, physical removal, absence guard, and focused
acceptance recorded as one bounded series. This is a scheduling priority, not
permission to delete a live caller. The I120 census remains the authority for
the entry condition; do not remove a caller to manufacture caller-zero. If a
fresh census still finds the selected edge live, keep this row in
`NoSafeSlice` and record the blocker rather than opening a warning facade or
switching a compatibility route.

## Deletion-lane recheck

The read-only I120 recheck at HEAD `c616161dd9` confirms
`NoSafeSlice`. `route_loop_break_recipe` remains registered in
`registry/mod.rs:63-69`, executed through `dispatch_entry` from the live
preflight path, and consumed by `RecipeComposer::compose_loop_break_recipe`.
The source adapter at `raw_loop_child_entry.rs:304-339` bypasses that registry
only for its limited direct shape; the parser composite still stops at
`GenericLoopV1NotSelected`. Compatibility production callers remain through
the `callable_handoff == None` branch into
`lower_non_callable_loop_legacy_v1`/`lower_loop_or_freeze_v1`, plus the
independent `RawLegacyChildLoweringPortV1` caller. Existing tests exercise the
registry, raw port, and route entry, while guards explicitly require the
legacy edges. No caller-zero or same-series switch is proven, so the owner
request is recorded as next-slice priority only; no physical deletion or
absence guard is authorized in I130.

## Acceptance

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Record both warning counts and the complete selected owner/caller inventory
before selecting the next fast card. Do not delete the parser source-admission
row accessors as a group without a fresh caller census.
