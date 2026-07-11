# 296x-1364 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-012

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after generic impl target
skeleton-safety is closed.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1363 generic impl target skeleton-safety
```

The closed row now has:

```text
generic_impl_target_parser_safe=1
target_emitted_name_adapter_owned=1
converter_core_generic_semantics=0
metadata_context_invalid_function_name_removed=1
```

Known next boundary exposed by `metadata_context`:

```text
first_failure=unsupported_reference_type_spelling_and_closure_handoff
failing_shape=function MetadataContext_value_caller(...): Option<&str>
secondary_shape=.map(null /* TODO: unsupported expression: Closure */)
```

Known older candidate:

```text
hakorune_mir_builder::type_context:
  first_failure=unsupported_reference_type_spelling_and_closure_handoff
```

## Candidate Directions

Evaluate the next row before implementation:

```text
candidate_A=reference type spelling skeleton-safety
candidate_B=closure unsupported handoff / method-chain skeleton-safety
candidate_C=next materializable module after metadata_context/type_context blockers
```

## Probe Result

Rechecked the two active real-front candidates after 296x-1363:

```text
hakorune_mir_builder::metadata_context:
  generated_skeleton_mir_emit=fail
  first_parser_failure=Option<&str> return type spelling
  closure_handoff_present=1
  closure_handoff_is_todo_expression=1

hakorune_mir_builder::type_context:
  generated_skeleton_mir_emit=fail
  first_parser_failure=Option<&str> return type spelling
  closure_handoff_present=1
  closure_handoff_is_todo_expression=1
```

Both fronts share the same first parser boundary. The closure appears inside
the body as an explicit unsupported expression handoff, so it is not the first
parser blocker. Reference type spelling is the smaller next owner.

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
real_front_evidence=1
fixture_or_manifest_gate_available=1
```

## Acceptance

Produce a decision with:

```text
selected_next_task=RUST-SUBSET-REFERENCE-TYPE-SPELLING-SKELETON-SAFETY-001
selected_scope=nested reference type spelling normalization for parser-safe skeletons
selected_reason=metadata_context and type_context both fail first on Option<&str>; closure is already an unsupported expression handoff
implementation_allowed=0
next_card_name=296x-1365-RUST-SUBSET-REFERENCE-TYPE-SPELLING-SKELETON-SAFETY-001
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
