Status: Done
Date: 2026-06-18
Scope: scaffold the in-main-crate MIR JSON export model boundary
Related:
  - docs/development/current/main/phases/phase-296x/296x-1100-BUILD-MIR-JSON-EMIT-BOUNDARY-SSOT-001.md
  - src/runner/mir_json_export_model.rs
  - src/runner/mir_json_emit

# BUILD-MIR-JSON-EXPORT-MODEL-SCAFFOLD-001

## Purpose

Add the first passive export-model vocabulary in the main crate so future
MIR JSON crate extraction has a named seam.

## Change

```text
new_owner=src/runner/mir_json_export_model.rs
new_vocabulary=MirJsonExportSchema,MirJsonExportRootKind,MirJsonExportModelSummary
mir_json_emit_behavior_changed=0
future_crate_created=0
```

The scaffold is intentionally not wired into `mir_json_emit` yet. It names the
future boundary without moving schema construction, route metadata emitters, or
file IO.

## Verification

```text
cargo_check=green
runner_export_model_unit_tests=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-mir-json-export-model-scaffold-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
future_crate_created=0
mir_json_schema_changed=0
ny_llvmc_route_changed=0
runner_route_changed=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-JSON-EXPORT-MODEL-ROOT-SUMMARY-WIRING-001
purpose=optionally wire passive summary construction without changing emitted JSON
```
