# 296x-1358 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-009

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after
`hakorune_mir_builder::core_context` materialization is closed.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1357 hakorune_mir_builder::core_context materialization
```

The selected blocker now has:

```text
manifest_checked_in=1
module_artifact_checked_in=1
generated_skeleton_expected_checked_in=1
focused_wrapper_added=1
generated_skeleton_mir_emit=green
wrapper_emit_exe=green
wrapper_exe_parity=green
crate_wrapper_exe_smoke=green
full_rust_subset_smoke=green
generated_program_execution_claim=0
```

Known candidate evidence:

```text
hakorune_mir_defs::call_unified:
  generated_skeleton_mir_emit=green
  previous_blocker=EffectMask_IO
  previous_blocker_cleared=1
  candidate_reason=defs-side call substrate is now materializable

hakorune_mir_builder::context:
  generated_skeleton_mir_emit=green
  candidate_reason=builder orchestration context is green but larger than leaf contexts

hakorune_mir_builder::metadata_context:
  generated_skeleton_mir_emit=fail
  first_failure=generic function name spelling is not parser-safe

hakorune_mir_builder::type_context:
  generated_skeleton_mir_emit=fail
  first_failure=unsupported reference type spelling / closure handoff
```

## Candidate Directions

Evaluate the next row before implementation:

```text
candidate_A=materialize hakorune_mir_defs::call_unified
candidate_B=materialize hakorune_mir_builder::context
candidate_C=select generic function name spelling skeleton-safety blocker
candidate_D=select reference-type / closure skeleton-safety blocker
candidate_E=next real crate/module inventory
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

Rechecked the green materialization candidates after 296x-1357.

```text
hakorune_mir_builder::context:
  source_path=src/context.rs
  generated_hako_lines=25
  generated_skeleton_mir_emit=green
  record_count=1
  function_count=3
  unsupported_marker_count=6
  selected=1
  reason=smallest green builder-owned context; continues MirBuilder context-family materialization with BTreeMap/BoxCompilationContext relevance

hakorune_mir_defs::call_unified:
  source_path=src/call_unified.rs
  generated_hako_lines=115
  generated_skeleton_mir_emit=green
  record_count=2
  function_count=17
  enum_comment_count=3
  unsupported_marker_count=20
  selected=0
  reason=green but broader defs-side call substrate; keep as next candidate after builder context

hakorune_mir_builder::metadata_context:
  source_path=src/metadata_context.rs
  generated_skeleton_mir_emit=fail
  first_failure=generic function name spelling is not parser-safe

hakorune_mir_builder::type_context:
  source_path=src/type_context.rs
  generated_skeleton_mir_emit=fail
  first_failure=unsupported reference type spelling / closure handoff
```

Focused checks:

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  --crate-root crates/hakorune_mir_defs \
  --out-dir /tmp/rust_subset_hakorune_mir_defs_1358 \
  --crate-name hakorune_mir_defs \
  --target-kind lib \
  --target-name hakorune_mir_defs

cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  --crate-root crates/hakorune_mir_builder \
  --out-dir /tmp/rust_subset_hakorune_mir_builder_1358 \
  --crate-name hakorune_mir_builder \
  --target-kind lib \
  --target-name hakorune_mir_builder

./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_builder_context_1358.mir.json \
  /tmp/hakorune_mir_builder_context_1358.hako

./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_defs_call_unified_1358.mir.json \
  /tmp/hakorune_mir_defs_call_unified_1358.hako
```

## Result

```text
selected_next_task=HAKORUNE-MIR-BUILDER-CONTEXT-MATERIALIZATION-001
selected_scope=hakorune_mir_builder::context single-module bundle
selected_reason=smallest green builder-owned context candidate after core_context; directly relevant to BoxCompilationContext / BTreeMap migration shape
implementation_allowed=0
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
next_card_name=296x-1359-HAKORUNE-MIR-BUILDER-CONTEXT-MATERIALIZATION-001
summary=ok
```

Next row:

```text
296x-1359-HAKORUNE-MIR-BUILDER-CONTEXT-MATERIALIZATION-001
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
