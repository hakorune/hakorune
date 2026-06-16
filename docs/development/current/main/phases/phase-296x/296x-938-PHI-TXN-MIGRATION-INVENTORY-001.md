# 296x-938 PHI-TXN-MIGRATION-INVENTORY-001

Status: Landed
Date: 2026-06-16

## Purpose

Inventory the remaining production PHI construction sites and choose the first
narrow migration slice toward `phi_lifecycle` / `PhiTxn`.

This row is docs-only. It does not migrate callsites yet.

## Findings

The codebase still has a small set of production PHI construction sites that
are good candidates for a narrow migration row:

```text
src/mir/utils/phi_helpers.rs
src/mir/builder/control_flow/plan/features/if_join.rs
src/mir/builder/control_flow/plan/lowerer/effect_emission.rs
src/mir/builder/control_flow/joinir/merge/loop_header_phi_builder.rs
src/mir/join_ir_vm_bridge/handlers/conditional_method_call.rs
src/mir/join_ir_vm_bridge/joinir_block_converter/handlers.rs
```

The best first migration seam is the shared helper surface:

```text
src/mir/utils/phi_helpers.rs
```

Rationale:

```text
1. One helper route fixes multiple callers.
2. It can be routed through phi_lifecycle without changing accepted shapes.
3. It is smaller than the bridge and loop-header batch-prepend paths.
```

## Decision

```text
phi_txn_inventory_done=1
phi_lifecycle_owner_preserved=1
raw_phi_generation_sites_classified=1
legacy_helper_migration_first=1
bridge_paths_deferred=1
loop_header_batch_prepend_deferred=1
```

## Selected Next

```text
selected_next=PHI-HELPERS-LIFECYCLE-ROUTE-001
```

The next row should route `src/mir/utils/phi_helpers.rs` through
`phi_lifecycle::define_phi_final` / `PhiTxn` with behavior unchanged.

## Stop Line

```text
do not mass-migrate all MirInstruction::Phi sites in one row
do not touch tests/fixtures yet
do not change accepted source shapes
do not reopen join_ir_vm_bridge as a broad rewrite row
do not change current PHI lifecycle SSOT contracts
```

## Proof Bundle

```bash
rg -n "MirInstruction::Phi\\s*\\{|add_instruction\\(MirInstruction::Phi|instructions\\.push\\(MirInstruction::Phi|emit_instruction\\(MirInstruction::Phi" src/mir/builder src/mir/join_ir_vm_bridge src/mir/join_ir src/mir/ssot -g'*.rs'
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Result

```text
output_contract=phi-txn-migration-inventory-v0
implementation_started=0
summary=ok
```
