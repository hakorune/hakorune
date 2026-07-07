# ProgramJSON Runtime Route Dual-Run Shadow Guard Inventory - 2026-07-07

Status: taskization inventory

## Current Boundary

3230 stopped before runtime route authority:

```text
Rust ASTNode route remains authority.
ProgramJSON route is shadow-only evidence.
runtime_route_switch=0
programjson_runtime_route_authority=0
```

Worker inventory confirms that the next safe task is not a runtime switch. The
safe seam is a dual-run shadow guard that compares canonical matcher-result
fields while Rust remains authority.

## Rust Authority Path

```text
RecipeMatcher::try_match_loop(facts)
  -> RecipeContractKind::LoopWithExit
  -> PlanBuildOutcome.recipe_contract
  -> route registry gate
  -> CorePlan
  -> PlanVerifier
  -> PlanLowerer / MIR mutation
```

The authority handoff begins once route candidates and lowering consume
`PlanBuildOutcome.recipe_contract`. ProgramJSON must not write to that outcome,
route registry, CorePlan, PlanVerifier, PlanLowerer, MIR builder, or ID
allocator in the next task.

## Existing ProgramJSON Shadow Path

```text
ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1
  -> ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot/1
  -> canonical matcher-result fields
```

The current accepted comparison fields are:

```text
matched
contract_kind
has_break
has_continue
has_return
```

## Selected Next Task

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-DUAL-RUN-SHADOW-GUARD-001
```

Card type:

```text
implementation-guard + fixture + AOT/EXE gate
```

Purpose:

```text
Run Rust ASTNode authority and ProgramJSON matcher-result shadow side by side
for the covered rows. Fail the gate on mismatch. Do not switch runtime
authority.
```

## Acceptance

The guard must:

```text
require 3229 shadow parity gate = green
run the ProgramJSON path through AOT/EXE
compare canonical matcher-result fields, not raw summary strings
report runtime_authority=rust_astnode
report dual_run_shadow_guard=1
report programjson_shadow_checked=1
report mismatch_count=0
fail fast on mismatch
```

The guard must keep these at zero:

```text
programjson_runtime_route_authority
runtime_route_switch
recipe_matcher_input_authority
full_recipe_matcher_execution
route_selection
mir_lowering
mir_mutation
id_allocation
runtime_fallback
source_selfhost_claim
new_backend_route
new_abi
```

## Implementation Boundary

Allowed:

```text
read Rust oracle canonical fields
run ProgramJSON shadow snapshot path
compare fields in a guard
emit a stable key=value report
```

Forbidden:

```text
write ProgramJSON result into PlanBuildOutcome.recipe_contract
modify RecipeMatcher authority
modify route registry candidate selection
compose or lower CorePlan from ProgramJSON
mutate MIR or allocate IDs
use fallback when ProgramJSON mismatches
claim runtime route authority or Source Selfhost
```

## Follow-Up Order

```text
1. MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-DUAL-RUN-SHADOW-GUARD-001
   Shadow-only guard. Rust remains authority.

2. MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-EXPANDED-ROWS-001
   Add rows only if the dual-run guard shows coverage is too narrow.

3. MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-CONSULTATION-002
   Re-open authority switch only after dual-run guard is green.
```
