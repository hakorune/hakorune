# 296x-1376 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-018

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after
`hakorune_mir_builder` crate-root materialization is closed.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1375 hakorune_mir_builder crate-root materialization
```

The closed row has:

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

Known current materialized `hakorune_mir_builder` module set:

```text
crate=materialized
binding_context=materialized
variable_context=materialized
core_context=materialized
context=materialized
type_context=materialized
metadata_context=materialized
```

That completes checked-in single-module coverage for the 7-module
`hakorune_mir_builder` crate.

## Candidate Directions

Evaluate the next row before implementation:

```text
candidate_A=hakorune_mir_builder crate/module bundle aggregation
candidate_B=next crate pilot inventory after builder coverage
candidate_C=closure unsupported handoff hardening for remaining source-shape coverage
candidate_D=return to broader MirBuilder migration support task
```

## Selection Result

```text
selected_next_task=HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-001
selected_scope=generate and check in one real 7-module hakorune_mir_builder crate-mode bundle, then consume it through one manifest-driven reusable Hako FileBox route in deterministic manifest order
selected_reason=all 7 modules are independently materialized; aggregation closes the current crate-coverage milestone without enabling name/use resolution, while avoiding another hand-unrolled wrapper
implementation_shape=A2-lite manifest-driven reusable file-route helper
implementation_allowed=0
next_card_name=296x-1377-HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-001
summary=ok
```

## Rationale

`hakorune_mir_builder` now has checked-in focused evidence for every module:

```text
7 single-module fixtures = focused regression evidence
1 aggregate crate bundle = crate-level transport evidence
```

The next row should not hand-unroll another 7-module wrapper. It should add a
thin reusable crate-bundle FileBox route helper and one wrapper that reads the
real crate-mode manifest in manifest order.

Do not treat aggregate skeleton MIR emit as a general namespace/linking claim.
It is fixture-only evidence for this crate bundle.

After aggregation, the intended next lane is MirBuilder migration support:

```text
next_after_aggregation=Hako MirBuilder authority migration support
```

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
selected_next_task=<token>
selected_scope=<short description>
selected_reason=<short reason>
implementation_allowed=0
next_card_name=<card>
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
