# JOINIR-LOOP-NESTED-PREDICATE-CLOSURE0-D2-B

Status: implementation task authorized after D2-A design audit.
Date: 2026-08-03

## Objective

Extend the caller-zero `LoopJoinSigElaboratorV1` just enough to accept one
bounded, source-free two-predicate nested recipe matching the real
`NestedLoopMinimal` grammar.  Keep the portable Recipe schema and physical
PHI/SSA owners unchanged.

## Exact semantic slice

```text
root L0: Predicate(i < literal), carriers i and sum
  body: child Loop -> i = i + 1 -> natural root Backedge

child L1: Predicate(j < literal), recurrence carrier j
  body: sum = sum + 1; j = j + 1; natural child Backedge
```

The child condition block is pure `ReadBinding`/`ConstI64`/`CompareI64`.
There is no `If`, explicit `Break`/`Continue`/`Return`, deeper child, or
post-child use of `j`.  `j`'s recurrence owner is L1 even though its source
declaration is in the outer LoopBody lexical scope; the distinction must stay
explicit.

## Required implementation boundaries

1. Remove the blanket nested-Predicate rejection only for the bounded pure
   condition/body shape above.  Keep `UnsupportedNestedPredicate` as the typed
   stop for all other nested-Predicate shapes.
2. Snapshot parent-visible bindings/available values before entering a child.
   When the child completes normally, project only those inherited bindings
   back to the parent, re-exposing updated ancestor values such as `sum` and
   dropping child-local `j` and temporary values.  A parent tail read of `j`
   must fail typed, not leak child state.
3. Keep existing edge roles and payload ordering.  Child edges are
   `Enter`, `PredicateTrue`, `PredicateFalse`, `Backedge`; root edges are
   `Enter`, `PredicateTrue`, `PredicateFalse`, `Backedge`.  Predicate edge
   payloads describe header-entry visibility; Backedge payloads carry final
   loop-carrier values.  Child false-path normal resume is represented by the
   recursive item order and parent Backedge; an explicit physical resume path
   belongs to a later M6-B/P1 design stop.
4. Do not add a Recipe field, JoinSig edge role, `ancestor_updates` side table,
   PHI/SSA writer, route, Retry, Builder, CorePlan, PlanLowerer, or physical
   identity in this task.

## Tests and guards

- Add a source-free `nested_predicate_v1` golden/fixture and assert verifier
  success, deterministic two-row JoinSig, exact edge roles, child-local scope
  projection, updated ancestor `sum`, and no root `j` payload.
- Add typed negatives for missing child carrier, unavailable predicate value,
  missing ancestor carrier, unreachable continuation, explicit child exit,
  branch/If shape, deeper nested predicate, and parent-tail `j` use.
- Preserve all nested-Always golden/materializer tests unchanged.
- Keep producer/route/Retry/PHI/SSA/physicalizer caller counts at zero; extend
  the existing compile-candidate scope guard if a new test-only anchor is
  introduced.
- Run focused JoinSig/Recipe tests, `cargo check --lib`, current-state and
  in-place guards, `git diff --check`, and keep every touched file below 800
  lines.

## Stop conditions

Stop and reopen design if the existing Recipe/JoinSig fields cannot express
parent-visible carrier projection without a new semantic field, or if the
physical map requires guessing a child-After→parent-tail edge.  Do not solve
either by leaking Flow state or by inventing a second PHI authority.

After this card is green, D2-C may design the resolver-owned AST-free source
projector.  The independent legacy V0 nested-carrier policy fix remains a
separate lane and is not evidence that this caller-zero path is production.
