---
Status: Design stop accepted; D0 execution authorized
Date: 2026-08-04
Decision: evaluate one direct static i64 Call in the then-assignment RHS of
  an implicit-fallthrough If; reuse the existing implicit JoinSig and direct
  call capability without adding a route or physical owner
Exception: genuine next-shape design consultation after the completed D3
  explicit-else Call-RHS proof
ParentCurrentCard: docs/development/current/main/investigations/joinir-if-recipe-call-branch-d3-d2-execution-task-2026-08-04.md
Related:
  - joinir-if-recipe-call-branch-d3-design-stop-2026-08-04.md
  - joinir-if-recipe-shape-envelope-d0-design-stop-2026-08-04.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../design/recipe-first-entry-contract-ssot.md
---

# Implicit-fallthrough Call-RHS — design stop

## Decision boundary

The completed D3 row proves one direct static `i64` call in one branch
assignment RHS when the `If` has an explicit `else`. The next candidate is a
strictly smaller topology variant:

```text
resolved-trivial function
  one root-level If
  no explicit else
  then branch assigns one outer i64 binding
  then assignment RHS contains exactly one direct static i64 call
  implicit fallthrough preserves the entry binding value
  one post-merge read/return
```

This is a separate shape because the source claim order and baseline value
are different even though the physical CFG is already supported. The
candidate is not an invitation to accept arbitrary calls in an implicit
branch, to generalize call routing, or to reopen the old scheduler.

## Audit conclusion

The existing owner chain is sufficient:

```text
same-pass If facts
  -> IfRecipe operation/claim
  -> existing implicit-fallthrough JoinSig [header, then_exit]
  -> existing physical receipt
  -> existing direct-call capability and emitter
  -> canonical If physicalizer
```

No new PHI/SSA owner, route, transaction, rollback journal, capability
schema, or physicalizer is needed. The existing analyzer already observes an
implicit then-only branch and the lowerer already emits a direct call in a
then assignment RHS. The current gap is four explicit contract rejects:

1. `TrivialIfRecipeFactsDraftV1::finish` rejects a direct call when
   `explicit_else == false`.
2. `recipe_mapper` rejects the same implicit direct-call profile.
3. `IfRecipeVerifier` uses the stale
   `DirectStaticCallRequiresExplicitElse` reason.
4. source-claim verification treats the direct-call claim as an Else-only
   claim instead of the implicit Then-RHS claim order.

The fix must replace those rejects with the narrower rule below. Do not leave
the stale reason variant as a hidden compatibility authority.

## Accepted candidate contract (provisional until D0)

The implicit Call-RHS candidate is accepted only when all of these are true:

* exactly one root-level If is present;
* the If has no explicit else body;
* the then body has exactly one assignment to the selected outer binding;
* the assignment RHS contains exactly one direct static i64 call;
* the implicit baseline is the entry value of that same binding;
* the condition, branch result, and continuation read are already admitted by
  the resolved-trivial profile;
* the direct-call target, arity, argument rows, result class, and conservative
  effect come from the existing co-sealed direct-call capability;
* no call appears in the condition, continuation, nested control, or another
  branch position.

The portable source-claim order is fixed for this shape:

```text
[IfNode, Condition, ThenAssignment, ImplicitBaseline, DirectStaticCall]
```

The logical and physical JoinSig remain the existing implicit shape:

```text
predecessors = [header, then_exit]
baseline    = header binding value
then value  = direct-call result
```

The call is a verified leaf operation. The recipe does not carry raw AST,
callable headers, `ValueId`, `BasicBlockId`, runtime handles, or effect
authority.

## Task order

### `JOINIR-IF-RECIPE-CALL-BRANCH-IMPLICIT-D0`

Close facts → artifact → source claims → JoinSig correspondence without
production behavior changes.

Required evidence:

* implicit baseline and then-assignment fact are captured in the same pass;
* the five source claims above are emitted in deterministic order;
* direct-call target/result/effect correspondence is co-sealed with the
  existing direct-call profile;
* explicit-else, two-call, call-in-condition, call-in-continuation,
  unsupported result, and non-direct-call variants reject before Builder
  effects;
* the stale `DirectStaticCallRequiresExplicitElse` reason is removed or
  replaced by a precise branch/path mismatch reason.

### `JOINIR-IF-RECIPE-CALL-BRANCH-IMPLICIT-D1-CENSUS`

Reconfirm local caller ownership, without widening any production route:

* `VerifiedTrivialDirectCallV1` production sealer caller = 1;
* `trivial_ssa::direct_call::emit` production caller = 1;
* If recipe physicalizer production caller = 1;
* implicit Call-RHS adds no second emitter, resolver, capability, or PHI
  writer;
* raw, MethodCall, dynamic, unified, CorePlan/JoinIR, and JSON-v0 call paths
  remain non-selected columns.

### `JOINIR-IF-RECIPE-CALL-BRANCH-IMPLICIT-D2-PARITY-ABORT`

Add a production-shaped implicit fixture and reuse the existing candidate
abort seam:

* true branch executes the direct call and false branch observes the header
  baseline;
* MIR contains one direct call and one two-input PHI with `[header, then_exit]`
  predecessor/value correspondence from the sealed JoinSig;
* interpreter results and direct-call capability metadata match the source
  claims;
* a late failure after call/branch/PHI work drops the unpublished candidate,
  leaves the live Builder fingerprint unchanged, and permits fresh reuse;
* no `Option`, retry, fallback, route reselection, or second transaction is
  introduced.

## Explicit non-claims

This row does not activate:

* multiple calls, method/receiver calls, dynamic or generic calls;
* nested, effect, return, record, match, or short-circuit If shapes;
* mixed transfers, Home/ownership production, or property retirement;
* global PHI/SSA sole-writer status;
* raw/A+/CorePlan/JoinIR route retirement or JSON-v0 widening;
* a new compiler transaction or rollback mechanism.

## Stop conditions

Return to design before implementation if the candidate requires any of:

* a new CFG topology, JoinSig variant, PHI/SSA owner, route, or capability;
* raw/name lookup after the sealed facts boundary;
* more than one direct call or any unsupported call family;
* call failure mapped to `None`, retry, fallback, or route reselection;
* field/container/Home transfer or an implicit ownership rule;
* a touched source/test file over 800 lines.

The design stop is accepted for the bounded D0 row. D1/D2 remain gated on the
green D0 contract and rejection matrix; no broader Call or If shape is
authorized.
