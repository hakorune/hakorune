# P381DW Phase1 Uniform MIR Definition Owner

Date: 2026-05-06
Scope: move Phase 1 direct-only retired capsules to the uniform MIR definition owner.

## Context

P381DV left the selected module function definition driver thin enough to use as
the uniform multi-function emission path. The remaining Phase 1 retired
capsules still serialized `definition_owner=module_generic`, even though their
direct-call identity is now MIR-owned proof and return-contract metadata.

Those capsules do not need source-owner-specific backend ownership:

- `typed_global_call_generic_string_void_logging`
- `typed_global_call_parser_program_json`
- `typed_global_call_static_string_array`
- `typed_global_call_mir_schema_map_constructor`

The C selected-set planner already treats `definition_owner=uniform_mir` as a
same-module body definition requirement, so switching the owner does not remove
body emission or weaken fail-fast behavior.

## Change

Updated `GlobalCallProof::definition_owner()` so the four Phase 1 direct-only
proofs serialize as `uniform_mir`.

Preserved:

- proof strings
- return shapes and value demands
- result-origin metadata
- selected-set body planning through the shared Stage0 function emitter

Updated route-plan and MIR JSON tests to pin the new owner and trace consumer.

## Verification

Commands:

```bash
cargo test -q global_call_route_plan::tests
cargo test -q runner::mir_json_emit::tests::global_call_routes
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

Phase 1 direct-only retired capsules now advertise the same uniform MIR
definition owner used for generic same-module MIR body emission gaps. The
remaining `module_generic` owner surface is limited to the still-active
module-generic/source-owner cleanup lane.
