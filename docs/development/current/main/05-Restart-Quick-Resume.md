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

- lane: `phase-134x nyash_kernel layer recut selection`
- current front: `exports/string.rs` split inventory + `plugin/map_substrate.rs` thin-alias inventory
- blocker: `.hako` 移植を先に始めない。先に `ABI / glue / substrate` を Rust 側で切り分ける
- landed: `phase-133x micro kilo reopen selection`
- active next:
  - `phase-135x string export split`
  - `phase-136x map substrate thin-alias recut`
  - `phase-137x main kilo reopen selection`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-134x/README.md`

## Decision Lock

- fixed perf order remains:
  - `leaf-proof micro`
  - `micro kilo`
  - `main kilo`
- `phase-133x` is closed:
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=3`
- current work is structural:
  - classify `nyash_kernel` into `keep / thin keep / compat glue / substrate candidate`
  - do not start broad `.hako` migration before that split is source-backed

## First Source Slices

- `crates/nyash_kernel/src/exports/string.rs`
- `crates/nyash_kernel/src/plugin/map_substrate.rs`

## Current Proof Bundle

```bash
cargo check --manifest-path Cargo.toml --bin hakorune
bash tools/selfhost/mainline/stage1_mainline_smoke.sh
tools/checks/dev_gate.sh quick
git diff --check
```
