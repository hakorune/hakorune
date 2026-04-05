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
- current front: `kilo_kernel_small_hk` 再ベースライン + `kilo_micro_substring_concat` / `kilo_micro_array_getset` 再確認
- blocker: `nyash_kernel` の構造分割は landed。split kernel 上で `main kilo` を reopen する
- landed:
  - `phase-134x nyash_kernel layer recut selection`
  - `phase-133x micro kilo reopen selection`
- active next:
  - `phase-kx vm-hako small reference interpreter recut`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-137x/README.md`

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

- `crates/nyash_kernel/src/exports/string.rs` split
- `crates/nyash_kernel/src/plugin/map_substrate.rs` thin-alias recut

## Current Proof Bundle

```bash
cargo check --manifest-path Cargo.toml --bin hakorune
bash tools/selfhost/mainline/stage1_mainline_smoke.sh
tools/checks/dev_gate.sh quick
git diff --check
```
