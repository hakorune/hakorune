# 3305 - MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PARITY-001

## Purpose
Prove that the RHS materialization intent snapshot preserves the RHS fields
from the symbolic compare-lowering command boundary.

## Contract
Input:

```text
CompareLoweringSymbolicCommandSnapshotV1
```

Output:

```text
CompareRhsMaterializationIntentSnapshotV1
```

The parity guard checks that these fields are preserved:

- RHS bound kind
- RHS bound i64
- RHS bound symbol id
- materialization intent kind
- all non-materializing claims

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
bash tools/checks/rust_lifecycle_mirbuilder_compare_rhs_materialization_intent_parity_gate.sh
```
