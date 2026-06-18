Status: Done
Date: 2026-06-18
Scope: define MIR JSON emitter boundary before crate extraction
Related:
  - docs/development/current/main/phases/phase-296x/296x-1099-BUILD-MIR-JSON-EMIT-CRATE-PREFLIGHT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - src/runner/mir_json_emit

# BUILD-MIR-JSON-EMIT-BOUNDARY-SSOT-001

## Purpose

Define a thin extraction boundary for MIR JSON emission before moving any code
into a new crate.

## Decision

Split the responsibility conceptually into projection and serialization.

```text
projection_owner=main_crate
serialization_owner=future_hakorune_mir_json_emit_crate
direct_mir_reading_in_future_crate=0
```

## Boundary

### Main Crate Projection

The main crate owns all reads from `crate::mir` and active metadata producers.

```text
MirModule -> MirJsonExportModel
```

Allowed in projection:

```text
read_mir_module=1
read_function_metadata=1
read_route_plans=1
read_object_plan_metadata=1
read_cfg_extractor=1
```

Forbidden in projection:

```text
file_io=0
pretty_json_writer=0
backend_tool_execution=0
```

### Future Serialization Crate

The future crate may only consume a JSON-ready export model.

```text
MirJsonExportModel -> serde_json::Value/String/File
```

Allowed in serialization:

```text
serde_json_serialization=1
stable_schema_validation=1
string_or_file_output=1
```

Forbidden in serialization:

```text
crate_mir_dependency=0
main_crate_dependency=0
runner_dependency=0
parser_dependency=0
runtime_dependency=0
route_refresh_logic=0
```

## Why Not Trait-Read MIR Directly?

A trait-based `MirJsonModuleView` would still force the future crate to know the
full MIR and metadata surface. That would recreate the current 372 `crate::mir`
dependency shape behind trait indirection.

The cleaner seam is a DTO/export model:

```text
main crate: knows MIR, builds export model
future crate: knows export model, writes JSON
```

## First Implementation Slice

Do not create the new crate yet.

First extract a small in-main-crate projection facade:

```text
next_task=BUILD-MIR-JSON-EXPORT-MODEL-SCAFFOLD-001
new_owner=src/runner/mir_json_export_model
purpose=define JSON-ready DTO names and move only root serialization shape behind a facade
behavior_changed=0
```

The first code slice should be intentionally small:

```text
move_io_helpers=0
move_route_json=0
move_metadata_emitters=0
move_schema=0
define_export_model_names=1
add_facade_tests=1
```

## Stop Lines

```text
do_not_create_future_crate_yet=1
do_not_move_runner_mir_json_emit_directly=1
do_not_move_mir_producers=1
do_not_change_mir_json_schema=1
do_not_change_ny_llvmc_route=1
do_not_add_main_crate_dependency_from_future_crate=1
behavior_change_allowed=0
```

## Contract

```text
output_contract=build-mir-json-emit-boundary-ssot-v0

projection_owner_main_crate=1
serialization_owner_future_crate=1
future_crate_reads_mir_directly=0
behavior_changed=0
boxcount_allowed=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-JSON-EXPORT-MODEL-SCAFFOLD-001
```
