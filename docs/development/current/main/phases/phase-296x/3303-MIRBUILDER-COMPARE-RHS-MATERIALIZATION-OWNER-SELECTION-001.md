# 3303 - MIRBUILDER-COMPARE-RHS-MATERIALIZATION-OWNER-SELECTION-001

## Purpose
Select the first RHS materialization owner after symbolic compare command parity.

## Decision
Select:

```text
MIRBUILDER-COMPARE-RHS-MATERIALIZATION-INTENT-PILOT-001
```

The selected pilot must publish a read-only RHS materialization intent snapshot
from the symbolic compare-lowering command.

It must not resolve symbols to `ValueId`, emit constants/helpers, or mutate MIR.

## Boundary
Allowed input:

```text
CompareLoweringSymbolicCommandSnapshotV1
```

Allowed output:

```text
CompareRhsMaterializationIntentSnapshotV1
```

The intent may classify:

- literal i64 RHS required
- symbol lookup RHS required
- unsupported RHS materialization kind

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
bash tools/checks/rust_lifecycle_mirbuilder_compare_rhs_materialization_owner_selection_guard.sh
```
