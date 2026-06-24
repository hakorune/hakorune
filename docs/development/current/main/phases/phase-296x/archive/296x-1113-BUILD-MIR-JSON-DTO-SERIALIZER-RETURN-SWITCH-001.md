Status: Done
Date: 2026-06-18
Scope: switch MIR JSON root builder return path to DTO serializer output
Related:
  - docs/development/current/main/phases/phase-296x/296x-1112-BUILD-MIR-JSON-DTO-SERIALIZER-PARITY-WIRING-001.md
  - src/runner/mir_json_export_model.rs
  - src/runner/mir_json_emit/root.rs

# BUILD-MIR-JSON-DTO-SERIALIZER-RETURN-SWITCH-001

## Purpose

Move `build_mir_json_root` one step closer to future crate extraction by making
the passive DTO serializer own the returned JSON payload.

## Change

```text
serializer_payload_returned_from_root_builder=1
serializer_parity_debug_assert=1
legacy_root_builder_payload_kept_as_parity_oracle=1
json_output_changed=0
future_crate_created=0
```

The root builder still projects from `MirModule` in the main crate. It now
constructs the JSON-ready DTO, serializes it, checks parity against the legacy
root payload, and returns the serializer payload.

## Verification

```text
cargo_check=green
runner_export_model_unit_tests=green
mir_json_root_smoke=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-mir-json-dto-serializer-return-switch-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
future_crate_created=0
serializer_reads_mir_directly=0
root_projection_still_main_crate=1

summary=ok
```

## Next

```text
next_task=BUILD-MIR-JSON-DTO-SERIALIZER-CLOSEOUT-001
purpose=close the serializer seam and select the next crate-split boundary
```
