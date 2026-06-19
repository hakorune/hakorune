# 296x-1337 HAKORUNE-MIR-CORE-ID-MODULES-RUSTSUBSET-PILOT-001

Status: blocked_by_source_shape
Date: 2026-06-20

## Purpose

Materialize the selected `hakorune_mir_core` ID-module slice selected by
296x-1336:

```text
crate::basic_block_id
crate::binding_id
crate::value_id
```

## Probe

Generated a local RustSubset bundle from the full `hakorune_mir_core` adapter
output and selected the three modules above.

The Python reference converter produced a skeleton containing tuple-struct
constructor expressions such as:

```hako
function BasicBlockId_new(id: i64): Self {
    return BasicBlockId(id)
}
```

Focused generated-skeleton MIR check:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_core_id_modules_generated.mir.json \
  apps/rust-subset-to-hako/examples/hakorune_mir_core_id_modules_expected.hako
```

Result:

```text
MIR compilation error:
  Unresolved function: 'BasicBlockId'
```

## Diagnosis

This is not a crate graph blocker and not a wrapper EXE route blocker.

It is a RustSubset skeleton source-shape blocker:

```text
rust_tuple_struct_constructor_expression=unsupported_as_mir_safe_skeleton
```

Rust tuple newtype constructors like `BasicBlockId(id)`, `ValueId(id)`, and
`BindingId(id)` are currently emitted as plain function calls. Hako does not
treat record names as callable constructors in this generated skeleton context,
so generated function bodies fail before MIR acceptance.

## Decision

Do not check in the generated ID-module bundle yet.

Open a focused source-shape row first:

```text
RUST-SUBSET-TUPLE-STRUCT-CONSTRUCTOR-SKELETON-001
```

That row should make tuple-struct constructor expressions MIR-safe in skeleton
output without adding Rust name resolution or executable tuple-struct
semantics.

## Stop Line

```text
generated_id_module_bundle_checked_in=0
new_hako_syntax_added=0
rust_name_resolution_enabled=0
record_constructor_semantics_claim=0
generated_program_execution_claim=0
```

## Next

Continue:

```text
RUST-SUBSET-TUPLE-STRUCT-CONSTRUCTOR-SKELETON-001
```

After that row is closed, resume this pilot and re-materialize the selected
ID-module bundle through parse / MIR / wrapper EXE acceptance.
