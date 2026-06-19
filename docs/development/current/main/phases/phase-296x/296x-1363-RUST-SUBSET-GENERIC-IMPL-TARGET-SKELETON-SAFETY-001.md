# 296x-1363 RUST-SUBSET-GENERIC-IMPL-TARGET-SKELETON-SAFETY-001

Status: open
Date: 2026-06-20

## Purpose

Make RustSubset generic impl targets parser-safe in generated `.hako`
skeletons.

The immediate real-front blocker is:

```text
hakorune_mir_builder::metadata_context
```

Current generated output contains invalid `.hako` function names and receiver
types such as:

```hako
function MetadataContext<SpanT, RegionIdT>_new(current_span: SpanT): Self {
```

This row must preserve generic target source spelling as provenance while
emitting parser-safe skeleton names.

## Scope

Allowed:

```text
adapter-owned emitted target name for generic impl targets
converter uses emitted target name for function prefix / receiver spelling
source target spelling remains available for diagnostics
fixture or real-front gate for metadata_context
```

Not allowed:

```text
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
type_parameter_model_added=0
reference_type_support_added=0
closure_handoff_changed=0
```

## Design Contract

```text
source_target:
  Rust spelling used for diagnostics/provenance

emitted_target_name:
  Hako-safe identifier owned by the external adapter
  used by the converter for generated impl function names and receiver type

converter_core:
  does not resolve Rust paths
  does not infer generics
  prints emitted target spelling when present
```

For `impl<SpanT, RegionIdT> MetadataContext<SpanT, RegionIdT>`, the safe
generated skeleton target is:

```text
emitted_target_name=MetadataContext
```

## Acceptance

Required checks:

```bash
cargo check -q --lib
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Focused evidence:

```text
metadata_context_generic_impl_target_parser_safe=1
metadata_context_generated_skeleton_reaches_next_boundary=1
converter_core_filebox_ownership=0
generated_program_execution_claim=0
summary=ok
```

## Stop Line

```text
do_not_add_hako_generic_function_syntax
do_not_parse_or_resolve_rust_generics_in_converter
do_not_claim_generic_semantics
do_not_fix_type_context_reference_or_closure_in_this_row
```
