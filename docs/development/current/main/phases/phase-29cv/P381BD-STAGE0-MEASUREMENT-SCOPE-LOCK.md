---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: docs-only Stage0 measurement scope lock for capsule-exit progress metrics
Related:
  - docs/development/current/main/phases/phase-29cv/P381BC-STAGE0-CAPSULE-EXIT-TASK-MAP.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - src/mir/global_call_route_plan.rs
  - src/mir/global_call_route_plan/
  - lang/c-abi/shims/
  - tools/smokes/v2/
---

# P381BD: Stage0 Measurement Scope Lock

## Problem

The Stage0 cleanup lane needs a stable size metric before capsule retirement,
`.inc` consolidation, or smoke pruning can use file/line counts as progress
evidence.

The previous `src/mir Stage0` count was not reproducible because it mixed
possible owner files, tests, and surrounding route surfaces without an explicit
file list. That makes later cleanup look better or worse depending on the
counter.

## Decision

Use three separate measurement scopes. Do not merge them into one Stage0 total.

### Scope A: Stage0 C Shim Surface

Purpose: measure C shim pressure and `.inc` consolidation progress.

Command:

```bash
find lang/c-abi/shims -maxdepth 1 -name '*.inc' -print | sort | wc -l
wc -l lang/c-abi/shims/*.inc | tail -n 1
```

Current baseline:

- 81 files
- 22,706 lines

### Scope B: Global-Call Capsule Owner Surface

Purpose: measure the Rust-side owner surface for `GlobalCallTargetShape`
capsules.

Included:

- `src/mir/global_call_route_plan.rs`
- non-test `src/mir/global_call_route_plan/*.rs`

Excluded:

- `src/mir/global_call_route_plan/tests.rs`
- `src/mir/global_call_route_plan/tests/**`
- generic-method / extern-call / runner JSON surfaces unless a future card
  explicitly opens a separate owner metric

Commands:

```bash
find src/mir/global_call_route_plan -maxdepth 1 \
  -type f -name '*.rs' ! -name 'tests.rs' | sort | wc -l
wc -l src/mir/global_call_route_plan.rs \
  $(find src/mir/global_call_route_plan -maxdepth 1 \
      -type f -name '*.rs' ! -name 'tests.rs' | sort) | tail -n 1
```

Current baseline:

- 21 files total: root module plus 20 non-test child files
- 7,173 lines

### Scope C: Smoke Script Surface

Purpose: measure smoke pruning after capsule and archive reachability work.

Use worktree count when auditing the current active tree:

```bash
find tools/smokes/v2 -type f -name '*.sh' | sort | wc -l
```

Current worktree baseline:

- 1,496 scripts

Use tracked-only count only when comparing committed history:

```bash
git ls-files tools/smokes/v2 | grep -E '\.sh$' | sort | wc -l
```

Current tracked-only baseline:

- 1,494 scripts

The delta is the two new LLVM/mainline smoke names that are still untracked in
the active worktree at this checkpoint.

## Boundary

Allowed:

- use Scope A for `.inc` cleanup progress
- use Scope B for `GlobalCallTargetShape` capsule owner progress
- use Scope C for smoke cleanup progress
- add a new scope in a future card only with exact include/exclude rules

Not allowed:

- quote one combined "Stage0 total" from mixed scopes
- use `26 files / 6,969 lines` as a progress baseline
- count tests in Scope B
- use smoke counts as proof that a smoke can be deleted

## Result

Done:

- locked the exact measurement scopes for the next capsule-exit tasks
- separated owner code metrics from smoke reachability metrics
- kept this card docs-only; no code or smoke semantics changed

Next:

1. write the uniform body-emitter contract inventory
2. then pick one capsule retirement candidate

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
