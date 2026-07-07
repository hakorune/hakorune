# 3245 - MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-FINAL-TOPLEVEL-RETURN-DECOUPLE-SNAPSHOT-BOUNDARY-001

Status: landed

## Scope

Decouple final top-level `Return` presence from loop-body exit usage at the
CanonicalLoopFacts snapshot boundary.

`build_snapshot(program_json)` now publishes:

```text
final_top_level_return_present
final_top_level_return_used_for_loop_body_has_return = 0
```

The snapshot no longer directly rejects on `missing_final_return` or
`final_stmt_not_return`. Loop-body `exit_has_return` remains derived from
loop-body scan only.

## Boundary

This card does not make Return-absent an accepted-floor row. It also does not
claim that the upstream verified-recipe producer accepts programs without a
final top-level `Return`.

## Non-Claims

```text
return_absent_accepted_floor = 0
matcher_result_equal = 0
recipe_matcher_accepted_floor = 0
producer_final_return_requirement_removed = 0
runtime_route_switch = 0
programjson_runtime_route_authority = 0
recipe_matcher_input_authority = 0
route_selection = 0
MIR lowering/mutation = 0
ID allocation = 0
Source Selfhost remains unclaimed.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_canonical_loop_facts_final_toplevel_return_decouple_snapshot_boundary_gate.sh
```
