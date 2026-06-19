# 296x-1338 RUST-SUBSET-TUPLE-STRUCT-CONSTRUCTOR-SKELETON-001

Status: open
Date: 2026-06-20

## Purpose

Make Rust tuple-struct constructor expressions MIR-safe in generated `.hako`
skeleton output.

The selected `hakorune_mir_core` ID-module slice exposed this blocker before
the generated bundle could be checked in:

```hako
function BasicBlockId_new(id: i64): Self {
    return BasicBlockId(id)
}
```

Hako does not treat a record name as a callable constructor in this context, so
the generated skeleton fails MIR emit with:

```text
Unresolved function: 'BasicBlockId'
```

## Scope

Add the smallest RustSubset skeleton handling for tuple-struct constructor
expressions.

Allowed:

```text
focused_fixture_added=1
python_reference_updated=1
hako_converter_updated=1
generated_skeleton_mir_safe=1
```

Not allowed:

```text
rust_name_resolution_enabled=0
record_constructor_semantics_claim=0
generated_program_execution_claim=0
new_hako_syntax_added=0
```

The output only needs to be skeleton-safe. It does not need to preserve
executable Rust tuple-struct construction semantics.

## Acceptance

Add a focused fixture containing at least:

```rust
pub struct BasicBlockId(pub u32);

impl BasicBlockId {
    pub fn new(id: u32) -> Self {
        BasicBlockId(id)
    }
}
```

Verify:

```bash
python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/tuple_struct_constructor_subset.json \
  -o /tmp/tuple_struct_constructor_py.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_tuple_struct_constructor_fixture \
  apps/rust-subset-to-hako/convert_tuple_struct_constructor_fixture.hako
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
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
hakorune_mir_core_id_bundle_checked_in=0
rust_name_resolution_enabled=0
record_constructor_runtime_semantics=0
generated_program_execution_claim=0
```

## Next

After this skeleton-safety row is closed, resume:

```text
HAKORUNE-MIR-CORE-ID-MODULES-RUSTSUBSET-PILOT-001
```
