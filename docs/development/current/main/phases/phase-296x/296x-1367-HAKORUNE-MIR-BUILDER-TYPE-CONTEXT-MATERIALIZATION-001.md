# 296x-1367 HAKORUNE-MIR-BUILDER-TYPE-CONTEXT-MATERIALIZATION-001

Status: open
Date: 2026-06-20

## Purpose

Materialize the selected `hakorune_mir_builder` RustSubset module slice:

```text
crate::type_context
```

This follows 296x-1366 selection. The probe shows the generated skeleton for
`type_context` reaches MIR emit after reference type spelling was made
parser-safe. It is directly relevant to MirBuilder type/kind/context
migration.

## Scope

Allowed:

```text
adapter_generated_json_checked_in=1
selected_manifest_checked_in=1
converter_wrapper_added_or_updated=1
generated_skeleton_expected_checked_in=1
generated_skeleton_mir_emit=1
wrapper_emit_exe=1
```

Not allowed:

```text
generated_program_execution_claim=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
new_hako_syntax_added=0
metadata_context_self_boundary_changed=0
closure_handoff_changed=0
```

## Acceptance

Check in a selected bundle under:

```text
apps/rust-subset-to-hako/examples/hakorune_mir_builder_type_context_expected/
```

Add/update a focused wrapper, then verify:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_builder_type_context_expected.mir.json \
  apps/rust-subset-to-hako/examples/hakorune_mir_builder_type_context_expected.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_hakorune_mir_builder_type_context_crate_file \
  apps/rust-subset-to-hako/convert_hakorune_mir_builder_type_context_crate_file.hako
```

General checks:

```bash
cargo check -q --lib
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
generated_program_execution_claim=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
new_hako_syntax_added=0
metadata_context_self_boundary_changed=0
closure_handoff_changed=0
```
