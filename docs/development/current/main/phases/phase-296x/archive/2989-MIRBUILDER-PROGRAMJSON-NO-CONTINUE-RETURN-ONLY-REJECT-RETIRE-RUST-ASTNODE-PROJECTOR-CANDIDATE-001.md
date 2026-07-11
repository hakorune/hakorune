# 2989 MIRBUILDER-PROGRAMJSON-NO-CONTINUE-RETURN-ONLY-REJECT-RETIRE-RUST-ASTNODE-PROJECTOR-CANDIDATE-001

Status: Landed  
Date: 2026-07-05  
Scope: One-shape Rust ASTNode projector retire-candidate for `LoopCondContinueNoContinueReturnOnlyRejectV1`.

## Decision

Mark only the `LoopCondContinueNoContinueReturnOnlyRejectV1` token snapshot
slice as a Rust ASTNode projector retire-candidate.

This is a reject shape. ProgramJSON traversal must observe a loop body made of a
single `Return` and produce the same `no_continue` snapshot decision as the Rust
ASTNode-token oracle. This does not remove any Rust runtime dependency. The Rust
ASTNode projector is still kept for oracle generation, and the full projector
remains active.

## Guarded Scope

```text
retire_candidate=LoopCondContinueNoContinueReturnOnlyRejectTokenSnapshotV1
shape_scope=LoopCondContinueNoContinueReturnOnlyRejectV1
programjson_snapshot_owner=ProgramJsonLoopCondContinueWithReturnSnapshotV1
parity_row=reject_no_continue_return_only
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
```

## Evidence

```text
python3 -m json.tool docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-no-continue-return-only-reject-retire-rust-astnode-projector-candidate-v0.json
bash tools/checks/rust_lifecycle_mirbuilder_programjson_no_continue_return_only_reject_retire_rust_astnode_projector_candidate_guard.sh
```

Guard result:

```text
decision=RetireCandidateScoped
parity_row=reject_no_continue_return_only
parity_gate=green
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
source_selfhost_claim=0
hako_adopted_decision=0
programjson_full_parser_claim=0
selected_next_card=MIRBUILDER-PROGRAMJSON-NEXT-SHAPE-SELECTION-008
```

## Non-Claims

```text
source_selfhost_claim = 0
hako_adopted_decision = 0
rust_astnode_projector_retired = 0
rust_astnode_projector_fully_retired = 0
full_astnode_projector_retired = 0
programjson_full_parser_claim = 0
programjson_all_shapes_supported = 0
recipe_matching_migrated = 0
route_selection_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
id_allocation_migration = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-NEXT-SHAPE-SELECTION-008
```

Select the next ProgramJSON traversal shape only if it reduces another Rust
ASTNode projection slice or exposes a concrete missing HHako traversal
capability.
