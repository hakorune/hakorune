# Generic Loop Step Placement

This box owns generic-loop step placement helpers.

## Boundary

```text
facts.rs:
  collects direct and conditional step-assignment observations
  analysis-only
  no route selection
  no lowering

plan.rs:
  classifies collected observations into StepPlacementDecision
  may use RejectReason vocabulary
  does not inspect unrelated route families
```

## Compatibility

Older deep paths under `facts/canon/generic_loop/step/placement` and
`plan/canon/generic_loop/step/placement` remain as facades while imports migrate.

Do not merge facts and plan truth into one function. The shared box is only a
physical owner for one semantic subtree.
