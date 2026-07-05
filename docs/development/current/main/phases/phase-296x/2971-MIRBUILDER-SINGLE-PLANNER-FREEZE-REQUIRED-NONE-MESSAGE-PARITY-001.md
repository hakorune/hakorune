---
Status: Landed
Date: 2026-07-05
Scope: single_planner planner-required None freeze message DTO parity slice.
---

# MIRBUILDER-SINGLE-PLANNER-FREEZE-REQUIRED-NONE-MESSAGE-PARITY-001

## Decision

Land parity for the planner-required None freeze message DTO.

```text
selected_owner=single_planner_freeze_required_none_message.authority_facade
input_contract=BackendSafeSinglePlannerFreezeRequiredNoneMessageTokenSnapshotV1
rust_oracle_symbol=freeze_planner_required_none message/hint formatting
rust_source=src/mir/builder/control_flow/plan/single_planner/rules.rs
hako_source=lang/src/compiler/lib/single_planner_freeze_required_none_message.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_single_planner_freeze_required_none_message_parity_gate.sh
oracle_rows=3
```

This facade owns only message/hint formatting. Reject-detail retrieval and
`Freeze` construction remain Rust.

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
MIRBUILDER-SINGLE-PLANNER-FREEZE-REQUIRED-NONE-MESSAGE-HAKOADOPTED-DECISION-001
```
