# 296x-1355 RUST-SUBSET-ASSOCIATED-CONST-VALUE-SKELETON-SAFETY-001

Status: closed
Date: 2026-06-20

## Purpose

Make type-qualified Rust value paths skeleton-safe when the adapter cannot
prove executable `.hako` semantics.

Current failure:

```text
EffectMask::IO -> EffectMask_IO -> undefined generated variable
```

This row should prevent the adapter from generating undefined variables for
type-qualified value paths that are out of v0 skeleton scope.

## Scope

Allowed:

```text
syn_adapter_expr_path_handling_changed=1
associated_const_value_fixture_added=1
python_reference_parity_updated=1
hako_converter_fixture_parity_updated=1
real_module_probe_rechecked=1
```

Not allowed:

```text
rust_name_resolution_enabled=0
use_resolution_enabled=0
associated_const_semantics_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
new_hako_syntax_added=0
generated_program_execution_claim=0
```

## Acceptance

Add a focused fixture for a type-qualified value path, then verify:

```bash
python3 apps/rust-subset-to-hako/convert.py <fixture-json>
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
```

Recheck the real failure advances:

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  --crate-root crates/hakorune_mir_defs \
  --out-dir /tmp/rust_subset_hakorune_mir_defs_1355 \
  --crate-name hakorune_mir_defs \
  --target-kind lib \
  --target-name hakorune_mir_defs

./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_defs_call_unified_1355.mir.json \
  /tmp/hakorune_mir_defs_call_unified_1355.hako
```

General checks:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Result

```text
syn_adapter_expr_path_handling_changed=1
associated_const_value_fixture_added=1
python_reference_parity_updated=1
hako_converter_fixture_parity_updated=1
empty_impl_python_hako_parity_gap_closed=1
real_module_probe_rechecked=1
rust_name_resolution_enabled=0
use_resolution_enabled=0
associated_const_semantics_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
new_hako_syntax_added=0
generated_program_execution_claim=0
summary=ok
```

Behavior:

```text
crate::util::add1:
  remains a module-path Call handoff

Id::ZERO / EffectMask::IO / EffectMask::PURE / Effect::Alloc:
  now emit explicit Unsupported expressions
  no undefined generated variable is produced
```

Real-module recheck:

```text
hakorune_mir_defs::call_unified:
  previous_failure=Undefined variable: EffectMask_IO
  current_result=generated_skeleton_mir_emit=green
```

Closeout checks:

```bash
python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/associated_const_value_subset.json

cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  apps/rust-subset-to-hako/examples/associated_const_value_input.rs

./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_defs_call_unified_1355.mir.json \
  /tmp/hakorune_mir_defs_call_unified_1355.hako

RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Next row:

```text
296x-1356-RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-008
```

## Stop Line

```text
rust_name_resolution_enabled=0
use_resolution_enabled=0
associated_const_semantics_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
new_hako_syntax_added=0
generated_program_execution_claim=0
```
