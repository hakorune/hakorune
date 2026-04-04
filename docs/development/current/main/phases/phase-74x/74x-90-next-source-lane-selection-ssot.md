---
Status: Landed
Date: 2026-04-04
Scope: choose the next source lane after phase-73x landed.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-73x/README.md
---

# 74x-90 Next Source Lane Selection SSOT

## Intent

- keep the post-73x read stable
- rank the next source lane without reopening the closed `emit_mir_mainline` blocker

## Current Read

- landed:
  - `phase-73x emit_mir_mainline blocker follow-up`
- now:
  - `phase-74x` selected `phase-75x selfhost top-level alias canonicalization`

## 74xA Inventory

- highest remaining source-shape pressure:
  - `tools/selfhost/*` top-level alias/wrapper drift after folder split
  - `lang/src/runner/*` top-level wrapper drift is smaller and follows after shell cleanup
- lower priority:
  - broad rust runner recut is already landed
  - blocker-only follow-up is closed

## 74xA2 Ranking

1. `75x selfhost top-level alias canonicalization`
2. `76x .hako top-level facade canonicalization`
3. `77x caller-zero alias/archive sweep rerun`

## 74xB1 Decision

- next lane is `phase-75x selfhost top-level alias canonicalization`
