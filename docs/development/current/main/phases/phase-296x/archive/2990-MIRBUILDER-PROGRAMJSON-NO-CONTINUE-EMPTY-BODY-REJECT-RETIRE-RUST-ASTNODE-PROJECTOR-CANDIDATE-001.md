# 2990 - MIRBUILDER-PROGRAMJSON-NO-CONTINUE-EMPTY-BODY-REJECT-RETIRE-RUST-ASTNODE-PROJECTOR-CANDIDATE-001

Status: landed

## Scope

Mark only the `LoopCondContinueNoContinueEmptyBodyRejectV1` ProgramJSON
snapshot row as a Rust ASTNode projector retire-candidate.

This card covers one reject shape:

```text
Loop.body = []
expected_summary = accepted=0;reason=no_continue
```

## Evidence

- Fixture:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-no-continue-empty-body-reject-retire-rust-astnode-projector-candidate-v0.json`
- Guard:
  `tools/checks/rust_lifecycle_mirbuilder_programjson_no_continue_empty_body_reject_retire_rust_astnode_projector_candidate_guard.sh`
- Shared parity fixture:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-cond-continue-with-return-snapshot-parity-v0.json`
- Shared parity gate:
  `tools/checks/rust_lifecycle_mirbuilder_programjson_loop_cond_continue_with_return_snapshot_parity_gate.sh`

## Decision

`LoopCondContinueNoContinueEmptyBodyRejectTokenSnapshotV1` is a
retire-candidate for the single `LoopCondContinueNoContinueEmptyBodyRejectV1`
shape only.

The Rust ASTNode projector is still retained as oracle generation. Runtime
dependency removal and full projector retirement are not claimed.

## Non-Claims

- `source_selfhost_claim = 0`
- `hako_adopted_decision = 0`
- `rust_astnode_projector_retired = 0`
- `rust_astnode_projector_fully_retired = 0`
- `full_astnode_projector_retired = 0`
- `programjson_full_parser_claim = 0`
- `programjson_all_shapes_supported = 0`
- `recipe_matching_migrated = 0`
- `route_selection_migration = 0`
- `backend_lowering_migration = 0`
- `mir_mutation_migration = 0`
- `id_allocation_migration = 0`
- `runtime_fallback = 0`
- `new_backend_route = 0`
- `new_abi = 0`
