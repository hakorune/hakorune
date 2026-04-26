---
Status: Landed
Date: 2026-04-27
Scope: normalization decline/fallback wording review
Related:
  - src/mir/builder/control_flow/normalization/README.md
  - src/mir/builder/control_flow/normalization/plan_box.rs
  - src/mir/builder/control_flow/normalization/suffix_router_box.rs
  - docs/development/current/main/phases/phase-291x/291x-431-normalized-shadow-loop-if-exit-wording-cleanup-card.md
---

# 291x-432: Normalization Decline/Fallback Wording Review

## Goal

Review normalization wording under the closeout cap.

This is a BoxShape review. No behavior changed.

## Findings

Normalization still has a real default-path behavior:

```text
NormalizationPlanBox returns Ok(None)
  -> this route declines the shape
  -> caller continues normal/default MIR lowering
```

That behavior is valid. The wording problem is narrower: active docs and
comments still describe this as `legacy fallback` / `use legacy`, which makes
the current owner boundary look older than it is.

Affected files:

- `normalization/README.md`
- `normalization/plan_box.rs`
- `normalization/suffix_router_box.rs`

Keep the stable debug tag unchanged:

```text
[normalization/fallback]
```

The tag is an observability token. Renaming it would be a separate logging
contract change.

## Decision

Clean wording only:

```text
legacy fallback -> route decline to default MIR lowering
use legacy      -> caller continues default lowering
```

Do not change:

- `Ok(None)` behavior
- non-strict execution-error behavior
- `[normalization/fallback]` debug tag
- `PlanKind::LoopOnly`
- route order
- accepted loop shapes

## Next Cleanup

`291x-433`: normalization default-path wording cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "legacy fallback|use legacy|Legacy path" \
  src/mir/builder/control_flow/normalization -g '*.rs' -g '*.md'
```

The final `rg` should produce no output.
