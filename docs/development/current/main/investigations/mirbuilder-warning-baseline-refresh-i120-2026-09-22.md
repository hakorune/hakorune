---
Status: design_stop__2026-09-22__WarningBaselineRefreshI120__AwaitingNextCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I120
Date: 2026-09-22
Parent: mirbuilder-warning-json-artifact-wrapper-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: prefer MIR-RETIRE-FIRST-OLD-EDGE-R0 when its caller-zero precondition is met
---

# MirBuilder warning baseline refresh I120

## Six-line brief

```text
Decision: refresh the warning surface and choose exactly one bounded next row;
the owner-requested deletion lane must be considered before another facade.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
warning classification policy, and the existing LoopBreak retirement card.
Non-authority: warning-count guesses, blanket allow, cargo-fix, AST rescans,
or a new semantic receipt without a named owner.
Fail-fast boundary: a new warning, unclassified red, nonzero old-edge caller,
or missing absence guard stops selection and records NoSafeSlice.
Smallest next slice: census the first caller-zero old-edge candidate, or one
production-zero warning row only if that deletion precondition is unmet.
Non-claims: no broad warning cleanup, parser semantic change, VM repair, or
LegacyCallV0 retirement beyond one explicitly selected slice.
```

## Baseline and selection boundary

I119 deleted the production-zero `runner/json_artifact` wrapper and recorded
lib **1,693** and lib-test **550**. I120 is design-only: refresh those counts,
classify the selected diagnostics, and identify one finite owner/caller/delete
set before implementation. The warning lane remains useful, but it must not
automatically outrank a safe physical deletion.

## Owner instruction: deletion lane is the next selection check

The next worker/card selection must explicitly evaluate
`MIR-RETIRE-FIRST-OLD-EDGE-R0` before adding another warning facade:

1. census every caller of the selected `route_loop_break_recipe` source edge;
2. prove the target is caller-zero after any required source-caller switch;
3. delete exactly one physical edge, related import/test only when owned by it;
4. add or update an absence guard and record build, focused-test, and pointer
   receipts.

The non-source compatibility route, `lower_loop_or_freeze_v1`, and
`LegacyCallV0` remain outside this slice. If the caller-zero or sole-owner
proof is absent, keep the old-edge lane `NoSafeSlice` and select one bounded
warning row instead. This is the tracked handoff for the owner instruction in
`/tmp/deletion_lane_message.txt`; no external worker message is required for
the instruction to be part of the next-card contract.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Record the lib/lib-test warning counts and the complete selected owner/caller
inventory. Do not implement from this card until one bounded row is named and
its source authority, canonical issuer, delete set, and non-claims are
written in the next card.
