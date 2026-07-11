# 296x-1369 RUST-SUBSET-SELF-VALUE-SKELETON-SAFETY-001

Status: closed
Date: 2026-06-20

## Purpose

Make Rust `Self` value references skeleton-safe for the rust-subset-to-hako
app front.

The immediate real-front target is `hakorune_mir_builder::metadata_context`,
where the generated skeleton currently stops at `HintSink_new` because
`return Self` becomes an undefined generated value.

## Current Evidence

Selected by:

```text
296x-1368 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-014
```

Known target boundary:

```text
hakorune_mir_builder::metadata_context:
  generated_skeleton_mir_emit=fail
  first_failure=undefined Self value in HintSink_new
```

Previous nearby blockers are already closed:

```text
generic_impl_target_emitted_names_parser_safe=1
reference_type_spelling_parser_safe=1
hakorune_mir_builder::type_context_materialized=1
```

## Scope

Allowed:

```text
Self value references become explicit skeleton-safe Unsupported handoffs or
another parser/MIR-safe placeholder owned by the RustSubset converter path.
```

Required behavior:

```text
metadata_context_self_value_boundary_removed=1
closure_handoff_changed=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
new_hako_syntax_added=0
generated_program_execution_claim=0
```

## Non-Goals

```text
do not implement Rust Self construction semantics
do not resolve Rust associated paths or type aliases
do not change closure unsupported handoff behavior
do not add new .hako syntax
do not claim generated program execution
```

## Acceptance

Expected closeout evidence:

```text
self_value_fixture_added=1
self_value_python_reference_green=1
self_value_hako_converter_parity_green=1
metadata_context_self_value_boundary_removed=1
metadata_context_next_boundary_recorded=<boundary-or-green>
generated_program_execution_claim=0
summary=ok
```

## Closeout

```text
self_value_fixture_added=1
self_value_adapter_json_green=1
self_value_python_reference_green=1
self_value_hako_converter_parity_green=1
metadata_context_self_value_boundary_removed=1
metadata_context_next_boundary=unresolved function Some in MetadataContext_set_source_file
closure_handoff_changed=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
new_hako_syntax_added=0
generated_program_execution_claim=0
full_rust_subset_smoke=green
summary=ok
```

The `Self` value expression now becomes an explicit Unsupported expression
handoff instead of a generated global/value name. `metadata_context` advances
to the next source-shape boundary:

```text
function MetadataContext_set_source_file(...):
  receiver.source_file = Some(source.into())
  first_failure=Unresolved function: 'Some'
```

General checks:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Run the rust-subset smoke when the implementation is ready:

```bash
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Line

```text
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
closure_handoff_changed=0
```
