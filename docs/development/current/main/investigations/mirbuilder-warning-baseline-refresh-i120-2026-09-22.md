---
Status: closed__2026-09-22__WarningBaselineRefreshI120__OldEdgeNoSafeSliceSelectedLoopBreakAccessor
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I120
Date: 2026-09-22
Parent: mirbuilder-warning-json-artifact-wrapper-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-LOOPBREAK-FACTS-SOURCE-KIND-I0
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

The next worker/card selection explicitly evaluated
`MIR-RETIRE-FIRST-OLD-EDGE-R0` before adding another warning facade:

1. census every caller of the selected `route_loop_break_recipe` source edge;
2. prove the target is caller-zero after any required source-caller switch;
3. delete exactly one physical edge, related import/test only when owned by it;
4. add or update an absence guard and record build, focused-test, and pointer
   receipts.

The read-only caller census found the old-edge lane is not yet caller-zero:

* `route_loop_loop_break_recipe` remains a live registry compatibility route
  through `dispatch_entry` and `RecipeComposer::compose_loop_break_recipe`;
* ledger-free compatibility roots can still reach
  `lower_non_callable_loop_legacy_v1`;
* `RawLegacyChildLoweringPortV1` still calls the shared
  `lower_loop_or_freeze_v1` owner; and
* the parser composite LoopBreak successor and absence guard are not landed.

Therefore `MIR-RETIRE-FIRST-OLD-EDGE-R0` remains `NoSafeSlice`; no route,
composer, helper, or `LegacyCallV0` code is deleted. This is the tracked
handoff for the owner instruction in `/tmp/deletion_lane_message.txt` and is
now part of the next-card contract.

The bounded warning row selected instead is the unused accessor
`VerifiedCallableLoopBreakSourceFactsV1::source_kind` at
`src/mir/builder/normal_callable_loop_source_facts/loop_break.rs:436`.
Repository search found only its declaration; the `source_kind` field remains
used for co-seal validation and candidate construction. The next card deletes
only that accessor and keeps all semantic fields and source ownership intact.

## Closeout evidence

```text
cargo check --profile quick --lib -j4          passed; lib 1,693 warnings
cargo test --profile quick --lib --no-run -j4  passed; lib-test 550 warnings
bash tools/checks/current_state_pointer_guard.sh passed
```

The old-edge census and the next warning selection are design evidence only;
no source implementation was performed under I120.

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
