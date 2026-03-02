---
Status: SSOT
Date: 2026-03-02
Scope: Build/Kernel lane の混線防止。日常 lane と保守 lane を明示分離する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Build Lane Separation SSOT

## Purpose

- 目的は `.hako` kernel mainline を既定運用に固定し、silent fallback を禁止すること。
- Rust/cargo は「保守 lane」に限定し、日常開発の主導線から外す。
- 入口文書（`CURRENT_TASK.md` / `10-Now.md` / `05-Restart-Quick-Resume.md`）の記述をこの方針に一致させる。

## Kernel Lane Naming

- `kernel-mainline`
  - `.hako` kernel 実行経路。
  - 既定条件: `NYASH_VM_USE_FALLBACK=0`（no-compat, fail-fast）。
- `kernel-bootstrap`
  - Rust static archive（`libnyash_kernel.a`）を使う保守経路。
  - 役割: portability 確認、artifact refresh、回帰切り分け。

## Build Lane Policy

- `build-mainline`（日常）
  - cargo 非依存を基本とする。
  - 既存 artifact を再利用して `.hako` 側の改善ループを回す。
  - 前提 artifact（`target/release/hakorune`, `target/release/ny-llvmc`, `libnyash_kernel.a`）が欠落した場合は `build-maintenance` を先に1回実行する。
  - 推奨入口:
    - `tools/selfhost/build_stage1.sh --artifact-kind launcher-exe --reuse-if-fresh 1`
    - `tools/checks/dev_gate.sh runtime-exec-zero`
    - `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
    - `bash tools/perf/run_kilo_hk_bench.sh strict 1 3`
- `build-maintenance`（必要時のみ）
  - cargo build/check/test を実行して host artifact を更新する。
  - 実行例:
    - `cargo check --release --bin hakorune`
    - `cargo build --release --bin hakorune`
    - `(cd crates/nyash_kernel && cargo build --release)`

## Route / Fallback Contract

- no-fallback 契約は `kernel-mainline` で常時有効にする。
- `kilo_kernel_small_hk` 計測は strict wrapper を既定入口にする。
  - `tools/perf/run_kilo_hk_bench.sh strict ...` は `PERF_VM_FORCE_NO_FALLBACK=1` を強制する。
- fallback を許容した run は性能比較の採用値にしない。

## Operation Rule

- 日常コミットは `build-mainline` の緑を優先する。
- `build-maintenance` は以下のタイミングに限定する。
  - host artifact が欠落/破損しているとき
  - portability 監査（macOS/Windows/CI）を更新するとき
  - host 側回帰の切り分けが必要なとき
