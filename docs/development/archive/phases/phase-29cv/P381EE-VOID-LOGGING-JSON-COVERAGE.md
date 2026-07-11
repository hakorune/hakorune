# P381EE Void Logging JSON Coverage

Date: 2026-05-06
Scope: add runner MIR JSON coverage for the void-logging uniform MIR owner.

## Context

P381DW already moved `GenericStringVoidLoggingBody` to
`definition_owner=uniform_mir`, and Rust route-plan tests pinned that contract.
The runner MIR JSON surface still lacked a direct void-logging test, and the
P381BE inventory row still described the selected body as module-generic.

## Change

Added runner MIR JSON route and lowering-plan coverage for
`typed_global_call_generic_string_void_logging`.

Pinned:

- `target_shape=null`
- `return_shape=void_sentinel_i64_zero`
- `value_demand=scalar_i64`
- `definition_owner=uniform_mir`
- `emit_trace_consumer=mir_call_global_uniform_mir_emit`

Updated the active capsule inventory to describe the current uniform MIR owner
contract.

## Verification

Commands:

```bash
cargo test -q runner::mir_json_emit::tests::global_call_routes::void_logging
cargo test -q mir::global_call_route_plan::tests::void_logging
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Void logging now has runner MIR JSON owner/trace evidence matching the existing
Rust route-plan contract.
