---
Status: Landed
Date: 2026-04-27
Scope: normalization default-path wording cleanup
Related:
  - src/mir/builder/control_flow/normalization/README.md
  - src/mir/builder/control_flow/normalization/plan_box.rs
  - src/mir/builder/control_flow/normalization/suffix_router_box.rs
  - docs/development/current/main/phases/phase-291x/291x-432-normalization-decline-wording-review-card.md
---

# 291x-433: Normalization Default-Path Wording Cleanup

## Goal

Clean normalization wording from legacy fallback terminology to default-path
route-decline terminology.

This is a BoxShape cleanup. No behavior changed.

## Change

Updated normalization comments/docs:

```text
legacy fallback -> default MIR lowering / default path
use legacy      -> route decline to caller default lowering
```

Kept the stable debug tag unchanged:

```text
[normalization/fallback]
```

## Preserved Behavior

- `Ok(None)` behavior is unchanged.
- non-strict execution-error behavior is unchanged.
- `[normalization/fallback]` debug tag is unchanged.
- `PlanKind::LoopOnly` is unchanged.
- route order is unchanged.
- accepted loop shapes are unchanged.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "legacy fallback|use legacy|Legacy path" \
  src/mir/builder/control_flow/normalization -g '*.rs' -g '*.md'
```

## Next Cleanup

Review loop-if-break-continue placeholder/fossil boundary under the closeout
cap.
