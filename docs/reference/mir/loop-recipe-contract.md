# Portable Loop Recipe Contract

Decision: accepted — `LOOP-RECIPE-PRODUCER-ID-S0` (building on
`JOINIR-LOOP-TRUE-REFERENCE-CLOSEOUT0-M7-S3-S3`).

Status: caller-zero logical reference. This page documents the portable
Recipe/JoinSig contract and the landed LoopTrue S2 producer; it does not
activate a production Loop route.

Primary design authority:
`docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md`

Executable authority:
`src/mir/loop_recipe_contract/`

Reference receipt — `LOOP-JOINSIG-MODULE-SPLIT-R0` (2026-08-06): the former
flat `join_sig.rs` is retired. `join_sig/mod.rs` remains the stable facade;
`join_sig/model.rs`, `join_sig/port.rs`, `join_sig/visibility.rs`, and
`join_sig/flow.rs` own the logical model, port projection, visible payloads,
and recursive dataflow/elaborator respectively. The existing
`join_sig_branch.rs` keeps direct branch-row helpers, while exit-edge
projection has one owner in `join_sig/port.rs`. This is a behavior-neutral
module split: the public-in-crate API and Recipe/JoinSig goldens are unchanged,
and no selector, physical lowering, or production caller is added.

Reference receipt — `LOOP-RECIPE-PRODUCER-ID-S0` (2026-08-06): portable
provenance now uses `producer_id: LoopRecipeProducerIdV1`; the old
`producer_route` wire key is rejected instead of being accepted as a V1 alias.
The current portable profiles use `direct_accum_v1`,
`loop_true_break_continue_v1`, and `nested_predicate_v1`; `generic_g0` is
reserved for the later canonical Generic producer. `LoopRouteId` remains a
legacy scheduler/policy/registry identity and is not imported by the portable
schema or producers. Test-only `LegacyRouteParityReceiptV1` records the three
profile mappings and marks legacy Generic V0/V1 as `legacy_only`. No selector,
registry, route-order, verifier dispatch, physicalizer dispatch, or production
caller changed.

Reference receipt — `LOOP-JOINSIG-NESTED-SHADOW-S0` (2026-08-06): visible
carrier projection now walks the verified Recipe parent chain from the target
loop toward the root, keeps the first `LoopBindingKeyV1` for each binding, and
emits one payload row per binding in binding-key order. The nearest recurrence
carrier therefore shadows an ancestor carrier; three or more nested duplicates
follow the same rule. Sibling carriers are isolated by ancestry. Unknown loop
owners and same-owner duplicate carriers remain `LoopRecipeVerifierV1`
rejects, while source owner/frame/`BindingRefV1` negatives remain deferred to
the source-bound core row. This is common JoinSig behavior with no Generic,
After, PHI, physical-ID, selector, schema, producer, or production-caller
change. Focused evidence is in `join_sig_nested_shadow_tests.rs`.

## Contract boundary

`LoopRecipeV1` is a Builder-free semantic wire. It owns canonical recipe-local
arenas for loops, blocks, items, values, carriers, and exits. It does not own
AST lookup, route choice, physical `BasicBlockId`/`ValueId`, MIR mutation,
runtime behavior, or backend lowering.

`LoopRecipeVerifierV1` checks the closed semantic shape. It cannot inspect
source ownership, select a route, retry a failed route, or mutate a Builder.
`LoopJoinSigElaboratorV1` consumes only `VerifiedLoopRecipeV1` and emits a
deterministic logical signature. The LoopTrue S2 producer is caller-zero: it
consumes the sealed policy demand, retains the policy receipt, verifies the
source-bound artifact, and returns the verified Recipe plus JoinSig without
touching a Builder.

## LoopTrue S2 envelope

The accepted `LoopTrueBreakContinue` producer emits exactly one `Always` loop
with three blocks (`body`, `then`, `else`), one I64 binding/carrier, four
values, six items, and two exits. Its body reads the binding, compares it with
the sealed branch bound using `Equal`, and publishes one explicit-else `If`:
the then arm exits with owner-targeted `Break`, while the else arm exits with
owner-targeted `Continue`. The resulting logical edge roles are
`Enter`, `BodyEntry`, `Break`, and `Continue`; there is no `Backedge` for this
shape. The producer preserves the policy frame receipt and uses the existing
Recipe verifier and JoinSig elaborator as the only downstream authorities.

Decision: accepted for caller-zero logical parity only. This is a source-bound
structural claim from the sealed projection; it is not AST re-inspection,
route activation, physical CFG/PHI construction, runtime execution, or
backend lowering.

## JoinSig products

The verified `LoopJoinSigV1` contains the existing logical loop rows plus
caller-zero `branches` rows:

```text
LoopJoinBranchV1
  owner_loop
  if_item
  condition
  then_exit: LoopJoinBranchExitV1
  else_exit: LoopJoinBranchExitV1

LoopJoinBranchExitV1
  exit_item
  role: Break | Continue
  target_loop
  payload
```

M7-S2-A admits exactly this branch shape inside an `Always` Loop:

```text
sole body item = explicit-else If
then block     = one direct Break targeting the owner Loop
else block     = one direct Continue targeting the owner Loop
```

The branch row is ordered by owner and If item. The Loop row receives the two
logical Body edges (`Break` to `After`, `Continue` to `Header`) and receives no
natural `Backedge` for this shape. Payloads are the already-visible logical
carrier rows; no hidden ownership operation is inserted.

### Visible carrier payloads

For a target loop, the logical visible payload snapshot is defined by the
Recipe ancestry alone:

```text
target -> parent -> ... -> root
  first carrier for each binding wins
  current binding value is projected
  output rows are sorted by binding key
```

The resulting vector contains no duplicate binding. A sibling's carrier is not
visible, and the JoinSig layer does not inspect source names or manufacture
physical `ValueId`/PHI identities. Structural owner errors are rejected before
the projection owner runs.

## Rejection boundary

The following remain typed rejects at this stage:

- implicit else or one-arm fallthrough;
- any branch binding write or divergent branch state;
- nested control inside either direct branch arm;
- Return or any non-owner exit in the branch pair;
- a branch block containing more than its one direct exit;
- calls, effects, physical CFG construction, PHI materialization, scheduler
  selection, retry, and legacy-route fallback.

`BranchMergeMismatch` is the logical rejection for a branch that is not the
accepted direct pair. Existing `UnreachableItem`, `UnsupportedExit`, and
carrier/value availability errors remain owned by their existing JoinSig
checks.

## Non-claims and next slice

This row claims only projection from the already sealed source shape into the
source-bound Recipe artifact. It does not claim fresh AST-to-Recipe discovery,
route activation, physical CFG/PHI parity, runtime execution, or deletion of
the located legacy Loop handoff. Binding merge and implicit-fallthrough
products require a separate design/implementation row after this logical
closure. The required README and reference updates are landed in this
closeout.
