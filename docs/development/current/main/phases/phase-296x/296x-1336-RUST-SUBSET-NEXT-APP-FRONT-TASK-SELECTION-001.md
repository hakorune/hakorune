# 296x-1336 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next RustSubset app-front task after crate-wrapper EXE route pinning.

This is a design / selection row only. It does not add `.hako` syntax, Rust name
resolution, crate graph semantics, converter behavior, or generated-program
execution claims.

## Current Baseline

Closed prerequisites:

```text
crate_manifest_v0=closed
synthetic_multi_module_probe=closed
crate_handoff_mir_acceptance=closed
hakorune_box_core_pilot=closed
hakorune_mir_core_control_ids_types_pilot=closed
crate_wrapper_exe_route=closed
crate_wrapper_exe_smoke=closed
```

The active smoke that protects the wrapper route is:

```bash
bash apps/rust-subset-to-hako/smoke_crate_wrappers_exe.sh
```

## Candidate Sweep

Fresh crate inventory:

```text
hakorune_backend_aot      modules=4   items=21  unsupported=42
hakorune_frontend_ast     modules=12  items=82  unsupported=69
hakorune_frontend_parser  modules=8   items=40  unsupported=34
hakorune_mir_builder      modules=7   items=41  unsupported=33
hakorune_mir_core         modules=8   items=70  unsupported=54
hakorune_mir_defs         modules=2   items=11  unsupported=12
hakorune_mir_joinir       modules=2   items=10  unsupported=10
hakorune_box_core         modules=3   items=7   unsupported=2
```

`hakorune_mir_core` module inventory:

```text
crate                    items=7   unsupported=7
crate::basic_block_id    items=8   unsupported=6
crate::binding_id        items=4   unsupported=4
crate::control_ids       items=3   unsupported=0
crate::effect            items=14  unsupported=16
crate::types             items=10  unsupported=2
crate::value_id          items=14  unsupported=8
crate::value_kind        items=10  unsupported=11
```

Already accepted slice:

```text
crate::control_ids
crate::types
```

## Options

### A. Expand `hakorune_mir_core` with ID modules

Candidate modules:

```text
crate::basic_block_id
crate::binding_id
crate::value_id
```

Rationale:

```text
mirbuilder_core_context_relevance=high
real_compiler_surface=1
module_count_small=3
adds_id_generator_shapes=1
keeps_generated_program_execution_claim=0
```

Expected blockers remain explicit skeleton / source-shape handoff items:

```text
Use
test module
Display impl / fmt
associated const
casts
struct literal
```

These should not be pre-implemented before the pilot proves which blocker is
actually next.

### B. Add source-shape support first

Potential shapes:

```text
Rust cast expression
associated const
struct literal
Display/fmt pattern
```

Rejected for the immediate next row because this would implement shapes before
the selected real slice has exposed the next concrete blocker.

### C. Move to larger frontend/parser crates

Rejected for now because unsupported counts are higher and would mix many
shape families before the ID module lane is exercised.

## Decision

Select:

```text
selected_next_task=HAKORUNE-MIR-CORE-ID-MODULES-RUSTSUBSET-PILOT-001
selected_crate=hakorune_mir_core
selected_modules=crate::basic_block_id,crate::binding_id,crate::value_id
selected_module_count=3
```

Acceptance scope for the next row:

```text
materialize_selected_bundle=1
adapter_generated_json_checked_in=1
converter_wrapper_updated_or_new=1
generated_skeleton_parse=1
generated_skeleton_mir_emit=1
wrapper_emit_exe=1
generated_program_exe_aot_claim=0
```

## Stop Line

```text
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
source_shape_preimplementation=0
```

## Next

Continue:

```text
HAKORUNE-MIR-CORE-ID-MODULES-RUSTSUBSET-PILOT-001
```

The next row should materialize the selected ID module bundle and run it through
the existing RustSubset skeleton pipeline. If it exposes a concrete
source-shape blocker, open that blocker after the pilot evidence is captured.
