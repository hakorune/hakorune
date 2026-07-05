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

Recommended first target:

```text
target_facade=loop_cond_continue_with_return_plan_rule.authority_facade
reason=smallest active Plan DTO; no lowering, MIR mutation, ID allocation, or RecipeMatcher execution
```

Alternative target:

```text
target_facade=single_planner_candidate_presence.authority_facade
reason=small Plan DTO over fact-slot presence, but still depends on PlanBuildOutcome shape
```

## Required Task Order

1. Inventory the existing HHako ProgramJSON shape for one minimal source fixture.
2. Choose one ProgramJSON path that can be traversed in `.hako` without new MIR
   mutation or lowering vocabulary.
3. Add a `.hako` ProgramJSON snapshot projector that emits the same token
   contract as the selected existing facade.
4. Add parity against the Rust ASTNode-token oracle for that one fixture.
5. If parity is green, mark the corresponding Rust projector slice as
   retire-candidate only, not retired.

## Allowed Implementation Slice

```text
allowed=ProgramJSON read-only traversal
allowed=string/list field extraction needed for one fixture
allowed=existing token snapshot output
allowed=parity gate against existing Rust oracle
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
Ask ChatGPT Pro with the question above before implementation.
```
