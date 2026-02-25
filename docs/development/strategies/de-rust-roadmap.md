# De‑Rust Roadmap (Phase 21.9+)

Purpose: reduce Rust surface (non‑plugin) while keeping correctness and reversibility. Make Hakorune (Hako) the primary host for parsing/building/executing; keep Rust for kernel/ABI/thin wrappers.

## Phases

### Phase 0 — Archive (low risk)
- Target: Rust LLVM backend (src/backend/llvm) — deprecated in favor of Python llvmlite.
- Action: move to `archive/rust-llvm-backend/` with a clear RESTORE.md.
- Acceptance: `cargo build --release` (default features) remains green; quick smokes green.
- Revert: `git mv archive/rust-llvm-backend/llvm src/backend/`.

### Phase 1 — Parser/MIR Hako‑first（22.0）
- Make Hako the primary for Parser/MIR; Rust builder becomes fallback.
- Verify quick canaries green under registry; keep hv1 inline parity.

### Phase 2 — TLV C shim & Resolver SSOT（22.1）
- TLV codec to C shim (+ Rust FFI); Resolver/Using SSOT in Hako shared by runner/analyzer.

Deliverables & Tests
- 22.0: registry builder default ON; hv1直列 green; Core/Interpreter is diagnostic.
- 22.1: TLV round‑trip smokes; Using SSOT parity between runner/analyzer.

### Phase 3 — Core Thinning I（22.2, 2–4 months）
- Plugin loader thin C wrapper (dlopen/dlsym) and basic boxes C core; Rust shim remains.

SSOT for Using/Resolver (summary)
- Resolution order: modules (nyash.toml) → relative path inference → not found (warn) with verbose details.
- Analyzer/HakoCheck follows the same order; runner shares policy helpers. No path‑literal using in strict profiles.

### Phase 4 — Long‑haul（22.3, 3–6 months）
- Python llvmlite → Hako IR builder + C ABI.
- Parser/MIR builder fully Hako‑first; Rust becomes fallback.
- NyKernel minimal C runtime (BoxCall dispatcher + collections + file).

### Phase 21.10 — LLVM Line Unification (SSOT + crate probe)
- SSOT builder (`tools/ny_mir_builder.sh`) selects backend by env; crate path opt‑in.
- Add crate S3 canaries (ternary/map/print); defaults unchanged.

### Phase 21.11 — Flip default to crate (ny-llvmc)
- Make crate default when available; llvmlite becomes opt‑in.
- S3 reps run via crate in quick; legacy remains available.

### Phase 21.12 — Hako Native LLVM Builder (bootstrap)
- Experimental native (Hako→LLVM C API) path for minimal EXE.
- Behind `NYASH_LLVM_BACKEND=native` toggle; no default impact.

### Phase 21.13 — llvmlite deprecation (default off)
- Remove llvmlite from auto paths; keep explicit toggle + optional CI job.

### Phase 21.14 — Optimization & Perf Harness
- Perf harness + PHI invariants; optimize hot paths; publish numbers.

## Principles
- Guard everything by env/features; defaults unchanged.
- Keep changes reversible (small diffs, RESTORE docs, fallbacks).
- Test gates: quick smokes + representative hv1/hakovm parity.

## Today (suggested)
1) Lock 22.0 (Parser/MIR Hako‑first) — builder registryを既定ON、quickが緑。
2) Prepare 22.1 (TLV C shim & Resolver SSOT) — I/F草案と最小スモーク。
3) LLVM統一（21.10–21.14）は並行で準備、切替は22.x完了後に本格実施。

## Test Strategy (gates)
- Quick: tools/smokes/v2/profiles/quick/core/* (phase2037 flow, phase2170 state) — green.
- Verify routing: HAKO_VERIFY_PRIMARY=hakovm (default); hv1_inline perf path parity (env toggles only).
- Build: `cargo build --release` (default features); LLVM paths are opt‑in.
- Docs: keep RESTORE steps for any archived parts; small diffs, easy rollback.

## Convergence Plan — Line Consolidation (A→D)

Goal: reduce parallel lines (Rust/Hako builders, VM variants, LLVM backends) to a clear SSOT while keeping reversibility.

Phase A — Stabilize (now)
- SSOT: semantics/normalization/optimization live in Hako (AotPrep/Normalize).
- Rust: limit to structure/safety/emit (SSA/PHI/guards/executor). No new rules.
- Gates: quick/integration canaries green; VM↔LLVM parity for representatives; no default flips.

Phase B — Defaultization (small flips)
- Stage‑B/selfhost builder: default ON in dev/quick; provider as fallback. Document toggles and rollback.
- AotPrep passes: enable normalize/collections_hot behind canaries; promote gradually.
- Docs: ENV_VARS + CURRENT_TASK に昇格条件/戻し手順を明記。

Phase C — Line Thinning
- LLVM: prefer crate (ny-llvmc) as default; llvmlite becomes optional job (deprecation window).
- VM: Hakorune VM = primary; PyVM = reference/comparison only.
- Remove duplicated heavy paths from default profiles; keep explicit toggles for restore.

Phase D — Toggle Cleanup & Sunsets
- Once stable in defaults for ≥2 weeks: remove legacy toggles and code paths (e.g., Rust normalize.rs).
- Record sunset plan (reason/range/restore) in CURRENT_TASK and changelog.

Acceptance (each phase)
- quick/integration green, parity holds (exit codes/log shape where applicable).
- Defaults unchanged until promotion; any flip is guarded and reversible.
- Small diffs; explicit RESTORE steps; minimal blast radius.
