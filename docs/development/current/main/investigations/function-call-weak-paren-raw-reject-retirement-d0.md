# FunctionCall weak Parentheses Raw Reject Retirement D0

Status: accepted with prerequisites
Parent: `function-call-special-namespace-source-registry-d0.md`
Row: `FUNCTION-CALL-WEAK-PAREN-RAW-REJECT-RETIREMENT-D0`

## Question

Can the obsolete raw `FunctionCall` name rejection for `weak(...)` be retired
without admitting a new source form, changing unary `weak expr`, or allowing a
generic callable named `weak` to bypass its real grammar authority?

The design audit must name the grammar/profile issuer, enumerate both parser
paths and active source callers, classify generic identifier-call shadowing,
and prove the rejection boundary occurs before child effects. No code, fixture,
fallback, or production change is authorized in this row.

## Final decision

Decision: Do not delete raw `WeakReject` yet. Close TokenCursor grammar parity,
then retire strict-12.7's accidental `WEAK -> IDENTIFIER` downgrade, and only
then remove the unreachable raw branch.
Source authority + canonical issuer: The existing `weak_unary_expr` and
`weak_paren_expr` registry rows plus EBNF own the language; each parser must
issue `UnaryOp(Weak)` or the stable parenthesized rejection before child parse.
Non-authority: Raw name priority, Builder state, ambient parser flags,
synthetic AST/JSON, tests, MIR, runtime weak handles, and backend code.
Fail-fast boundary: Both profiles and parser paths keep `weak` reserved and
reject `weak (` before operand/argument construction. No raw branch retires
while a source ingress can still issue `FunctionCall { name: "weak" }`.
Smallest next slice: `FUNCTION-CALL-WEAK-TOKENCURSOR-GRAMMAR-PARITY-I0` adds
the missing TokenCursor unary/reject projection and its four-profile/path tests.
Non-claims: No strict-mode retirement, raw branch deletion, weak runtime/field
change, generic callable named weak, other special calls, or backend work.

## Ordered bounded rows

1. `FUNCTION-CALL-WEAK-TOKENCURSOR-GRAMMAR-PARITY-I0` — BoxCount: the opt-in
   parser newly accepts the already-canonical `weak expr` shape.
2. `FUNCTION-CALL-WEAK-STRICT127-KEYWORD-RETIREMENT-R0` — retire the accidental
   noncanonical identifier downgrade; do not reinterpret it as a callable.
3. `FUNCTION-CALL-WEAK-PAREN-RAW-REJECT-RETIREMENT-R0` — BoxShape deletion of
   the now-unreachable duplicate Builder authority.

Combining these rows under a BoxShape label is forbidden: the first changes an
accepted parser shape, while the second retires a compatibility acceptance.
