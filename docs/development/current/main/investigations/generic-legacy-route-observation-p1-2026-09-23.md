---
Status: design_stop__serial_route_observation_open
Task: GENERIC-LEGACY-ROUTE-OBSERVATION-P1
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-loop-mainline-derived-predicate-front-d0-2026-09-23.md
NextCard: same-row__serial_observation_until_bounded_batch_recorded
Implementation permission: false; record route/bypass/RC/output for the
bounded serial batch only. No source, fixture, corpus-universe,
semantic-receipt, producer, selector, or route changes.
---

# Generic legacy route observation P1 — serial generic-fixture batch

## Six-line brief

```text
Decision: run the next serial P1 route observation for the normalized
  Generic corpus; the bounded batch is the twelve unobserved
  generic-fixture fast-gate cases in manifest order.
Source authority + canonical issuer: the P0-normalized case universe
  (design/fixtures/generic-loop-legacy-disposition-v1.tsv) fixes
  membership; the green front receipt is the selected MIR source-backed
  lane established by the three landed Main0 D0 cards, exercised here by
  two direct invocations per case (release VM run + callable-lane MIR
  dump). No smoke wrapper, no strict-shadow pin.
Non-authority: fixture/backend names, manifest planner_tag expectations
  (historical lane evidence only), VM output as route proof, and
  unobserved cases as Declined.
Fail-fast boundary: a timeout or a stop before the GenericLoop route at
  a named owner stays unclassified for that axis; no manufactured result,
  no wrapper substitution, no parallel census.
Smallest next slice: observe the first generic-fixture case
  (general_if, phase29bq_fast_gate_cases.tsv:29), then continue serially
  through line 169; record each outcome below and set the manifest
  observation_state for observed rows only.
Non-claims: no producer/Recipe/selector arm, no route_loop switch, no
  source/fixture/smoke change, no disposition classification
  (GENERIC-LEGACY-DISPOSITION-D0 owns that), no S6E/S6G/M9/M10 closeout,
  no corpus re-census, no guard vocabulary change.
```

## Fixed boundary

The bounded batch is exactly the twelve `corpus=generic-fixture,
mode=fast-gate` case rows with `observation_state=unobserved`, in
`phase29bq_fast_gate_cases.tsv` line order:

```text
tsv:29  general_if                                  expected=4   planner=LoopSimpleWhile
tsv:30  nested_if_min                               expected=10  planner=LoopExitIfBreakContinue
tsv:31  generic_loop_v1_recipe_nested_if_min        expected=0   planner=LoopSimpleWhile
tsv:32  generic_loop_v1_recipe_nested_if_loop_min   expected=2   planner=LoopSimpleWhile
tsv:33  generic_loop_v1_recipe_nested_if_loop_else_min expected=20 planner=LoopSimpleWhile
tsv:34  generic_loop_v1_recipe_then_only_empty_join_min expected=3 planner=LoopSimpleWhile
tsv:35  generic_loop_v1_recipe_loop_if_loop_callguard_min expected=10 planner=LoopExitIfBreakContinue
tsv:36  generic_loop_v1_recipe_loop_if_loop_pure_min expected=10  planner=LoopExitIfBreakContinue
tsv:122 generic_loop_v1_local_def_continue_min      expected=3   planner=LoopContinueOnly
tsv:123 generic_loop_v1_nested_min                  expected=1   planner=LoopSimpleWhile
tsv:159 loop_header_shortcircuit_generic_loop_v0_min expected=0  planner=LoopSimpleWhile
tsv:169 generic_loop_v1_if_cond_prelude_loop_min    expected=3   planner=LoopSimpleWhile
```

All other corpus rows stay `unobserved` for a later serial slice.

## Front definition

Per case, two direct invocations of the same binary
(`target/debug/hakorune`, built with `--features vm-reference`), working
directory = repo root, timeout 10s. No smoke wrappers.

```text
A. release VM run   : env NYASH_DISABLE_PLUGINS=1 NYASH_JOINIR_DEV=0
                      NYASH_JOINIR_STRICT=0 HAKO_JOINIR_STRICT=0
                      NYASH_JOINIR_DEBUG=0 HAKO_JOINIR_DEBUG=0
                      hakorune --backend vm <fixture>
                      -> record exit code + stdout tail
B. callable-lane MIR: hakorune --dump-mir <fixture>
                      -> record outcome class + first diagnostic token
                         or @main signature
```

Invocation A is the runtime receipt the three landed D0 cards used
(release-adopt shape). Invocation B is the materializer observation the
premise recheck designated as the selected source front. Neither is the
retired strict-shadow wrapper.

## Per-case record schema

```text
case_id | tsv:line | A_rc | A_stdout_tail | B_outcome | B_evidence |
observation_state | notes
```

`B_outcome` vocabulary:

- `canonical-main`   : `define i64 @main()` with a value `ret` (a Main0
  selection arm admitted the shape; name the arm).
- `legacy-void-main` : `define void @main()` + `ret void` (declined all
  arms and lowered through the retained legacy callable path).
- `named-reject(owner)` : fail-fast contract error naming an owner.
- `build-red(owner)` : compile failure before MIR emission.
- `timeout` / `spawn-error`.

`observation_state` mapping (TSV column only; the card holds the full
record):

- A ran within timeout and produced the manifest `expected` output or an
  `allowed_rc`-consistent exit, and B produced a classified outcome ->
  `accepted` for cases whose front result matches the recorded
  expectation, `rejected` when B is a typed named-reject/build-red.
- A or B stopped before the GenericLoop route at a named owner ->
  `failed-before-loop` with the owner recorded in the card.
- Timeout -> `timeout`. Anything else stays `unobserved`.

## Finite outcomes and acceptance

| Outcome | Evidence | Allowed next step |
| --- | --- | --- |
| All 12 recorded, none unclassified | each row has A_rc + B_outcome; TSV observation_state set | Card closes `landed`; next serial P1 batch (e.g. phase29bq corpus) is selected by the family scheduler. |
| A case stops at a named owner | owner + token recorded; state stays `unobserved` or `failed-before-loop` per mapping | Keep the case unclassified on the missing axis; the owner is a separate repair row, not a disposition. |
| Front itself breaks (build red on A or B) | recorded as build-red | Stop the batch; classification is not manufactured. |

Non-claims: this card does not claim any case is production-accepted,
does not switch or retire any route, does not add profile membership,
and does not classify dispositions. `GENERIC-LEGACY-DISPOSITION-D0`
remains the only row that may check dispositions.

## Observation log

(pending — filled serially)
