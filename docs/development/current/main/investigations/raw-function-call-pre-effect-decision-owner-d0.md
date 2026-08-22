# Raw FunctionCall Pre-effect Decision Owner D0

Status: closed `NoSafeSlice`
Scope: all existing raw `FunctionCall` classification and target decisions  
Parent: `../workstreams/mirbuilder-inplace-replacement-current.md`
Row: `RAW-FUNCTION-CALL-PRE-EFFECT-DECISION-OWNER-D0`

## Current execution brief

Decision: `NoSafeSlice`; moving every exact raw `FunctionCall` target before
argument lowering is not behavior-preserving under the current semantics.
Source authority + canonical issuer: Special-call route selection occurs before
arguments, while exact callee resolution currently occurs after argument
lowering; no source-semantic owner defines their binding/evaluation order.
Non-authority: Name/arity alone, catalog candidates alone, Builder snapshots,
physical headers alone, Script transport, MIR, Dynamic/S6C evidence, C, and ASM.
Fail-fast boundary: Missing, ambiguous, conflicting, unsupported, or late-only
decisions stop before effect; the current raw route remains unchanged.
Smallest next slice: `FUNCTION-CALL-CALLEE-BINDING-AND-EVALUATION-ORDER-D0`
defines the source-semantic callee binding point, argument order, target issuer,
and unknown-target diagnostic order. No implementation is authorized here.
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

## Closed counterexample

The raw ordinary route lowers arguments before calling `resolve_call_target`.
An argument such as a grouped assignment may update `variable_map`, so a shape
equivalent to `f((f = 1))` can resolve `f` from post-argument state today.
Moving target selection earlier would change the selected callee. Likewise, an
unknown or ambiguous target with a failing/effectful argument currently observes
the argument failure first. A pre-effect target reject would reorder diagnostics.

Therefore the requested move is not BoxShape. The existing raw route remains
unchanged, and Script `FunctionCall` stays typed `Deferred`.
