# 3224 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-INPUT-DESIGN-STOP-001

Status: active

## Design Stop

The next active task was:

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-MINIMAL-001
```

Source inspection found a real input-boundary mismatch:

```text
Rust authority:
  RecipeMatcher::try_match_loop(facts: &CanonicalLoopFacts)

Current ProgramJSON publication:
  RecipeBodiesPublicationSnapshotV1
```

The runtime publication bridge is green, but it publishes a read-only snapshot.
It does not publish `CanonicalLoopFacts` or a matcher input equivalent.

## Source Authority

```text
src/mir/builder/control_flow/plan/recipe_tree/matcher/mod.rs
src/mir/builder/control_flow/plan/recipe_tree/contracts.rs
src/mir/builder/control_flow/plan/single_planner/rules.rs
```

Observed contract:

```text
RecipeMatcher consumes CanonicalLoopFacts
RecipeMatcher produces RecipeContractKind::LoopWithExit
try_build_outcome stores outcome.recipe_contract
```

## Non-Authority

```text
RecipeBodiesPublicationSnapshotV1
RecipePortSigBox counts
ProgramJsonRecipeBodiesRuntimePublicationBridgeBox
```

These prove runtime-readable publication, not matcher input readiness.

## Required Selection

Before implementation, select one:

```text
A. ProgramJSON -> CanonicalLoopFacts projection bridge
B. RecipeBodiesPublicationSnapshot -> matcher input adapter
C. Minimal Hako RecipeMatcher over publication snapshot
```

Current recommendation: A or B. C is risky unless the accepted matcher input is
renamed away from `RecipeMatcher` to avoid a fake authority.

## Forbidden Until Selection

```text
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
runtime route switch
runtime fallback
Source Selfhost claim
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_execution_boundary_input_design_stop_guard.sh
```

Expected result:

```text
design_stop=1
rust_matcher_input=CanonicalLoopFacts
publication_snapshot_is_not_matcher_input=1
selected_next_card=CONSULTATION_REQUIRED
```
