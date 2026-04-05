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

- lane: `phase-138x nyash_kernel semantic owner cutover`
- current front: `Rust host microkernel` / `.hako semantic kernel` / `native accelerators` の最終 owner model を固定し、`Array owner` pilot を次の実装 lane にする
- blocker: `nyash_kernel` の4層 split は landed。`main kilo` を reopen する前に semantic ownership の最終形を current SSOT に固定する
- landed:
  - `phase-134x nyash_kernel layer recut selection`
  - `phase-133x micro kilo reopen selection`
- active next:
  - `phase-139x array owner pilot`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-138x/README.md`
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
- current work is architectural:
  - fix the final owner model after the split
  - keep host/kernel keep, ABI thin facade, and hot leaf substrate in Rust
  - move semantic ownership and collection policy toward `.hako`
  - keep compat quarantine out of the permanent owner graph

## First Design Slices

- `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
- `crates/nyash_kernel/src/plugin/array_substrate.rs`
- `crates/nyash_kernel/src/plugin/map_aliases.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch/`

## Current Proof Bundle

```bash
cargo check --manifest-path Cargo.toml --bin hakorune
bash tools/selfhost/mainline/stage1_mainline_smoke.sh
tools/checks/dev_gate.sh quick
git diff --check
```
