# Callable single-loop source-to-Recipe map D0

Status: `Decision: design stop; implementation closed until mapping is sealed`

Parent: `GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-LEDGER-S1`

## Change

Close one row-by-row, AST-free correspondence from the resolver-owned callable
ledger to the common portable Recipe/JoinSig/effect products for the selected
single-loop profile. This is a design contract only. It must not add a
Generic-specific Recipe or physical owner.

## Contract

- Every admitted source site has one typed disposition: mapped, unsupported,
  opaque, or missing; duplicate and foreign sites reject before lowering.
- The mapping names condition read/operator/bound, body read, assignment
  target/value/step, initial carrier, effect relation, Loop membership/scope,
  After, and function tail/continuation. Derived Recipe glue is not a source
  site or AST rewrite.
- Resolver owns source identity and `LoopExecutionFrameKeyV1`; the compiler
  projector owns profile policy. `CanonicalSsaFunctionSessionV2` remains the
  sole physical owner, and DraftSeal remains the sole completion boundary.
- The selected callable single-loop profile remains distinct from nested
  Generic G0. No direct shape projection, retry, fallback, route selection,
  or production caller is allowed.

## Done

- A compact mapping table proves source site, Recipe item/key, effect row,
  carrier/merge, scope, After, and tail correspondence for the positive
  fixture.
- The common Recipe schema is shown to represent the exact recurrence and
  function completion, or the row is explicitly `NoSafeSlice`.
- Positive, missing, duplicate, foreign, opaque, and tail/scope counterexamples
  are named before implementation; one verifier input and one physical input
  owner are identified.
- The design and reference pointers are synchronized. Only after this row is
  accepted may a separate source-to-Recipe implementation row be opened.

## Stop

Return to a design review if any source operation, carrier, effect, scope,
After, or tail is inferred from a path suffix or an existing AST-bearing
Recipe. Do not add another transport adapter, deepen the task suffix, open a
physicalizer, switch production selection, or delete legacy callers while the
mapping is incomplete.
