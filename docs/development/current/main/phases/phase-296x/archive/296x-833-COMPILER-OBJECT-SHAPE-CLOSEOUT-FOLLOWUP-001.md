# 296x-833 COMPILER-OBJECT-SHAPE-CLOSEOUT-FOLLOWUP-001

Status: Landed
Date: 2026-06-16

## Purpose

Close the safe residue found after `COMPILER-OBJECT-SHAPE-CLOSEOUT-001`
without reopening object-shape implementation.

This row is a closeout follow-up. It may align documentation and guards, but it
must not change runtime behavior, backend lowering behavior, or retire legacy
proof-chain code.

## Scope

Included:

- align the ObjectStoragePlan SSOT enum with the current code vocabulary
- make the compiler object-shape closeout guard run its source sub-guards
- park backend method-name proof drift as an investigation task
- park array receiver proof-chain retirement as a scoped project

Excluded:

- changing `src/llvm_py/instructions/flattened_nested_fields.py`
- deleting `src/array_receiver_representation_source.rs`
- touching live `src/mir/array_receiver_proof.rs`
- changing product runtime, backend execution, or ObjectPlan execution state

## Result

```text
output_contract=hako-compiler-object-shape-closeout-followup-v0
source_evidence=296x-832,post-closeout-audit

object_storage_plan_ssot_enum_aligned_with_code=1
object_storage_plan_variant_count=7
flattened_nested_fields_variant_documented=1

closeout_guard_executes_subguards=1
closeout_subguard_count=7

backend_method_name_selfproof_investigation_required=1
proof_chain_retire_project_required=1
risky_code_change_count=0

product_default_changed=0
backend_lowering_changed=0
object_plan_execution_enabled=0
standalone_publication_plan_enabled=0

selected_next=MIMALLOC-FRESH-FRONT-SELECTION-001
summary=ok
```

## Follow-Up Tasks

### BACKEND-METHOD-NAME-PROOF-AUDIT-001

Investigate whether the flattened-nested backend method-name tables are
semantic field maps or forbidden route inference.

Acceptance:

```text
flattened_nested_method_tables_classified=1
route_inference_from_method_name_count=<n>
semantic_field_map_count=<n>
backend_method_name_special_case_selfproof_updated_if_needed=1
implementation_started=0
```

Stop lines:

```text
do not drive-by rewrite flattened_nested_fields.py
do not claim backend_method_name_special_case_enabled=0 if route inference exists
do not remove the flattened-nested consumer without a replacement plan
```

### ARRAY-RECEIVER-RESIDENCE-PROOF-CHAIN-RETIRE-INVENTORY-001

Inventory the legacy array receiver proof-chain module and all docs/guards that
still reference it before any retirement.

Acceptance:

```text
array_receiver_representation_source_consumers_classified=1
guard_reference_count=<n>
doc_reference_count=<n>
retire_gate_required=1
implementation_started=0
```

Stop lines:

```text
do not delete src/array_receiver_representation_source.rs in this row
do not touch live src/mir/array_receiver_proof.rs
do not remove guard references before the retire gate exists
do not collapse ArrayReceiver proof-chain residue into ObjectPlan by rename only
```
