# 2979 MIRBUILDER-PROGRAMJSON-LOOP-COND-CONTINUE-WITH-RETURN-SNAPSHOT-AOT-RUNTIME-PARITY-001

Status: Landed  
Date: 2026-07-05  
Scope: First ProgramJSON-to-token snapshot AOT runtime parity for `LoopCondContinueThenReturnMinimalV1`.

## Decision

Promote the first ProgramJSON traversal pilot from MIR-verify-only to AOT
runtime parity.

Selected slice:

```text
ProgramJSON -> ProgramJsonLoopCondContinueWithReturnSnapshotV1 -> token summary
shape = LoopCondContinueThenReturnMinimalV1
```

This is still below Rust ASTNode projector retirement. The only claim is that
the HHako ProgramJSON route can execute the narrow snapshot owner and match the
fixture summaries for this one shape family.

## Implementation

- Let generic string body analysis treat `GenericI64Body` / `ScalarI64`
  callees as `I64` even when the source return annotation is unknown.
- Allow same-module generic i64 self-recursion for scanner cursor helpers.
- Keep ProgramJSON scanner cursors as i64 values; do not re-run
  `StringHelpers.to_i64` on cursor parameters in AOT-lowerable scanner paths.
- Add fixed ProgramJSON v0 field-token vocabulary for known scanner keys so
  dynamic-key needle construction does not create a hidden AOT runtime mismatch.
- Upgrade the scanner inventory guard from AOT-emit-only to AOT runtime output
  checks, including a nonzero `_read_char` cursor probe.
- Upgrade the snapshot implementation gate to emit and run an AOT executable,
  then compare canonical summary lines from the fixture.

## Evidence

```text
cargo test -q refresh_module_global_call_routes_accepts_self_recursive_generic_i64_body --lib
cargo build -q --release --bin hakorune
bash tools/checks/rust_lifecycle_mirbuilder_programjson_v0_scanner_aot_blocker_inventory_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_loop_cond_continue_with_return_snapshot_implementation_gate.sh
```

Gate result:

```text
owner=ProgramJsonLoopCondContinueWithReturnSnapshotV1
input_contract=ProgramJsonLoopCondContinueThenReturnMinimalV1
execution_backend=aot
aot_execution_status=green
runtime_summary_parity=green
parity_status=implementation_fixture_green
source_selfhost_claim=0
hako_adopted_decision=0
rust_astnode_projector_retired=0
programjson_full_parser_claim=0
```

## Non-Claims

```text
source_selfhost_claim = 0
hako_adopted_decision = 0
rust_astnode_projector_retired = 0
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
MIRBUILDER-PROGRAMJSON-LOOP-COND-CONTINUE-WITH-RETURN-SNAPSHOT-PARITY-001
```

Compare the ProgramJSON-produced token snapshot against the Rust
ASTNode-token oracle for the same shape. Do not mark any Rust ASTNode projector
as retired until that oracle parity is green and the retire scope names one
shape only.
