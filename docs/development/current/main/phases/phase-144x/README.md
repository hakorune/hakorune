# Phase 144x: string semantic owner follow-up

- Status: Landed
- 目的: `phase-141x` で固定した String stop-line の上で、semantic owner を `.hako` 側に保ったまま enforcement を強める。

## Decision Now

- `.hako` semantic owner:
  - `runtime/kernel/string/**`
- VM-facing wrapper:
  - `string_core_box.hako`
- Rust thin facade:
  - `string.rs`
- Rust lifetime/native substrate:
  - `string_view.rs`
  - `string_helpers.rs`
  - `string_plan.rs`
- quarantine:
  - `module_string_dispatch/**`

## Next

1. `string_view.rs` / `string_helpers.rs` / `string_plan.rs` stayed untouched as Rust lifetime/native substrate
2. `StringCoreBox.{size,indexOf,lastIndexOf,substring}` now reads through helperized wrapper paths
3. `indexOf(search, fromIndex)` delegates to `.hako` search owner via `StringSearchKernelBox.find_index_from(...)`
4. hand off to `phase-137x main kilo reopen selection`
