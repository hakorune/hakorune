# 296x-1346 RUST-SUBSET-VEC-NEW-CALL-SKELETON-SAFETY-001

Status: open
Date: 2026-06-20

## Purpose

Make Rust `Vec::new()` call expressions skeleton-safe.

After 296x-1345, `hakorune_mir_core::effect` no longer fails on enum variant
value references. The next first failure is an unresolved `Vec_new` function
generated from `Vec::new()`.

## Evidence

```text
before_row=296x-1345
effect_first_failure_after_enum_variant_fix=Unresolved function: Vec_new
```

Representative generated output:

```hako
function EffectMask_effect_names(receiver: EffectMask): Array {
    local names: Unknown = Vec_new()
    ...
}
```

## Scope

Allowed:

```text
small_fixture_added=1
adapter_or_converter_skeleton_safety_fix=1
effect_first_failure_advances_past_vec_new=1
python_reference_parity_updated_if_needed=1
```

Not allowed:

```text
general_vec_runtime_semantics=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
new_hako_syntax_added=0
```

## Design Constraint

This row should not implement Rust `Vec` semantics. It should only avoid
generating an unresolved `Vec_new` symbol in skeleton output.

Prefer the same conservative handoff style as other unsupported expression
shapes:

```text
Vec::new() -> null /* TODO: ... */
```

unless a smaller existing skeleton-safe representation is already available.

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
# unresolved Vec_new.
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
general_vec_runtime_semantics=0
rust_name_resolution_enabled=0
use_resolution_enabled=0
trait_semantics_enabled=0
generic_semantics_enabled=0
generated_program_execution_claim=0
new_hako_syntax_added=0
```
