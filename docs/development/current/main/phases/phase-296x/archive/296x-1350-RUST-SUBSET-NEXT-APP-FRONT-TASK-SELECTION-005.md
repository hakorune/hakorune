# 296x-1350 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-005

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after
`hakorune_mir_builder::binding_context` is materialized and guarded.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1349 hakorune_mir_builder::binding_context materialization
```

The selected `binding_context` bundle now has:

```text
manifest_checked_in=1
module_artifact_checked_in=1
generated_skeleton_mir_emit=green
wrapper_emit_exe=green
full_rust_subset_smoke=green
generated_program_execution_claim=0
```

Known candidate evidence from 296x-1348:

```text
hakorune_mir_builder::core_context:
  generated_skeleton_mir_emit=fail
  first_failure=Unresolved function: BindingId_new

hakorune_mir_defs::call_unified:
  generated_skeleton_mir_emit=fail
  first_failure=Unresolved function: CallFlags_new

hakorune_mir_joinir::ownership_types:
  generated_skeleton_mir_emit=fail
  first_failure=unsupported type spelling / unsupported loop shape
```

## Candidate Directions

Evaluate the next row before implementation:

```text
candidate_A=next MirBuilder context module materialization or blocker
candidate_B=next unresolved constructor-like call skeleton safety
candidate_C=next real crate/module inventory
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

If a real module exposes a small source-shape blocker, select the blocker row
instead of materializing around it.

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

## Probe

Regenerated `hakorune_mir_builder` RustSubset artifacts and tested generated
`.hako` skeleton MIR emit for the next small modules.

```text
hakorune_mir_builder::context:
  source_path=src/context.rs
  generated_hako_lines=23
  generated_skeleton_mir_emit=green
  selected=0
  reason=lower_forward_value_than_variable_context

hakorune_mir_builder::variable_context:
  source_path=src/variable_context.rs
  generated_hako_lines=61
  generated_skeleton_mir_emit=green
  selected=1
  reason=MirBuilder variable_map / SSA carrier relevance

hakorune_mir_builder::core_context:
  generated_skeleton_mir_emit=fail
  first_failure=Unresolved function: BindingId_new

hakorune_mir_builder::metadata_context:
  generated_skeleton_mir_emit=fail
  first_failure=generic function name spelling is not parser-safe

hakorune_mir_builder::type_context:
  generated_skeleton_mir_emit=fail
  first_failure=unsupported reference type spelling / closure handoff
```

Focused checks:

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  --crate-root crates/hakorune_mir_builder \
  --out-dir /tmp/rust_subset_hakorune_mir_builder_1350 \
  --crate-name hakorune_mir_builder \
  --target-kind lib \
  --target-name hakorune_mir_builder

python3 apps/rust-subset-to-hako/convert.py \
  /tmp/rust_subset_hakorune_mir_builder_1350/modules/0006.json \
  > /tmp/hakorune_mir_builder_variable_context_1350.hako

./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_builder_variable_context_1350.mir.json \
  /tmp/hakorune_mir_builder_variable_context_1350.hako
```

## Result

```text
selected_next_task=HAKORUNE-MIR-BUILDER-VARIABLE-CONTEXT-MATERIALIZATION-001
selected_scope=hakorune_mir_builder::variable_context single-module bundle
selected_reason=green generated-skeleton MIR emit with direct MirBuilder variable_map relevance
implementation_allowed=0
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
next_card_name=296x-1351-HAKORUNE-MIR-BUILDER-VARIABLE-CONTEXT-MATERIALIZATION-001
summary=ok
```

Next row:

```text
296x-1351-HAKORUNE-MIR-BUILDER-VARIABLE-CONTEXT-MATERIALIZATION-001
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
