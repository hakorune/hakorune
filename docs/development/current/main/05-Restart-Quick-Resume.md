---
Status: Active
Date: 2026-04-05
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/10-Now.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
```

## Current

- lane: `phase-137x main kilo reopen selection`
- current front: semantic-owner corridor が landed した前提で、split kernel 上の `main kilo` baseline を取り直す
- blocker: architecture corridor は閉じた。次は `main kilo` を reopen して split kernel の next hot leaf を選ぶ
- landed:
  - `phase-140x map owner pilot`
  - `phase-139x array owner pilot`
  - `phase-138x nyash_kernel semantic owner cutover`
  - `phase-134x nyash_kernel layer recut selection`
  - `phase-133x micro kilo reopen selection`
- active next:
  - `phase-137x main kilo reopen selection`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-137x/README.md`
4. `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`

## Decision Lock

- fixed perf order remains:
  - `leaf-proof micro`
  - `micro kilo`
  - `main kilo`
- `phase-133x` is closed:
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=3`
- `phase-134x` landed the split:
  - `keep / thin keep / compat glue / substrate candidate`
- `phase-138x` landed the final owner model:
  - `Rust host microkernel`
  - `.hako semantic kernel`
  - `native accelerators`
  - `ABI facade`
  - `compat quarantine`
- `phase-139x` landed the first pilot:
  - `ArrayCoreBox` / `ArrayStateCoreBox` hold visible semantics
  - `RawArrayCoreBox` / `PtrCoreBox` stay substrate
  - Rust `array_substrate.rs` stays thin ABI facade
  - Rust `array_runtime_facade.rs` stays compat/runtime forwarding
  - Rust cache/fast-path leaves stay native accelerators
- `phase-140x` landed the second pilot:
  - `MapCoreBox` / `MapStateCoreBox` hold visible semantics
  - `RawMapCoreBox` stays substrate
  - Rust `map_aliases.rs` stays thin facade
  - Rust `map_runtime_facade.rs` stays compat/runtime forwarding
  - Rust `map_probe.rs` / `map_slot_load.rs` / `map_slot_store.rs` stay native/raw leaves
- `phase-141x` landed the final boundary review:
  - `string.rs` stays thin ABI facade
  - `string_view.rs` / `string_helpers.rs` / `string_plan.rs` stay Rust lifetime/native substrate
  - `.hako` semantic owner lives under `runtime/kernel/string/**`
  - `string_core_box.hako` is the VM-facing runtime wrapper
  - `module_string_dispatch/**` stays quarantine, not owner

## First Design Slices

- `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
- `lang/src/runtime/kernel/string/README.md`
- `lang/src/runtime/kernel/string/chain_policy.hako`
- `lang/src/runtime/kernel/string/search.hako`
- `crates/nyash_kernel/src/exports/string.rs`
- `crates/nyash_kernel/src/exports/string_view.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch/README.md`

## Current Proof Bundle

```bash
cargo check --manifest-path Cargo.toml --bin hakorune
bash tools/selfhost/mainline/stage1_mainline_smoke.sh
tools/checks/dev_gate.sh quick
git diff --check
```
