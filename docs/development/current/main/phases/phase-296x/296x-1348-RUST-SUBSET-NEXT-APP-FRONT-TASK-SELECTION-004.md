# 296x-1348 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-004

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after
`hakorune_mir_core::effect` is materialized and guarded.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1347 hakorune_mir_core::effect materialization
```

The selected `effect` bundle now has:

```text
manifest_checked_in=1
module_artifact_checked_in=1
generated_skeleton_mir_emit=green
wrapper_emit_exe=green
full_rust_subset_smoke=green
generated_program_execution_claim=0
```

Completed `hakorune_mir_core` module materializations:

```text
crate::control_ids
crate::types
crate::basic_block_id
crate::binding_id
crate::value_id
crate::value_kind
crate::effect
```

Remaining `hakorune_mir_core` root module:

```text
crate:
  generated_skeleton_mir_emit=green
  content=Use comments only
  forward_value=low
```

## Candidate Directions

Evaluate the next row before implementation:

```text
candidate_A=materialize_remaining_hakorune_mir_core_root_if_worth_it
candidate_B=next_real_crate_or_module_inventory
candidate_C=RustSubset source-shape blocker exposed by current real crates
candidate_D=crate-wrapper duplication cleanup / app-front template hardening
candidate_E=creat subset inventory follow-up
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
real_forward_value=1
```

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

Fresh candidate probes:

```text
hakorune_mir_core::crate:
  generated_skeleton_mir_emit=green
  hako_lines=15
  content=Use comments only
  selected=0
  reason=low_forward_value

hakorune_frontend_ast::build_predicate:
  generated_skeleton_mir_emit=green
  hako_lines=10
  content=enum comment-only
  selected=0
  reason=low_forward_value

hakorune_mir_builder::binding_context:
  generated_skeleton_mir_emit=green
  hako_lines=47
  selected=1
  reason=MirBuilder migration relevant context slice

hakorune_mir_defs::call_unified:
  generated_skeleton_mir_emit=fail
  first_failure=Unresolved function: CallFlags_new

hakorune_mir_joinir::ownership_types:
  generated_skeleton_mir_emit=fail
  first_failure=unsupported type spelling / unsupported loop shape

hakorune_mir_builder::core_context:
  generated_skeleton_mir_emit=fail
  first_failure=Unresolved function: BindingId_new
```

## Decision

Select the smallest real MirBuilder-adjacent module that already reaches
generated-skeleton MIR emit:

```text
selected_next_task=HAKORUNE-MIR-BUILDER-BINDING-CONTEXT-MATERIALIZATION-001
selected_scope=hakorune_mir_builder::binding_context single-module bundle
selected_reason=green generated-skeleton MIR acceptance with direct MirBuilder migration relevance
implementation_allowed=0
next_card_name=296x-1349-HAKORUNE-MIR-BUILDER-BINDING-CONTEXT-MATERIALIZATION-001
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
