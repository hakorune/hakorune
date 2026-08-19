# FunctionCall weak Parentheses Raw Reject Retirement D0

Status: selected design stop
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
