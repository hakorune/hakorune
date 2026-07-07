# 3304 - MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PILOT-001

## Purpose
Publish a read-only RHS materialization intent snapshot from a symbolic compare
lowering command.

## Implementation
Added:

```text
lang/src/compiler/mirbuilder/compare_rhs_materialization_intent_snapshot.hako
```

The owner consumes:

```text
CompareLoweringSymbolicCommandSnapshotV1
```

and publishes:

```text
CompareRhsMaterializationIntentSnapshotV1
```

## Rows
- `command_literal_i64`
- `command_symbol_ref`

## Explicit Non-Claims
- RHS `ValueId` resolution: `0`
- RHS runtime materialization: `0`
- constant/helper emission: `0`
- MIR Compare emission: `0`
- MIR Branch emission: `0`
- BasicBlock mutation / `ValueId` allocation: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- ProgramJSON Var-rhs full dispatcher authority: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_rhs_materialization_intent_pilot_gate.sh
```
