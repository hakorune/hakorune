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

## Landing Snapshot

- `phase255x` is landed:
  - `joinAll()` now returns `Err(TaskJoinTimeout: timed out after Nms)` when bounded join hits deadline without a latched first failure
- `phase254x` is landed:
  - explicit-scope aggregate failures now live on `TaskGroupBox.failureReport()` as `[first_failure, additional_failures...]`
- `phase253x` is landed:
  - `joinAll()` now returns `ResultBox::Err(first_failure_payload)` from the same first-failure latch as explicit scope exit
- `phase252x` is landed:
  - explicit `task_scope` exit now surfaces the popped scope's latched `first_failure` after bounded join
- `phase251x` is landed:
  - explicit `task_scope` exit now does `cancel -> bounded join` per popped explicit scope
  - nested explicit scopes now clean up lexically instead of waiting for the outermost scope
- `phase250x` is landed:
  - closed explicit/root scopes now immediately cancel late registrations
  - `FutureBox` success is now single-assignment
- `phase249x` is landed:
  - explicit-scope first failure now cancels pending siblings with stable reason `sibling-failed`
- `phase248x` is landed:
  - explicit `task_scope` policy is pinned as `first failure cancels siblings`
- `phase247x` is landed:
  - bare `nowait` is not detached; outside explicit `task_scope` it belongs to the implicit root scope
- `phase246x` is landed:
  - `Cancelled(reason)` now exists as a narrow scope-owned future path with stable `scope-cancelled` reason
- latest semantic simplification cut:
  - copied-constant `Branch` terminators, constant `Compare` instructions, and empty trampoline jump-threading now fold before CFG merge

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-255x/README.md`
4. `docs/development/current/main/phases/phase-163x/README.md`
5. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
git diff --check
```
