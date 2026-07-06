# 3225 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-INPUT-BOUNDARY-CONSULTATION-001

Status: landed

## Decision

Select a read-only ProgramJSON-to-matcher-input projection before any
RecipeMatcher execution claim:

```text
A_PROGRAMJSON_TO_CANONICAL_LOOP_FACTS_INPUT_SNAPSHOT
```

Selected next card:

```text
MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-001
```

## Why

Worker inventory found:

```text
Rust RecipeMatcher input:
  CanonicalLoopFacts

Current ProgramJSON publication:
  RecipeBodiesPublicationSnapshotV1
```

The publication snapshot is read-only and AOT-safe, but it has only counts and
presence bits. It cannot derive matcher input by itself.

The ProgramJSON route still has richer data before publication:

```text
verified_recipe
recipe_root
Loop VarLtInt condition facts
assignment/update facts
return/exit shape facts
```

So the next safest step is an analysis-only projection snapshot:

```text
ProgramJsonCanonicalLoopFactsInputSnapshotV1
```

This must be a matcher-input materialization proof, not RecipeMatcher
execution.

## Acceptance For Next Card

```text
must read ProgramJSON / verified_recipe path, not the collapsed publication summary
must emit source=verified_recipe
must emit matcher_input_present=1
must emit exit_has_continue=1 and exit_has_return=1 for covered rows
must emit loop_cond_continue_with_return_present=1
must keep unrelated route families absent
must keep recipe_matcher_execution=0
```

## Forbidden In Next Card

```text
RecipeMatcher execution
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_input_boundary_consultation_guard.sh
```

Expected result:

```text
selected_option=A_PROGRAMJSON_TO_CANONICAL_LOOP_FACTS_INPUT_SNAPSHOT
selected_next_card=MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-001
recipe_matcher_execution=0
```
