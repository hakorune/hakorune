---
Status: SSOT
Date: 2026-07-05
Scope: ProgramJSON traversal capability batching for MirBuilder Rust-to-Hako migration.
Related:
  - docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
  - lang/src/compiler/mirbuilder/program_json_loop_cond_continue_with_return_snapshot.hako
  - tools/checks/rust_lifecycle_mirbuilder_programjson_loop_cond_continue_with_return_snapshot_parity_gate.sh
---

# MirBuilder ProgramJSON Capability Batch Migration Policy

## Decision

Stop the `1 shape = 1 card` ProgramJSON cadence.

The next migration unit is:

```text
1 traversal capability = 1 implementation card + 1 parity gate + N parity rows
```

Do not switch to a full MirBuilder rewrite. ProgramJSON traversal should move in
capability batches that are large enough to reduce Rust ASTNode projector
surface, but still below RecipeMatcher execution, route execution, lowering,
MIR mutation, and ID allocation.

## Current Queue

```text
landed batch:
  2995 Loop.body control-flow scan
  3004..3043 condition/return/local/call/method/new/field/record-field/
    binary/compare/logical/local-array/print/throw/LoopRange scan and scoped
    retire-candidate pairs, plus Try/Assign/EnumMatch/local-string scan-retire
    pairs, and local null scan

next batch:
  MIRBUILDER-PROGRAMJSON-LOCAL-NULL-LITERAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

required shape:
  1. select the next concrete ProgramJSON traversal capability;
  2. land `.hako` implementation, parity fixture rows, and AOT parity gate in
     the implementation card;
  3. immediately follow with the scoped Rust ASTNode projector retire-candidate
     for only the covered snapshot rows.
```

Default implementation card type:

```text
implementation-capability + parity gate
```

Retire-candidate cards are allowed only immediately after a green
implementation capability. They must not introduce new acceptance rows.

Selection-only cards are allowed only when the next traversal capability is
genuinely ambiguous. If the capability is already named, merge selection into
the implementation card; do not land docs/guard-only selection as progress.

Each implementation capability card must include `.hako` implementation,
fixture rows, a parity gate, and a retire-candidate decision for at least one
Rust ASTNode projector slice.

## Capability

Landed baseline:

Owner:

```text
ProgramJsonLoopBodyControlFlowScanV1
```

Input:

```text
ProgramJSON v0
```

Output:

```text
LoopBodyControlFlowSnapshotV1
```

Minimum snapshot fields:

```text
continue_count
break_count
return_count
has_nested_loop
if_then_tail_continue_count
if_else_null_count
unsupported_node_count
```

This capability corresponds to the first authority checks in Rust
`try_extract_loop_cond_continue_with_return_facts`: observe continue, break,
return, and nested-loop state before recipe construction.

## Required Traversal Vocabulary

Use structured ProgramJSON field scanning only:

```text
program_body_array_range(program_json)
first_stmt_object_in_array(array_start)
next_stmt_object_in_array(array_start, previous_end)
node_type_at(object_start)
loop_body_array_range(loop_object_start)
if_then_array_range(if_object_start)
if_else_state(if_object_start) -> Null | Array | Unsupported
scan_stmt_control_flow(stmt_object_start)
scan_stmt_array_control_flow(array_range)
```

Allowed comparison:

```text
node_type_at(stmt_start) == "Continue"
```

Forbidden proof:

```text
program_json contains "\"Continue\""
source contains "continue"
regex over raw source or raw JSON text
```

String scanning is allowed only through the ProgramJSON scanner vocabulary that
locates object fields and node values structurally.

## Initial Parity Rows

The first capability card should include at least these rows:

```text
empty_body
return_only
continue_only
break_present
continue_then_return
if_then_continue_else_null_then_return
nested_loop
if_hidden_nested_loop
if_then_continue_no_return
second_stmt_not_return
```

The parity gate must compare canonical fields, not raw JSON strings.

## Next Implementation Capability

After 3032, choose the next concrete capability from actual ProgramJSON
projector surface. Selection is implementation prep, not progress by itself.
If the capability is clear, do not add a standalone selection-only card.

The next implementation card must name:

```text
ProgramJson*ScanV1 owner
SnapshotV1 output contract
covered parity rows
retire-candidate target projector slice
```

Minimum implementation scope:

```text
real `.hako` ProgramJSON traversal
fixture rows that prove structural field traversal
AOT parity gate
unsupported rows fail fast or return explicit unsupported tokens
```

Do not count these as progress:

```text
guard-only card
docs-only selection row
string-only facade expansion
prebuilt token snapshot pass-through
```

The purpose is to continue replacing Rust ASTNode projection with HHako
ProgramJSON traversal while keeping the work below RecipeMatcher execution,
route selection, lowering, MIR mutation, and ID allocation.

## Gate

Primary gate:

```text
tools/checks/rust_lifecycle_mirbuilder_programjson_loop_body_control_flow_scan_parity_gate.sh
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-loop-body-control-flow-scan-parity-v0.json
```

The gate must prove:

```text
programjson_traversal_used = 1
string_only_facade = 0
token_snapshot_equal = 1
covered_rows >= 6
```

## Retire Candidate

If the capability parity is green, mark only this slice as retire-candidate:

```text
LoopBodyControlFlowSnapshotV1 for covered ProgramJSON loop body shapes
```

Not retired:

```text
full Rust ASTNode projector
full loop_cond_continue_with_return facts extractor
RecipeMatcher
route selection
MIR lowering
MIR mutation
ID allocation
```

## Stop Conditions

Stop and open missing expressivity consultation if any of these become true:

```text
.hako implementation accepts prebuilt token strings only
source string / raw ProgramJSON contains / regex matching is used as proof
ProgramJSON route calls Rust ASTNode projector to build the target snapshot
output includes MIR block mutation, ID allocation, backend lowering, or RecipeMatcher execution
unsupported node is silently ignored
```

Also stop if a second consecutive ProgramJSON facade/capability card adds no
Rust ASTNode projector retire-candidate. A capability card must reduce or mark
retire-candidate at least one Rust projector slice, or expose a concrete missing
HHako capability.

## Non-Claims

Keep these at zero unless a later card explicitly changes the contract:

```text
source_selfhost_claim
mir_mutation
id_allocation
backend_lowering
full_recipe_matcher_execution
route_selection
block_creation
phi_materialization
native_seed_materialization
hako_generation
hako_adopted_for_full_owner
rust_astnode_projector_fully_retired
programjson_full_parser_claim
programjson_all_shapes_supported
runtime_fallback
new_backend_route
new_abi
```

Allowed claim:

```text
ProgramJSON Loop.body control-flow traversal is parity-green for the covered
LoopBodyControlFlowSnapshotV1 rows.
```
