# 296x-834 BACKEND-METHOD-NAME-PROOF-AUDIT-001

Status: Landed
Date: 2026-06-16

## Purpose

Resolve the apparent mismatch between:

```text
backend_method_name_special_case_enabled=0
```

and the existing flattened-nested backend consumer tables:

```text
READ_METHOD_TO_FIELD
WRITE_METHODS
```

This row is an audit and vocabulary clarification. It does not rewrite backend
lowering.

## Finding

The method-name tables in `src/llvm_py/instructions/flattened_nested_fields.py`
are not a new generic backend route inference path.

They are a guarded semantic map inside the existing flattened-nested ObjectPlan
consumer:

```text
receiver must be flattened_nested_field_state
nested_object must be HakoAllocObjectLifecycleAlignmentResult
owner_handle must be present
method map only maps the nested object's visible methods to flattened fields
generic method calls still fall through to normal dispatch
```

The self-proof therefore means:

```text
backend_method_name_special_case_enabled=0
  = no generic backend direct-call / route target selection from method names
```

It does not mean:

```text
no guarded semantic method map exists inside an already-authorized backend
ObjectPlan consumer
```

## Result

```text
output_contract=hako-backend-method-name-proof-audit-v0
source_evidence=296x-831,296x-833

flattened_nested_method_tables_classified=1
flattened_nested_read_method_map_count=4
flattened_nested_write_method_map_count=3
guarded_flattened_nested_method_semantic_map_allowed=1

generic_backend_method_name_route_inference_count=0
backend_method_name_special_case_scope=generic_backend_route_inference
backend_method_name_special_case_selfproof_scope_clarified=1

flattened_nested_receiver_guard_required=1
flattened_nested_objectplan_consumer_required=1
method_call_route_enabled_is_existing_flattened_nested_consumer=1

backend_lowering_changed=0
implementation_started=0
product_default_changed=0

selected_next=ARRAY-RECEIVER-RESIDENCE-PROOF-CHAIN-RETIRE-INVENTORY-001
summary=ok
```

## Stop Line

```text
do not remove READ_METHOD_TO_FIELD / WRITE_METHODS as a drive-by cleanup
do not treat guarded flattened-nested semantic maps as generic route inference
do not introduce new method-name route selection outside an ObjectPlan consumer
do not change flattened_nested_fields.py in this audit row
```
