# 1872 - MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt the carrier-merge assignment mutation-frame leaf semantic owner after its
native Hako source seed is materialized outside the generated artifact tree.

This is a narrow leaf-family adoption decision. It does not claim Source
Selfhost, does not delete Rust, and does not claim full assignment lowering or
full VariableContext map semantics.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-merge-assignment-hako-adoption-decision-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_merge_assignment_hako_adoption_decision_guard.sh
```

## Result

```text
decision = Adopt
selected_next_route = native_hako_source_owner
native_source_owner_present = 1
generator_overwrite_guard = 1
source_selfhost_claim = 0
```

## Acceptance

```text
CarrierMergeAssignment native source seed guard green
CarrierMergeAssignment HakoMainline promotion evidence consumed
CarrierMergeAssignment mutation-frame contract consumed
decision = Adopt
native_hako_source_owner_present = 1
generator_overwrite_guard = 1
rust_bootstrap_retained = 1
rust_oracle_retained = 1
manual_family_selection = 0
support_lane_projector_as_hako_adoption_candidate = 0
source_selfhost_claim = 0
rust_deletion = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no generated artifact edit authority
no runner semantic ownership
no full assignment lowering claim
no full VariableContext map-semantics claim
```
