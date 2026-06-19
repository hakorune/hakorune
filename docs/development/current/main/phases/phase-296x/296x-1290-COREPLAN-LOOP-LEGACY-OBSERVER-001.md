---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Rename the B-lite loop shadow as a legacy registry observer and keep
  suppression retirement behind a fail-closed inventory.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1289-COREPLAN-LOOP-ROUTE-RETIRE-SELECTION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1288-COREPLAN-LOOP-RESOLVER-SHADOW-001.md
  - docs/development/current/main/phases/phase-296x/296x-1287-COREPLAN-LOOP-RESOLVER-B-LITE-SSOT-001.md
---

# COREPLAN-LOOP-LEGACY-OBSERVER

## Decision

The B-lite shadow is now named and reported as a legacy registry observer, not
as an independent semantic resolver. Its report vocabulary is intentionally
legacy-scoped:

```text
legacy_matched_candidates
legacy_effective_candidates
legacy_suppressed_candidates
```

The misleading `route_disagreement` field is removed because the observer still
derives its Allow/Deny decision from the legacy effective candidate set. A
single effective candidate therefore does not prove independent resolver
parity.

Registry suppression retirement is intentionally not implemented in this row.
The earlier candidate branch remains selected for investigation, but deletion
requires a fail-closed inventory because the phase29bq gate already contains an
existing `loop_continue_only_multidelta_min` output mismatch on pushed HEAD.

## Implementation

Code changes:

```text
src/mir/builder/control_flow/joinir/route_entry/registry/legacy_observer.rs
src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs
src/mir/builder/control_flow/joinir/route_entry/registry/types.rs
```

Implementation details:

```text
resolver.rs -> legacy_observer.rs
LoopRouteId typed route vocabulary added
LoopRouteFact selected_route now uses LoopRouteId
trace tag changed to [plan/trace:loop_legacy_observer]
trace fields changed to legacy_matched/effective/suppressed
route_disagreement removed from trace/report code
registry suppression behavior preserved
```

Behavior boundary:

```text
route_selection_changed=0
new_named_route_added=0
observer_selects_lowering=0
registry_suppression_changed=0
loop_cond_break_continue_global_suppression_retired=0
```

## Evidence

Local gates:

```bash
cargo check --all-targets
```

Result:

```text
Finished dev profile target(s)
```

Unit coverage added:

```text
loop_cond_continue_only_keeps_existing_loop_continue_only_suppression
loop_cond_break_continue_keeps_existing_loop_continue_only_suppression
```

## Next Row

The next cleanup row should collect actual legacy route selection rather than
candidate-only observation.

```text
next_task=COREPLAN-LOOP-ACTUAL-SELECTION-TRACE-001
```

Suggested scope:

```text
legacy_selected_route records the handler that actually returned Some(value)
candidate-only observer stays diagnostic
feedback_to_resolver=0
```

## Stop Lines

```text
do not promote legacy observer to route selection
do not treat legacy_effective singleton as independent resolver parity
do not delete loop_cond_break_continue global suppression without full inventory
do not add new suppression branches
do not change app-front/json_native behavior
```

## Report

```text
output_contract=coreplan-loop-legacy-observer-v0
implementation_changed=1
behavior_changed=0
legacy_observer_named=1
typed_loop_route_id_enabled=1
route_disagreement_report_removed=1
retired_branch=none
registry_suppression_changed=0
loop_cond_break_continue_global_suppression_retired=0
next_task=COREPLAN-LOOP-ACTUAL-SELECTION-TRACE-001
summary=ok
```
