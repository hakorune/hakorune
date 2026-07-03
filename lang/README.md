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
  - boundary artifact space, not semantic-owner space
  - `README.md` — responsibilities, build notes, platform caveats
  - `include/` — public headers (mirrored or thin wrappers)
  - `shims/` — libc-backed shim(s) for canaries and local testing
- `src/runtime/kernel/` — logical `hako_kernel` runtime semantic owner lane
- `src/runtime/substrate/` — logical `hako_substrate` runtime algorithm substrate lane
- `src/runtime/host/` — host-call routing facade only
- `src/runtime/meta/` — compiler semantic tables and future owner-policy boxes
  - runtime/kernel owns runtime behavior; runtime/meta owns compiler semantic tables
- `src/compat/` — compat/proof and legacy bridge surfaces
- `src/hako_alloc/` — `.hako` alloc-layer (policy plane) helpers (e.g. `ArcBox`, `RefCellBox`)
- `src/hako_std/` — reserved future library root for process/env/fs/time/net/plugin-host/C ABI facades

Layering contract
- public layering: `hako_core / hako_alloc / hako_std`
- runtime internal layering: `hako_kernel / hako_substrate`
- capability floor: `hako.abi / hako.value_repr / hako.mem / hako.buf / hako.ptr / hako.atomic / hako.tls / hako.gc / hako.osvm`
- native metal keep: final ABI stubs, alloc/free backend, root snapshot, reachability walk, final GC hooks, TLS/atomic fallback, OS VM glue, backend emission
- do not introduce `hako.sys` as a catch-all layer noun
- do not use `hako.rt` as a competing kernel-owner noun

Build & Link (dev)
- C shim: build a shared library to satisfy symbols for the LLVM line canaries.
- Link flags example:
  - Linux: `-L$(pwd)/target/release -Wl,-rpath,$(pwd)/target/release -lhako_kernel_shim`

Non‑Goals
- Plugin loader, HostBridge router, Box/Type system — kept in Rust.

## Selfhost Launcher (AOT)

### Dev line (phase-1 core compatibility – experimental)

- Dev build: `tools/selfhost/mainline/build_stage1.sh` → produces `target/selfhost/hakorune`
- Role:
  - Fast iteration用の phase-1 selfhost バイナリ（Ny Executor / CLI 実験など）。
  - bridge/proof line only; not daily distribution truth.
  - new CLI/runner 機能はまずこちらで開発・検証する。

### Stable line (lang bin – snapshot)

- Stable binary: `lang/bin/hakorune`
- Build (pure-lang launcher, legacy bring-up):
  - `lang/build/build_runner.sh` → produces `lang/bin/hakorune`
  - Requirements: LLVM 18 dev (`llvm-config-18`)
- Policy（Phase 25.1 以降の想定）:
  - `target/selfhost/hakorune` で十分に安定したら、その成果物を `lang/bin/hakorune` に昇格させる（手動コピー or 専用スクリプト）。
  - `lang/bin/hakorune` は「last known good」の phase-1 コア EXE として扱い、配布や外部からの参照時は原則こちらを基準にする。
  - ただしこれは phase-1 bridge/proof reading であり、final distribution truth ではない。

Notes
- `lang/` 以下は「最終的に 1 つの phase-1 コア EXE（hakorune）を構成するソース群」という前提で整理する。
- Daily selfhost vocabulary is owned by
  `docs/development/current/main/design/selfhost-program-json-boundary-vocabulary-ssot.md`.
  Read the current compiler vocabulary boundary as `Program(JSON v0)`:
  legacy `stage0` is the Rust authority side and legacy `stage1` is the `.hako`
  side crossing that boundary one shape at a time. This is not a claim that Program(JSON v0)
  is the preferred day-to-day runner route; MIR-first routes remain owned by
  the selfhost route-map docs.
- `hako_core / hako_alloc / hako_std` are logical library layers; the physical roots today are `lang/src/runtime/kernel/`, `lang/src/runtime/substrate/`, and `lang/src/hako_alloc/`.
- compat/proof payloads live under `lang/src/compat/` so owner-looking paths can stay thin.
- `hako_kernel` / `hako_substrate` are logical owner nouns; do not read them as same-named physical directories.
- `hako_std` is reserved as a logical future layer until a physical `lang/src/hako_std/` root is intentionally materialized.
- `target/selfhost/hakorune` は開発中の最新版、`lang/bin/hakorune` は安定版スナップショットという役割分担にする。
- artifact/lane の親SSOTは `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`。
- artifact-role detail と future interpreter reservation は `docs/development/current/main/design/artifact-policy-ssot.md` を正本にする。
- `phase-1` / `K2+` は artifact / proof / mainline の distribution 軸であって、kernel owner/substrate 軸とは別だよ。
- The following K-axis terms are roadmap / historical distribution vocabulary,
  not the daily bug-owner selection model:
  - `K0 = all-Rust hakorune`
  - `K1 = .hako kernel migration stage`
  - `K2 = .hako kernel mainline / zero-rust daily-distribution stage`
  - `K2-core` / `K2-wide` are task packs inside `K2`
- owner/substrate の current truth は `docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md` と `docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md` を正本にする。
- kernel implementation phase plan SSOT is `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`.
- final distribution target は K2+ line であり、`lang/bin/hakorune` そのものを final 配布物の意味で読むのはやめる。
- default distribution shape は `hakoruneup + self-contained release bundle` であり、単一の phase artifact をそのまま配布正本とは読まない。
- selfhost と `hako_core/alloc/std` の end-state は `docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md` を正本にする。
- current artifact reality:
  - `target/release/hakorune`
  - `target/selfhost/hakorune`
  - `lang/bin/hakorune`
- target artifact contract:
  - `target/k0/hakorune`
  - `target/k1/hakorune`
  - `artifacts/k0/hakorune`
  - `artifacts/k1/hakorune`
  - `dist/k2/<channel>/<triple>/bundle/`
- roadmap reading only: read `K0/K1` primarily as binaries and `K2`
  primarily as a bundle.
- roadmap reading only: phase-1 may complete domain phases and still remain
  bridge/proof; K2+ is an end-state/mainline distribution reading, not the
  daily bug-owner vocabulary.
- `.hako` complete は authority completion を意味し、kernel substrate や native keep の wholesale removal は意味しない。
