# De‑Rust Gap Analysis (Phase 21.9)

This memo enumerates gaps to execute Phase‑1/2 tasks safely.

## hv1_inline → Parity
- JSON support: ensure compare/branch/jump/phi/mir_call minimal are covered.
- Parity canaries: phase2037/* (flow), phase2170/* (state).
- Env guard: consider `HAKO_VERIFY_DISABLE_INLINE=1` for hard opt‑out (optional).

## TLV Codec → C shim
- Minimal C API: tlv_encode(buf,len, out_ptr, out_len) / tlv_decode(...)
- FFI wrapper in Rust; map errors to Result.
- Tests: round‑trip on byte arrays; malformed inputs; size limits.

## MIR Interpreter (Rust) — Diagnostic mode
- Keep as fallback/diagnostic; Primary = Hakovm.
- Ensure boxcall/mir_call minimal parity for sample set.
- Gate via env to avoid accidental use in primary flows.

## LLVM Wrapper → Hako/C Harness
- Keep Python llvmlite primary until Hako IR is mature.
- Hako wrapper CLI to drive harness with env (`NYASH_LLVM_USE_HARNESS=1`).
- Tests: small end‑to‑end const/ret and simple branch; IR dump gate remains.

## Resolver/Using SSOT (Hako First)
- Runner policy and HakoCheck strip/order unified; document precedence.
- Strict profiles disallow path‑literal using.

## FileBox Core → C (later)
- Small C layer for read‑only (Analyzer) and read/write (dev) under caps; Rust shim remains.

## Acceptance Summary
- Build: cargo build (default) green.
- Smokes: quick/core green; parity canaries pass.
- Docs: roadmap + plan page + restore steps present.
