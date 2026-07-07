# 3302 - MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PARITY-001

## Purpose
Prove that the symbolic compare-lowering command preserves the fields from the
BoolRecipe Compare lowering intent boundary.

## Contract
Input:

```text
BoolRecipeCompareLoweringIntentSnapshotV1
```

Output:

```text
CompareLoweringSymbolicCommandSnapshotV1
```

The parity guard checks that these fields are preserved:

- lhs symbol id
- MIR compare op code
- rhs bound kind
- rhs bound i64
- rhs bound symbol id
- all non-mutating claims

## Rows
- `intent_var_le_literal`
- `intent_var_lt_symbol`

## Explicit Non-Claims
- BoolRecipe lowering executed: `0`
- operand `ValueId` resolution: `0`
- rhs runtime materialization: `0`
- MIR Compare emission: `0`
- MIR Branch emission: `0`
- BasicBlock mutation / `ValueId` allocation: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- ProgramJSON Var-rhs full dispatcher authority: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_lowering_symbolic_command_parity_gate.sh
```
