# 3230 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-DESIGN-STOP-001

Status: design-stop

Decision: CONSULTATION_REQUIRED

## Stop Reason

3229 proves shadow parity for the covered RecipeMatcher result:

```text
Rust ASTNode route oracle == ProgramJSON route matcher result snapshot
```

The next step would touch runtime route authority. That crosses the wider
route-selection design-stop boundary, so implementation must stop before any
runtime route switch or authority claim.

## Current Authority

```text
Rust ASTNode route remains authority.
ProgramJSON route is shadow-only evidence.
runtime_route_switch=0
programjson_runtime_route_authority=0
```

## Candidate Decisions

```text
A_SHADOW_ONLY_DUAL_RUN_GUARD
   Recommended default.
   Run the ProgramJSON route beside the Rust ASTNode authority and fail the
   gate on mismatch. Do not switch runtime authority.

B_DIRECT_RUNTIME_ROUTE_SWITCH
   Rejected for now.
   This would change runtime authority before a dual-run mismatch guard and
   before wider route-selection approval.

C_MORE_DTO_OR_MATCHER_ROWS_BEFORE_SWITCH
   Consultation alternative.
   More rows may increase confidence, but they do not by themselves answer the
   authority switch boundary.
```

## Consultation Question

```text
After RecipeMatcher shadow parity is green for the covered rows, should the
next slice add a dual-run shadow guard while Rust remains authority, or require
more matcher rows before any runtime route switch work?

Recommended default:
  A_SHADOW_ONLY_DUAL_RUN_GUARD
```

## Forbidden Until Resolved

```text
runtime route switch
ProgramJSON runtime route authority
RecipeMatcher input authority
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
runtime fallback
Source Selfhost claim
new backend route
new ABI
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_runtime_route_shadow_switch_design_stop_guard.sh
```

Expected result:

```text
design_stop=1
programjson_shadow_parity_green=1
recommended_default=A_SHADOW_ONLY_DUAL_RUN_GUARD
selected_next_card=CONSULTATION_REQUIRED
runtime_route_switch=0
programjson_runtime_route_authority=0
recipe_matcher_input_authority=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
source_selfhost_claim=0
```
