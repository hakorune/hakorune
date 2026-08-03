# JOINIR-LOOP-NESTED-PREDICATE-CLOSURE0-D2-C-SOURCE-PROJECTION

Status: design audit complete; bounded caller-zero implementation authorized.
Date: 2026-08-04

## Objective

Consume the resolver-issued source forest for the real two-predicate
`NestedLoopMinimal` shape exactly once and issue a sealed, AST-free source
projection. This is source evidence only. It does not produce a Recipe,
JoinSig, PHI/SSA plan, MIR, or production route demand.

## Hard boundary

`VerifiedResolvedFunctionV1` proves source sites, owner, regions, and binding
identity, but it does not carry `<`, `+`, integer literals, or ordered body
operations. Therefore D2-C has two explicit stages:

```text
one bounded FunctionSourceViewV1 observation
  -> resolver-owned BindingRef/site evidence
  -> consuming forest adapter
  -> VerifiedNestedLoopSourceProjectionV1
```

The source-view seam is the only syntax observer. The projection is
non-`Clone`, contains no AST/Located nodes or hand-built source paths, and
uses forest member indices rather than Recipe keys. If a fully AST-free
implementation is required without this one observation seam, stop and open a
larger resolver `ResolvedExprShape` design; do not read legacy
`NestedLoopMinimalFacts` from the projector.

## Exact positive shape

The accepted source is the existing fixture grammar:

```text
root:    i < literal; local j; j = 0; child loop; i = i + 1
child:   j < literal; sum = sum + 1; j = j + 1
```

The resolver forest must contain exactly two members in preorder:

```text
root  parent = None
child parent = Some(root)
```

The projection records exact resolver-issued sites, predicate/assignment
operation enums, literal facts, ordered body schedule, and binding evidence:

- `i`: function lexical owner, root recurrence evidence, parent-visible;
- `sum`: function lexical owner, root recurrence evidence, child write,
  parent-visible;
- `j`: outer loop-body lexical owner, child recurrence evidence,
  child-local after normal child resume.

The lexical owner of `j` must never be used as its recurrence or resume owner.
Final Recipe carrier ownership and resume visibility are validated by D2-D;
D2-C only emits source-observed evidence.

## Required output and ownership

Issue one non-`Clone` product equivalent to:

```text
VerifiedNestedLoopSourceProjectionV1 {
    source_forest_binding: VerifiedLoopSourceForestBindingV1,
    source_shape: VerifiedNestedLoopSourceShapeV1,
    root_frame_key: LoopExecutionFrameKeyV1,
}
```

The forest is consumed once. D2-C must not consume it to build a D1
projection and then recreate or reissue it for shape observation. Preserve
the existing portable source binding inside the returned product. The shape
must not import or own Recipe/JoinSig/PHI/SSA/Builder/CorePlan/PlanLowerer,
route/Retry, `ValueId`, `BasicBlockId`, or physical identities.

## Typed reject boundary

Keep a small fail-closed vocabulary for:

- foreign owner/frame, missing forest, wrong cardinality, parent/index/site
  mismatch, sibling/deeper/skip shape;
- non-`<` predicate, non-literal bound, unsupported operation/exit/If or body
  order;
- missing, upvar, field/index, or mismatched assignment target/binding;
- `j` lexical-versus-recurrence mismatch, ancestor `sum` ownership mismatch,
  and post-child `j` escape;
- unsupported scope lineage.

No `Option`/Retry projection is permitted. Production caller count remains
zero; D2-D is the first producer/Recipe consumer.

## Acceptance gates

1. The real source fixture produces deterministic two-member forest and the
   exact root/child body schedule without hand-built paths.
2. Binding evidence fixes the `i`/`sum`/`j` lexical-versus-recurrence
   distinction and rejects the counterexamples above.
3. The returned product retains D1's portable source binding and consumes the
   forest exactly once.
4. A source-module import/grep guard keeps Recipe, JoinSig, PHI/SSA, Builder,
   route, Retry, and physical identity out of the projection.
5. Focused projector tests, `cargo check --lib`, current-state and in-place
   guards, `git diff --check`, and per-file `<800` lines are green.

## Next order

```text
D2-C source projection (caller-zero)
  -> D2-D NestedLoopMinimal Recipe producer (one consumer)
  -> D3 canonical CFG / BindingSSA / PhiTxn physical pilot
```

PHI/SSA ownership remains the existing
`CanonicalCfgSessionV1 -> BindingSsaBuilderV1 -> PhiTxn` chain. D2-C must not
add or move a PHI/SSA writer.
