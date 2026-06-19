# 296x-1347 HAKORUNE-MIR-CORE-EFFECT-MATERIALIZATION-001

Status: open
Date: 2026-06-20

## Purpose

Materialize the next selected `hakorune_mir_core` RustSubset module slice:

```text
crate::effect
```

This follows 296x-1345 and 296x-1346, which made the generated skeleton safe for
enum variant value references and `Vec::new()` calls. The temporary probe now
shows `crate::effect` reaches generated-skeleton MIR emit.

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
general_vec_runtime_semantics=0
full_enum_runtime_semantics=0
new_hako_syntax_added=0
```

## Acceptance

Check in a selected bundle under:

```text
apps/rust-subset-to-hako/examples/hakorune_mir_core_effect_expected/
```

Add/update a focused wrapper, then verify:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_core_effect_expected.mir.json \
  apps/rust-subset-to-hako/examples/hakorune_mir_core_effect_expected.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_hakorune_mir_core_effect_crate_file \
  apps/rust-subset-to-hako/convert_hakorune_mir_core_effect_crate_file.hako
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
general_vec_runtime_semantics=0
full_enum_runtime_semantics=0
new_hako_syntax_added=0
```
