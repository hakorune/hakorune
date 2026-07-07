# 3252 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-DECISION-ROW-CONSULTATION-001

Status: landed

## Scope

Resolve the nested-loop decision row selected by 3251.

This card does not add nested-loop RecipeMatcher acceptance. Existing
ProgramJSON traversal already has a scoped nested-loop reject projector
candidate, and the CanonicalLoopFacts snapshot publishes `has_nested_loop`.
The gap is that the observe-only RecipeMatcher boundary must reject
`has_nested_loop=1` before any authority switch can be reconsidered.

## Decision

```text
selected:
  B_REJECT_BOUNDARY_IMPLEMENTATION

rejected:
  A_ACCEPT_NESTED_LOOP_MATCHER_ROW
  C_SCAN_ONLY_DIAGNOSTIC
```

Selected next:

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-REJECT-BOUNDARY-001
```

## Boundary

```text
ProgramJSON may observe has_nested_loop=1.
ProgramJSON must not accept it as LoopWithExit matcher input.
The next implementation row should return matched=0 with a stable
nested_loop_present reason at the observe-only matcher boundary.
```

## Non-Claims

```text
nested_loop_accepted_floor = 0
nested_loop_reject_boundary_green = 0
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_nested_loop_decision_row_consultation_guard.sh
```
