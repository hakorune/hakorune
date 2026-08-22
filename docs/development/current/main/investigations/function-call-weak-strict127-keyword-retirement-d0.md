# FunctionCall weak strict-12.7 Keyword Retirement D0

Status: accepted
Parent: `function-call-weak-paren-raw-reject-retirement-d0.md`
Row: `FUNCTION-CALL-WEAK-STRICT127-KEYWORD-RETIREMENT-D0`

## Question

May `NYASH_STRICT_12_7` stop downgrading `WEAK` to `IDENTIFIER`, making the
registry-owned weak grammar profile-invariant without changing unrelated
extended keywords or resurrecting a generic callable named `weak`?

## Decision brief

Decision: Retire only the strict-12.7 downgrade of `WEAK`; this is an explicit
compatibility contraction, not behavior-neutral cleanup.
Source authority + canonical issuer: The language registry and corpus own
profile-invariant `weak expr` and rejection of `weak(...)`; the tokenizer is the
sole lexical issuer and must always emit `WEAK` for that spelling.
Non-authority: `NYASH_STRICT_12_7`, its legacy extended-keyword cohort, raw
`WeakReject`, parser tests, and archived phase documents cannot issue a generic
callable or identifier spelling named `weak`.
Fail-fast boundary: Both parser paths and both profiles reject immediate
`weak (` with the stable grammar tag before operand descent; no parser-issued
`FunctionCall { name: "weak" }` remains possible.
Smallest next slice: `FUNCTION-CALL-WEAK-STRICT127-KEYWORD-RETIREMENT-R0`
removes one cohort member, proves strict-on parser parity, and pins the seven
unrelated keywords plus shift gating in a reusable guard.
Non-claims: No change to the other strict-12.7 keywords, shifts, grammar
profiles, weak fields/runtime, MIR lowering, raw `WeakReject`, or generic-call
policy.

## Census

No checked-in command, CI job, fixture, test, or CLI mapping sets
`NYASH_STRICT_12_7`; it is an ambient compiler environment knob, default OFF.
Its exact downgrade cohort has eight tokens. Only `WEAK` conflicts with the
current profile-invariant registry rows; the remaining seven tokens and the
separate `<<`/`>>` gate remain untouched.

## Classification and order

This is a compatibility-retirement BoxCount because strict-on accepted source
changes: identifier uses of `weak` close and canonical unary weak reopens.
It must land before the later raw `WeakReject` BoxShape retirement.
