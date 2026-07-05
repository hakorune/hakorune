# 3003 - MIRBUILDER-PROGRAMJSON-NEXT-CAPABILITY-SELECTION-001

Status: landed

## Scope

Select the next ProgramJSON traversal capability after
`LoopBodyControlFlowSnapshotV1` was marked as a scoped Rust ASTNode projector
retire-candidate.

This is a selection card only. It must not add another string-only facade and
must not claim full projector retirement.

## Inputs

```text
2995 ProgramJsonLoopBodyControlFlowScanV1 parity green
2998 LoopBodyControlFlowSnapshotV1 retire-candidate scoped green
3002 dynamic typing hint inventory parked for later Tier-2/Tier-3 work
```

## Selection Rule

Choose one capability that:

- consumes ProgramJSON structure directly;
- can include multiple parity rows;
- can name a concrete Rust ASTNode projector slice as retire-candidate;
- stays below MIR mutation, lowering, route selection, RecipeMatcher, and ID
  allocation.

## Forbidden

- full Rust ASTNode projector retirement;
- ProgramJSON full parser claim;
- HakoAdoption for a full MirBuilder owner;
- backend lowering, MIR mutation, route selection, ID allocation, or new ABI;
- string-only formatter/classifier facade expansion.

## Output

Name one implementation-capability card and its parity gate shape, or stop for
design consultation if no capability can meet the selection rule.

## Decision

```text
selected_capability=ProgramJsonConditionShapeScanV1
selected_next_card=MIRBUILDER-PROGRAMJSON-CONDITION-SHAPE-SCAN-CAPABILITY-001
output_contract=ConditionShapeSnapshotV1
minimum_parity_rows=8
```

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_next_capability_selection_guard.sh

consumes_programjson_structure=1
string_only_facade=0
implementation_done=0
parity_gate_green=0
rust_astnode_projector_retire_candidate=0
source_selfhost_claim=0
```
