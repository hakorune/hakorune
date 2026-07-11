# 296x-1368 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-014

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after
`hakorune_mir_builder::type_context` materialization is closed.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1367 hakorune_mir_builder::type_context materialization
```

The closed row now has:

```text
manifest_checked_in=1
module_artifact_checked_in=1
generated_skeleton_expected_checked_in=1
focused_wrapper_added=1
generated_skeleton_mir_emit=green
wrapper_exe_parity=green
full_rust_subset_smoke=green
generated_program_execution_claim=0
```

Known remaining candidate evidence:

```text
hakorune_mir_builder::metadata_context:
  generated_skeleton_mir_emit=fail
  next_boundary=undefined Self value in HintSink_new
```

## Candidate Directions

Evaluate the next row before implementation:

```text
candidate_A=Self type/value skeleton-safety for metadata_context
candidate_B=closure unsupported handoff hardening
candidate_C=next green module inventory
candidate_D=crate/module bundle aggregation after current context set
```

## Selection Result

```text
selected_next_task=RUST-SUBSET-SELF-VALUE-SKELETON-SAFETY-001
selected_scope=Self value skeleton-safety for metadata_context / HintSink_new
selected_reason=metadata_context now fails first on undefined Self value after type_context materialization; closure remains a later unsupported-expression boundary
implementation_allowed=0
next_card_name=296x-1369-RUST-SUBSET-SELF-VALUE-SKELETON-SAFETY-001
summary=ok
```

`Self` value handling is the smallest real-front blocker now exposed by
`hakorune_mir_builder::metadata_context`. It is more direct than closure
handoff hardening because the current first MIR-emission failure is
`HintSink_new` returning an undefined generated `Self` value.

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
selected_next_task=RUST-SUBSET-SELF-VALUE-SKELETON-SAFETY-001
selected_scope=Self value skeleton-safety for metadata_context / HintSink_new
selected_reason=metadata_context now fails first on undefined Self value after type_context materialization
implementation_allowed=0
next_card_name=296x-1369-RUST-SUBSET-SELF-VALUE-SKELETON-SAFETY-001
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
