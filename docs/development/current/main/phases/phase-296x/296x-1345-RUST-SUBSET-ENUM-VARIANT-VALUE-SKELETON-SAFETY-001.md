# 296x-1345 RUST-SUBSET-ENUM-VARIANT-VALUE-SKELETON-SAFETY-001

Status: open
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
effect_generated_skeleton_mir_emit=1
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

# Convert crate::effect only, then verify generated-skeleton MIR emit.
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
full_enum_runtime_semantics=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
new_hako_syntax_added=0
```
