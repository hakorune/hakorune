---
Status: closed; exact two-call legacy-edge I0/R0 and reference closeout landed
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

## D0 completion evidence

Implementation landed in `8987682711` (`feat(if): admit two direct call arms
in recipe contract`) after the behavior-neutral extraction in `befb558d42`.
The mapper now pairs the ordered Then/Else fact slots with the ordered
profile rows exactly. The source verifier accepts claim lengths 4/5/6 and
rejects swapped, duplicate, wrong-arm, or implicit-baseline two-call claims.
The recipe operation remains identity-free and no physical owner changed.

Focused evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_value_profile -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_contract -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib direct_call -- --test-threads=1
cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
```

The compiler contract references were updated in
`src/mir/if_recipe_contract/README.md` and
`docs/development/current/main/design/joinir-if-recipe-contract-ssot.md` in
the same implementation row. No language grammar/reference surface changed.

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

### D1 completion evidence

The post-D0 census is unchanged and remains single-owner at every boundary:

| authority | production owner/caller | test-only evidence |
| --- | --- | --- |
| `VerifiedTrivialDirectCallV1::seal` | `resolved_value_profile/analyzer.rs:745` (1) | none |
| `claim_direct_call` | `trivial_ssa/lowerer.rs:377` (1) | ledger order/exact-once tests |
| direct-call capability install | `trivial_ssa/lowerer.rs:68` (1) | capability fixture tests |
| capability verification | `trivial_ssa/direct_call.rs:45` (1) | direct-call emission tests |
| ABI/target materialization | `trivial_ssa/direct_call.rs:49-54` (1) | direct-call tests |
| `trivial_ssa::direct_call::emit` | `trivial_ssa/lowerer.rs:383` (1) | direct-call tests |
| If physicalizer | `lowerer/if_materialization.rs:50` -> `if_recipe_physicalizer.rs:339` (1) | recipe physicalizer tests |

No second sealer, resolver, emitter, capability owner, physicalizer, route,
transaction, CFG, SSA, or PHI owner was introduced. The two D0 profile rows
remain consumed by the existing ordered exact-once ledger; D1 made no source
acceptance or production-caller change.

Read-only evidence was collected with:

```text
rg -n "VerifiedTrivialDirectCallV1::seal|claim_direct_call|direct_call::emit|physicalize_if_recipe_v1|verify_for_emission" src/mir
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
```

D1 closes without a code commit. The next row is D2 parity and candidate
abort/reuse evidence, still without a new owner or fallback route.

## D2 completion evidence

The D2 proof is implemented in
`src/mir/compiler/if_recipe_candidate_abort_d2_tests.rs` (388 lines, below the
800-line limit). The explicit-else fixture now calls distinct `left` and
`right` static helpers from the two assignment RHSs. The VM-gated parity test
proves:

* both sealed target symbols are present as exactly two MIR `Call`s;
* both results retain the existing Integer ABI receipt and the single
  function-level direct-call capability marker;
* the recipe emits one shared merge `Phi` whose two input predecessors equal
  the actual branch predecessors;
* the interpreter returns the left and right helper results for the two runtime
  conditions.

The existing DraftSeal-failure candidate tests now use the same explicit
two-call fixture. They prove that failure after call/CFG/PHI work leaves the
live Builder fingerprint and module state unchanged, and that a fresh compile
on the same compiler succeeds. The implicit one-call candidate proof remains
separate.

Evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib compiler::if_recipe_candidate_abort_d2_tests -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test --features vm-reference -q --lib compiler::if_recipe_candidate_abort_d2_tests -- --test-threads=1
```

D2 made no physicalizer, route, capability, CFG/SSA/PHI, or fallback-owner
change. The next and final implementation row is the exact two-call shape's
legacy-edge I0/R0 cutover.

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

## I0/R0 cutover evidence

The exact two-call facts now produce `CanonicalIfRecipePreflightV1::Selected`
before Builder effects, so this shape no longer traverses the
`NotThisShape -> legacy If` edge. The existing shape-scoped split in
`trivial_ssa/lowerer/if_materialization.rs` routes selected demand through the
named recipe physicalizer and retains `lower_if_legacy_unselected` only for
unselected shapes. The D2 explicit two-call parity test is the acceptance
witness for this selected route; the reusable logical-demand guard confirms
the selected physicalizer has zero legacy-helper references.

No new route, physicalizer, CFG/SSA/PHI, capability, transaction, fallback,
retry, or reselection owner was added. Selected failure remains terminal
`Freeze`; all other If shapes retain their existing legacy or rejected route.

## Closeout and next selection

D0/D1/D2/I0/R0 and the compiler-contract reference closeout are complete. The
task is intentionally not a license to add a third If shape. The next active
selection is the existing Loop M4 design/test-only row
`JOINIR-LOOP-GENERIC-POST-EFFECT-DEBT-CLASSIFICATION0-D0-S0`; it requires a
worker-backed design/caller census before taskization. Generic overlap remains
an honest `UnresolvedStop` until that row closes.

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
