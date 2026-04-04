---
Status: Active
Date: 2026-04-05
Scope: rerun selfhost proof/compat callers against the canonical wrapper homes after the runner/selfhost recut and wrapper policy freeze.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-91x/README.md
  - lang/src/runner/README.md
---

# Phase 92x: Selfhost Proof/Compat Caller Rerun

## Goal

- rerun selfhost proof/compat callers against the canonical wrapper homes
- confirm top-level `.hako` wrappers remain thin front-door keeps after the policy freeze
- keep proof/compat reruns explicit and non-growing

## Big Tasks

1. `92xA1` caller inventory lock
2. `92xA2` candidate caller ranking
3. `92xB1` caller rerun
4. `92xC1` proof refresh
5. `92xD1` closeout

## Current Read

- current front:
  - `92xD1 closeout`
- likely corridor:
  - `TBD`
- rerun scope is limited to the ranked proof/compat callers that still mention the canonical wrapper homes
- canonical homes stay under `compat/`, `facade/`, and `entry/`
