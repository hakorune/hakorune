# 3229 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-001

Status: active

## Scope

Prove shadow parity between the Rust ASTNode RecipeMatcher oracle and the
ProgramJSON route matcher-result snapshot for the covered rows.

Rust oracle:

```text
RecipeMatcher::try_match_loop -> RecipeContractKind::LoopWithExit
```

ProgramJSON route:

```text
ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1
  -> ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot/1
```

The comparison is field-level:

```text
matched
contract_kind
has_break
has_continue
has_return
```

This does not make the ProgramJSON route the RecipeMatcher authority. It does
not select routes, lower MIR, mutate MIR, allocate IDs, switch runtime routes,
or claim Source Selfhost.

Rows:

```text
local_loop_body_if_branch_return
local_loop_body_if_branch_return_alt_names
```

## Acceptance

```text
must run the 3228 observe-only RecipeMatcher boundary gate
must check Rust RecipeContractKind::LoopWithExit source contract
must run ProgramJSON matcher result through AOT/EXE
must compare canonical matcher result fields against the Rust oracle fixture
must keep full_recipe_matcher_execution=0
must keep route_selection/lowering/mutation/id_allocation/runtime_switch=0
```

## Implementation Notes

The Rust oracle is held as a fixture-backed source contract for the current
covered rows. The gate verifies the Rust contract shape in source and compares
the ProgramJSON AOT result against the same canonical fields.

This is a shadow parity proof only. Runtime route switch is deliberately held
for a design-stop card because switching authority crosses the wider route
selection boundary.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_shadow_parity_gate.sh
```

Expected result:

```text
recipe_matcher_shadow_parity=1
matcher_result_equal=1
rust_astnode_route_oracle_checked=1
programjson_route_shadow_checked=1
recipe_matcher_input_authority=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_route_switch=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-DESIGN-STOP-001
```
