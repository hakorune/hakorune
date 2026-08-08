# JOINIR Loop M8 variable-accum recurrence S6A — execution brief

Status: `accepted; blocked on LOOP-INPUT-SOURCE-RELATION-SET-R0`
Date: 2026-08-08
Parent: `JOINIR-LOOP-M8-LOOPV0-RECURRENCE-S6A-D0`
Row after R0: `JOINIR-LOOP-M8-LOOPV0-RECURRENCE-S6A`

## Change

Add one resolver-backed, caller-zero `VariableAccumRecurrenceV1` observer and
provenance-only producer for the exact `acc = acc + i; i = i + 1` loop. Private
input/condition/update/step/coverage observations co-seal into one move-only
`VerifiedVariableAccumRecurrenceFactsV1`, then deterministically issue only the
existing Recipe, JoinSig, Core, input-set, and operation/effect products.

## Contract

The accepted D0 card and Loop pipeline SSOT own the exact source membership,
C/D/U/R precedence, normalized 2-binding/11-operation/2-carrier golden, and
2 input / 2 binding / 8 Core-effect / 11 item-source relation counts. Recipe
keys begin at the producer; AST navigation stays in compiler projection and
neutral Facts remain AST-free. DirectAccum, Recipe/operation kinds, Core,
JoinSig elaboration, the family selector, and physical owners are not widened.

## Done

The natural fixture is Candidate only for the new observation; existing
SimpleWhile/DirectAccum/G0 observations remain Declined. Golden and negative
tests seal exact source identity, total coverage, all relation cardinalities,
Header/Body/After carrier visibility, and zero Builder effect. Source/check
files stay below 800 lines.

The same implementation commit updates `src/mir/compiler/README.md`,
`src/mir/loop_structural_facts/README.md`,
`src/mir/loop_route_policy/README.md`,
`src/mir/loop_recipe_contract/README.md`, and
`docs/reference/mir/loop-recipe-contract.md` with the landed producer ID and
2/2/8/11 receipt. A second reference update remains mandatory at M10b cutover.
Focused tests, current-pointer, Recipe in-place/no-fallback guards, formatting,
and `git diff --check` must pass.

## Stop

Return to design for missing resolver authority, incomplete/overlapping source
roles, a need to re-read AST in the producer, or any second selector/physical
route. Do not claim all-19 coverage, M9 parity, production activation,
retry/fallback retirement, outer print/return tail coverage, or legacy deletion.
