# 3222 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-EXPANDED-DTO-COVERAGE-NEXT-CONTRACT-SELECTION-001

Status: active

## Decision

After expanded verifier-boundary DTO coverage and its scoped retire-candidate,
select:

```text
B_RUNTIME_RECIPEBODIES_PUBLICATION_BRIDGE
```

Next card:

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-PUBLICATION-BRIDGE-001
```

## Why

3220/3221 proved two ProgramJSON verifier-boundary DTO rows through the existing
`ProgramJsonRecipeBodiesVerifierBoundarySnapshotBox` and marked those rows as
scoped Rust ASTNode projector retire-candidates.

More verifier-boundary DTO rows are useful only as a secondary coverage task.
The next mainline boundary should reduce the remaining runtime route dependency
by proving a read-only runtime publication snapshot for verifier-accepted
RecipeBodies data.

Full RecipeMatcher execution remains after publication. It must not be mixed
with runtime publication, route selection, lowering, mutation, ID allocation, or
runtime route switching.

## Acceptance For Next Card

```text
must publish RecipeBodiesPublicationSnapshotV1
must use a read-only result-map / map-handle boundary
must preserve verifier_boundary_used=1 and verified_recipe_present=1
must keep recipe_matcher_executed=0
must keep runtime route switch at 0
```

Rows:

```text
local_loop_body_if_branch_return
local_loop_body_if_branch_return_alt_names
```

## Forbidden In Next Card

```text
runtime RecipeBodies authority
RecipeBodies::bodies runtime access authority
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
runtime route switch
runtime fallback
new backend route
new ABI
Source Selfhost
Rust ASTNode projector full retirement
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_after_expanded_dto_coverage_next_contract_selection_guard.sh
```

Expected result:

```text
selected_option=B_RUNTIME_RECIPEBODIES_PUBLICATION_BRIDGE
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-PUBLICATION-BRIDGE-001
runtime_recipe_bodies_publication_bridge=0
full_recipe_matcher_execution=0
runtime_route_switch=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-PUBLICATION-BRIDGE-001
```
