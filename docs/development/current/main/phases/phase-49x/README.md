---
Status: Active
Date: 2026-04-04
Scope: rewrite the remaining current docs and helper comments that still read like `rust-vm` is a day-to-day owner, keep compat/proof routes explicit, and preserve the direct/core mainline narrative.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-49x/49x-90-legacy-wording-compat-route-cleanup-ssot.md
  - docs/development/current/main/phases/phase-49x/49x-91-task-board.md
---

# Phase 49x: Legacy Wording / Compat Route Cleanup

## Goal

- inventory the current docs/guides/comments that still read like `rust-vm` is a day-to-day owner
- rewrite those surfaces so `rust-vm` is clearly compat/proof keep only
- keep the `stage-a` branch and proof smoke routes explicit and non-growing
- preserve `cargo check --bin hakorune` and `git diff --check`

## Plain Reading

- phase-48x cleaned smoke/source routes and landed the direct/core mainline split.
- phase-49x is the follow-up pass for wording, examples, and helper comments that still imply vm is a default owner.
- the remaining live route surface is mostly explicit keep; the cleanup target is stale narration, not proof route deletion.

## Success Conditions

- current docs no longer read like `--backend vm` is a day-to-day default owner
- compat/proof keeps stay explicit
- stage-a remains labeled as compat-only in the current docs and helper commentary
- `cargo check --bin hakorune` stays green

## Failure Patterns

- reintroducing default-vm phrasing in current docs
- widening compat keeps while cleaning prose
- deleting proof-only smoke helpers before replacement or classification

## Big Tasks

1. `49xA` current-doc inventory
   - `49xA1` current-doc stale wording inventory lock (landed)
   - `49xA2` top-level docs compat wording rewrite (active)
2. `49xB` guide cleanup
   - `49xB1` current guides compat wording rewrite
   - `49xB2` example-command stale-route sweep
3. `49xC` runtime / helper clarity
   - `49xC1` runtime `stage-a` compat label lock
   - `49xC2` helper comment stale-route cleanup
4. `49xD` proof / closeout
   - `49xD1` proof / closeout

## Boundaries

- proof-only gates remain proof-only
- compat-only routes remain explicit and non-growing
- direct/core mainline routes stay the default narrative
- historical docs may still mention vm paths, but current docs must not read as if vm owns the mainline
