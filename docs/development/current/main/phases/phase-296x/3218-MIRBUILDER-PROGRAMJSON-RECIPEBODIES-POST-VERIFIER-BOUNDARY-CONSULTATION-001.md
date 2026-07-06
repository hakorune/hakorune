# 3218 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-CONSULTATION-001

Status: active

## Scope

Prepare the design consultation required by 3217 before opening any post-verifier
RecipeBodies runtime or RecipeMatcher seam.

The `.hako` ProgramJSON route now proves a covered recursive nested arena DTO
and validates that same covered shape through the existing
`RecipeVerifierBox.verify/2` boundary. The next move can change the migration
surface, so it needs an explicit decision.

## Consultation Question

```text
After RecipeBodies verifier-boundary parity is green, which next seam should be
opened?

A. More DTO coverage rows
   Add more ProgramJSON RecipeBodies DTO/parity rows while keeping runtime
   RecipeBodies publication, RecipeMatcher execution, and route switch at 0.

B. Narrow runtime RecipeBodies publication bridge
   Define an explicit temporary bridge from `.hako` DTO/result-map shape to a
   runtime RecipeBodies-like publication boundary, with removal conditions.

C. Full RecipeMatcher execution consultation
   Start the design for `.hako`-side RecipeMatcher execution and route contract
   migration, still before MIR lowering/mutation/ID allocation.
```

Recommended default:

```text
A_MORE_DTO_COVERAGE_ROWS
```

Reason:

```text
It keeps movement inside the already proven ProgramJSON DTO/verifier boundary
and avoids widening runtime authority before a route-switch decision exists.
```

## If B Is Chosen

Require a separate implementation contract before code:

```text
runtime publication shape
bridge owner
removal condition
fallback policy
DirectAbi/publication policy
runtime route switch remains 0 unless separately approved
```

## If C Is Chosen

Require a separate RecipeMatcher design card before code:

```text
matched contract kind surface
input RecipeBodies/RecipeBlock authority
failure/freeze behavior
route selection boundary
lowering/mutation/ID allocation non-claims
```

## Non-Claims

```text
runtime RecipeBodies publication
RecipeBodies::bodies direct access
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
DirectAbi route publication expansion
runtime route switch
ProgramJSON full parser
new backend route
new ABI
Source Selfhost
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_post_verifier_boundary_consultation_guard.sh
```

Expected result:

```text
consultation_prepared=1
recommended_option=A_MORE_DTO_COVERAGE_ROWS
implementation_selected=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-DECISION-001
```
