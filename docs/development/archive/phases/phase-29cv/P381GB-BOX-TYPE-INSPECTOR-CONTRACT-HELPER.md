# P381GB BoxTypeInspector Contract Helper

Date: 2026-05-06
Scope: make the BoxTypeInspector describe body module own its proof/return contract pair.

## Problem

`BoxTypeInspectorDescribeBody` is already retired as a public target shape and
uses `definition_owner=uniform_mir`.

The last T5 body-handling cleanup was the same SSOT shape as PatternUtil:

- the BoxTypeInspector describe module owns candidate and reject analysis
- the top-level global-call classifier still repeated the proof/return pair
  when publishing the accepted direct contract

The accepted route contract should live with the body recognizer that proves it.

## Change

The BoxTypeInspector describe module now exposes:

```text
box_type_inspector_describe_classification()
```

The helper returns the direct contract:

```text
proof=typed_global_call_box_type_inspector_describe
return_shape=map_handle
value_demand=runtime_i64_or_handle
```

The top-level classifier now asks the body module for that classification after
the reject pass succeeds.

## Boundary

Allowed:

- move only proof/return ownership
- preserve candidate and reject behavior
- preserve `definition_owner=uniform_mir`
- preserve `result_origin=map_birth`

Not allowed:

- merge BoxTypeInspector describe with MIR-schema map construction
- widen source-owner matching
- add C fallback logic

## Verification

```bash
cargo test -q box_type_inspector_describe
cargo test -q runner::mir_json_emit::tests::global_call_routes::box_type_inspector_describe
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```
