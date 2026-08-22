# FunctionCall weak strict-12.7 Keyword Retirement R0

Status: landed
Parent: `function-call-weak-strict127-keyword-retirement-d0.md`
Row: `FUNCTION-CALL-WEAK-STRICT127-KEYWORD-RETIREMENT-R0`
Classification: compatibility-retirement BoxCount

## Execution brief

Decision: Remove only `WEAK` from the strict-12.7 identifier downgrade cohort.
Source authority + canonical issuer: Registry/corpus own weak grammar and the
tokenizer always issues its `WEAK` token before either parser consumes it.
Non-authority: Ambient strict mode, raw Builder rejection, tests, and the other
legacy keyword policies cannot reinterpret weak as an identifier.
Fail-fast boundary: Strict ON/OFF and both parser implementations accept unary
weak and reject immediate parentheses with the stable tag before child parse.
Smallest next slice: One tokenizer arm deletion plus a focused reusable guard
that exercises the strict-on classic and TokenCursor profile witnesses.
Non-claims: No sibling keyword, shift, raw Builder, weak runtime, MIR, generic
call, backend, or production-route change.

## Acceptance

- Strict ON no longer produces `IDENTIFIER("weak")`.
- The two-profile grammar witness passes in classic and TokenCursor modes with
  strict ON.
- All seven sibling downgrade tokens and both strict shift gates remain.
- Raw `PreparedRawFunctionPreflightRouteV1::WeakReject` remains for the next
  ordered retirement row.

## Landed receipt

The tokenizer no longer downgrades `WEAK` under strict-12.7. The reusable guard
proves both strict-on parser paths retain the two-profile weak contract, all
seven sibling downgrades and both shift gates remain, and raw `WeakReject` is
still present for its separate retirement.
