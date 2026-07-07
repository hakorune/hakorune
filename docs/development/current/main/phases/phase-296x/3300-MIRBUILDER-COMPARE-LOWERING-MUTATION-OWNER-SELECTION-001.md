# 3300 - MIRBUILDER-COMPARE-LOWERING-MUTATION-OWNER-SELECTION-001

## Purpose
Select the first compare-lowering owner after the shared If/Loop compare and
Var-rhs producer surface is closed.

## Decision
Select:

```text
MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PILOT-001
```

The selected pilot must publish a read-only symbolic command snapshot from the
existing `BoolRecipeCompareLoweringIntentSnapshotV1`.

It must not emit MIR or mutate compiler state.

## Boundary
Allowed input:

```text
BoolRecipeCompareLoweringIntentSnapshotV1
```

Allowed output:

```text
CompareLoweringSymbolicCommandSnapshotV1
```

The symbolic command may name:

- lhs symbol id
- MIR compare op code
- rhs bound kind / value / symbol id
- destination allocation policy
- branch target policy

## Explicit Non-Claims
- BoolRecipe lowering executed: `0`
- operand `ValueId` resolution: `0`
- rhs runtime materialization: `0`
- MIR Compare emission: `0`
- MIR Branch emission: `0`
- BasicBlock mutation / `ValueId` allocation: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_lowering_mutation_owner_selection_guard.sh
```
