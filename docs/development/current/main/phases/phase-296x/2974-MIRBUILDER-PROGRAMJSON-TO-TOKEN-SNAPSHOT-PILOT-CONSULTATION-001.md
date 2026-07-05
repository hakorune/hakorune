---
Status: Consultation
Date: 2026-07-05
Scope: ProgramJSON traversal pilot for MirBuilder selfhost migration.
---

# MIRBUILDER-PROGRAMJSON-TO-TOKEN-SNAPSHOT-PILOT-CONSULTATION-001

## Problem

The recent MirBuilder HakoAdopted owners are useful authority seams, but they
are still token/DTO surfaces. They do not prove that HHako can traverse
ProgramJSON or retire the Rust `ASTNode -> token snapshot` projector.

```text
current_route=RHako parser -> Rust ASTNode -> Rust token snapshot -> HHako facade
target_probe=HHako parser -> ProgramJSON -> HHako token snapshot -> existing facade
```

More string-only facade work is stopped. The next meaningful selfhost step
should prove one minimal ProgramJSON traversal path, or identify the missing
`.hako` expressivity that blocks it.

## Gap Classification

Default assumption: the next blocker is library/data-model work, not syntax.

```text
ProgramJSON_AST_traversal=library_gap_first
Recipe_Plan_construction=data_model_and_builder_library_gap_first
FailFast_Freeze_contract=contract_library_gap_first
```

Examples of library/data-model work:

```text
program_json_cursor=object/array/field access helpers
stmt_traversal=block/list visitor helpers
recipe_plan_builder=RecipeItem/RecipeBody/Plan DTO builders
contract_result=accepted/reason/hint structured result helpers
```

Escalate to `.hako` language/runtime feature work only if the pilot proves one
of these cannot be expressed cleanly as libraries:

```text
feature_gap=recursive traversal cannot be expressed
feature_gap=variable-length list iteration/append cannot be expressed
feature_gap=nested object access cannot be made safe
feature_gap=builder-style accumulation cannot be represented
feature_gap=fail-fast early return/result propagation cannot be expressed
```

This consultation must classify any blocker as `LibraryGap` or `FeatureGap`
before implementation continues.

## Proposed Pilot

Select one already-adopted MirBuilder facade input contract and feed it from
ProgramJSON instead of Rust ASTNode projection.

Consultation result:

```text
target_facade=loop_cond_continue_with_return_plan_rule.authority_facade
minimal_shape=Program.body[0]=Loop(cond, body=[If(cond, then=[Continue], else=null), Return(Int)])
reason=first real ProgramJSON traversal below MIR mutation, ID allocation, backend lowering, and full RecipeMatcher execution
```

The pilot must traverse real ProgramJSON structure:

```text
Program -> body
Loop -> cond + body
If -> cond + then + else
Continue
Return
```

## Required Task Order

1. `MIRBUILDER-PROGRAMJSON-LOOP-COND-CONTINUE-WITH-RETURN-SNAPSHOT-BASIS-001`
   defines `ProgramJsonLoopCondContinueWithReturnSnapshotV1` vocabulary.
2. `MIRBUILDER-PROGRAMJSON-LOOP-COND-CONTINUE-WITH-RETURN-SNAPSHOT-IMPLEMENTATION-001`
   implements `.hako` ProgramJSON -> token snapshot traversal.
3. `MIRBUILDER-PROGRAMJSON-LOOP-COND-CONTINUE-WITH-RETURN-SNAPSHOT-PARITY-001`
   compares Rust ASTNode-token oracle vs HHako ProgramJSON-token snapshot.
4. `MIRBUILDER-PROGRAMJSON-TOKEN-SNAPSHOT-RETIRE-RUST-ASTNODE-PROJECTOR-CANDIDATE-001`
   marks only this one shape/snapshot as retire-candidate.

First success claim:

```text
ProgramJSON traversal for LoopCondContinueThenReturnMinimalV1 token snapshot is parity-green.
```

## Allowed Implementation Slice

```text
allowed=ProgramJSON read-only traversal
allowed=string/list field extraction needed for one fixture
allowed=existing token snapshot output
allowed=parity gate against existing Rust oracle
```

Smallest traversal vocabulary:

```text
program_body_array
field_value_start / field_object_start / field_array_start_or_null
node_type_at
find_first_top_level_loop
read_loop_cond / read_loop_body
scan_loop_body_for_control_flow
if_then_tail_is_continue
if_else_is_null
```

Initial supported node tokens:

```text
supported=Program,Loop,If,Continue,Break,Return,Int,Var,Compare
unsupported=LoopRange,Try,Throw,ScopeBox,TaskScope,Match,Array,MapBox traversal,RecipeMatcher
```

## Forbidden Claims

```text
source_selfhost_claim=0
parser_integration_done=0
rust_astnode_projector_removed=0
recipe_construction_migrated=0
RecipeMatcher_execution_migrated=0
route_execution_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
new_backend_route=0
new_abi=0
```

## Stop Conditions

```text
stop=implementation accepts only precomputed token strings
stop=source string contains / regex / route-name matching is used
stop=MIR, IDs, blocks, route decisions, or backend output are constructed
stop=full RecipeMatcher execution starts
stop=unsupported ProgramJSON shapes are silently ignored
stop=parity is green only because Rust ASTNode token snapshot is still target input
stop=a second facade is added before one Rust ASTNode projector slice becomes retire-candidate
```

The ProgramJSON route must produce its own snapshot, and parity must compare
canonical fields rather than raw JSON strings.

## Retire Candidate Scope

If parity is green, only this may become retire-candidate:

```text
retire_candidate=LoopCondContinueWithReturnTokenSnapshotV1
shape_scope=LoopCondContinueThenReturnMinimalV1
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
```

## Questions For ChatGPT Pro

We are migrating a Rust MirBuilder to HHako (`.hako`) and have stopped adding
string-only token/DTO facades. Current adopted `.hako` owners can classify or
format token snapshots, but they do not traverse AST/ProgramJSON or mutate MIR.

Context:

```text
current_route=source -> RHako parser -> Rust ASTNode -> Rust token snapshot -> HHako facade
target_probe=source -> HHako parser -> ProgramJSON -> HHako token snapshot -> same HHako facade
already_adopted=loop_cond_continue_with_return_plan_rule, single_planner_* small DTOs
forbidden_now=MIR mutation, ID allocation, backend lowering, full RecipeMatcher execution
goal=prove the first ProgramJSON traversal path and identify missing .hako expressivity
default_assumption=library/data-model gap first, not syntax; escalate to feature gap only if recursion/list traversal/nested access/builder accumulation/fail-fast result cannot be expressed as libraries
```

Question:

```text
What is the safest first ProgramJSON-to-token-snapshot pilot?

Please choose one concrete target facade and one minimal ProgramJSON shape to
traverse. Then propose:
1. the smallest `.hako` traversal vocabulary needed,
2. the exact parity gate shape against the existing Rust ASTNode-token oracle,
3. the stop conditions that should prevent this from turning into another
   string-only facade loop,
4. what may be called retire-candidate for the Rust ASTNode projector,
5. what must remain explicitly unclaimed.

Prefer an approach that advances real selfhost migration by reducing reliance on
Rust ASTNode projection, while avoiding MIR mutation, lowering, ID allocation,
or full RecipeMatcher migration in the first slice.
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LOOP-COND-CONTINUE-WITH-RETURN-SNAPSHOT-BASIS-001
```
