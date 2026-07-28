Status: Done
Date: 2026-06-18
Scope: wire passive MIR JSON DTO construction in root projection
Related:
  - docs/development/current/main/phases/phase-296x/296x-1107-BUILD-MIR-JSON-DTO-SCAFFOLD-001.md
  - src/runner/mir_json_export_model.rs
  - src/runner/mir_json_emit/root.rs

# BUILD-MIR-JSON-DTO-ROOT-PROJECTION-WIRING-001

## Purpose

Construct the passive MIR JSON DTO in `build_mir_json_root` without changing the
emitted JSON payload. This proves the root/function/block/instruction DTO can be
formed from JSON-ready values while the main crate remains the MIR projection
owner.

## Change

```text
dto_document_constructed=1
dto_source=current_json_ready_values
dto_reads_mir_directly=0
json_output_changed=0
future_crate_created=0
```

The root builder now creates `MirJsonExportDocument` alongside the existing
payload. The DTO is validated through debug assertions and is not serialized or
returned.

## Verification

```text
cargo_check=green
runner_export_model_unit_tests=green
mir_json_root_smoke=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-mir-json-dto-root-projection-wiring-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
future_crate_created=0
future_crate_reads_mir_directly=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-JSON-DTO-CLOSEOUT-001
purpose=decide whether DTO wiring is sufficient before selecting the next crate split step
```
