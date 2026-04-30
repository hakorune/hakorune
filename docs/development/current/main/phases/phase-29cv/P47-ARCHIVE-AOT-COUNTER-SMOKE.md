---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive the historical JIT/AOT counter root smoke wrapper.
Related:
  - tools/smokes/jit-migration-plan.md
  - tools/archive/manual-smokes/README.md
---

# P47 Archive AOT Counter Smoke

## Goal

Continue root smoke cleanup with the next low-risk singleton wrapper.

## Decision

Move `tools/aot_counter_smoke.sh` to
`tools/archive/manual-smokes/aot_counter_smoke.sh`.

The only current-tree reference is `tools/smokes/jit-migration-plan.md`, which
is itself a historical migration note and already lists this smoke as an
archive target.

## Non-goals

- do not archive broad build helpers such as `tools/build_aot.sh`
- do not change AOT builder behavior
- do not run the historical JIT/AOT smoke as acceptance

## Acceptance

```bash
bash -n tools/archive/manual-smokes/aot_counter_smoke.sh
! rg -g '!docs/development/current/main/phases/phase-29cv/P47-ARCHIVE-AOT-COUNTER-SMOKE.md' --fixed-strings 'tools/aot_counter_smoke.sh' docs/development/current/main docs/development/testing tools src lang Makefile dev README.md README.ja.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
