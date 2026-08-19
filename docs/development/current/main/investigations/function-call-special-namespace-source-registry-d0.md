# FunctionCall Special Namespace Source Registry D0

Status: selected design stop
Scope: source/context-owned identity for existing explicit special call forms
Parent: `function-call-lexical-callee-classification-d0.md`
Row: `FUNCTION-CALL-SPECIAL-NAMESPACE-SOURCE-REGISTRY-D0`

## Current execution brief

Decision: Design one exhaustive source/context-owned registry for the currently
accepted explicit special call forms before Script FreeStatic classification.
Source authority + canonical issuer: Grammar/profile decisions plus the exact
resolver source context must issue special identity once; later preflight may
consume that identity but may not rediscover it from a name.
Non-authority: AST `FunctionCall`, name/arity alone, raw branch order, Builder
Brand/FastMem state, `variable_map`, callable catalog misses, MIR, tests, C, ASM.
Fail-fast boundary: Every accepted weak/externcall/TypeOp/Math/Brand/FastMem/str
shape must be classified or explicitly parked from one inventory; overlap,
missing context, or unsupported shape rejects before arguments and Builder effect.
Smallest next slice: Census each current raw special arm, its actual source/context
dependency and precedence, then decide whether a behavior-neutral registry
BoxShape exists; otherwise close NoSafeSlice and name the missing source issuer.
Non-claims: No lexical callable capability, FreeStatic activation, new accepted
special syntax, parser rewrite, Builder edge retirement, fallback, or retry.

## Questions to close

1. Which arms are true source syntax and which are library/runtime call names?
2. Which arms require Brand/FastMem/type/environment context unavailable to resolver?
3. Is the current precedence intentional language meaning or implementation order?
4. Can every existing accepted special site be represented without a Builder probe?
5. Which arms must be parked instead of forced into one registry?

## Future I0 acceptance

- One registry/inventory owns every selected existing special identity.
- Its issuer consumes exact source/profile/context, never Builder success.
- Precedence and overlap are explicit and exhaustively negative-tested.
- Arguments remain untouched until identity issuance succeeds.
- Raw string classifier callsites for the selected cohort become caller-zero.
- FreeStatic/catalog misses never fall through into or out of special handling.
