# Generic Loop Canon

This box owns shallow generic-loop canon helper surfaces.

## Boundary

```text
update/:
  observes loop-variable update expressions
  builds facts-side UpdateCanon
  no route selection
  no lowering

types.rs:
  owns ConditionCanon, UpdateCanon, StepPlacement, and StepPlacementDecision
  old facts/plan type paths are compatibility facades

condition/:
  observes loop-condition candidates and bounds
  builds facts-side ConditionCanon
  no route selection
  no lowering

step_extract/:
  owns loop increment extraction order
  observes legacy helper, var-step, next-step, and complex-step shapes
  no placement classification
  no lowering

step_placement/:
  observes direct and conditional step assignments
  classifies observations into StepPlacementDecision
  keeps facts and plan files separate
```

## Compatibility

Older deep paths remain facades while imports migrate:

```text
facts/canon/generic_loop/update.rs
facts/canon/generic_loop/condition.rs
facts/canon/generic_loop/step/extract.rs
facts/canon/generic_loop/step/placement/matcher.rs
plan/canon/generic_loop/step/placement/decision.rs
facts/canon/generic_loop/types.rs
plan/canon/generic_loop/types.rs
```

Do not add more top-level `control_flow/` siblings for generic-loop canon
helpers. Add related helpers under this grouped owner instead.
