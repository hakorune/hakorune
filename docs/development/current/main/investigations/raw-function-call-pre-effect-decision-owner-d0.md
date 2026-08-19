# Raw FunctionCall Pre-effect Decision Owner D0

Status: selected design stop  
Scope: all existing raw `FunctionCall` classification and target decisions  
Parent: `../workstreams/mirbuilder-inplace-replacement-current.md`
Row: `RAW-FUNCTION-CALL-PRE-EFFECT-DECISION-OWNER-D0`

## Current execution brief

Decision: Decide whether all existing raw `FunctionCall` decisions can be
owned once before argument lowering and Builder effect, without changing the
accepted call set or diagnostic precedence.
Source authority + canonical issuer: The exact source call occurrence, existing
special-call classifiers, canonical same-module callable catalog, and callable
header/result authorities are the inputs; this D0 must name one issuer before I0.
Non-authority: Name/arity alone, catalog candidates alone, Builder snapshots,
physical headers alone, Script transport, MIR, Dynamic/S6C evidence, C, and ASM.
Fail-fast boundary: Missing, ambiguous, conflicting, unsupported, or late-only
decisions stop before effect; the current raw route remains unchanged.
Smallest next slice: Census the full weak/extern/Brand/TypeOp/Math/FastMem/ordinary
precedence and target/recovery/header/tail chain, then select one BoxShape or
close `NoSafeSlice`. No implementation is authorized by this card.
Non-claims: No Script activation, new receipt/index/header/Recipe, diagnostic
change, raw caller retirement, production switch, fallback, or retry.

## Questions to close

1. What exact source-bound product owns the existing classifier precedence?
2. Can exact ordinary target and header be decided before arguments are lowered?
3. Which late `resolve_call_target`, bare-static recovery, header, and tail
   decisions become consumers rather than independent classifiers?
4. Can the move preserve every accepted/rejected shape and first diagnostic?
5. What caller-zero census proves that no second target decision remains?
6. Must the 790-line preflight owner be split behavior-neutrally before I0?

## Acceptance for a future implementation

- One pre-effect owner covers every current raw `FunctionCall` classifier arm.
- Exact source occurrence, ordered arguments, arity, target, header/result, and
  classifier precedence share one cohort where the route requires them.
- Later lowering consumes the decision and performs no target recovery, header
  search, tail resolution, or fallback classification.
- Current accepted shapes, effect order, and diagnostic precedence are unchanged;
  therefore the prerequisite implementation is BoxShape only.
- Focused positives cover every classifier arm and exact ordinary resolution;
  negatives cover ambiguous/missing target, wrong arity, foreign cohort, and
  missing/duplicate/conflicting decisions.
- Script activation remains a later, separate one-shape BoxCount.

## Stop condition

If one pre-effect issuer cannot own the complete precedence and exact target
without copying Builder state, guessing by name, or changing behavior, close
`NoSafeSlice`. Do not use a Script-only adapter to bypass this prerequisite.
