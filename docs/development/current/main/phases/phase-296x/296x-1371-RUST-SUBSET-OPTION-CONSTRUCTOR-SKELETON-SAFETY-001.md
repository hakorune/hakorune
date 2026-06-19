# 296x-1371 RUST-SUBSET-OPTION-CONSTRUCTOR-SKELETON-SAFETY-001

Status: closed
Date: 2026-06-20

## Purpose

Make Rust `Option` constructor/value path expressions skeleton-safe for the
rust-subset-to-hako app front.

The immediate real-front target is `hakorune_mir_builder::metadata_context`,
where the generated skeleton currently stops at `Some(source.into())` as an
unresolved generated function.

## Current Evidence

Selected by:

```text
296x-1370 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-015
```

Known target boundary:

```text
hakorune_mir_builder::metadata_context:
  generated_skeleton_mir_emit=fail
  first_failure=Unresolved function: 'Some'
  failing_site=MetadataContext_set_source_file
```

Nearby source-shape blockers already closed:

```text
generic_impl_target_emitted_names_parser_safe=1
reference_type_spelling_parser_safe=1
self_value_skeleton_safe=1
```

## Scope

Allowed:

```text
Rust Option constructor/value path expressions become explicit skeleton-safe
Unsupported handoffs.
```

Required behavior:

```text
some_call_boundary_removed=1
none_value_boundary_removed=1
metadata_context_option_boundary_removed=1
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
do not implement Option semantics
do not lower Some/None to native .hako enum constructors
do not resolve Rust paths or imports
do not change closure unsupported handoff behavior
do not claim generated program execution
```

## Acceptance

Expected closeout evidence:

```text
option_constructor_fixture_added=1
option_constructor_adapter_json_green=1
option_constructor_hako_converter_parity_green=1
metadata_context_option_boundary_removed=1
metadata_context_next_boundary_recorded=<boundary-or-green>
generated_program_execution_claim=0
summary=ok
```

## Closeout

```text
option_constructor_fixture_added=1
option_constructor_adapter_json_green=1
option_constructor_python_reference_green=1
option_constructor_hako_converter_parity_green=1
some_call_boundary_removed=1
none_value_boundary_removed=1
metadata_context_option_boundary_removed=1
metadata_context_generated_skeleton_mir_emit=green
metadata_context_next_boundary=green
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

`Some(...)` and `None` now become explicit Unsupported expression handoffs
instead of generated globals/functions. This keeps Option semantics out of the
converter while making the generated skeleton parser/MIR-safe.

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
