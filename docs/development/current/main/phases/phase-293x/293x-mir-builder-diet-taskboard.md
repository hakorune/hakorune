---
Status: Active
Date: 2026-05-15
Lane: phase-293x MIR builder diet cleanup sidecar
Canonical SSOT:
  - docs/development/current/main/design/mir-builder-diet-flowplanner-boundary-ssot.md
---

# Phase 293x MIR Builder Diet Taskboard

## Purpose

This is a temporary BoxShape cleanup sidecar before returning to
`MIMAP-021C`. It does not change allocator behavior, source semantics, release
defaults, or backend routes.

## Current Status

Current primary row:

```text
FLOWPLANNER-ENTRY-001
```

`MIMAP-021C` is parked until this sidecar pins the builder / FlowPlanner
boundary strongly enough that the next mimalloc rows do not add more ambiguity.

## Rows

| Row | Status | Purpose | Expected size |
| --- | --- | --- | --- |
| `MIRBUILDER-DIET-001` | landed | Open the sidecar, add the boundary SSOT, and update current pointers. | 1 commit |
| `FLOWPLANNER-ENTRY-001` | ready | Inventory builder -> FlowPlanner public entries and document rejected bypasses. | 1 commit |
| `FLOWPLANNER-V0-001` | ready | Add `loop_*_v0` retire/promote rules and no-new-v0 guard wording. | 1 commit |
| `MIR-SEMANTIC-PLANS-001` | ready | Classify top-level MIR plan/route/seed owners as SemanticPlans without physical moves. | 1 commit |
| `JOINIR-FENCE-001` | parked | Revisit JoinIR merge/bridge fence after FlowPlanner entry is stable. | later |

## Stop Lines

- No behavior changes.
- No physical directory or crate move in the first row.
- No allocator row changes.
- No new accepted control-flow shape.
- No release-default change.
- No silent fallback.

## Guard

```bash
bash tools/checks/current_state_pointer_guard.sh
```

Run `tools/checks/dev_gate.sh quick` when a cleanup row touches code or smoke
contracts.
