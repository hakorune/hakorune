# 2982 MIRBUILDER-PROGRAMJSON-NO-RETURN-REJECT-SNAPSHOT-PARITY-001

Status: Landed  
Date: 2026-07-05  
Scope: ProgramJSON snapshot parity for `LoopCondContinueNoReturnRejectV1`.

## Decision

Select and prove the next ProgramJSON traversal shape without adding a new
facade or new scanner capability.

Selected shape:

```text
LoopCondContinueNoReturnRejectV1
Program.body[0]=Loop(cond=Compare, body=[
  If(cond=Compare, then=[Continue], else=null)
])
expected=accepted=0;reason=no_return
```

This shape reduces another Rust ASTNode-token projection slice using the
existing `ProgramJsonLoopCondContinueWithReturnSnapshotV1` owner.

## Evidence

```text
python3 -m json.tool docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-cond-continue-with-return-no-return-reject-snapshot-parity-v0.json
bash tools/checks/rust_lifecycle_mirbuilder_programjson_loop_cond_continue_with_return_no_return_reject_snapshot_parity_gate.sh
```

Gate result:

```text
shape_scope=LoopCondContinueNoReturnRejectV1
parity_rows=1
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_snapshot_matches_rust_astnode_oracle=1
source_selfhost_claim=0
rust_astnode_projector_retired=0
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
```

## Next

```text
MIRBUILDER-PROGRAMJSON-NO-RETURN-REJECT-RETIRE-RUST-ASTNODE-PROJECTOR-CANDIDATE-001
```

Only this no-return reject shape may become retire-candidate. Do not merge it
with the already guarded minimal accept-shape retire-candidate.
