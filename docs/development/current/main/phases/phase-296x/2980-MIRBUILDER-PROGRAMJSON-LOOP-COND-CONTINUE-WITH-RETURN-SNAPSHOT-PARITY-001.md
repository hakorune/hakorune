# 2980 MIRBUILDER-PROGRAMJSON-LOOP-COND-CONTINUE-WITH-RETURN-SNAPSHOT-PARITY-001

Status: Landed  
Date: 2026-07-05  
Scope: ProgramJSON snapshot parity against the Rust ASTNode-token oracle for `LoopCondContinueThenReturnMinimalV1`.

## Decision

Prove the first ProgramJSON-produced token snapshot against the existing Rust
ASTNode-token oracle before any projector retirement claim.

Selected slice:

```text
ProgramJSON -> ProgramJsonLoopCondContinueWithReturnSnapshotV1
Rust ASTNode-token oracle -> canonical token snapshot
shape = LoopCondContinueThenReturnMinimalV1
```

The parity gate compares canonical summary fields, not raw JSON strings. The
accept row also runs the same already adopted
`loop_cond_continue_with_return_plan_rule` facade output for both routes.

## Implementation

- Add a parity fixture with five ProgramJSON rows:
  - accept minimal loop-if-continue-then-return;
  - reject break present;
  - reject nested loop;
  - reject if else not null;
  - reject unsupported loop condition.
- Add an AOT parity gate that:
  - executes `ProgramJsonLoopCondContinueWithReturnSnapshotBox.build_summary`
    for each ProgramJSON row;
  - compares the emitted snapshot summary against the Rust ASTNode-token oracle
    summary in the fixture;
  - executes the existing `LoopCondContinueWithReturnPlanRuleBox` facade for
    the accepted row and compares the facade output against the Rust oracle.

## Evidence

```text
python3 -m json.tool docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-cond-continue-with-return-snapshot-parity-v0.json
bash tools/checks/rust_lifecycle_mirbuilder_programjson_loop_cond_continue_with_return_snapshot_parity_gate.sh
```

Gate result:

```text
owner=ProgramJsonLoopCondContinueWithReturnSnapshotV1
input_contract=LoopCondContinueThenReturnMinimalV1
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
same_facade_output_parity=green
programjson_snapshot_matches_rust_astnode_oracle=1
source_selfhost_claim=0
hako_adopted_decision=0
rust_astnode_projector_retired=0
rust_astnode_projector_fully_retired=0
programjson_full_parser_claim=0
```

## Non-Claims

```text
source_selfhost_claim = 0
hako_adopted_decision = 0
rust_astnode_projector_retired = 0
rust_astnode_projector_fully_retired = 0
programjson_full_parser_claim = 0
recipe_matching_migrated = 0
route_selection_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
id_allocation_migration = 0
full_recipe_matcher_execution = 0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-TOKEN-SNAPSHOT-RETIRE-RUST-ASTNODE-PROJECTOR-CANDIDATE-001
```

Only the `LoopCondContinueThenReturnMinimalV1` snapshot projector slice may
become retire-candidate. Do not claim the full Rust ASTNode projector, full
fact extractor, RecipeMatcher, route selection, lowering, MIR mutation, ID
allocation, or Source Selfhost.
