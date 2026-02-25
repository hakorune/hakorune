# MIR Builder — Calls SSOT

Scope
- This directory is the single source of truth for call shaping in the builder.
- Responsibilities: target resolution, extern mapping, method lookup, flags/effects, MIR emission.

Out of scope
- Runtime dispatch details (VM/LLVM) and legacy by-name resolution. The VM keeps a legacy resolver only behind a dev flag for bring-up.

Contract
- Builder must populate `MirInstruction::Call` with a concrete `Callee` whenever possible.
- Arity and canonical names are normalized here so runtimes can be simple routers.

Phase-3 alignment
- VM call resolver is treated as legacy-only. Default runtime disables by-name fallback.
- Extern interface normalization aligns with `handlers/calls/externs.rs` (runtime SSOT for extern dispatch).

