# 293x-494 PROOF-APPS-MANIFEST-SCHEMA-002 Post-Manifest Row Selection

Status: landed
Date: 2026-05-16

## Decision

`PROOF-APPS-MANIFEST-SCHEMA-001` closed the proof-app manifest schema mismatch.

Select exactly one next cleanup row:

```text
EXPRS-INDEXING-001:
  move index target inference, static-data index load, and ArrayBox/MapBox
  get/set lowering out of exprs.rs into a dedicated builder indexing owner
```

## Why This Row

Worker inventory found several row-sized cleanup candidates after the
record-values helper cleanup:

```text
EXPRS-COLLECTION-LITERAL-001
EXPRS-INDEXING-001
EXPRS-CHECK-001
OSVM-EXPORT-VALIDATION-HELPER-001
PARSER-RECORD-DECL-OWNER-001
parked USERBOX-ROUTE-SPLIT-007*
```

`EXPRS-INDEXING-001` is selected first because it removes a real mixed owner
from `exprs.rs` while preserving the accepted AST shape surface.

## Selected Row

```text
row:
  EXPRS-INDEXING-001
owner:
  new src/mir/builder/indexing.rs
scope:
  move index target inference, static-data index load, ArrayBox/MapBox get/set
  lowering out of exprs.rs
stop_line:
  no new indexable classes
  no error text/tag changes
  no static table element support beyond existing u16
  no backend/provider behavior
evidence:
  bash tools/checks/k2_wide_static_const_table_load_guard.sh
  cargo test -q static_const_table_load
  cargo test -q array_value_get_uses_unified_receiver_arg_shape_and_element_return
  cargo test -q map_value_get_existing_key_uses_unified_receiver_arg_shape_and_stored_value_return
  bash tools/checks/current_state_pointer_guard.sh
  tools/checks/dev_gate.sh quick
```

## Stop Lines

- Do not add accepted syntax or new indexable runtime classes.
- Do not change parser behavior, static-data schema, ArrayBox/MapBox route
  semantics, or error messages.
- Do not touch allocator behavior, provider activation, hooks, host allocator
  replacement, or `#[global_allocator]`.
- Do not combine this with collection literal, CheckExpr, OSVM, parser-record,
  or user-box route splits.

## Closeout

This row closes when `EXPRS-INDEXING-001` has a selected current card with owner,
scope, stop lines, and evidence.
