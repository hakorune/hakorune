---
Status: Landed
Date: 2026-06-14
Scope: planner_required route-exhaustion inventory guard.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-293x/293x-1006-COREPLAN-FOUND-002-REMAINING-FAMILY-INVENTORY.md
  - tools/checks/coreplan_planner_required_route_exhaustion_guard.sh
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
  - src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs
---

# COREPLAN-C1-001: Planner Required Route Exhaustion

## Purpose

Fix the C1 boundary before any normalizer or v0 route retirement work.

This card does not change route selection. It adds a guard that keeps the
current planner-required diagnostic seams visible and classifies `Ok(None)`
sites as either a non-candidate probe or a silent fallback candidate.
This keeps the boundary explicit: non-candidate probe vs silent fallback.

## Decision

```text
planner_required_target_like_route_exhaustion_classified=1
planner_required_silent_ok_none_inventory=1
candidate_ambiguity_owner_documented=1
accepted_shape_added=0
release_default_changed=0
summary=ok
```

## Guard

```bash
bash tools/checks/coreplan_planner_required_route_exhaustion_guard.sh
```

The guard requires:

```text
entry_ambiguous diagnostic stays in router.rs
route_exhausted detail includes facts_present and candidates
planner_none expected-plan freeze path remains visible
route_entry/registry remains candidate collection owner
generic_loop_v0/v1 release Ok(None) fallbacks remain explicit inventory sites
```

## Stop Lines

```text
do not convert all optional facts Ok(None) into errors
do not hide ambiguity with route priority scoring
do not duplicate route truth between single_planner and route_entry/registry
do not add accepted source shapes
```
