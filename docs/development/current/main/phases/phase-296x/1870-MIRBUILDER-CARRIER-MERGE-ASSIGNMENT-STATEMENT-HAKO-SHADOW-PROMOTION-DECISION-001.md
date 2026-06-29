# 1870 - MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PROMOTION-DECISION-001

## Token

```text
MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PROMOTION-DECISION-001
```

## Purpose

Close the carrier-merge assignment HakoShadow stage-state with a
machine-checkable promotion decision.

The HakoShadow parity result is green and the projector now participates in the
shared stage-state inventory. This promotes the stage to `HakoMainline` while
keeping Python as the oracle/bootstrap reference.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    carrier-merge-assignment-hako-shadow-promotion-decision-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_merge_assignment_hako_shadow_promotion_decision.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_merge_assignment_hako_shadow_promotion_decision_guard.sh
```

## Acceptance

```text
hako_shadow_parity_result_consumed = 1
stage_state_inventory_consumed = 1
current_stage = HakoShadow
selected_stage = HakoMainline
decision = Promote
reason_token = CarrierMergeAssignmentHakoShadowParityGreen
promotion_token_explicit = 1
retirement_token_explicit = 1
python_oracle_retained = 1
hako_shadow_retained = 1
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
hako_adopted = 0
python_semantic_projector_growth = 0
```

## Recommended Next Tasks

```text
1. MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-NATIVE-SOURCE-SEED-001
   Materialize a native Hako source seed for the HakoMainline carrier-merge
   assignment stage.
```

## Non-Claims

```text
no HakoAdopted decision
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new ABI
```
