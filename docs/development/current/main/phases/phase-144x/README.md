# Phase 144x: string semantic owner follow-up

- Status: Successor
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

1. follow `phase-143x` Map cutover
2. strengthen String semantic-owner enforcement without moving lifetime substrate
3. hand off to `phase-137x main kilo reopen selection`
