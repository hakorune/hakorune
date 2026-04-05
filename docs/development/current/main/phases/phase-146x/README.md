# Phase 146x: string semantic boundary tighten

- Status: Landed
- 目的: `.hako` string semantic owner, VM wrapper, Rust native substrate の stop-line を source/docs 上でさらに読みやすくする。
- 対象:
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `lang/src/runtime/kernel/string/README.md`
  - `lang/src/runtime/collections/string_core_box.hako`
  - `crates/nyash_kernel/src/exports/string_view.rs`
  - `crates/nyash_kernel/src/exports/string_plan.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`

## Decision Now

- `.hako` owns:
  - search/route/control policy
  - semantic owner vocabulary
- Rust owns:
  - borrowed view/span lifetime
  - materialize/copy/search fast leaf
  - native accelerator helpers
- `string_core_box.hako` stays a VM-facing wrapper, not the final semantic owner

## Exit Criteria

1. `.hako` string kernel README names policy/control ownership directly
2. `string_core_box.hako` names itself as wrapper residue, not owner
3. `string_view.rs` / `string_plan.rs` / `string_helpers.rs` state their Rust substrate role clearly
4. wrapper helper names stop looking like final owner entrypoints where avoidable
5. `phase-137x` can reopen optimization without string-boundary ambiguity

## Current Slice

- `StringCoreBox` helper names should read as wrapper-via-owner adapters
- `.hako` string README should say policy/control truth lives above the wrapper
- Rust `string_view.rs` / `string_plan.rs` / `string_helpers.rs` stay native substrate only

## Next

1. reopen `phase-137x main kilo reopen selection`
2. re-bundle string const-path vs array string-store leaves
