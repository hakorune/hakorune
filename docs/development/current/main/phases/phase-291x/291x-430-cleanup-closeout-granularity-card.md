---
Status: Landed
Date: 2026-04-27
Scope: phase-291x cleanup closeout granularity
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-429-normalized-shadow-if-only-fossil-boundary-note-card.md
---

# 291x-430: Cleanup Closeout Granularity

## Goal

Prevent the compiler-cleanliness lane from becoming open-ended cleanup.

This is a planning / task-granularity card. No behavior changed.

## Closeout Rule

Finish the current normalized-shadow / normalization cleanup burst with at most
five more small cards:

1. loop-if-exit route-decline wording cleanup
2. normalization decline/fallback wording review
3. normalization README/status sync if still stale
4. placeholder/fossil boundary review for loop-if-break-continue
5. closeout review

If a candidate requires more than comment/docs/surface cleanup, defer it to a
new lane instead of extending this burst.

## Current Findings

Immediate safe next seam:

```text
src/mir/control_tree/normalized_shadow/common/loop_if_exit_contract.rs
```

It still uses "graceful fallback" wording for out-of-scope route declines. The
module is a contract SSOT, so wording should match the normalized-shadow
contract language used by ANF:

```text
Ok(None) route decline
not fallback semantics
```

## Decision

Take the loop-if-exit wording cleanup first.

Do not change:

- `LoopIfExitShape`
- `LoopIfExitThen`
- `OutOfScopeReason`
- validation rules
- accepted loop-if-exit shapes
- generated JoinIR

## Next Cleanup

`291x-431`: normalized-shadow loop-if-exit route-decline wording cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "graceful fallback|fall back to Ok\\(None\\)" \
  src/mir/control_tree/normalized_shadow/common/loop_if_exit_contract.rs
```

The final `rg` should produce no output.
