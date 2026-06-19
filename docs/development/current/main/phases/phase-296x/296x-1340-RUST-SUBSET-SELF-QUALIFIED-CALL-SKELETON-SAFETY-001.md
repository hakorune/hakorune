# 296x-1340 RUST-SUBSET-SELF-QUALIFIED-CALL-SKELETON-SAFETY-001

Status: closed
Date: 2026-06-20

## Purpose

Make Rust `Self::...` call expressions skeleton-safe in generated `.hako`
output.

After 296x-1339 cleared compound assignment, the selected `hakorune_mir_core`
ID-module probe advanced to the next generated skeleton failure:

```hako
function BasicBlockIdGenerator_default(): Self {
    return Self_new()
}
```

MIR emit fails with:

```text
Unresolved function: 'Self_new'
```

## Diagnosis

This is a RustSubset skeleton source-shape blocker. The adapter preserves
structured path provenance, but `Self::new()` is emitted as the Hako call
`Self_new()`. There is no generated top-level function with that name, and this
row must not add Rust name resolution or executable associated-function
semantics.

## Scope

Allowed:

```text
focused_fixture_added=1
adapter_self_qualified_call_handoff_updated=1
generated_skeleton_mir_safe=1
```

Not allowed:

```text
self_type_resolution_enabled=0
associated_function_runtime_semantics_claim=0
rust_name_resolution_enabled=0
new_hako_syntax_added=0
generated_program_execution_claim=0
```

The output only needs to be skeleton-safe. It does not need to preserve
executable Rust `Self::new()` semantics.

## Acceptance

Add a focused fixture containing at least:

```rust
pub struct Counter {
    next_id: u32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { next_id: 0 }
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}
```

Verify:

```bash
python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/self_qualified_call_subset.json \
  -o /tmp/self_qualified_call_py.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_self_qualified_call_fixture \
  apps/rust-subset-to-hako/convert_self_qualified_call_fixture.hako
```

Then re-run the selected ID-module probe:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_core_id_modules_generated.mir.json \
  <generated-id-module-skeleton.hako>
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
hakorune_mir_core_id_bundle_checked_in=0
self_type_resolution_enabled=0
associated_function_runtime_semantics=0
rust_name_resolution_enabled=0
new_hako_syntax_added=0
generated_program_execution_claim=0
```

## Result

```text
focused_fixture_added=1
adapter_self_qualified_call_handoff_updated=1
generated_skeleton_mir_safe_for_self_qualified_call=1
self_type_resolution_enabled=0
associated_function_runtime_semantics=0
rust_name_resolution_enabled=0
new_hako_syntax_added=0
generated_program_execution_claim=0
summary=ok
```

Implementation:

```text
apps/rust-subset-to-hako/tools/syn_adapter/src/exprs.rs
```

`Self::...` call paths now become explicit `Unsupported` expression handoffs
before emitted Hako call names such as `Self_new()` can reach generated
skeleton output. The converter core remains unchanged and does not infer the
self type.

Focused fixtures:

```text
apps/rust-subset-to-hako/examples/self_qualified_call_input.rs
apps/rust-subset-to-hako/examples/self_qualified_call_subset.json
apps/rust-subset-to-hako/examples/self_qualified_call_expected.hako
apps/rust-subset-to-hako/convert_self_qualified_call_fixture.hako
```

Verification:

```text
python_reference_self_qualified_call_fixture=green
syn_adapter_self_qualified_call_fixture=green
hako_converter_self_qualified_call_fixture_exe_parity=green
RUST_SUBSET_RUN_ADAPTER=1 apps/rust-subset-to-hako/smoke.sh=green
cargo_check_lib=green
current_state_pointer_guard=green
git_diff_check=green
```

Selected ID-module re-probe:

```text
self_new_unresolved_call=cleared
hakorune_mir_core_id_modules_generated_skeleton_mir_emit=green
next_blocker=HAKORUNE-MIR-CORE-ID-MODULES-MATERIALIZATION-001
```

## Next

After this skeleton-safety row is closed, resume:

```text
HAKORUNE-MIR-CORE-ID-MODULES-RUSTSUBSET-PILOT-001
```
