# Portable Loop Recipe Contract

Decision: accepted — `JOINIR-LOOP-TRUE-BRANCH-EXIT-CLOSURE0-M7-S2-A-S0`.

Status: caller-zero logical reference. This page documents the portable
Recipe/JoinSig contract; it does not activate a production Loop route.

Primary design authority:
`docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md`

Executable authority:
`src/mir/loop_recipe_contract/`

## Contract boundary

`LoopRecipeV1` is a Builder-free semantic wire. It owns canonical recipe-local
arenas for loops, blocks, items, values, carriers, and exits. It does not own
AST lookup, route choice, physical `BasicBlockId`/`ValueId`, MIR mutation,
runtime behavior, or backend lowering.

`LoopRecipeVerifierV1` checks the closed semantic shape. It cannot inspect
source ownership, select a route, retry a failed route, or mutate a Builder.
`LoopJoinSigElaboratorV1` consumes only `VerifiedLoopRecipeV1` and emits a
deterministic logical signature. No production caller is permitted in this
row.

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

This row does not claim source-to-Recipe projection, route activation, physical
CFG/PHI parity, runtime execution, or deletion of the located legacy Loop
handoff. Binding merge and implicit-fallthrough products require a separate
design/implementation row after this logical closure. Reference updates are a
required part of the implementation closeout, not a later undocumented task.

