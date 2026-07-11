# 296x-1344 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-003

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after the
`hakorune_mir_core::value_kind` bundle is materialized and guarded.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1343 hakorune_mir_core::value_kind materialization
```

The selected `value_kind` bundle now has:

```text
manifest_checked_in=1
module_artifact_checked_in=1
generated_skeleton_mir_emit=green
wrapper_emit_exe=green
full_rust_subset_smoke=green
generated_program_execution_claim=0
```

Previous candidate probes from 296x-1342:

```text
hakorune_mir_core::effect:
  generated_skeleton_mir_emit=fail
  first_failure=Undefined variable: Effect_Mut

hakorune_mir_defs::call_unified:
  generated_skeleton_mir_emit=fail
  first_failure=Unresolved function: CallFlags_new

hakorune_mir_joinir::ownership_types:
  generated_skeleton_parse=fail
  first_failure=Result<void, String> type spelling
```

## Candidate Directions

Evaluate the next row before implementation:

```text
candidate_A=next_hakorune_mir_core_small_module_slice
candidate_B=RustSubset source-shape blocker exposed by current real crates
candidate_C=crate-wrapper duplication cleanup / app-front template hardening
candidate_D=creat subset inventory follow-up
candidate_E=diagnostic hardening exposed by wrapper/materialization work
```

## Selection Rules

```text
implementation_started=0
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
generated_program_execution_claim=0
```

Prefer a task with:

```text
single_owner=1
small_scope=1
fixture_or_manifest_gate_available=1
no_trait_generic_name_resolution_dependency=1
generated_skeleton_acceptance_or_clear_blocker=1
```

If the next best candidate exposes a true schema/source-shape gap, select the
smallest blocker row rather than materializing a larger bundle around it.

## Acceptance

Produce a decision with:

```text
selected_next_task=<token>
selected_scope=<short description>
selected_reason=<short reason>
implementation_allowed=0
next_card_name=<card>
summary=ok
```

## Probe

Fresh `hakorune_mir_core` module probe after 296x-1343:

```text
crate:
  generated_skeleton_mir_emit=green
  hako_lines=15
  content=Use comments only
  selected=0
  reason=low_forward_value

crate::effect:
  generated_skeleton_mir_emit=fail
  first_failure=Undefined variable: Effect_Mut
  blocker_kind=enum_variant_value_reference_without_value_surface
  selected_blocker=1
```

Already materialized or selected earlier:

```text
crate::control_ids
crate::types
crate::basic_block_id
crate::binding_id
crate::value_id
crate::value_kind
```

## Decision

Select the smallest source-shape blocker exposed by the next real compiler
module:

```text
selected_next_task=RUST-SUBSET-ENUM-VARIANT-VALUE-SKELETON-SAFETY-001
selected_scope=make RustSubset enum variant value references skeleton-safe
selected_reason=crate::effect fails because expression output references Effect_Mut while enum output is comment-only
implementation_allowed=0
next_card_name=296x-1345-RUST-SUBSET-ENUM-VARIANT-VALUE-SKELETON-SAFETY-001
summary=ok
```

The next row must not implement full enum semantics. It should only remove the
undefined-symbol skeleton hazard for enum variant references used as values.

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
