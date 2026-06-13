# Generic Loop Canon

This box owns shallow generic-loop canon helper surfaces.

## Boundary

```text
update/:
  observes loop-variable update expressions
  builds facts-side UpdateCanon
  no route selection
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
facts/canon/generic_loop/step/placement/matcher.rs
plan/canon/generic_loop/step/placement/decision.rs
```

Do not add more top-level `control_flow/` siblings for generic-loop canon
helpers. Add related helpers under this grouped owner instead.
