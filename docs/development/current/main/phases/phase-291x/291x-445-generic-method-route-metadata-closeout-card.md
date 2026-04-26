---
Status: Landed
Date: 2026-04-27
Scope: generic method route metadata string-thinning closeout
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-442-next-lane-selection-card.md
  - docs/development/current/main/phases/phase-291x/291x-443-generic-method-route-metadata-string-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-444-generic-method-route-kind-metadata-helper-card.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
---

# 291x-445: Generic Method Route Metadata Closeout

## Goal

Close the GenericMethodRoute metadata string-thinning slice and prevent it from
turning into an open-ended `.inc` migration.

This is a closeout card. No behavior changed.

## Closed In This Slice

- `291x-442` selected generic method route metadata string thinning as the next
  compiler-cleanliness lane.
- `291x-443` inventoried remaining method-string-derived metadata.
- `291x-444` moved `route_id`, `emit_kind`, and `effects` derivation from raw
  `method` string matching to decided `GenericMethodRouteKind` helpers.

## Final Boundary

The current MIR metadata boundary is:

```text
GenericMethodRoute
  -> route_kind owns route_id / emit_kind / effects helper tokens
  -> core_method carries manifest-backed op/proof/tier when available
  -> method / box_name remain JSON compatibility/debug fields
```

JSON token spellings are intentionally unchanged. `.inc` readers were not
changed in this slice.

## Deferred To New Lanes

- `.inc` generated enum/table consumer migration
- `route_id` retirement or replacement with generated enum ids
- removal of compatibility `method` / `box_name` JSON fields
- MapGet proof/lowering or hot-inline work
- launcher/stage1 facade cleanup

## Decision

Stop this slice here. The next card should choose the next
compiler-cleanliness lane instead of continuing `.inc` migration by momentum.

## Guards

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS.
