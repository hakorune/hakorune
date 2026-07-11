---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for single_planner promotion hint tag DTO.
---

# MIRBUILDER-SINGLE-PLANNER-PROMOTION-HINT-TAG-HAKOADOPTED-DECISION-001

## Decision

Adopt the single_planner promotion hint tag facade.

```text
decision=HakoAdoptedScoped
adopted_owner=single_planner_promotion_hint_tag.authority_facade
input_contract=BackendSafeSinglePlannerPromotionHintTagTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/single_planner_promotion_hint_tag.hako
```

This adopts only shape-token to hint-tag DTO formatting. Shape extraction and
log emission remain Rust.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-single-planner-promotion-hint-tag-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/single_planner_promotion_hint_tag.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_single_planner_promotion_hint_tag_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_single_planner_promotion_hint_tag_hako_adoption_decision_guard.sh
oracle_rows=4
parity_status=green
```

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
MIRBUILDER-PLAN-TRACK-NEXT-PILOT-SELECTION-006
```
