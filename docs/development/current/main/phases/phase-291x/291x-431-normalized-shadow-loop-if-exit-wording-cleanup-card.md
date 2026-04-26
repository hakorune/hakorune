---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow loop-if-exit route-decline wording cleanup
Related:
  - src/mir/control_tree/normalized_shadow/common/loop_if_exit_contract.rs
  - docs/development/current/main/phases/phase-291x/291x-430-cleanup-closeout-granularity-card.md
---

# 291x-431: Normalized-Shadow Loop-If-Exit Wording Cleanup

## Goal

Align loop-if-exit contract wording with the normalized-shadow route-decline
contract.

This is a BoxShape cleanup. No behavior changed.

## Change

Updated `loop_if_exit_contract.rs` comments from fallback terminology to
route-decline terminology:

```text
OutOfScopeReason = explicit out-of-scope route-decline cases
Err(OutOfScopeReason) = route-decline reason
```

## Preserved Behavior

- `LoopIfExitShape` is unchanged.
- `LoopIfExitThen` is unchanged.
- `OutOfScopeReason` variants are unchanged.
- Validation rules are unchanged.
- Accepted loop-if-exit shapes are unchanged.
- Generated JoinIR is unchanged.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "graceful fallback|fall back to Ok\\(None\\)" \
  src/mir/control_tree/normalized_shadow/common/loop_if_exit_contract.rs
```

## Next Cleanup

Review normalization decline/fallback wording under the closeout cap.
