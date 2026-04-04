---
Status: Active
Date: 2026-04-04
Scope: decide whether rust-vm can retire fully or must remain as a residual explicit keep after the 60x->62x corridor.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-62x/README.md
  - docs/development/current/main/phases/phase-62x/62x-90-rust-vm-delete-ready-removal-wave-ssot.md
---

# Phase 63x: Rust-VM Final Retirement Decision

## Goal

- decide whether full rust-vm retirement is now defensible
- if not, define the residual explicit keep set and stop-line clearly
- keep `vm-hako` out of scope as reference/conformance

## Decision Inputs

- `60x`: proof/compat keep pruning continuation
- `61x`: caller-zero audit rerun
- `62x`: delete-ready removal wave (no-op)

## Big Tasks

1. decision inventory
   - `63xA1` retirement-decision evidence lock
   - `63xA2` retire-vs-residual decision
2. stop-line definition
   - `63xB1` residual keep stop-line or retirement plan freeze
3. prove and close
   - `63xD1` proof / closeout
