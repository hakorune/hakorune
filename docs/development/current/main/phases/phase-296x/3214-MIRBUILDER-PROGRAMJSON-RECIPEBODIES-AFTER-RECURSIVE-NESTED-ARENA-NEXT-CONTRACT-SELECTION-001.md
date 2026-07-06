# 3214 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-RECURSIVE-NESTED-ARENA-NEXT-CONTRACT-SELECTION-001

Status: active

## Decision

After the recursive nested RecipeBodies arena retire-candidate, select:

```text
A_RECIPEBODIES_VERIFIER_BOUNDARY_PARITY
```

Next card:

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-PARITY-001
```

## Why

3212/3213 proved that the ProgramJSON route can build and summarize a
map-backed recursive nested arena DTO with root, loop, and nested if branch body
references.

The next durable contract is not runtime `RecipeBodies` publication. It is a
verifier boundary that consumes the composed DTO shape and proves that the
existing `RecipeVerifierBox.verify/2` path can validate the covered arena
without reimplementing verifier policy in the ProgramJSON builder.

This keeps the migration moving from DTO construction toward Layer4 recipe
validation while keeping route selection, lowering, mutation, ID allocation, and
runtime route switching out of scope.

## Acceptance For Next Card

```text
must consume the recursive nested arena DTO shape
must call the existing RecipeVerifierBox.verify/2 boundary
must prove verifier result-map output is stable
must keep ProgramJSON builder policy-free
must keep runtime RecipeBodies publication at 0
```

Rows:

```text
local_loop_body_if_branch_return
```

## Forbidden In Next Card

```text
full RecipeBodies runtime publication
RecipeBodies::bodies direct access
full RecipeMatcher execution
verifier policy reimplementation inside ProgramJSON builder
route selection
MIR lowering
MIR mutation
ID allocation
runtime route switch
DirectAbi route publication expansion
ProgramJSON full parser
new backend route
new ABI
Source Selfhost
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_after_recursive_nested_arena_next_contract_selection_guard.sh
```

Expected result:

```text
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-PARITY-001
recipe_bodies_verifier_boundary_implemented=0
recipe_bodies_materialization=0
runtime_route_switch=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-PARITY-001
```
