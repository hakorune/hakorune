# 164x-90: repo-wide fmt drift cleanup SSOT

Status: SSOT
Date: 2026-04-11
Scope: formatting-only cleanup for the repo-wide `cargo fmt --check` drift. This is separate from `phase-163x` optimization work.

## Confirmed Inventory

The worker investigation confirmed that these files are the current repo-wide fmt drift set:

- `crates/nyash_kernel/src/exports/string_helpers/concat.rs`
- `crates/nyash_kernel/src/plugin/array.rs`
- `crates/nyash_kernel/src/tests/string.rs`
- `src/backend/wasm/codegen/tests.rs`
- `src/boxes/array/mod.rs`
- `src/core/instance_v2.rs`
- `src/grammar/generated.rs`
- `src/mir/phi_query.rs`
- `src/mir/string_corridor.rs`
- `src/mir/sum_placement.rs`
- `src/runner/mir_json_emit/emitters/sum.rs`

## Exclusion

- `src/mir/passes/escape.rs` passes `rustfmt --check` and stays outside this cleanup set.

## Acceptance

- repo-wide `cargo fmt --check` passes
- only formatting changes are included
- no behavior changes, refactors, or optimization slices are mixed into this corridor

## Boundary Rule

- do not commit this cleanup together with `phase-163x` optimization work
- if a future formatting diff touches other files, extend the inventory first and keep the SSOT in sync
