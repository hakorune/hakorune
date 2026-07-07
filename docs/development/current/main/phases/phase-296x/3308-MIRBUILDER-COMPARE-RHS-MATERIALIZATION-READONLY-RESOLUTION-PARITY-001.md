# 3308 - MIRBUILDER-COMPARE-RHS-MATERIALIZATION-READONLY-RESOLUTION-PARITY-001

## Purpose
Prove that the read-only RHS `ValueId` resolution plan preserves the fields
from the RHS materialization intent boundary.

## Contract
Input:

```text
CompareRhsMaterializationIntentSnapshotV1
```

Output:

```text
CompareRhsValueIdResolutionPlanSnapshotV1
```

The parity guard checks that these fields are preserved:

- RHS materialization kind
- RHS bound kind
- RHS bound i64
- RHS bound symbol id
- read-only resolution plan kind
- all non-resolution claims

## Rows
- `intent_literal_i64`
- `intent_symbol_ref`

## Explicit Non-Claims
- RHS `ValueId` resolution: `0`
- literal constant `ValueId` allocation: `0`
- constant/helper emission: `0`
- LocalSSA `finalize_compare` execution: `0`
- MIR Compare emission: `0`
- MIR Branch emission: `0`
- BasicBlock mutation / `ValueId` allocation: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- ProgramJSON Var-rhs full dispatcher authority: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_rhs_materialization_readonly_resolution_parity_gate.sh
```
