# Generic legacy observation front G0

Status: `next implementation row; P0 corpus universe landed 2026-08-07`

Parent SSOT: `../design/generic-loop-source-to-portable-recipe-ssot.md`.

## Scope

Run exactly one canonical case from the checked Generic legacy corpus universe
through the exact selected front and produce a green Loop-reached receipt. The
front is an observation seam only: it must not select a new route, widen
Generic support, patch a fixture, or classify the full corpus.

## Required product

```text
one normalized case
  -> one explicit mode/profile invocation
  -> exact selected front
  -> Loop-reached receipt or named pre-Loop owner failure
```

The receipt must retain the manifest case identity, canonical fixture,
mode/profile, command, exit/timeout state, first Loop-reached evidence, and
the real owner of any failure before Loop. A failed front is not a Generic
disposition and does not authorize a workaround.

## Acceptance

- the input case is selected by manifest identity and not by a filename or
  route-shaped name;
- the command is serial and its mode/profile is copied from the manifest;
- a successful run proves only that the exact front reached Loop;
- a pre-Loop failure names its actual owner in a separate repair row;
- timeout remains unclassified and blocks route observation;
- no Generic producer, Recipe, Builder/MIR, physical, retry/fallback, or
  production caller is changed;
- focused receipt tests, the shared replacement guard, pointer guard, and
  `git diff --check` are green;
- the implementation commit updates the exact `docs/reference/**` page,
  active workstream, task pointer, and related README; the final production
  implementation row must update its reference documentation again in the
  same commit.

## Non-goals

Do not run the complete corpus, infer dispositions, compare release/strict
routes, open M10b, delete legacy code, or create a second selection/route
authority. Those remain later ordered rows.
