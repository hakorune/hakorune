Status: Done
Date: 2026-06-18
Scope: wire serializer parity assertion in MIR JSON root builder
Related:
  - docs/development/current/main/phases/phase-296x/296x-1111-BUILD-MIR-JSON-DTO-SERIALIZER-SCAFFOLD-001.md
  - src/runner/mir_json_export_model.rs
  - src/runner/mir_json_emit/root.rs

# BUILD-MIR-JSON-DTO-SERIALIZER-PARITY-WIRING-001

## Purpose

Prove the DTO serializer produces the same payload as the current root builder
before changing the returned value.

## Change

```text
serializer_called_from_root_builder=1
serializer_parity_debug_assert=1
root_builder_returns_existing_payload=1
json_output_changed=0
future_crate_created=0
```

`build_mir_json_root` now serializes the passive DTO and debug-asserts equality
with the existing root payload. It still returns the existing payload.

## Verification

```text
cargo_check=green
runner_export_model_unit_tests=green
mir_json_root_smoke=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-mir-json-dto-serializer-parity-wiring-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
future_crate_created=0
serializer_reads_mir_directly=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-JSON-DTO-SERIALIZER-RETURN-SWITCH-001
purpose=return serializer payload from build_mir_json_root after parity gate remains green
```
