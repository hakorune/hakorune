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

- current implementation lane: `phase270x closure split env scalarization owner seam`
- sibling guardrail lane: `phase137x main kilo reopen selection`
- immediate next: `closure split`
- immediate follow-on: `IPO / build-time optimization`
- top queued cut: `closure split`

## Landing Snapshot

- concurrency manual sync:
  - `docs/reference/concurrency/semantics.md` / `lock_scoped_worker_local.md` / pre-selfhost SSOT now point to the current `task_scope` / `joinAll()` / `failureReport()` contract
- `phase255x` is landed:
  - `joinAll()` now returns `Err(TaskJoinTimeout: timed out after Nms)` when bounded join hits deadline without a latched first failure
- `phase254x` is landed:
  - explicit-scope aggregate failures now live on `TaskGroupBox.failureReport()` as `[first_failure, additional_failures...]`
- `phase260x` is landed:
  - the memory-effect owner seam and stats surface now sit on their own top-level pass, and the same-block private-carrier slices are fully landed
- `phase261x` is landed:
  - the first LLVM attrs policy seam for runtime helper declarations is closed out
- `phase262x` is landed:
  - the first numeric-loop / SIMD policy seam is closed out
- `phase263x` is landed:
  - the first numeric-loop induction proof seam is closed out
- `phase264x` is landed:
  - the first numeric-loop reduction recognition proof seam is closed out
- `phase265x` is landed:
  - the LoopSimdContract owner seam now exists in code and is closed out
- `phase266x` is landed:
  - integer map loop widening is the first actual widening cut
- `phase267x` is landed:
  - integer sum reduction widening is the next actual widening cut
- `phase268x` is landed:
  - compare/select widening is the numeric lane closeout cut
- `phase269x` is landed:
  - closure split now starts with a shared capture classification owner seam
- `phase270x` is active:
  - closure split now classifies single-capture envs as scalarizable while preserving current ctor lowering
- latest semantic simplification cut:
  - copied-constant `Branch` terminators, constant `Compare` instructions, and empty trampoline jump-threading now fold before CFG merge
  - branch arms may now thread through an empty trampoline into a final block when the final PHIs can be trivially rewritten to the branching predecessor
  - branch arms may also drop dead edge-args while threading through an empty trampoline into a PHI-free final target
  - constant compare / branch folding may also follow single-input PHIs
  - semantic simplification closeout is now handed off to the memory-effect layer

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/reference/concurrency/semantics.md`
4. `docs/development/current/main/phases/phase-270x/README.md`
5. `docs/development/current/main/phases/phase-163x/README.md`
6. `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
git diff --check
```
