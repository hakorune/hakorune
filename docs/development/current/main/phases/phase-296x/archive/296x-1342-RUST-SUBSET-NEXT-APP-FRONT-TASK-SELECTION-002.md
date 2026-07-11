# 296x-1342 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-002

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after the selected
`hakorune_mir_core` ID-module bundle is materialized and guarded.

## Current Evidence

Closed immediately before this row:

```text
296x-1338 tuple-struct constructor skeleton safety
296x-1339 compound assignment skeleton safety
296x-1340 Self-qualified call skeleton safety
296x-1341 hakorune_mir_core ID-module materialization
```

The selected ID-module bundle now has:

```text
manifest_checked_in=1
module_artifacts_checked_in=3
generated_skeleton_mir_emit=green
wrapper_emit_exe=green
generated_program_execution_claim=0
```

## Candidate Directions

Evaluate the next slice before implementation:

```text
candidate_A=next_hakorune_mir_core_small_module_slice
candidate_B=RustSubset source-shape blocker exposed by current real crates
candidate_C=crate-wrapper duplication cleanup / app-front template hardening
candidate_D=creat subset inventory follow-up
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
small_module_count<=3
clear_generated_skeleton_acceptance=1
fixture_or_manifest_gate_available=1
no trait/generic/name-resolution dependency=1
```

## Acceptance

Produce a decision with:

```text
selected_next_task=<token>
selected_scope=<short description>
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

## Probe

Fresh candidate probes:

```text
hakorune_mir_core::value_kind:
  generated_skeleton_mir_emit=green
  hako_lines=106
  json_lines=593

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

## Decision

Select the smallest real compiler module that already passes generated-skeleton
MIR emit:

```text
selected_next_task=HAKORUNE-MIR-CORE-VALUE-KIND-MATERIALIZATION-001
selected_scope=hakorune_mir_core::value_kind single-module bundle
selected_crate=hakorune_mir_core
selected_modules=crate::value_kind
selected_module_count=1
implementation_allowed=0
next_card_name=296x-1343-HAKORUNE-MIR-CORE-VALUE-KIND-MATERIALIZATION-001
summary=ok
```

Rationale:

```text
small_module_count=1
clear_generated_skeleton_acceptance=1
fixture_or_manifest_gate_available=1
no_trait_generic_name_resolution_dependency=1
```

## Stop Line

```text
implementation_started=0
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
generated_program_execution_claim=0
```
