# Generic legacy observation front G0

Status: `landed 2026-08-07; front failed before Loop at a shared owner; S0-D0 design accepted; next implementation row is GENERIC-RAW-STRUCTURED-DEMANDS-REPAIR-S0-I0`

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

## Landed receipt

The fixed direct VM invocation for canonical case
`generic_loop_continue_strict_shadow_vm` was run serially with the sealed
`vm-strict-planner-direct-v1` invocation profile against its recorded fixture
`apps/tests/phase29ca_generic_loop_continue_min.hako`. It exited `1` before
Loop reached and emitted
`[freeze:contract][raw-structured/unconsumed-demands] expressions=1 bodies=0`
from `src/mir/builder/raw_structured_child_scope.rs:108`. The actual source
owner is the `BinaryOp` arm of
`src/mir/builder/raw_expression_dispatch/mod.rs::build_expression_impl_with_port_v1`
for the prelude `StringifyOperator.apply/1` `Body(1)/IfCondition` demand. The
first failing expression is `value.stringify != null`, not the final return
and not a user local initializer.
The receipt is
`../design/fixtures/generic-legacy-observation-front-g0-v1.json`; its guard
resolves the exact non-alias P0 case and rejects smoke-wrapper/fallback route
claims. This closes G0 as a named pre-Loop failure only. The owner is opened
separately as `GENERIC-RAW-STRUCTURED-DEMANDS-REPAIR-S0-I0`; that repair is
now verified by the fresh I0 receipt
`../design/fixtures/generic-raw-structured-demands-repair-s0-i0-v1.json`,
which preserves the primary callable-semantic error. No Generic route,
Recipe, physical, disposition, or production claim was made. The next
boundary is the separate design row
`GENERIC-RAW-STRUCTURED-DEMANDS-ADMISSION-S1-D0`.

## Non-goals

Do not run the complete corpus, infer dispositions, compare release/strict
routes, open M10b, delete legacy code, or create a second selection/route
authority. Those remain later ordered rows.
