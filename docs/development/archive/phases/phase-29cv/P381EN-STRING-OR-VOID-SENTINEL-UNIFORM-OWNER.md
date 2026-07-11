# P381EN String-Or-Void Sentinel Uniform Owner

Date: 2026-05-06
Scope: move string-or-void sentinel direct-call definitions to the uniform MIR owner.

## Context

`GenericStringOrVoidSentinelBody` was the last retired capsule still serialized
as `definition_owner=module_generic`. Earlier cleanup made the Stage0 selected
same-module planning and direct-call emission paths owner-neutral:

- same-module definition prepass uses a shared definition helper
- module-generic/i64 and uniform-MIR metadata helpers use owner-family names
- direct global-call readiness is centralized before call emission

The sentinel body still carries its behavior through MIR-owned facts:
`proof=typed_global_call_generic_string_or_void_sentinel`,
`return_shape=string_handle_or_null`, and `result_origin=string`.

## Change

Moved `GlobalCallProof::GenericStringOrVoidSentinel` to
`definition_owner=uniform_mir`.

Updated route-plan and runner MIR JSON tests to pin:

- `definition_owner=uniform_mir`
- `emit_trace_consumer=mir_call_global_uniform_mir_emit`

Preserved proof, return shape, result origin, and value demand.

## Verification

Commands:

```bash
cargo test -q mir::global_call_route_plan::tests::void_sentinel
cargo test -q runner::mir_json_emit::tests::global_call_routes::void_sentinel
cargo test -q mir::global_call_route_plan::tests::hostbridge
cargo test -q mir::global_call_route_plan::tests::runtime_methods
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

String-or-void sentinel direct-call definitions now use the uniform MIR owner
while continuing to lower through the shared same-module function-emitter path.
