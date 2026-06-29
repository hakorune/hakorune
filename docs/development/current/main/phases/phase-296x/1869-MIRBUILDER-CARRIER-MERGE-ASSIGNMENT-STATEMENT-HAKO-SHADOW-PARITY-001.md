# 1869 - MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PARITY-001

## Token

```text
MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PARITY-001
```

## Purpose

Add a HakoShadow projector for the contracted carrier-merge assignment
mutation frame.

This is a support-lane parity card. It does not emit derived Hako artifacts,
materialize a native source seed, run HakoAdopted, or claim Source Selfhost.

## Output

```text
projector:
  lang/src/compiler/lib/carrier_merge_assignment_projector.hako

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-merge-assignment-hako-shadow-result-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_merge_assignment_statement_hako_shadow_parity_guard.sh
```

## Acceptance

```text
contract_fixture_consumed = 1
hako_projector_verify_green = 1
canonical_json_parity = 1
stage_state_tokens_present = 1
python_oracle_retained_as_contract_fixture = 1
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Recommended Next Tasks

```text
1. MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-SHADOW-PROMOTION-DECISION-001
   Decide whether the HakoShadow projector can become HakoMainline.
```

## Non-Claims

```text
no Hako generation
no HakoAdopted decision
no native source seed
no Source Selfhost claim
no route repair
```
