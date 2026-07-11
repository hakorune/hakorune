# 296x-1366 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-013

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after reference type spelling
skeleton-safety is closed.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1365 reference type spelling skeleton-safety
```

The closed row now has:

```text
reference_type_fixture_parser_safe=1
metadata_context_option_ref_str_parser_safe=1
type_context_option_ref_str_parser_safe=1
closure_handoff_changed=0
full_rust_subset_smoke=green
```

Real-front status:

```text
hakorune_mir_builder::type_context:
  generated_skeleton_mir_emit=green

hakorune_mir_builder::metadata_context:
  generated_skeleton_mir_emit=fail
  next_boundary=undefined Self value in HintSink_new
```

## Candidate Directions

Evaluate the next row before implementation:

```text
candidate_A=materialize hakorune_mir_builder::type_context
candidate_B=Self type/value skeleton-safety for metadata_context
candidate_C=closure unsupported handoff hardening
candidate_D=next green module inventory
```

## Selection Result

```text
selected_next_task=HAKORUNE-MIR-BUILDER-TYPE-CONTEXT-MATERIALIZATION-001
selected_scope=materialize crate::type_context single-module RustSubset bundle
selected_reason=type_context already reaches generated-skeleton MIR emit after reference type spelling was fixed; metadata_context still has a source-shape blocker
implementation_allowed=0
next_card_name=296x-1367-HAKORUNE-MIR-BUILDER-TYPE-CONTEXT-MATERIALIZATION-001
summary=ok
```

`type_context` is the highest-value green candidate because it is a
MirBuilder-owned context module and now reaches generated-skeleton MIR emit.
`metadata_context` remains a source-shape blocker lane because its next
boundary is undefined `Self` value in `HintSink_new`.

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
selected_next_task=HAKORUNE-MIR-BUILDER-TYPE-CONTEXT-MATERIALIZATION-001
selected_scope=materialize crate::type_context single-module RustSubset bundle
selected_reason=type_context already reaches generated-skeleton MIR emit; metadata_context still has undefined Self boundary
implementation_allowed=0
next_card_name=296x-1367-HAKORUNE-MIR-BUILDER-TYPE-CONTEXT-MATERIALIZATION-001
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
