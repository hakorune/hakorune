---
Status: Design stop — selected Call-valued If branch before implementation
Date: 2026-08-04
Decision: select one direct static i64 call as one branch assignment RHS;
  preserve the existing explicit-else topology and canonical SSA owner
Outcome: no implementation is authorized until the D0/D1/D2 evidence below is
  recorded; nested/effect/return/record/match/short-circuit shapes remain
  separate design rows
Related:
  - joinir-if-recipe-shape-envelope-d0-design-stop-2026-08-04.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../design/recipe-first-entry-contract-ssot.md
---

# Call-valued If branch — D3 design stop

## Selected shape

The next If shape is deliberately one leaf-operation extension, not a new
control topology:

```text
resolved-trivial function
  one root-level If
  explicit else
  fallthrough-only branches
  one outer BindingRef assigned in each branch
  one direct static i64 call in exactly one branch assignment RHS
  one post-merge read of that binding
  i64/Bool condition and homogeneous i64 merge value
```

The call is a value-producing direct static call whose target, arguments,
return representation, and conservative effect are already sealed by the
existing direct-call profile. The other branch remains a pure admitted
trivial expression. There is exactly one call operation in the selected
recipe. The first implementation slice is explicit-else only; a call-valued
implicit-fallthrough branch is a later shape, not a free extension.

## Why this is the smallest next boundary

The selected four-block topology, `IfJoinSigV1`, `CanonicalCfgSessionV1`,
`BindingSsaBuilderV1`, and `PhiTxn` remain unchanged. Only the value-operation
algebra and the source/effect correspondence gain one sealed leaf. This is
narrower than the other available families:

* nested If needs recursive control rows, multiple JoinSigs, and multiple
  physical receipts;
* a general effect shape needs multi-binding/effect-plan closure;
* return changes completion and terminal topology;
* record/match changes representation/dispatch and ownership products;
* short-circuit changes the condition CFG itself.

The Call-RHS shape therefore tests whether the portable recipe can carry an
already-owned operation contract without making the If physicalizer a call
resolver or a second effect authority.

## Authority map

### Source and logical authority

* same-pass `VerifiedTrivialIfRecipeFactsV1` owns the If site, branch writes,
  call expression site, continuation read, and exact source path;
* the existing sealed `VerifiedTrivialDirectCallV1`/direct-call profile owns
  target identity, argument rows, return representation, and conservative
  call effect;
* `IfRecipeArtifactV1` owns the portable leaf operation and source claim;
* `IfJoinSigV1` owns branch ports, merge obligation, and predecessor roles;
* the mapper/verifier pair is the only place that pairs these products before
  Builder effects.

### Physical authority

`CanonicalSsaFunctionSessionV2` remains the sole selected physical owner:

```text
CanonicalCfgSessionV1 + BindingSsaBuilderV1 + one PhiTxn
```

The existing `trivial_ssa::direct_call` emitter is the sole call emission
owner. The If physicalizer may pass a verified direct-call demand to that
owner, but may not resolve names, inspect raw call syntax, choose a route, or
emit a second call/SSA transaction.

### Explicitly non-authoritative paths

The following remain outside this shape:

* method/receiver calls, unified generic calls, dynamic calls, and raw call
  lowering;
* CorePlan/JoinIR, raw IfForm, located `IfCfgSessionV1`, and JSON-v0 bridges;
* nested/effect/return/record/match/short-circuit If shapes;
* implicit-fallthrough Call-RHS and more than one call operation;
* any global PHI/SSA sole-writer or call retirement claim.

## D0 — call-leaf correspondence and pre-effect contract

Design and test the contract without changing production behavior:

1. Admit exactly one direct static i64 call in one branch assignment RHS.
2. Require the call source claim to be owned by the same function and exact
   branch path as the If facts; no AST/name rescan may fill a missing claim.
3. Require the direct-call ABI/capability row to match the recipe operation's
   target, argument count/order, result class, and effect summary.
4. Require the call result to be the branch write value and the other branch
   to remain a pure admitted trivial value. The merge binding/class must match
   the call result and the other branch value.
5. Reject method calls, unresolved/dynamic targets, unsupported result classes,
   call-in-condition, call-in-continuation, multiple calls, nested control,
   and effect/return escape before any Builder effect.
6. Keep the fixed explicit-else JoinSig and physical receipt contract. A call
   failure after selection is terminal `Freeze`; it is never `None`, retry, or
   route fallback.

The D0 product is a sealed operation contract plus fail-fast tests. It does
not add a new SSA writer, call resolver, route, or production caller.

## D1 — caller and capability census

Record exact production and test callers for:

* `VerifiedTrivialDirectCallV1` production construction and its sealed ABI;
* `trivial_ssa/direct_call.rs` emission and its sole selected caller;
* the If recipe mapper, `IfJoinSigV1`, physical input, and physicalizer;
* raw/MethodCall/unified/JSON-v0 call paths as separate non-selected columns.

The census must distinguish production callers from fixtures and parity
helpers. Existing direct-call ownership is evidence to reuse, not permission
to widen the selected If route.

## D2 — parity and candidate-abort proof

Use one explicit-else fixture with the same outer binding and continuation
read as the current selected shape. Change only one branch assignment RHS to
the direct static i64 call.

The proof must cover, in order:

* same-pass facts, sealed call ABI, artifact, JoinSig, topology, receipt, and
  value classes;
* direct-call target/argument/result correspondence;
* MIR predecessor/value pairs, PHI count/inputs, Binding SSA continuation,
  interpreter result, and diagnostics parity with the existing direct-call
  oracle;
* a late selected-physicalization failure after call/branch/PHI work that
  drops the unpublished whole-compile candidate and leaves the live Builder
  fingerprint unchanged;
* a fresh compile on the same compiler succeeding after that failure.

Reuse the existing candidate-abort seam. Do not add a second transaction,
rollback journal, production fault toggle, or live Builder snapshot API.

## D3 execution boundary after design

Only after D0–D2 are accepted may implementation begin:

```text
same-pass Call fact
  -> verified If recipe Call leaf
  -> one-shot JoinSig/physical-input demand
  -> existing direct-call emitter + canonical If physicalizer
  -> typed physical receipt
```

The selected production physicalizer remains one caller. The old source-driven
If helper may remain for all unselected shapes, but the selected Call-RHS arm
must not invoke it for topology selection or call resolution.

## Acceptance and stop conditions

Done means the D0 contract, D1 census, and D2 paired parity/abort evidence are
written in this card; focused tests, shared guards, and line budgets are green.

Stop and reopen design if any of the following appears:

* a second call resolver or second direct-call emitter is needed;
* call ABI is inferred from raw syntax after selection;
* the recipe needs more than one call, a method/dynamic target, or a new
  control topology;
* the call result cannot be mapped to the exact PHI/JoinSig value row;
* call failure is converted to `None`, retry, fallback, or route reselection;
* the selected physicalizer needs a second CFG/SSA/PHI owner;
* any claim broadens to nested/effect/return/record/match/short-circuit or
  global PHI/SSA retirement;
* a touched Rust/test file would exceed 800 lines.

## Required evidence commands (future implementation)

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_contract -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_value_profile -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib direct_call -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_candidate_abort_d2_tests -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
```

These are future acceptance commands, not evidence that the Call-RHS shape
is implemented today.
