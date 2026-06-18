Status: Done
Date: 2026-06-18
Scope: wire main-crate MIR JSON export facade to hakorune-mir-json-emit
Related:
  - docs/development/current/main/phases/phase-296x/296x-1115-BUILD-MIR-JSON-EMIT-CRATE-SCAFFOLD-001.md
  - crates/hakorune_mir_json_emit
  - src/runner/mir_json_export_model.rs
  - src/runner/mir_json_emit/root.rs

# BUILD-MIR-JSON-EMIT-CRATE-FACADE-WIRING-001

## Purpose

Move the DTO model and serializer implementation out of the main crate while
preserving the existing `runner::mir_json_export_model` import path.

## Change

```text
main_crate_dependency_added=1
compat_facade=src/runner/mir_json_export_model.rs
facade_reexports_new_crate_api=1
main_crate_owns_projection=1
new_crate_owns_serializer=1
json_output_changed=0
```

`build_mir_json_root` still constructs the DTO in the main crate because it
walks `MirModule`. The DTO types and `serialize_document` now come from
`hakorune-mir-json-emit`.

## Verification

```text
cargo_check=green
cargo_test_hakorune_mir_json_emit=green
mir_json_root_smoke=green
current_state_pointer_guard=green
```

## Contract

```text
output_contract=build-mir-json-emit-crate-facade-wiring-v0

boxshape_only=1
boxcount_allowed=0
behavior_changed=0
json_schema_changed=0
new_crate_reads_mir_directly=0
projection_owner=main_crate
serialization_owner=hakorune_mir_json_emit
direct_mir_json_emit_crate_extraction_selected=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-JSON-EMIT-CRATE-CLOSEOUT-001
purpose=close JSON-ready serializer crate split and select the next build-time split boundary
```
