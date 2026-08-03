# JOINIR-LOOP-NESTED-PREDICATE-CLOSURE0-D2-C1A-ROOT-INITIALIZERS

Status: design stop opened by the D2-D producer audit.
Date: 2026-08-03

## Why this stop exists

The D2-C source projection already seals the root/child predicates, ordered
body schedule, BindingRef identity, lexical scope, and recurrence evidence.
It does not yet retain the root `i`/`sum` initializer facts. The real source
initializes both to `0`, and the portable Recipe producer needs those facts to
issue root carrier entry values. D2-D must not reread syntax, consult legacy
facts, or fabricate constants.

## Authorized slice

Extend `VerifiedNestedLoopSourceShapeV1` with two resolver/site-bound root
initializer evidences:

```text
root local declaration statement
  -> LocalInitializer(0/1) source site
  -> BindingRef and exact i64 literal
```

The observer must use the existing `FunctionSourceViewV1` local-initializer
role and the resolver's declaration/BindingRef map. It accepts exactly the
real `i = 0; sum = 0` shape, keeps the initializer sites in the source DTO,
and rejects missing, non-integer, reordered, or mismatched initializers with
typed failure. No Recipe key/value key is issued here.

## Non-goals and owner boundary

- Do not change Recipe schema, JoinSig, PHI/SSA, CFG, Builder, route, Retry,
  or physical identity.
- Do not re-consume/reissue the source forest; the existing D2-C projection
  remains the single forest consumer.
- D2-D will consume the initializer evidence and validate final root carrier
  ownership. D2-C1a must not mint carrier owners or resume visibility.
- Keep the lexical-vs-recurrence distinction: root `i`/`sum` are
  function-owned lexical-block bindings; child `j` remains outer-loop-body
  lexical but child recurrence-owned.

## Acceptance gates

1. Positive nested fixture exposes both initializer sites, BindingRefs, and
   exact values `0` without an AST in the returned product.
2. Typed negatives cover non-integer/missing/reordered initializer evidence
   and initializer binding mismatch.
3. Existing D2-C source projection, D2-B JoinSig, Recipe, PHI/SSA tests remain
   green; all touched files stay below 800 lines.
4. Existing caller-zero source projection guard remains green and no D2-D
   production producer is wired.

After this card closes, reopen the D2-D producer design with root initializer
evidence as a required input. If the source view cannot provide the exact
initializer role without a new resolver schema, stop and open a separate
resolver data-sufficiency design instead of widening D2-C by-name.
