---
Status: Closed
Date: 2026-06-28
Card: MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-SHADOW-PROMOTION-DECISION-001
---

# MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-SHADOW-PROMOTION-DECISION-001

## Summary

Close the `FunctionRegionStackPop` HakoShadow stage-state with a
machine-checkable promotion decision. The shadow projector parity is already
green, and the roadmap names HakoShadow promotion / retirement token closure
as the next meaningful checkpoint. This card promotes the stage to
`HakoMainline` while keeping Python as the explicit oracle/bootstrap reference.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Authority

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-derived-hako-verifier-result-v0.json
docs/development/current/main/phases/phase-296x/296x-1767-HAKO-SHADOW-PROJECTOR-STAGE-STATE-INVENTORY-001.md
docs/development/current/main/phases/phase-296x/296x-1747-MIRBUILDER-FUNCTION-REGION-STACK-POP-DERIVED-HAKO-ARTIFACT-001.md
docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
tools/checks/rust_lifecycle_mirbuilder_function_region_stack_pop_derived_artifact_guard.sh
tools/checks/rust_lifecycle_hako_shadow_projector_stage_state_inventory_guard.sh
tools/checks/rust_lifecycle_mirbuilder_function_region_stack_pop_hako_shadow_promotion_decision_guard.sh
```

## Required Delta

```text
consume the FunctionRegionStackPop derived-artifact verifier result as
promotion evidence
consume the HakoShadow stage-state inventory as the closure vocabulary
select HakoMainline for the FunctionRegionStackPop stage if parity remains green
keep Python as bootstrap/oracle and preserve explicit tokens
keep the promotion narrow and stage-scoped
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_function_region_stack_pop_derived_artifact_guard.sh = green
bash tools/checks/rust_lifecycle_hako_shadow_projector_stage_state_inventory_guard.sh = green
bash tools/checks/rust_lifecycle_mirbuilder_function_region_stack_pop_hako_shadow_promotion_decision_guard.sh = green
current_stage = HakoShadow
selected_stage = HakoMainline
python_oracle_retained = 1
hako_shadow_retained = 1
promotion_token_explicit = 1
retirement_token_explicit = 1
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```

## Non-Claims

```text
HakoAdopted = 0
Source Selfhost = 0
Rust bootstrap removal = 0
Python semantic projector growth = 0
new ABI = 0
runtime fallback = 0
```

## Closeout

```text
output_contract=rust-lifecycle-function-region-stack-pop-hako-shadow-promotion-decision-v0
family_id=hakorune_mir_builder::function_region_stack_pop
stage_id=function_region_stack_pop
current_stage=HakoShadow
selected_stage=HakoMainline
decision=Promote
reason_token=FunctionRegionStackPopHakoShadowParityGreen
python_oracle_retained=1
hako_shadow_retained=1
promotion_token=FunctionRegionStackPopHakoShadowPromotionTokenV1
retirement_token=FunctionRegionStackPopHakoShadowRetirementTokenV1
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
summary=ok
```

## Closed

```text
closed_by=MIRBUILDER-SLOT-REGISTRY-RELEASE-HAKO-SHADOW-PROMOTION-DECISION-001
support_lane_promotion_closed_as_provenance=1
```
