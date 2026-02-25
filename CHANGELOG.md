# Nyash Project – Changelog (Work in progress)

This changelog tracks high‑level milestones while Core MIR and Phase 12 evolve. For detailed per‑file history, see git log and docs under `docs/development/roadmap/`.

## 2025‑09‑06

### Phase 22.3 (Kernel Minimal C Runtime — design wrap)
- Added minimal C runtime crate (design-stage): `crates/nyash_kernel_min_c` (staticlib).
- LLVM extern lowering: normalized `nyash.console.*` → `nyash_console_*` for C linkage.
- Hako-first MIR emit stabilized via wrapper (`tools/hakorune_emit_mir.sh`): Stage‑B JSON is emitted with `NYASH_JSON_ONLY=1` and sanitized; builder falls back to Rust CLI on failure (defaults keep quick green).

### Prep for Phase 21.10 (LLVM line — crate backend)
- Backend selector and S3 canaries exist; print EXE canary currently SKIP pending small normalization. Next: enable print EXE PASS with env‑guarded normalization in ny‑llvmc (no default behavior changes).
- Core‑13 flip complete: code/tests enforce Core‑13 minimal kernel. Normalizations (Array/Ref→BoxCall, TypeCheck/Cast/Barrier/WeakRef unification) are ON by default via env (NYASH_MIR_CORE13=1). New tests validate normalization.
- Docs synced: step‑50 marked done; DEV quickstart points to Core‑13 reference.

## 2025‑09‑04
- Phase 12.7‑A complete: peek, continue, `?` operator, lambda, field type annotations. Language reference updated.
- Phase 12.7‑B (basic) complete: parser‑level desugaring for `|>`, `?.`, `??`, `+=/-=/*=/=`, `..` behind `NYASH_SYNTAX_SUGAR_LEVEL`.
- Docs: language reference and Phase 12.7 README updated to reflect basic completion; extensions tracked under gated plan.
- MIR Core migration: previously enforcing Core‑15 during transition; superseded by 2025‑09‑06 Core‑13 flip.

## 2025‑09‑03
- Nyash ABI TypeBox integration stabilized across core boxes; differential tests added; loader defaults adjusted (builtin + plugins).

---

Notes
- Core‑13 is canonical minimal kernel. Historical Core‑15 notes remain under `docs/development/roadmap/` for reference.
- Phase 12.7‑B desugaring is gated by `NYASH_SYNTAX_SUGAR_LEVEL`; tokenizer additions are non‑breaking.
## 2025‑09‑11 (Phase 15)
- llvm: BoxCall arm cleanup — unreachable legacy block removed; arm now delegates solely to `instructions::lower_boxcall`.
- llvm/docs: Documented LLVM lowering rules (StringBox i8* fast path, ExternCall ptr/handle selection, minimal fallback policy for string concat).
- docs: Added ARCHITECTURE.md, LOWERING_LLVM.md, EXTERNCALL.md, PLUGIN_ABI.md.
- nyrt: resolved plugin module duplication; build green.
- builder: suppressed StringBox birth (constructed in LLVM path).
## 2025‑11‑06 — Phase 21.0 (Full Self‑Hosting)
- Full Self‑Hosting achieved (DoD met):
  - S1/S2 determinism: v1 emit for const/compare/threeblock‑collect repeats 3× with identical normalized hash.
  - PRIMARY no‑fallback (hv1 inline): selfhost_v1 minimal Option‑A/B reps PASS (rc=42).
  - S3 (llvmlite + Nyash Kernel link + run): ternary collect (rc=44) / map set→size (rc=1) PASS. Auto‑runs when LLVM18/llvmlite is present.
- Harness/root changes:
  - ny_mir_builder.sh routes v1 JSON directly to llvmlite harness; EXE links against libnyash_kernel.a.
  - Python LLVM builder fixes: cmp field normalization (Lt/Le/… → < <= …), ret PHI synthesis (avoid ret=0), flat mir_call shape acceptance.
- Using/alias polish (prod): modules mapping preferred; missing module aliases added (StrCast / MethodAliasPolicy). Duplicate using in VM extern_provider cleaned.
- Docs: phase‑21.0 marked COMPLETE (Quick Verify updated). CURRENT_TASK closed.

## 2025‑11‑06 — Phase 21.1 (LLVM C‑API emit‑only) finalize
- C‑API path green for both reps (ternary rc=44 / map set→size rc=1). `tools/smokes/v2/profiles/quick/core/phase2111/run_all.sh` runs both.
- VM handler cleanup (SSOT): removed duplicated `hostbridge.extern_invoke` branches in `calls.rs`; all externs route through `extern_provider_dispatch`.
- Trace polish: consolidated `[hb:*]` to low‑noise, gated by `HAKO_CABI_TRACE=1`.
- Docs updated: phase‑21.1 README set to COMPLETE with Quick Verify and troubleshooting.
