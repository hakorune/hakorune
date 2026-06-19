# 296x-1370 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-015

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after Self value
skeleton-safety is closed.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1369 Self value skeleton-safety
```

The closed row has:

```text
self_value_fixture_added=1
self_value_adapter_json_green=1
self_value_hako_converter_parity_green=1
metadata_context_self_value_boundary_removed=1
full_rust_subset_smoke=green
```

Known remaining real-front evidence:

```text
hakorune_mir_builder::metadata_context:
  generated_skeleton_mir_emit=fail
  next_boundary=unresolved function Some in MetadataContext_set_source_file
```

## Candidate Directions

Evaluate the next row before implementation:

```text
candidate_A=Option constructor/value path skeleton-safety for Some/None in metadata_context
candidate_B=closure unsupported handoff hardening for value_caller map closure
candidate_C=next green module inventory
candidate_D=crate/module bundle aggregation after current context set
```

## Selection Result

```text
selected_next_task=RUST-SUBSET-OPTION-CONSTRUCTOR-SKELETON-SAFETY-001
selected_scope=Option constructor/value path skeleton-safety for Some/None in metadata_context
selected_reason=metadata_context now fails first on unresolved function Some; None is the same Option value-constructor family
implementation_allowed=0
next_card_name=296x-1371-RUST-SUBSET-OPTION-CONSTRUCTOR-SKELETON-SAFETY-001
summary=ok
```

Option constructor/value paths are the smallest real-front blocker now exposed
by `hakorune_mir_builder::metadata_context`. Closure handoff hardening remains
later because the current first MIR-emission failure is `Some(source.into())`
inside `MetadataContext_set_source_file`.

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
selected_next_task=RUST-SUBSET-OPTION-CONSTRUCTOR-SKELETON-SAFETY-001
selected_scope=Option constructor/value path skeleton-safety for Some/None in metadata_context
selected_reason=metadata_context now fails first on unresolved function Some
implementation_allowed=0
next_card_name=296x-1371-RUST-SUBSET-OPTION-CONSTRUCTOR-SKELETON-SAFETY-001
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
