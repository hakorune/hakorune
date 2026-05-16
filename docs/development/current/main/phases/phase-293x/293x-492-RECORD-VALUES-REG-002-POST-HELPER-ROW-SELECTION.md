# 293x-492 RECORD-VALUES-REG-002 Post-Helper Row Selection

Status: landed
Date: 2026-05-16

## Decision

`RECORD-VALUES-REG-002` is the planning-only row after the landed
`RECORD-VALUES-REG-001` builder-local record registration helper cleanup.

It selects exactly one next row:

```text
PROOF-APPS-MANIFEST-SCHEMA-001:
  normalize trailing proof_apps.toml M214/M215 rows to the active
  [[proof_apps]] manifest schema
```

It does not land code.

## Worker Inventory

Worker inventory refined the remaining cleanup into these row-sized candidates.
The list is intentionally more granular than the previous broad buckets.

```text
candidate:
  PROOF-APPS-MANIFEST-SCHEMA-001
owner:
  tools/checks/proof_apps.toml
  tools/checks/manifest_runner_pilot_guard.sh
scope:
  normalize M214/M215 trailing [[proof_app]] rows to active [[proof_apps]]
  schema so run_proof_app.sh --list exposes them
stop_line:
  no manifest_runner.py selection semantic changes
  no dev_gate / allocator-wide wiring
  no proof guard body changes
evidence:
  tools/checks/run_proof_app.sh --list | rg 'M214|M215'
  bash tools/checks/manifest_runner_pilot_guard.sh
risk:
  low

candidate:
  EXPRS-COLLECTION-LITERAL-001
owner:
  new src/mir/builder/collection_literals.rs
scope:
  move ArrayLiteral / MapLiteral lowering out of exprs.rs
stop_line:
  no ArrayBox / MapBox route changes
  preserve array element inference and type/origin registry writes
  no backend/provider behavior
evidence:
  cargo test -q array_value_get_uses_unified_receiver_arg_shape_and_element_return
  cargo test -q map_value_set_uses_unified_receiver_arg_shape_and_receipt_string_return
risk:
  low-medium

candidate:
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
evidence:
  bash tools/checks/k2_wide_static_const_table_load_guard.sh
  cargo test -q static_const_table_load
  cargo test -q array_value_get_uses_unified_receiver_arg_shape_and_element_return
  cargo test -q map_value_get_existing_key_uses_unified_receiver_arg_shape_and_stored_value_return
risk:
  medium

candidate:
  EXPRS-CHECK-001
owner:
  new src/mir/builder/exprs_check.rs
scope:
  move CheckExpr lowering only
stop_line:
  no boolean coercion changes
  no parser/check-block surface changes
  no Select semantics changes
evidence:
  cargo test -q c198_check_block_parses_default_route
  cargo test -q c198_check_block_parses_token_cursor_route
  bash tools/checks/k2_wide_check_block_surface_guard.sh
risk:
  low payoff

candidate:
  OSVM-EXPORT-VALIDATION-HELPER-001
owner:
  crates/nyash_kernel/src/exports/osvm.rs
scope:
  factor repeated base/len validation in commit/decommit/unreserve
stop_line:
  no new exports
  no page-size behavior change
  no mmap/mprotect/munmap flag changes
  no provider/hook/global allocator work
evidence:
  cargo test -q -p nyash_kernel osvm
  bash tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh
  bash tools/checks/k2_wide_mimalloc_osvm_unreserve_exe_guard.sh
risk:
  low-medium

candidate:
  PARSER-RECORD-DECL-OWNER-001
owner:
  src/parser/declarations/box_def/record.rs
scope:
  move parse_record_declaration out of box_def/mod.rs
stop_line:
  no accepted syntax changes
  no record initializer/weak/from/implements behavior changes
  no MIR/record lowering edits
evidence:
  cargo test -q parser_record_surface
  cargo test -q parser_contract_surface
risk:
  medium

parked:
  USERBOX-ROUTE-SPLIT-007A route contract impl extraction
  USERBOX-ROUTE-SPLIT-007B value box resolver extraction
  USERBOX-ROUTE-SPLIT-007C return-shape route class mapping extraction
  USERBOX-ROUTE-SPLIT-007D value-type publication sub-owner split
reason:
  user_box_method_route_plan now has clear owners; origin_inference.rs is
  large but still one coherent inference family
```

Selection note:

- Choose `PROOF-APPS-MANIFEST-SCHEMA-001` first because it is tooling-only,
  concrete, and low-risk.
- Keep `EXPRS-INDEXING-001` as the next likely compiler cleanup once the tiny
  manifest schema mismatch is closed.
- Keep user-box route splits parked unless the route module starts growing
  again or a new route bug points at one of the parked owner seams.

## Selection Criteria

The selected row must:

- name one owner, proof/guard, and stop lines before implementation
- keep BoxShape cleanup separate from allocator behavior
- avoid adding, removing, or renaming accepted language/compiler shapes
- avoid broad planner/validator rewrites
- preserve pure-first diagnostics layer/contract output
- keep provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive unless an explicit provider ladder is reopened

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next row is selected with a clear owner, stop lines,
and evidence plan.

## Selection Result

```text
selected:
  PROOF-APPS-MANIFEST-SCHEMA-001
owner:
  tools/checks/proof_apps.toml
  tools/checks/manifest_runner_pilot_guard.sh
scope:
  manifest schema normalization for M214/M215 rows only
stop_line:
  no runner behavior changes
  no guard body changes
  no gate wiring changes
```
