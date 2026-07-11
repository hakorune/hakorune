---
Status: Landed
Date: 2026-07-05
Scope: single_planner promotion hint tag DTO parity slice.
---

# MIRBUILDER-SINGLE-PLANNER-PROMOTION-HINT-TAG-PARITY-001

## Decision

Land parity for the promotion hint tag DTO used by `try_build_outcome`.

```text
selected_owner=single_planner_promotion_hint_tag.authority_facade
input_contract=BackendSafeSinglePlannerPromotionHintTagTokenSnapshotV1
rust_oracle_symbol=promotion_hint_tag / emit_loop_break_promotion_hint_tag
rust_source=src/mir/builder/control_flow/plan/single_planner/rules.rs
hako_source=lang/src/compiler/lib/single_planner_promotion_hint_tag.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_single_planner_promotion_hint_tag_parity_gate.sh
oracle_rows=4
```

This facade owns only shape-token to hint-tag DTO formatting. Shape extraction
and log emission remain Rust.

## Non-Claims

```text
source_selfhost_claim=0
promotion_shape_extraction_migrated=0
log_emission_migrated=0
route_execution_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
hako_generation=0
runtime_fallback=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-SINGLE-PLANNER-PROMOTION-HINT-TAG-HAKOADOPTED-DECISION-001
```
