# 296x-1636 MIRBUILDER-IMMUTABLE-LEAF-BORROW-PROJECTION-001

Status: Closed
Date: 2026-06-23

## Decision

Implement immutable leaf borrow projection as a direct shape:

```text
map.immutable_leaf_projection
```

The selected pilot is:

```text
MetadataContext::value_caller(&self, value_id) -> Option<&str>
```

This is a leaf projection from `HashMap<ValueId, String>` to
`Option<StringBox>`. It does not return the aggregate map.

## Scope

Included:

```text
MetadataContext.value_origin_callers get-only read
Option<&str> -> Option<StringBox>
ValueId key as i64
MapBox storage
MIR/EXE generated artifact proof
```

Excluded:

```text
MetadataContext::record_value_caller
MetadataContext::value_origin_callers
MetadataContext::current_region_stack
VariableContext::variable_map
returned mutable borrow
```

## Acceptance

```text
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family metadata-context-value-caller --check
bash tools/checks/rust_lifecycle_metadata_context_value_caller_derived_artifact_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

The generated artifact must not contain a raw aggregate return:

```text
return ctx.value_origin_callers
```

## Next

```text
AGGREGATE-RETURNED-READ-BORROW-001
```
