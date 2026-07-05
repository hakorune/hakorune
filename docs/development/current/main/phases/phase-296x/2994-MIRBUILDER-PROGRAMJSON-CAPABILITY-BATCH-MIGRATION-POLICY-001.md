# 2994 - MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-MIGRATION-POLICY-001

Status: landed

## Scope

Replace the `1 shape = 1 card` ProgramJSON migration cadence with capability
batch migration.

Durable policy:

```text
docs/development/current/main/design/mirbuilder-programjson-capability-batch-migration-policy-ssot.md
```

## Decision

Next ProgramJSON migration work must use:

```text
1 traversal capability = 1 implementation card + 1 parity gate + N parity rows
```

The next implementation card is:

```text
MIRBUILDER-PROGRAMJSON-LOOP-BODY-CONTROL-FLOW-SCAN-CAPABILITY-001
```

It must implement `.hako` ProgramJSON `Loop.body` control-flow scanning, add a
parity fixture/gate, cover multiple rows, and mark at least one Rust ASTNode
projector slice as retire-candidate.

## Stop

Do not add another one-shape ProgramJSON retire-candidate card before the
capability card unless the capability work exposes a concrete HHako expressivity
blocker.

## Non-Claims

- `source_selfhost_claim = 0`
- `hako_adopted_decision = 0`
- `rust_astnode_projector_retired = 0`
- `rust_astnode_projector_fully_retired = 0`
- `programjson_full_parser_claim = 0`
- `programjson_all_shapes_supported = 0`
- `recipe_matching_migrated = 0`
- `route_selection_migration = 0`
- `backend_lowering_migration = 0`
- `mir_mutation_migration = 0`
- `id_allocation_migration = 0`
- `new_backend_route = 0`
- `new_abi = 0`
