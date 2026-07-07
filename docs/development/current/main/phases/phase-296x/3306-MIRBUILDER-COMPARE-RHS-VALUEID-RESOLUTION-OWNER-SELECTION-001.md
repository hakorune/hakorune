# 3306 - MIRBUILDER-COMPARE-RHS-VALUEID-RESOLUTION-OWNER-SELECTION-001

## Purpose
Select the next safe boundary after RHS materialization intent parity.

## Decision
Selected:

```text
CompareRhsValueIdResolutionPlanSnapshotV1
```

This is a read-only resolution plan pilot. It may classify whether an RHS needs
a literal constant path or symbol lookup path, but it must not produce a
`ValueId`.

## Rejected For Now
- Resolve RHS `ValueId` now
- Emit literal constants now
- Execute symbol lookup now
- Emit Compare with resolved RHS now

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
bash tools/checks/rust_lifecycle_mirbuilder_compare_rhs_valueid_resolution_owner_selection_guard.sh
```
