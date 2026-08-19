# FunctionCall weak strict-12.7 Keyword Retirement D0

Status: selected design stop
Parent: `function-call-weak-paren-raw-reject-retirement-d0.md`
Row: `FUNCTION-CALL-WEAK-STRICT127-KEYWORD-RETIREMENT-D0`

## Question

May `NYASH_STRICT_12_7` stop downgrading `WEAK` to `IDENTIFIER`, making the
registry-owned weak grammar profile-invariant without changing unrelated
extended keywords or resurrecting a generic callable named `weak`?

Audit the environment flag's production callers, checked-in commands and
fixtures, tokenizer tests, declaration/member consequences, and both parser
paths. The next implementation must remove only WEAK from the downgrade cohort,
retain every other strict keyword policy, and prove `weak()` cannot produce a
FunctionCall before the raw Builder defense is retired.
