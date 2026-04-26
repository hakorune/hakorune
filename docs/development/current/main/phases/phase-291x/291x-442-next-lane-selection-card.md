---
Status: Landed
Date: 2026-04-27
Scope: next compiler-cleanliness lane selection after Stage-B thinning
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-441-stageb-adapter-thinning-closeout-card.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/core_method_op.rs
---

# 291x-442: Next Lane Selection

## Goal

Choose the next compiler-cleanliness lane after closing Stage-B adapter
thinning.

This card is lane selection only. No behavior changed.

## Candidate Lanes

| Lane | Shape | Decision |
| --- | --- | --- |
| Generic method route metadata string thinning | BoxShape | select next |
| CoreMethodContract generated enum/table consumer in `.inc` | larger consumer lane | defer behind MIR metadata thinning |
| MapGet return-shape proof/lowering | proof/lowering lane | defer; no hot lowering without evidence |
| launcher/stage1 facade cleanup | selfhost facade lane | defer until CoreMethod metadata seam is cleaner |

## Decision

Select **Generic method route metadata string thinning** as the next lane.

Reason:

- The CoreMethodContract seed, generated manifest, no-growth guard, MIR
  carrier, and initial `.inc` consumers already exist.
- The next visible dirt is that `GenericMethodRoute` still derives
  `route_id`, `emit_kind`, and `effects` from the raw `method` string.
- Moving those exported metadata tokens to the decided route/core-op side is a
  bounded BoxShape step before touching `.inc` behavior.
- This keeps Hotline/CoreMethodContract migration moving without adding hot
  lowering or widening accepted method surfaces.

## Next Card

Create `291x-443-generic-method-route-metadata-string-inventory` before code
edits.

The inventory must classify:

- route metadata tokens that can be derived from `route_kind`
- route metadata tokens that should later derive from `core_method`
- compatibility fields that must remain in JSON for now
- `.inc` readers that still require existing token spellings

## Guards

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
