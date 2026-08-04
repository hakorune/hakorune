---
Status: Selected D0 task — semantic admission only
Date: 2026-08-04
Parent: joinir-if-recipe-shape-envelope-d0-design-stop-2026-08-04
Decision: admit exactly one direct static i64 call in each explicit-else
  branch assignment RHS, while reusing the existing direct-call profile,
  emitter, selected If physicalizer, and canonical CFG/Binding-SSA/PhiTxn
  owner
---

# Two direct Call-RHS arms — D0/D1/D2 execution task

## Selected shape

```text
one resolved-trivial function
one root If with explicit else
one shared i64 merge binding
one assignment in each branch
one direct static i64 call as the RHS of each assignment
one post-merge read of the shared binding
```

The condition remains the existing admitted InlineBool expression. Calls are
root assignment values only. Calls in the condition, call arguments,
continuation, nested control, method/dynamic/unified targets, implicit
fallthrough, or any third call remain rejected before Builder effects.

## Authority map

```text
same-pass facts
  -> exact Then/Else call sites
  -> existing VerifiedTrivialDirectCallV1 rows
  -> identity-free IfOperationV1::DirectStaticCall values
  -> existing IfJoinSig / physical input
  -> existing selected If physicalizer
  -> existing trivial_ssa::direct_call::emit
  -> existing CanonicalSsaFunctionSessionV2
```

The direct-call sealer remains the sole producer (`analyzer.rs`), the
`claim_direct_call` ledger remains the exact-once source-order consumer, and
`trivial_ssa::direct_call::emit` remains the sole call emitter. The two-call
artifact carries no callable header, target name, argument ABI, `ValueId`,
`BasicBlockId`, or runtime handle. `SourceExprSiteV1` is the only pairing key.

## D0 — facts and portable source contract

Change only the pre-Builder facts/recipe contract and focused tests.

Required contract:

1. Facts retain one direct-call site per arm (`Then`, `Else`) instead of one
   global `Option`; duplicate calls in an arm and calls outside an assignment
   RHS are typed rejection.
2. The explicit-else two-call shape is admitted only when both arms have one
   assignment, both assignment values are direct calls, and both calls are
   InlineI64 rows owned by the same function.
3. The source claim order is fixed and verified as:

   ```text
   IfNode, Condition, ThenAssignment, ElseAssignment,
   DirectStaticCall(then path), DirectStaticCall(else path)
   ```

   The role remains `DirectStaticCall`; branch identity is carried by the
   exact path and fixed claim position. Do not add a second identity system or
   name-based matching.
4. The mapper requires the profile direct-call rows to be exactly the two
   fact sites as a set, while preserving Then-before-Else source order. The
   recipe operation remains identity-free and emits one call operation in each
   branch.
5. Existing no-call and one-call explicit/implicit shapes remain unchanged;
   implicit two-call, one-arm two-call, and three-call inputs remain rejected.

No production physicalizer, adapter demand, route, CFG/SSA/PHI, capability,
transaction, fallback, retry, or runtime code may change in D0.

## 800-line structure rule

`recipe_mapper.rs` is already 736 lines. Before adding D0 logic, move its
behavior-neutral source-path helpers (`root_body_index`, claim path builders,
and `verify_entry_definition`) into one private
`resolved_value_profile/recipe_source_paths.rs` module. The extraction must
preserve diagnostics and mapper behavior and leave both files below 800 lines.
Do not grow `analyzer.rs` (768 lines), `if_recipe_adapter.rs` (736 lines), or
the canonical lowerer for this semantic row.

## D0 focused tests

Add or update tests for:

* facts retain exactly the Then and Else call sites;
* mapper emits two identity-free call operations and six ordered source claims;
* Then/Else source-path swap rejects before physicalization;
* one-arm duplicate, third call, implicit two-call, condition call,
  continuation call, and nested/argument call remain typed rejection;
* existing one-call and no-call recipe tests remain green.

Required D0 evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_value_profile -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_contract -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib direct_call -- --test-threads=1
cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
```

Also verify the production caller ledger remains unchanged:

```text
VerifiedTrivialDirectCallV1::seal = 1 production caller
trivial_ssa::direct_call::emit = 1 production caller
If recipe physicalizer = 1 production caller
new route/transaction/PHI/SSA owner = 0
```

## D1 — caller and capability census

Record the exact production/test caller split after D0. Reconfirm that the
two rows are consumed by the existing exact-once ledger and that capability
installation, ABI verification, call emission, and If physicalization each
still have one owner. D1 changes no source acceptance and adds no production
caller.

## D2 — parity and candidate-abort proof

Only after D0 and D1 close, use the existing selected If physicalizer and
direct-call emitter to prove:

* both runtime branches call their exact target;
* MIR has one fixed explicit-else join and one PHI with the actual two
  predecessor/value pairs;
* the two source claims, two sealed call rows, JoinSig value rows, and
  interpreter result correspond exactly;
* a late draft-seal failure after Call/CFG/PHI work leaves the live Builder
  unchanged and a fresh compile on the same compiler succeeds.

Reuse the existing unpublished candidate boundary. Do not add a second call
resolver/emitter, rollback journal, fault environment variable, production
snapshot API, or physicalizer transaction.

## I0/R0 cutover boundary

After D0/D1/D2, remove only the exact two-call shape's
`NotThisShape -> legacy If` edge. The selected owner and physical topology do
not change. All other If shapes keep their existing legacy or rejected route.
Selected failure is terminal `Freeze`; fallback, retry, and route reselection
remain zero.

## Stop conditions

Return to the parent design stop if any step requires:

* a second direct-call sealer, resolver, emitter, capability owner, or
  physicalizer;
* raw AST/name lookup after the facts/profile boundary;
* a call key or ABI copied into the portable artifact;
* implicit/nested/effect/return/record/match/short-circuit support;
* a second CFG/SSA/PHI owner, `Option`, retry, fallback, or route reselection;
* any touched Rust/test file reaching 800 lines.

## Reference closeout

After the implementation row lands, update the applicable compiler contract,
IfRecipe schema/registry reference, status index, and any language/reference
document that describes the accepted Call-RHS surface before marking the row
closed. D0 itself changes the portable compiler contract; it does not change
surface syntax or the language reference grammar.
