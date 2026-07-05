---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for single_planner freeze-required-none message DTO.
---

# MIRBUILDER-SINGLE-PLANNER-FREEZE-REQUIRED-NONE-MESSAGE-HAKOADOPTED-DECISION-001

## Decision

Adopt the single_planner freeze-required-none message facade.

```text
decision=HakoAdoptedScoped
adopted_owner=single_planner_freeze_required_none_message.authority_facade
input_contract=BackendSafeSinglePlannerFreezeRequiredNoneMessageTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/single_planner_freeze_required_none_message.hako
```

This adopts only message/hint formatting. Reject-detail retrieval and `Freeze`
construction remain Rust.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-single-planner-freeze-required-none-message-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/single_planner_freeze_required_none_message.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_single_planner_freeze_required_none_message_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_single_planner_freeze_required_none_message_hako_adoption_decision_guard.sh
oracle_rows=3
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
reject_detail_retrieval_migrated=0
freeze_construction_migrated=0
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
MIRBUILDER-PLAN-TRACK-NEXT-PILOT-SELECTION-007
```
