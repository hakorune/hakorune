# Phase 141x-90: string semantic boundary review SSOT

## Goal

Lock the final String stop-line: semantic ownership may move upward, but
lifetime-sensitive and native fast-path substrate stays in Rust.

## Boundary Graph

- `.hako` semantic owner:
  - `lang/src/runtime/kernel/string/README.md`
  - `lang/src/runtime/kernel/string/chain_policy.hako`
  - `lang/src/runtime/kernel/string/search.hako`
- VM-facing runtime wrapper:
  - `lang/src/runtime/collections/string_core_box.hako`
- Rust thin facade:
  - `crates/nyash_kernel/src/exports/string.rs`
- Rust lifetime/native substrate:
  - `crates/nyash_kernel/src/exports/string_view.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `crates/nyash_kernel/src/exports/string_plan.rs`
- compat quarantine:
  - `crates/nyash_kernel/src/plugin/module_string_dispatch/**`

## Stop Lines

- do not move borrowed `StringView` / `StringSpan` ownership out of Rust in this lane
- do not move raw copy/search/materialize fast paths out of Rust in this lane
- keep `string.rs` as thin ABI facade only
- do not let `module_string_dispatch/**` become a permanent owner

## Success Condition

- the String semantic boundary is explicit
- `.hako` string-kernel policy/control modules are fixed as semantic owner
- `string_core_box.hako` is fixed as VM-facing wrapper, not final owner
- lifetime/native substrate stays Rust-owned
- quarantine is fixed as non-owner
- `phase-137x` can reopen perf work on top of the settled architecture
