# 296x-1365 RUST-SUBSET-REFERENCE-TYPE-SPELLING-SKELETON-SAFETY-001

Status: open
Date: 2026-06-20

## Purpose

Make RustSubset reference type spellings parser-safe when they appear inside
generated `.hako` type annotations.

The immediate real-front blockers are:

```text
hakorune_mir_builder::metadata_context
hakorune_mir_builder::type_context
```

Both currently produce invalid skeleton type spelling:

```hako
function MetadataContext_value_caller(...): Option<&str> {
function TypeContext_get_origin_box(...): Option<&str> {
```

## Scope

Allowed:

```text
adapter/converter type spelling normalization for reference types
fixture covering nested Option<&str>
metadata_context and type_context reprobe
```

Not allowed:

```text
new_hako_syntax_added=0
borrow_semantics_enabled=0
lifetime_semantics_enabled=0
rust_name_resolution_enabled=0
generic_semantics_enabled=0
closure_handoff_changed=0
generated_program_execution_claim=0
```

## Design Contract

```text
reference_type_source:
  Rust source spelling may contain &str / &T

generated_skeleton_type:
  must be parser-safe .hako type spelling
  does not claim Rust borrow/lifetime semantics

converter_core:
  may map conservative type spellings
  must not infer Rust borrow semantics
```

For this row, `&str` inside a generic type argument may become `String` in the
skeleton type spelling. This is skeleton transport only, not a semantic Rust
borrow model.

## Acceptance

Required checks:

```bash
cargo check -q --lib
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Focused evidence:

```text
reference_type_fixture_parser_safe=1
metadata_context_option_ref_str_parser_safe=1
type_context_option_ref_str_parser_safe=1
closure_handoff_changed=0
generated_program_execution_claim=0
summary=ok
```

## Stop Line

```text
do_not_add_reference_or_lifetime_semantics
do_not_add_hako_reference_syntax
do_not_fix_closure_handoff_in_this_row
do_not_claim_metadata_context_or_type_context_materialization_unless_full_gate_reaches_mir_emit
```
