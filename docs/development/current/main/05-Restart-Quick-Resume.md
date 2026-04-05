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

- lane: `phase-139x array owner pilot`
- current front: `ArrayCoreBox` / `ArrayStateCoreBox` を visible semantics owner として固定し、Rust を ABI facade + raw substrate + accelerators に保つ
- blocker: final owner graph は fixed。次は `Array owner` pilot で、owner と forwarding の境界を source-backed に固定する
- landed:
  - `phase-138x nyash_kernel semantic owner cutover`
  - `phase-134x nyash_kernel layer recut selection`
  - `phase-133x micro kilo reopen selection`
- active next:
  - `phase-140x map owner pilot`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-139x/README.md`
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
- current work is the first pilot:
  - `ArrayCoreBox` / `ArrayStateCoreBox` hold visible semantics
  - `RawArrayCoreBox` / `PtrCoreBox` stay substrate
  - Rust `array_substrate.rs` stays thin ABI facade
  - Rust `array_runtime_facade.rs` stays compat/runtime forwarding
  - Rust cache/fast-path leaves stay native accelerators

## First Design Slices

- `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
- `lang/src/runtime/collections/array_core_box.hako`
- `lang/src/runtime/collections/array_state_core_box.hako`
- `crates/nyash_kernel/src/plugin/array_substrate.rs`
- `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`

## Current Proof Bundle

```bash
cargo check --manifest-path Cargo.toml --bin hakorune
bash tools/selfhost/mainline/stage1_mainline_smoke.sh
tools/checks/dev_gate.sh quick
git diff --check
```
