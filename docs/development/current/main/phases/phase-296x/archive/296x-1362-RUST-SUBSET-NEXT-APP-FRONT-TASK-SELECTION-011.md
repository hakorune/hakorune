# 296x-1362 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-011

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after
`hakorune_mir_defs::call_unified` materialization is closed.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1361 hakorune_mir_defs::call_unified materialization
```

The selected blocker now has:

```text
manifest_checked_in=1
module_artifact_checked_in=1
generated_skeleton_expected_checked_in=1
focused_wrapper_added=1
enum_payload_comment_parity_fixed=1
generated_skeleton_mir_emit=green
wrapper_emit_exe=green
wrapper_exe_parity=green
crate_wrapper_exe_smoke=green
full_rust_subset_smoke=green
generated_program_execution_claim=0
```

Known remaining candidate evidence:

```text
hakorune_mir_builder::metadata_context:
  generated_skeleton_mir_emit=fail
  first_failure=generic function name spelling is not parser-safe

hakorune_mir_builder::type_context:
  generated_skeleton_mir_emit=fail
  first_failure=unsupported reference type spelling / closure handoff
```

## Candidate Directions

Evaluate the next row before implementation:

```text
candidate_A=select generic function name spelling skeleton-safety blocker
candidate_B=select reference-type / closure skeleton-safety blocker
candidate_C=next real crate/module inventory
```

## Probe Result

Re-probed the remaining `hakorune_mir_builder` candidates after
`hakorune_mir_defs::call_unified` was materialized.

```text
hakorune_mir_builder::metadata_context:
  source_path=src/metadata_context.rs
  generated_skeleton_mir_emit=fail
  first_failure=generic_impl_target_spelling_not_parser_safe
  failing_shape=function MetadataContext<SpanT, RegionIdT>_new(...)
  selected=1

hakorune_mir_builder::type_context:
  source_path=src/type_context.rs
  generated_skeleton_mir_emit=fail
  first_failure=unsupported_reference_type_spelling_and_closure_handoff
  failing_shape=Option<&str>
  selected=0
```

`metadata_context` is the smaller next blocker. It exposes a single
adapter/converter boundary: generic impl targets must have parser-safe emitted
names and receiver type spellings. `type_context` also involves reference type
spelling and closure handoff, so it remains a later row.

## Selection Rules

```text
implementation_started=0
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
```

Prefer a task with:

```text
single_owner=1
small_scope=1
fixture_or_manifest_gate_available=1
real_forward_value=1
```

If a real module exposes a small source-shape blocker, select the blocker row
instead of materializing around it.

## Acceptance

Produce a decision with:

```text
selected_next_task=RUST-SUBSET-GENERIC-IMPL-TARGET-SKELETON-SAFETY-001
selected_scope=generic impl target emitted-name / parser-safe skeleton spelling
selected_reason=metadata_context first failure is a single target-spelling owner; type_context is broader
implementation_allowed=0
next_card_name=296x-1363-RUST-SUBSET-GENERIC-IMPL-TARGET-SKELETON-SAFETY-001
summary=ok
```

General checks:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
implementation_started=0
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
```
