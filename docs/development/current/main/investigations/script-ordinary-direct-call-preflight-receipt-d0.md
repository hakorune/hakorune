# Script Ordinary Direct Call Preflight Receipt D0

Status: selected design stop  
Scope: one ordinary same-module exact-I64 static `FunctionCall` shape  
Parent: `../workstreams/mirbuilder-inplace-replacement-current.md`

## Current execution brief

Decision: Decide only whether the existing raw ordinary-call preflight decision
can become the single affine authority consumed by Script admission.
Source authority + canonical issuer: One real Script `FunctionCall` occurrence
and the normal callable catalog own target/arguments; the existing
`PreparedRawFunctionPreflightV1` currently owns special-versus-ordinary call kind.
Non-authority: Stale OPEN labels, name lookup, catalog/header alone, Main-only
trivial-call receipts, Builder state, Dynamic/S6C observation, C/ASM, and perf.
Fail-fast boundary: Before Builder effect, co-seal exact target, ordered arguments,
arity, exact-I64 result/header, ordinary decision, and cohort; otherwise retain
the current typed Deferred/R4 path without fallback or a second classifier.
Smallest next slice: `SCRIPT-ORDINARY-DIRECT-CALL-PREFLIGHT-RECEIPT-D0` names
the receipt owner and one-way transfer seam only. No implementation is authorized.
Non-claims: No general FunctionCall, method/special call, weak/extern/Brand/TypeOp/
Math/FastMem, S6C production, Script sunset, production switch, fallback, or retry.

## Exact current edge

```text
source Script FunctionCall
  -> DirectPortAwareExpression
  -> traversal unsupported
  -> Deferred
  -> RawInvocationSourceTransportV1::script_root
  -> PreparedRawFunctionPreflightV1
  -> special / ordinary classification
  -> raw lowering
```

The design must not copy the raw decision into a second Script classifier. It
must either lend/move the existing decision through one source-bound lineage or
close `NoSafeSlice` and keep the current Deferred edge.

## Questions this D0 must close

1. Which exact source occurrence and callable-index row form the cohort?
2. Which existing preflight field is the sole ordinary-call decision?
3. Can that decision be issued before Script admission without moving Builder
   effects or searching again by name?
4. Which owner holds ordered arguments, exact target, arity, result/header, and
   special-call exclusion?
5. What one constructor prevents foreign/missing/duplicate/conflicting receipts?
6. What exact raw caller becomes zero in the future one-shape I0?

## Acceptance for a future I0

- Admit exactly one ordinary same-module exact-I64 static call as a BoxCount.
- One receipt lineage owns exact target, ordered arguments, arity, result/header,
  and the ordinary decision before effect.
- Unknown/ambiguous targets, wrong arity, foreign cohort, unsupported ABI, and
  weak/extern/Brand/TypeOp/Math/FastMem calls reject before receipt issuance.
- Missing, duplicate, conflicting, or incomplete receipts reject before effect.
- The admitted shape deletes its selected
  `Deferred -> RawInvocationSourceTransportV1::script_root` edge in the same
  commit; no raw fallback remains for that shape.
- All other Deferred families and the R4 ratchet remain unchanged.

## Stop condition

If the raw preflight decision cannot be transported without reclassification,
name lookup, a second header authority, or Builder-derived inference, close this
D0 as `NoSafeSlice`. Do not create a proof-only receipt or widen Script admission.
