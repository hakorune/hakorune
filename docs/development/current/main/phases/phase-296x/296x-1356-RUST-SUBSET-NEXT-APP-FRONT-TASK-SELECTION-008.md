# 296x-1356 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-008

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after associated
const/value path skeleton-safety is closed.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1355 associated const/value path skeleton-safety
```

The selected blocker now has:

```text
associated_const_value_fixture_added=1
python_reference_parity_updated=1
hako_converter_fixture_parity_updated=1
full_rust_subset_smoke=green
generated_program_execution_claim=0
```

Known candidate evidence:

```text
hakorune_mir_builder::core_context:
  generated_skeleton_mir_emit=green
  previous_blocker=BindingId_new
  previous_blocker_cleared=1

hakorune_mir_defs::call_unified:
  generated_skeleton_mir_emit=green
  previous_blocker=EffectMask_IO
  previous_blocker_cleared=1

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
candidate_A=materialize hakorune_mir_builder::core_context
candidate_B=materialize hakorune_mir_defs::call_unified
candidate_C=select type-spelling / generic-name skeleton-safety blocker
candidate_D=select reference-type / closure skeleton-safety blocker
candidate_E=next real crate/module inventory
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

Rechecked the two green materialization candidates after 296x-1355.

```text
hakorune_mir_builder::core_context:
  source_path=src/core_context.rs
  generated_hako_lines=54
  generated_skeleton_mir_emit=green
  selected=1
  reason=smallest green MirBuilder-owned ID generation context; follows binding_context and variable_context owner family

hakorune_mir_defs::call_unified:
  source_path=src/call_unified.rs
  generated_hako_lines=113
  generated_skeleton_mir_emit=green
  selected=0
  reason=green but larger defs-side call substrate; keep as next candidate after MirBuilder core context
```

Focused checks:

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  --crate-root crates/hakorune_mir_builder \
  --out-dir /tmp/rust_subset_hakorune_mir_builder_1356 \
  --crate-name hakorune_mir_builder \
  --target-kind lib \
  --target-name hakorune_mir_builder

./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_builder_core_context_1356.mir.json \
  /tmp/hakorune_mir_builder_core_context_1356.hako

cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  --crate-root crates/hakorune_mir_defs \
  --out-dir /tmp/rust_subset_hakorune_mir_defs_1356 \
  --crate-name hakorune_mir_defs \
  --target-kind lib \
  --target-name hakorune_mir_defs

./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_defs_call_unified_1356.mir.json \
  /tmp/hakorune_mir_defs_call_unified_1356.hako
```

## Result

```text
selected_next_task=HAKORUNE-MIR-BUILDER-CORE-CONTEXT-MATERIALIZATION-001
selected_scope=hakorune_mir_builder::core_context single-module bundle
selected_reason=green generated-skeleton MIR emit with direct MirBuilder ID generation relevance
implementation_allowed=0
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
next_card_name=296x-1357-HAKORUNE-MIR-BUILDER-CORE-CONTEXT-MATERIALIZATION-001
summary=ok
```

Next row:

```text
296x-1357-HAKORUNE-MIR-BUILDER-CORE-CONTEXT-MATERIALIZATION-001
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
