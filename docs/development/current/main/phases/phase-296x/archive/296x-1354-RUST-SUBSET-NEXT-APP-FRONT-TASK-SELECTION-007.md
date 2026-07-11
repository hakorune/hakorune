# 296x-1354 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-007

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after associated function
call skeleton-safety is closed.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1353 associated function call skeleton-safety
```

The selected blocker now has:

```text
associated_function_call_fixture_added=1
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
  generated_skeleton_mir_emit=fail
  previous_blocker=CallFlags_new
  previous_blocker_cleared=1
  current_first_failure=Undefined variable: EffectMask_IO

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
candidate_B=select enum/associated-const value skeleton-safety for EffectMask_IO
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

Rechecked the two nearest paths after 296x-1353.

```text
hakorune_mir_builder::core_context:
  generated_hako_lines=54
  generated_skeleton_mir_emit=green
  selected=0
  reason=green materialization candidate, but a smaller shared source-shape blocker is exposed by call_unified

hakorune_mir_defs::call_unified:
  generated_hako_lines=113
  generated_skeleton_mir_emit=fail
  first_failure=Undefined variable: EffectMask_IO
  source_shape=EffectMask::IO
  current_emission=EffectMask_IO
  selected=1
  reason=type-qualified value path currently emits an undefined generated symbol
```

Focused checks:

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  --crate-root crates/hakorune_mir_builder \
  --out-dir /tmp/rust_subset_hakorune_mir_builder_1353 \
  --crate-name hakorune_mir_builder \
  --target-kind lib \
  --target-name hakorune_mir_builder

./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_builder_core_context_1353.mir.json \
  /tmp/hakorune_mir_builder_core_context_1353.hako

cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  --crate-root crates/hakorune_mir_defs \
  --out-dir /tmp/rust_subset_hakorune_mir_defs_1353 \
  --crate-name hakorune_mir_defs \
  --target-kind lib \
  --target-name hakorune_mir_defs

./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_defs_call_unified_1353.mir.json \
  /tmp/hakorune_mir_defs_call_unified_1353.hako
```

## Result

```text
selected_next_task=RUST-SUBSET-ASSOCIATED-CONST-VALUE-SKELETON-SAFETY-001
selected_scope=syn-adapter expression Path handling for type-qualified value paths
selected_reason=EffectMask::IO currently emits EffectMask_IO as an undefined generated variable
implementation_allowed=0
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
next_card_name=296x-1355-RUST-SUBSET-ASSOCIATED-CONST-VALUE-SKELETON-SAFETY-001
summary=ok
```

Next row:

```text
296x-1355-RUST-SUBSET-ASSOCIATED-CONST-VALUE-SKELETON-SAFETY-001
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
