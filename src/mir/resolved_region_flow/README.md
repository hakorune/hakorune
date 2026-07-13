# Resolved Region Flow

This module owns lifetime-free, pre-Builder control-flow facts derived from
one owner-closed `ResolvedFunctionLoweringInputV1`.

```text
immutable located source + VerifiedResolvedFunctionV1
  -> construction-only flow draft
  -> coverage and semantic verification
  -> VerifiedResolvedFunctionFlowV1
```

The sealed function product stores statement-`If` rows explicitly in source
preorder. Each row owns its exact statement site, the S1 region bundle,
condition effects, typed fallthrough ports, join-source rows, and exact source
coverage. Nested rows may be analyzed bottom-up, but publication order is
never inferred from `BTreeMap` key order.

## Authority boundary

- `resolved_semantics` owns BindingRef, ScopeId, RegionId, and exact If
  topology.
- this module owns condition/branch effect summaries, fallthrough ports,
  join-source rows, and their source-coverage seal;
- `resolved_lowering` will own only ValueId/BasicBlock materialization from a
  sealed flow product.

The analyzer may depend on the compiler's immutable function input and located
source carriers as a leaf transport dependency. This module must not import
compiler capability/orchestration or any Builder, Planner, or JoinIR module.

## V1 limits

V1 describes fallthrough statement `If` only. It has no `falls_through` bool:
the port types prove the accepted route. Return, QMark, Break, Continue, Throw,
Try, Loop/CorePlan, If-expression results, and Lambda runtime flow remain
outside this product and must fail before Builder effects.

The product contains no borrowed syntax, names, Span, AST pointer identity,
ValueId, BasicBlockId, or mutable lowering state.
