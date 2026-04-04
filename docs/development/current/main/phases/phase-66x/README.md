---
Status: Active
Date: 2026-04-04
Scope: choose the next source lane after phase-65x stage1/selfhost mainline hardening.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-65x/README.md
  - docs/development/current/main/phases/phase-65x/65x-90-stage1-selfhost-mainline-hardening-ssot.md
---

# Phase 66x: Next Source Lane Selection

## Goal

- choose the next source lane after `65x`
- keep the current read stable:
  - rust-vm remains residual explicit keep
  - vm-hako remains reference/conformance
  - stage1/selfhost mainline hardening is landed
  - focused `emit_mir_mainline` parse red is a tracked follow-up, not an implicit reopen
  - next progress should come from tree/folder separation, not from expanding design prose

## Big Tasks

1. `66xA1` successor lane inventory lock
2. `66xA2` candidate lane ranking
3. `66xB1` successor lane decision
4. `66xB2` folder-separation corridor lock
5. `66xD1` proof / closeout
