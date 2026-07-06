# 3127 - MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3126 parity is green, mark only the covered
`SeqRecipeDtoSnapshotV1` ProgramJSON Layer4 rows as a scoped Rust ASTNode
projector retire-candidate.

This is a future runtime route-switch candidate, not Rust bootstrap/oracle
deletion.  It does not claim HakoAdoption, full RecipeMatcher execution,
route selection, backend lowering, MIR mutation, ID allocation, ProgramJSON
full parser, or Source Selfhost.

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_seq_recipe_dto_parity_gate.sh
```

## Retire Candidate Scope

```text
SeqRecipeDtoSnapshotV1
for covered ProgramJSON Layer4 Seq Recipe DTO rows
```

Covered rows:

```text
return_int
return_new_box
local_return_var
empty_body_reject
```

Deferred rows:

```text
top_level_assignment
top_level_print
```

Those deferred rows belong to a separate PhaseState consumer capability and are
not claimed by this retire-candidate.

## Not Retired

```text
full Rust ASTNode projector
runtime route dependency
full RecipeMatcher
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
HakoAdoption for a full owner
Source Selfhost
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_seq_recipe_dto_retire_rust_astnode_projector_candidate_guard.sh
```

Guard result:

```text
decision=RetireCandidateScoped
parity_gate=green
programjson_runtime_parity_green=1
covered_rows=4
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
full_astnode_projector_retired=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-001
```
