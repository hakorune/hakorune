---
Status: selected__Fast__R7TypedDiagnostics__2026-09-12
Task: MIR-CALL-COMPATIBILITY-RETIRE-R7-TYPED-DIAGNOSTICS-I0
Date: 2026-09-12
Parent: mir-call-compatibility-retire-r7-typed-diagnostics-d0-2026-09-12.md
---

# R7 typed diagnostic transport

## Authority and bounded change

Existing stage owners remain the source of reject meaning. A transport-only
`CanonicalResolvedCutoverStageErrorV1` retains each owned inner error, and
`CanonicalResolvedCutoverFailureV1` adds only the direct/nested family tag
before the existing `CanonicalLoweringErrorV1` boundary. The two cutover
helpers are the sole conversion points. `FunctionDraftSeal` pending,
`backend_codegen_request_defaults`, and all unrelated diagnostics remain
outside this slice.

No semantic `Verified*`/`Prepared*` receipt, fallback, string parser, broad
`From` implementation, or new production route is allowed.

## Required implementation

- add the two transport enums in `src/mir/compiler/lowering_input.rs` with
  the existing typed owner payloads;
- replace Debug-string conversion in both `bridge_error` helpers and their
  22 callers with explicit stage constructors;
- preserve preflight unopened-builder behavior and late candidate discard;
- keep `resolved_lowering/mod.rs` unchanged unless compilation proves a
  narrow visibility fix is unavoidable;
- keep every changed source file below 760 lines and the hard stop below 800.

## Focused acceptance

- direct and nested positive paths compile through the existing production
  cutovers;
- direct and nested preflight failures match family + stage + original
  inner error type;
- direct and nested late candidate failures match family + stage and retain
  the typed inner error before candidate discard;
- no `format!("{error:?}")` remains in either cutover bridge;
- pending draft-seal behavior is unchanged and not counted as closed;
- existing typed diagnostics, pointer guard, `git diff --check`, and source
  size checks pass.

## Non-claims and follow-up

This card does not close the pending `FunctionDraftSealErrorV1` bridge or
whole R7. The next design slice must decide the owned pending error boundary
and its session-restoration tests.
