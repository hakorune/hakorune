---
Status: selected__fast__2026-09-15
Task: MIR-CALL-RETURN-NULL-VALUE-CLASSIFICATION-I0
Date: 2026-09-15
Priority: resolve the return-null vs return-value classification split in function completion
Parent: mir-call-parameter-contract-declared-box-i0-2026-09-15.md
NextCard: next named merged-route terminal after Dynamic/Completion
Implementation permission: true for the resolved_control_flow owner only
---

# Return-null value classification I0

## Six-line brief

```text
Decision: the merged-route lane stops at Dynamic/Completion/ReturnClassificationInvariant because classify_return_value maps `return null` to Void(ExplicitNull) while sibling exits return values; decide whether source `null` is a value-returning expression or a unit origin, and make the completion invariant match that one meaning.
Source authority + canonical issuer: classify_return_value and verify_explicit_return_set in src/mir/resolved_control_flow/function_control.rs own terminal-return classification; DeclaredFunctionResultContractV1 remains the declared-result authority.
Non-authority: `null` classification must not mint a MIR ValueId, a nullable-slot representation, or a result ABI; declared result annotations stay transport data only.
Fail-fast boundary: mixed `return <value>`/`return` (bare) and declared-result mismatches keep named rejections; no fallback classification and no per-site tolerance.
Smallest next slice: census the merged cohort's mixed null/value return sets, fix the single classification meaning, and re-run the merged route to the next named terminal.
Non-claims: Dynamic admission widening, PHI/JoinSig physicalization, production caller switch, VM parity, and legacy retirement.
```

`Census boundary: merged entry program -> return statements per function;
includes every `return`, `return null`, `return void`, and `return <expr>`
inside each resolved declaration; excludes loop bodies that never return and
nested box field initializers.`

## Entry contract

The previous card admitted declared `ArrayBox`/`MapBox` parameters as
`DeclaredHandle`, so the merged route now reaches the dynamic-admission
completion check. Batch slot 108 fails
`FunctionCompletionVerificationErrorV1::ReturnClassificationInvariant`: the
same function returns both a value (`return <expr>`) and `null`
(`return null`, classified `Void`/`ExplicitNull`). The representative cohort
is `variant_payload_type`-shaped bodies (`return null` on miss paths,
`return <expr>` on hit paths).

First census how many merged functions mix `return null`/`return` with
`return <expr>`; then decide the single meaning of source `null` in terminal
position — a value (nullable result) or a unit origin — and align
`classify_return_value`, `verify_explicit_return_set`, and
`verify_declared_return_value` so one meaning holds.

## Acceptance

Focused tests prove the chosen classification: mixed `return null`/`return
<expr>` accepted-or-rejected exactly once per the decided meaning, bare
`return` vs `return null` distinguished if the design keeps both, and
declared-result mismatches still named rejections. Run one quick-profile lib
test process with at most four build jobs, classify repository warning debt
as baseline, and update the owner README and pointer in the same closeout
slice. Route evidence: the merged entry must advance past
`Dynamic/Completion/ReturnClassificationInvariant` to the next named
terminal.
