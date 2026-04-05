# Phase 141x: string semantic boundary review

- Status: Landed
- 目的: String を Array/Map のような owner cutover ではなく、`.hako semantic owner` と Rust lifetime/native substrate の二層で読む最終 stop-line を source-backed に固定する。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `lang/src/runtime/kernel/string/README.md`
  - `lang/src/runtime/kernel/string/chain_policy.hako`
  - `lang/src/runtime/kernel/string/search.hako`
  - `lang/src/runtime/collections/string_core_box.hako`
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/exports/string_view.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `crates/nyash_kernel/src/exports/string_plan.rs`
  - `crates/nyash_kernel/src/plugin/module_string_dispatch/**`
- success:
  - String seam is source-backed
  - `.hako` owner candidate is explicit
  - Rust thin facade and lifetime/native substrate are explicit
  - `module_string_dispatch/**` is fixed as quarantine, not owner
  - next lane returns to `phase-137x main kilo reopen selection`

## Decision Now

- `.hako` semantic owner:
  - `runtime/kernel/string/README.md`
  - `chain_policy.hako`
  - `search.hako`
- VM-facing runtime wrapper:
  - `string_core_box.hako`
- Rust thin facade:
  - `string.rs`
- Rust lifetime/native substrate:
  - `string_view.rs`
  - `string_helpers.rs`
  - `string_plan.rs`
- compat quarantine:
  - `module_string_dispatch/**`

## Fresh Read

- String is not a direct Array/Map-style owner move
- final semantic ownership belongs in `.hako` string-kernel policy/control modules
- `indexOf` / `lastIndexOf` search wrappers should read through `.hako` search owner helpers
- `string_core_box.hako` remains a VM/runtime wrapper, not the final semantic owner
- borrowed view/span ownership and materialize boundaries stay in Rust
- raw copy/search fast paths stay in Rust
- `module_string_dispatch/**` stays quarantine and must not become the permanent String owner

## Next

1. reopen `phase-137x main kilo reopen selection`
2. keep String host/lifetime substrate stable while perf work resumes
