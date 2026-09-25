# MIR-CALL-R7-CALLER-ZERO-D4 — next bounded LegacyCallV0 deletion census

Status: open__census__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R7-CALLER-ZERO-D3 (accepted 2026-09-25;
        S8 landed this session)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             Call/R7 frontier.

## Question

All production `LegacyCallV0` ingress minters are retired and every
canonicalize repair arm is deleted (S6 Method, S7 array-write fn,
S8 Closure). Which remaining `LegacyCallV0` surface is caller-zero and
bounded for the next deletion?

## Census boundary

- Start: `MirInstruction::LegacyCallV0` type definition and every
  match arm / constructor / reader on it.
- End: terminal readers (interpreter dispatch, published view
  admission, v0 compat emission, llvmlite projection, test fixtures).
- Includes: pass arms, admission arms, `func`-slot readers,
  `exec/block.rs` dispatch arm, emit projections.
- Excludes: the `LegacyCallV0` enum variant itself, the documented
  `llvmlite-compat` projection owner, and v0 wire spellings — each is
  an explicit compatibility owner pending its own retirement slice.

## Candidates

- A. `callsite_canonicalize` `LegacyCallV0{Global}` no-op arm —
     behaviorally identical to the `.. => 0` catch-all after S2/S3.
- B. `exec/block.rs` interpreter `LegacyCallV0` dispatch arm —
     dead only if every admitted module route now excludes legacy
     carriers (S2 selected-normal + S3 selected-dynamic evidence
     plus a census of any bypass route).
- C. published-view `func != INVALID` named-error arm —
     dead only if no admitted input can still carry it.
- D. `MirInstruction::LegacyCallV0` variant + `func` slot — still
     minted by the llvmlite projection and test fixtures; NOT
     caller-zero in this census.

## Evidence

(recorded during census)

## Six-line Decision

(recorded during census)

## Exit

- [ ] Bounded next slice selected, or `NoSafeSlice` with observable
      reopen trigger.
- [ ] Guard/pointer/workstream synced.
