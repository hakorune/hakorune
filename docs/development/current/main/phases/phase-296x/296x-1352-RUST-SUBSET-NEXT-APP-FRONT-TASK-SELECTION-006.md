# 296x-1352 RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-006

Status: closed
Date: 2026-06-20

## Purpose

Select the next rust-subset-to-hako app-front task after
`hakorune_mir_builder::variable_context` is materialized and guarded.

This is a design/selection row. Do not start implementation in this row.

## Current Evidence

Closed immediately before this row:

```text
296x-1351 hakorune_mir_builder::variable_context materialization
```

The selected `variable_context` bundle now has:

```text
manifest_checked_in=1
module_artifact_checked_in=1
generated_skeleton_mir_emit=green
wrapper_emit_exe=green
wrapper_exe_parity=green
full_rust_subset_smoke=green
generated_program_execution_claim=0
```

Known nearby candidate evidence from 296x-1350:

```text
hakorune_mir_builder::context:
  generated_skeleton_mir_emit=green
  reason=small but lower forward value than variable_context

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

## Candidate Directions

Evaluate the next row before implementation:

```text
candidate_A=materialize hakorune_mir_builder::context
candidate_B=select next constructor-like call skeleton-safety blocker
candidate_C=select next type-spelling / generic-name skeleton-safety blocker
candidate_D=select next reference-type / closure skeleton-safety blocker
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

Regenerated focused candidate skeletons and confirmed that the next shared
blocker is a type-qualified associated function call emitted as an unresolved
global symbol.

```text
hakorune_mir_builder::core_context:
  source_path=src/core_context.rs
  generated_skeleton_mir_emit=fail
  first_failure=Unresolved function: BindingId_new
  source_shape=BindingId::new(...)
  current_emission=BindingId_new(...)

hakorune_mir_defs::call_unified:
  source_path=src/call_unified.rs
  generated_skeleton_mir_emit=fail
  first_failure=Unresolved function: CallFlags_new
  source_shape=CallFlags::new(...)
  current_emission=CallFlags_new(...)
```

This is not a Rust name-resolution row. The current app-front skeleton should
avoid emitting undefined global calls for associated functions whose semantics
are out of v0 scope.

Focused checks:

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  --crate-root crates/hakorune_mir_builder \
  --out-dir /tmp/rust_subset_hakorune_mir_builder_1352 \
  --crate-name hakorune_mir_builder \
  --target-kind lib \
  --target-name hakorune_mir_builder

./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_builder_core_context_1352.mir.json \
  /tmp/hakorune_mir_builder_core_context_1352.hako

cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  --crate-root crates/hakorune_mir_defs \
  --out-dir /tmp/rust_subset_hakorune_mir_defs_1352 \
  --crate-name hakorune_mir_defs \
  --target-kind lib \
  --target-name hakorune_mir_defs

./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_defs_call_unified_1352.mir.json \
  /tmp/hakorune_mir_defs_call_unified_1352.hako
```

## Result

```text
selected_next_task=RUST-SUBSET-ASSOCIATED-FUNCTION-CALL-SKELETON-SAFETY-001
selected_scope=syn-adapter expression Call handling for non-Self/non-Vec associated function calls
selected_reason=two independent real crate modules fail by emitting type-qualified associated calls as unresolved global symbols
implementation_allowed=0
new_hako_syntax_added=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
next_card_name=296x-1353-RUST-SUBSET-ASSOCIATED-FUNCTION-CALL-SKELETON-SAFETY-001
summary=ok
```

Next row:

```text
296x-1353-RUST-SUBSET-ASSOCIATED-FUNCTION-CALL-SKELETON-SAFETY-001
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
