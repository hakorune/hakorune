---
Status: SSOT
Date: 2026-04-13
Scope: current lane / blocker / next pointer だけを置く薄い mirror。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Self Current Task — Now (main)

## Current

- current implementation lane: `phase163x primitive and user-box fast path`
- sibling guardrail lane: `phase137x main kilo reopen selection`
- immediate next: `semantic simplification bundle`
- immediate follow-on: `memory-effect layer`
- top queued cut: `S2 first SCCP propagation widening beyond direct Compare`

## Landing Snapshot

- concurrency manual sync:
  - `docs/reference/concurrency/semantics.md` / `lock_scoped_worker_local.md` / pre-selfhost SSOT now point to the current `task_scope` / `joinAll()` / `failureReport()` contract
- `phase255x` is landed:
  - `joinAll()` now returns `Err(TaskJoinTimeout: timed out after Nms)` when bounded join hits deadline without a latched first failure
- `phase254x` is landed:
  - explicit-scope aggregate failures now live on `TaskGroupBox.failureReport()` as `[first_failure, additional_failures...]`
- latest semantic simplification cut:
  - copied-constant `Branch` terminators, constant `Compare` instructions, and empty trampoline jump-threading now fold before CFG merge
  - branch arms may now thread through an empty trampoline into a final block when the final PHIs can be trivially rewritten to the branching predecessor
  - branch arms may also drop dead edge-args while threading through an empty trampoline into a PHI-free final target

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/reference/concurrency/semantics.md`
4. `docs/development/current/main/phases/phase-163x/README.md`
5. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
git diff --check
```
