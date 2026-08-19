# FunctionCall weak Parentheses Raw Reject Retirement R0

Status: landed
Parent: `function-call-weak-paren-raw-reject-retirement-d0.md`
Row: `FUNCTION-CALL-WEAK-PAREN-RAW-REJECT-RETIREMENT-R0`
Classification: BoxShape

## Execution brief

Decision: Delete the now-unreachable raw `WeakReject` classifier without adding
a replacement name check.
Source authority + canonical issuer: Registry and both parsers remain the sole
source authority; they emit unary Weak or reject parentheses before children.
Non-authority: Forged Rust AST/AST JSON, raw names, MIRBuilder priority, tests,
Program JSON, runtime weak values, and backend code do not issue source grammar.
Fail-fast boundary: Source `weak(...)` remains rejected before child parse;
non-source `FunctionCall { name: "weak" }` becomes ordinary trusted transport
with the existing arguments-before-resolution behavior.
Smallest next slice: Delete the enum variant, classifier arm, lowering arm, and
priority assertion; add source-unreachability and explicit transport tests.
Non-claims: No source acceptance, ingress validator, generic weak declaration,
unary/runtime behavior, Program JSON change, other special route, or backend
work.

## Acceptance

- Classic/TokenCursor and Canonical/Compat weak grammar guards remain green.
- No parser mode can emit a named weak FunctionCall.
- A deliberately forged raw FunctionCall follows Ordinary resolution and does
  not acquire a replacement weak-specific check.
- `WeakReject` has zero production/test occurrences after retirement.
- `Call { callee, arguments }` and Program JSON v0 remain unchanged.

## Landed receipt

The raw enum, name classifier, error/log arm, and priority assertion are gone.
A focused forged-AST test proves the ordinary path still lowers its child before
header lookup. The source grammar matrix, strict-on classic/TokenCursor guard,
and raw preflight focused tests are green.
