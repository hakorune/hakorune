# 3095 - MIRBUILDER-PROGRAMJSON-CAPABILITY-CANDIDATE-VALIDATION-001

Status: active

## Scope

Validate the next ProgramJSON traversal capability from actual ProgramJSON v0
emit evidence after 3094.

This is not a migration step by itself. It exists only to name the next
`.hako` implementation card.

## Required Output

```text
selected_next_card = MIRBUILDER-PROGRAMJSON-*-SHAPE-SCAN-CAPABILITY-001
owner = ProgramJson*ScanV1
snapshot = *ShapeSnapshotV1
covered_rows = N named parity rows
retire_candidate_target = covered Rust ASTNode projector slice
```

## Acceptance

- inspect ProgramJSON v0 emitted shapes, not only Rust ASTNode fixture names;
- select a capability that can land `.hako` traversal, fixture rows, and an AOT
  parity gate in the next implementation card;
- do not land a guard-only or selection-only card as progress;
- keep Tier-2/Tier-3 AOT/MIR typing debt parked unless the active validation or
  implementation gate directly reaches that route.

## Non-Claims

- no `.hako` implementation landed by this card;
- no ProgramJSON traversal capability claim;
- no Rust ASTNode projector retirement;
- no HakoAdoption, ProgramJSON full parser, MIR mutation, lowering, route
  selection, ID allocation, Source Selfhost, new backend route, or new ABI.
