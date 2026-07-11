# 296x-1372 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-016

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after Option constructor
skeleton-safety is closed.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1371 Option constructor skeleton-safety
```

The closed row has:

```text
option_constructor_fixture_added=1
option_constructor_adapter_json_green=1
option_constructor_hako_converter_parity_green=1
metadata_context_generated_skeleton_mir_emit=green
full_rust_subset_smoke=green
generated_program_execution_claim=0
```

Known real-front status:

```text
hakorune_mir_builder::metadata_context:
  generated_skeleton_mir_emit=green

hakorune_mir_builder::type_context:
  materialized=1
```

## Candidate Directions

Evaluate the next row before implementation:

```text
candidate_A=materialize hakorune_mir_builder::metadata_context single-module bundle
candidate_B=closure unsupported handoff hardening for value_caller map closure
candidate_C=next green module inventory
candidate_D=crate/module bundle aggregation after current context set
```

## Selection Result

```text
selected_next_task=HAKORUNE-MIR-BUILDER-METADATA-CONTEXT-MATERIALIZATION-001
selected_scope=materialize crate::metadata_context single-module RustSubset bundle
selected_reason=metadata_context now reaches generated-skeleton MIR emit after Self and Option constructor boundaries were closed; it is the next green MirBuilder-owned context module
implementation_allowed=0
next_card_name=296x-1373-HAKORUNE-MIR-BUILDER-METADATA-CONTEXT-MATERIALIZATION-001
summary=ok
```

`metadata_context` is now the highest-value green candidate. It is
MirBuilder-owned, context-related, and already reaches generated-skeleton MIR
emit. Closure handoff hardening remains deferred because the active
`metadata_context` skeleton is now acceptable without changing closure
semantics.

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
selected_next_task=HAKORUNE-MIR-BUILDER-METADATA-CONTEXT-MATERIALIZATION-001
selected_scope=materialize crate::metadata_context single-module RustSubset bundle
selected_reason=metadata_context now reaches generated-skeleton MIR emit after Self and Option constructor boundaries were closed
implementation_allowed=0
next_card_name=296x-1373-HAKORUNE-MIR-BUILDER-METADATA-CONTEXT-MATERIALIZATION-001
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
