# Phase 144x: string semantic owner follow-up

- Status: Active
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

1. keep `string_view.rs` / `string_helpers.rs` / `string_plan.rs` untouched as Rust lifetime/native substrate
2. move `StringCoreBox` visible wrapper residue toward `.hako` helper / string-kernel delegation
3. hand off to `phase-137x main kilo reopen selection`
