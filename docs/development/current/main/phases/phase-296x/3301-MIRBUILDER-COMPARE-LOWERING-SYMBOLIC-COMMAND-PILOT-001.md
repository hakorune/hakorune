# 3301 - MIRBUILDER-COMPARE-LOWERING-SYMBOLIC-COMMAND-PILOT-001

## Purpose
Publish a read-only symbolic compare-lowering command snapshot from the existing
BoolRecipe Compare lowering intent boundary.

## Implementation
Added:

```text
lang/src/compiler/mirbuilder/compare_lowering_symbolic_command_snapshot.hako
```

The owner consumes:

```text
BoolRecipeCompareLoweringIntentSnapshotV1
```

and publishes:

```text
CompareLoweringSymbolicCommandSnapshotV1
```

## Rows
- `intent_var_le_literal`
- `intent_var_lt_symbol`

Rows start from intent maps, not the full ProgramJSON phase-state route. The
full ProgramJSON dispatcher authority is still blocked by the known phase-state
AOT route boundary and remains unclaimed here.

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
bash tools/checks/rust_lifecycle_mirbuilder_compare_lowering_symbolic_command_pilot_gate.sh
```
