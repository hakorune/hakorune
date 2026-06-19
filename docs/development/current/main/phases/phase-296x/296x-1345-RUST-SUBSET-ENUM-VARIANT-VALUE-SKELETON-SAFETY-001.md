# 296x-1345 RUST-SUBSET-ENUM-VARIANT-VALUE-SKELETON-SAFETY-001

Status: closed
Date: 2026-06-20

## Purpose

Make RustSubset enum variant value references skeleton-safe.

The immediate blocker is `hakorune_mir_core::effect`: generated functions
reference values such as `Effect_Mut`, while enum declarations are currently
comment-only. This produces an undefined variable during generated-skeleton MIR
emit.

## Evidence

From 296x-1344:

```text
crate::effect:
  generated_skeleton_mir_emit=fail
  first_failure=Undefined variable: Effect_Mut
  blocker_kind=enum_variant_value_reference_without_value_surface
```

Representative generated output:

```hako
// enum Effect
//   Pure
//   Mut

function EffectMask_is_mut(receiver: EffectMask): bool {
    return receiver.contains(Effect_Mut)
}
```

## Scope

Allowed:

```text
small_fixture_added=1
adapter_or_converter_skeleton_safety_fix=1
effect_first_failure_advances_past_enum_variant=1
python_reference_parity_updated_if_needed=1
```

Not allowed:

```text
full_enum_runtime_semantics=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
new_hako_syntax_added=0
```

## Design Constraint

This row should make the skeleton parse and reach MIR emit. It must not claim
that generated enum values behave like Rust enum runtime values.

Prefer a representation that keeps the converter output self-contained and
parser-safe. If enum values are emitted as placeholder constants or declarations,
their scope and removal path must be explicit in tests/docs.

## Acceptance

Focused checks:

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  --crate-root crates/hakorune_mir_core \
  --out-dir /tmp/hakorune_mir_core_effect_probe \
  --crate-name hakorune_mir_core \
  --target-kind lib \
  --target-name hakorune_mir_core

# Convert crate::effect only, then verify the first failure advances past
# undefined enum variant values.
```

General checks:

```bash
cargo check -q --lib
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Result

```text
small_fixture_added=1
enum_variant_value_reference_unsupported_handoff=1
python_reference_parity=green
hako_converter_exe_parity=green
fixture_generated_skeleton_mir_emit=green
full_rust_subset_smoke=green
effect_first_failure_before=Undefined variable: Effect_Mut
effect_first_failure_after=Unresolved function: Vec_new
effect_generated_skeleton_mir_emit=0
next_blocker=RUST-SUBSET-VEC-NEW-CALL-SKELETON-SAFETY-001
full_enum_runtime_semantics=0
generated_program_execution_claim=0
summary=ok
```

Files:

```text
apps/rust-subset-to-hako/examples/enum_variant_value_input.rs
apps/rust-subset-to-hako/examples/enum_variant_value_subset.json
apps/rust-subset-to-hako/examples/enum_variant_value_expected.hako
apps/rust-subset-to-hako/convert_enum_variant_value_fixture.hako
```

Closeout checks:

```bash
cargo run --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml --quiet -- \
  apps/rust-subset-to-hako/examples/enum_variant_value_input.rs \
  --module enum_variant_value_fixture \
  -o /tmp/enum_variant_value_subset.json

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/enum_variant_value_expected.mir.json \
  apps/rust-subset-to-hako/examples/enum_variant_value_expected.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_enum_variant_value_fixture \
  apps/rust-subset-to-hako/convert_enum_variant_value_fixture.hako

cargo check -q --lib
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Next row:

```text
296x-1346-RUST-SUBSET-VEC-NEW-CALL-SKELETON-SAFETY-001
```

## Stop Line

```text
full_enum_runtime_semantics=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
new_hako_syntax_added=0
```
