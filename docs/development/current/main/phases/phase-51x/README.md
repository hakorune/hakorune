---
Status: Active
Date: 2026-04-04
Scope: archive the canonical compat-codegen bucket and redirect live docs / aliases to the archive path.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-50x/README.md
  - docs/development/current/main/phases/phase-50x/50x-91-task-board.md
  - tools/compat/README.md
  - tools/selfhost/README.md
  - tools/archive/legacy-selfhost/compat-codegen/README.md
  - tools/archive/legacy-selfhost/README.md
---

# Phase 51x: Compat-Codegen Archival Sweep

## Goal

- inventory live compat-codegen callers and confirm the bucket is caller-free
- move `tools/compat/legacy-codegen/` into `tools/archive/legacy-selfhost/compat-codegen/`
- update live docs and aliases to point at the archive path
- keep `cargo check --bin hakorune` and `git diff --check` green

## Plain Reading

- phase-50x finished rust-vm source/archive cleanup.
- phase-51x deals with the canonical compat-codegen payload / wrapper bucket now moved under `tools/archive/legacy-selfhost/compat-codegen/`.
- the bucket is archive-later material, not a daily owner.
- the job is to keep the archive move clean, update live docs / aliases, and avoid widening compat behavior.

## Success Conditions

- live docs no longer imply `tools/compat/legacy-codegen/` is a current owner lane
- payload, transport wrapper, and pack orchestrator live under `tools/archive/legacy-selfhost/compat-codegen/`
- old live aliases point at the archive path or are retired cleanly
- source / shell changes stay green on `cargo check --bin hakorune`

## Failure Patterns

- leaving the canonical compat-codegen bucket in the live tree after caller drain
- reintroducing daily callers for proof-only / compat payloads
- moving proof-only keeps into mainline by accident

## Big Tasks

1. `51xA` inventory and classification
   - `51xA1` compat-codegen caller inventory lock
   - `51xA2` proof-only / archive-later classification
2. `51xB` archive compat-codegen bucket
   - `51xB1` archive payload / transport wrapper
   - `51xB2` archive pack orchestrator / live alias cleanup
3. `51xC` docs / alias cleanup
   - `51xC1` live docs / alias rewrite
   - `51xC2` archive README / example cleanup
4. `51xD` proof / closeout
   - `51xD1` proof / closeout

## Boundaries

- proof-only keeps remain explicit and non-growing
- compat keeps remain explicit and non-growing
- historical PyVM stays historical/direct-only
- delete only after caller drain
