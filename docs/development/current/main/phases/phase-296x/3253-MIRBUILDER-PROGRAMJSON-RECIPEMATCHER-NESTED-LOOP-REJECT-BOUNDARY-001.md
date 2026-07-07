# 3253 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-NESTED-LOOP-REJECT-BOUNDARY-001

Status: landed

## Scope

Implement the ProgramJSON RecipeMatcher observe-only reject boundary selected
by 3252.

When a ProgramJSON-derived CanonicalLoopFacts snapshot publishes
`has_nested_loop=1`, the observe-only matcher boundary returns `matched=0`
with the stable `nested_loop_present` reason. This does not accept nested
loops as `LoopWithExit` matcher input.

## Row

```text
nested_loop_reject_boundary:
  input snapshot ok=1
  input snapshot has_nested_loop=1
  matcher ok=0
  matcher reason=nested_loop_present
  matcher matched=0
```

## Non-Claims

```text
nested_loop_accepted_floor = 0
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_nested_loop_reject_boundary_gate.sh
```
