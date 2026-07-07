# 3255 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-UNSUPPORTED-CONDITION-OPERATOR-REJECT-ROW-001

Status: landed

## Scope

Implement the reject-floor row selected by 3254.

Covered row:

```text
row_id = unsupported_condition_operator_reject
loop condition operator = <=
```

Expected boundary:

```text
CanonicalLoopFacts snapshot:
  ok = 0
  reason = unsupported_loop_cond
  matcher_input_present = 0

RecipeMatcher boundary:
  ok = 0
  reason = snapshot_not_ok
  matched = 0
```

## Boundary

`LoopStmtHandler` may build a verified Recipe DTO for structurally valid
Compare conditions that are not CanonicalLoopFacts-supported matcher input.
CanonicalLoopFacts remains the owner that rejects unsupported loop condition
operators with `unsupported_loop_cond`.

This is still shadow-only. ProgramJSON does not write
`PlanBuildOutcome.recipe_contract`, does not feed route predicates, and does not
select/release routes.

## Non-Claims

```text
programjson_runtime_route_authority = 0
runtime_route_switch = 0
recipe_matcher_input_authority = 0
route_selection = 0
MIR lowering/mutation = 0
ID allocation = 0
runtime_fallback = 0
Source Selfhost remains unclaimed.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_unsupported_condition_operator_reject_row_gate.sh
```
