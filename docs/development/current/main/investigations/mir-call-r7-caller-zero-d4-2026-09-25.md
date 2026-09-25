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

- A. `callsite_canonicalize` `LegacyCallV0{Some(Callee::Global)}` arm
     returns `0`; the immediately following `LegacyCallV0{..} => 0`
     arm catches the same rows — the Global arm is a literal no-op
     duplicate (src/mir/passes/callsite_canonicalize/pass.rs:64-71).
     Bounded delete, behavior-identical.
- B. `src/backend/mir_interpreter/exec/block.rs:168` routes
     `LegacyCallV0` to `reject_legacy_call`
     (src/backend/mir_interpreter/handlers/calls/mod.rs:11) — a named
     fail-fast stop, not a repair. The `--backend vm` lane hands the
     built module straight to `MirInterpreter`
     (src/runner/modes/common_util/vm_execution.rs:94-95) with no
     admission gate upstream, so this arm is the last residual-row
     boundary on that lane. KEEP.
- C. published-view `func != INVALID` arms produce named errors
     (`StaticCallUsesLegacyFunctionCarrier`,
     `FreeFunctionCallUsesLegacyFunctionCarrier`,
     `BuiltinPrintUsesLegacyFunctionCarrier`) — fail-fast boundaries
     for residual old-form carriers. KEEP.
- D. `MirInstruction::LegacyCallV0` + `func` slot are still minted by
     `project_module_to_legacy_calls` (llvmlite ExplicitCompatibility
     owner) and consumed by v0 compat emission + fixtures.
     NOT caller-zero.
- ~140 production files match `LegacyCallV0` in generic pass/analysis
     arms (DCE/CSE/value-uses/route plans) — shared structural
     readers required for match exhaustiveness while the variant
     exists. Out of retirement scope.
- Worker census (e08b4135, spot-checked): the interpreter arms
     (`exec/block.rs:168`, `handlers/mod.rs:180`,
     `reject_legacy_call`) are the SOLE deny boundary on every VM lane
     — `compile_normal` bypasses published-view admission and the
     verifier gate classifies `LegacyCallV0` as `Kept`, not
     `LoweredAway`. KEEP verdicts above confirmed.
- Flagged inconsistency (owner note, not this slice):
     `llvmlite_emit_obj_lib` mints `LegacyCallV0{ArrayBox.push/set/
     insert}` via `project_module_to_legacy_calls`, but its own egress
     (`emit_mir_json_for_harness` → MirJsonExport refresh →
     `reject_residual_calls`) rejects exactly those shapes post-S7.
     The llvmlite-compat lane is functionally dead on those inputs;
     its disposition belongs to the llvmlite feature retirement, not
     R7 cleanup.

## Six-line Decision (accepted 2026-09-25)

```text
Decision: delete the redundant callsite_canonicalize
          LegacyCallV0{Global} no-op arm — it is observably identical
          to the LegacyCallV0{..} catch-all that follows it.
Source authority + canonical issuer: typed Global calls are issued by
          production builders (R6-S1); residual legacy rows of any
          callee shape pass through untouched.
Non-authority: the Global no-op arm — dead structure with zero
          behavioral content.
Fail-fast boundary: interpreter reject_legacy_call, published-view
          named errors, and selected admission gates remain the
          residual-row boundaries; none are touched.
Smallest next slice:
          MIR-CALL-R7-CANONICALIZE-LEGACY-GLOBAL-NOOP-ARM-DELETE-S9 —
          pass.rs arm deletion + a residual Global passthrough pin.
Non-claims: does not remove the LegacyCallV0 catch-all arm, the
          interpreter/published-view boundaries, compat owners, or
          the variant itself.
```

## Exit

- [x] Bounded next slice selected —
      `MIR-CALL-R7-CANONICALIZE-LEGACY-GLOBAL-NOOP-ARM-DELETE-S9`.
- [x] Guard/pointer/workstream synced.
