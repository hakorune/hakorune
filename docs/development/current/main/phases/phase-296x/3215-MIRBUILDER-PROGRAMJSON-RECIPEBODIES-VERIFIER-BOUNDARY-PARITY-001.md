# 3215 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-PARITY-001

Status: active

## Scope

Implement the first ProgramJSON RecipeBodies verifier-boundary parity slice:

```text
ProgramJsonRecipeBodiesVerifierBoundarySnapshotV1
```

The slice proves the covered recursive nested arena DTO can reach the existing
`RecipeVerifierBox.verify/2` result-map boundary. It does not publish runtime
`RecipeBodies`, execute RecipeMatcher, select routes, lower MIR, mutate MIR, or
allocate IDs.

Covered row:

```text
local_loop_body_if_branch_return
```

## Acceptance

```text
must build the recursive nested arena DTO
must require arena_ready=1 before verifier boundary
must call existing RecipeVerifierBox.verify/2
must summarize verifier result-map and PortSig output
must keep ProgramJSON builder verifier-policy-free
```

Expected result:

```text
snapshot_kind=ProgramJsonRecipeBodiesVerifierBoundarySnapshotV1
err=0
arena_ready=1
verifier_boundary_used=1
verified_recipe_present=1
body_count=4
def_count=1
update_count=2
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_verifier_boundary_parity_gate.sh
```

## Non-Claims

```text
runtime RecipeBodies publication
RecipeBodies::bodies direct access
full RecipeMatcher execution
verifier policy reimplementation inside ProgramJSON builder
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

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
