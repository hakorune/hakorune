---
Status: Active
Date: 2026-05-28
Scope: correctness contract for nested call argument expression lowering.
Related:
  - docs/development/current/main/phases/phase-296x/296x-125-HAKO-MIMALLOC-POST-HAKO-REASON-BIND-SOURCE-MIR-REFRESH.md
---

# Nested Argument Single Evaluation SSOT

## Decision

Nested call arguments must be evaluated exactly once.

This is a correctness contract, not a performance-only optimization. A source
shape like:

```hako
return me.wrap(Side.tick())
```

must not lower to a MIR shape that semantically evaluates `Side.tick()` twice:

```text
Side.tick()
Side.tick()
me.wrap(second_result)
```

MIR may materialize receiver or argument values with `copy`, but it must not
re-lower the same argument AST into another semantic call.

## Boundaries

Do:

```text
source nested arg expression -> one MIR semantic call -> optional copies -> outer call
```

Do not:

```text
source nested arg expression -> two MIR semantic calls -> outer call uses only second result
```

## Owners

Likely MIR builder owners:

```text
src/mir/builder/stmts/return_stmt.rs
src/mir/builder/calls/build.rs
src/mir/builder/method_call_handlers.rs
src/mir/builder/calls/unified_emitter.rs
src/mir/builder/utils/boxcall_emit.rs
```

The first implementation row should inspect `MeCallPolicyBox::resolve_me_call`,
`try_inline_record_helper_call`, and `build_call_args` before touching lower-level
receiver/arg copy materialization.

## Phase Plan

```text
1. hako-alloc facade reason-call inventory guard
   - fixes the proven bad shape and keeps current allocator evidence visible

2. generic nested argument single-evaluation fixture
   - verifies return me.wrap(Side.tick()) lowers Side.tick/0 once

3. MIR builder correction
   - fixes duplicate semantic evaluation without adding generic CSE

4. static scalar method fact contract
   - later optimization only, separate from single-evaluation correctness
```

## hako_check Role

`hako_check` should not require call-site local binding as a long-term rule.
If used, it should validate helper method eligibility for future static-scalar
contracts, not encode source workarounds for this correctness issue.

## Static Scalar Separation

This SSOT does not authorize generic MIR CSE. Static scalar lowering is a later
optimization with a separate contract:

```text
verified return-literal-only static method -> Const lowering
unverified method -> keep call
```

Until that contract exists, default global call effects stay conservative.
