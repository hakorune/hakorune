---
Status: Landed
Date: 2026-04-04
Scope: decide which zero-caller top-level selfhost façades stay as public compatibility surfaces and which can move out of the live front door; phase is now landed and handed off.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-82x/README.md
---

# Phase 83x: Selfhost Top-Level Facade/Archive Decision

## Goal

- inspect top-level `tools/selfhost/*` façade wrappers after the folder split settled
- separate true public/front-door keeps from archive-ready caller-zero aliases
- keep proof-only and mainline canonical paths readable

## Big Tasks

1. `83xA1` top-level facade inventory lock
2. `83xA2` keep/archive decision freeze
3. `83xB1` archive-ready sweep or explicit keep proof
4. `83xC1` proof refresh
5. `83xD1` closeout

## Current Read

- handoff complete
- result:
  - top-level selfhost wrappers stay as explicit public/front-door keeps
- selected successor lane:
  - `phase-84x runner wrapper/source contract thinning`
