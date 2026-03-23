# Hakorune Lang Line — Rust-less Kernel (C ABI)

Scope
- This `lang/` tree hosts the script-driven C ABI kernel artifacts for Phase 20.9+.
- Goal: keep the runtime data plane callable without Rust on the hot path (Hakorune → LLVM → C ABI).
- Backend-zero target: `.hako` callers should ultimately hit a thin backend C ABI/plugin boundary from this tree, not a Rust-only backend owner.

Principles
- Separation: do not mix Rust crates or cargo-specific layout under this tree.
- Ownership & ABI:
  - Any `char*` returned across the ABI is owned by the callee and must be freed via `hako_mem_free()`.
  - Do not mix CRT `free()` across boundaries.
- Fail‑Fast: no silent fallbacks. Missing symbols must be observable via short diagnostics.

Layout (initial)
- `c-abi/` — C shim(s) and headers for the minimal kernel surface
  - `README.md` — responsibilities, build notes, platform caveats
  - `include/` — public headers (mirrored or thin wrappers)
  - `shims/` — libc-backed shim(s) for canaries and local testing
- `src/runtime/kernel/` — `.hako` runtime kernel logic (default edit lane)
- `src/runtime/host/` — host-call routing facade only
- `src/hako_alloc/` — `.hako` alloc-layer (policy plane) helpers (e.g. `ArcBox`, `RefCellBox`)

Build & Link (dev)
- C shim: build a shared library to satisfy symbols for the LLVM line canaries.
- Link flags example:
  - Linux: `-L$(pwd)/target/release -Wl,-rpath,$(pwd)/target/release -lhako_kernel_shim`

Non‑Goals
- Plugin loader, HostBridge router, Box/Type system — kept in Rust.

## Selfhost Launcher (AOT)

### Dev line (Stage1 core – experimental)

- Dev build: `tools/selfhost/build_stage1.sh` → produces `target/selfhost/hakorune`
- Role:
  - Fast iteration用の Stage1 selfhost バイナリ（Ny Executor / CLI 実験など）。
  - bridge/proof line only; not daily distribution truth.
  - new CLI/runner 機能はまずこちらで開発・検証する。

### Stable line (lang bin – snapshot)

- Stable binary: `lang/bin/hakorune`
- Build (pure-lang launcher, legacy bring-up):
  - `lang/build/build_runner.sh` → produces `lang/bin/hakorune`
  - Requirements: LLVM 18 dev (`llvm-config-18`)
- Policy（Phase 25.1 以降の想定）:
  - `target/selfhost/hakorune` で十分に安定したら、その成果物を `lang/bin/hakorune` に昇格させる（手動コピー or 専用スクリプト）。
  - `lang/bin/hakorune` は「last known good」の Stage1 コア EXE として扱い、配布や外部からの参照時は原則こちらを基準にする。
  - ただしこれは stage1 snapshot/proof reading であり、final distribution truth ではない。

Notes
- `lang/` 以下は「最終的に 1 つの Stage1 コア EXE（hakorune）を構成するソース群」という前提で整理する。
- `target/selfhost/hakorune` は開発中の最新版、`lang/bin/hakorune` は安定版スナップショットという役割分担にする。
- stage/artifact/lane の親SSOTは `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`。
- `Stage1` / `Stage2+` は artifact / proof / mainline の stage 軸であって、kernel owner/substrate 軸とは別だよ。
- owner/substrate の current truth は `docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md` と `docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md` を正本にする。
- final distribution target は Stage2+ line であり、`lang/bin/hakorune` そのものを final 配布物の意味で読むのはやめる。
- stage/selfhost と `hako_core/alloc/std` の end-state は `docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md` を正本にする。
