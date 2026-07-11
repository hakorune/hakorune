# P381EF MirSchema Map JSON Coverage

Date: 2026-05-06
Scope: add runner MIR JSON coverage for the MirSchema map-constructor uniform MIR owner.

## Context

P381DW already moved `MirSchemaMapConstructorBody` to
`definition_owner=uniform_mir`, and Rust route-plan tests pinned that contract.
The runner MIR JSON surface still lacked a direct MirSchema map-constructor
test, and the P381BE inventory row still described the selected body as
module-generic.

## Change

Added runner MIR JSON route and lowering-plan coverage for
`typed_global_call_mir_schema_map_constructor`.

Pinned:

- `target_shape=null`
- `return_shape=map_handle`
- `value_demand=runtime_i64_or_handle`
- `result_origin=map_birth`
- `definition_owner=uniform_mir`
- `emit_trace_consumer=mir_call_global_uniform_mir_emit`

Updated the active capsule inventory to describe the current uniform MIR owner
contract.

## Verification

Commands:

```bash
cargo test -q runner::mir_json_emit::tests::global_call_routes::mir_schema_map_constructor
cargo test -q mir::global_call_route_plan::tests::mir_schema_map_constructor
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

MirSchema map-constructor direct calls now have runner MIR JSON owner/trace
evidence matching the existing Rust route-plan contract.
