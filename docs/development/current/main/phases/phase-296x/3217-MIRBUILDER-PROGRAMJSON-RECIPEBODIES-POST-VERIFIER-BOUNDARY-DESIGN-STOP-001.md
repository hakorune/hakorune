# 3217 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-DESIGN-STOP-001

Status: active

## Scope

Stop after the covered RecipeBodies verifier-boundary retire-candidate.

The ProgramJSON route now proves:

```text
DTO BodyId/StmtRef surface
one-shape arena DTO
If branch arena DTO
Loop body arena DTO
recursive nested arena DTO
RecipeVerifierBox.verify/2 boundary over the covered recursive row
```

The next step is no longer another DTO-only proof unless a new decision selects
one. It either opens runtime RecipeBodies publication, a full RecipeMatcher
execution seam, or a route-switch plan. Those are policy boundaries.

## Consultation

Recommended next consultation card:

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-CONSULTATION-001
```

Question:

```text
After verifier-boundary parity is green, which next seam is allowed:
A. add more DTO coverage rows before runtime publication,
B. define a narrow runtime RecipeBodies publication bridge,
C. start a full RecipeMatcher execution consultation?
```

Recommended default:

```text
A. add more DTO coverage rows, unless runtime route-switch authority is approved.
```

## Forbidden Without New Decision

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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_post_verifier_boundary_design_stop_guard.sh
```

Expected result:

```text
boundary=RecipeBodiesPostVerifierBoundaryDesignStop
implementation_allowed_now=0
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-CONSULTATION-001
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-CONSULTATION-001
```
