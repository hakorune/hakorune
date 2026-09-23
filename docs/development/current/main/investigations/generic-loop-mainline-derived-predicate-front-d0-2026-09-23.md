---
Status: fast__derived_predicate_static_classification_open__2026-09-23
Task: GENERIC-LOOP-MAINLINE-DERIVED-PREDICATE-SOURCE-FRONT-D0
Date: 2026-09-23
Parent: generic-loop-mainline-in-body-step-front-d0-2026-09-23.md
PreviousCard: generic-loop-mainline-in-body-step-front-d0-2026-09-23.md
NextCard: selected by CURRENT_STATE after this card closes
Implementation permission: true for this card's exact Main0
derived-predicate profile, its canonical Main publication, and removal of
that profile's raw body/raw finish edge in the same bounded series.
---

# Main0 derived-predicate source-to-portable migration

## Six-line brief

```text
Decision: select the existing Main0 derived-predicate source profile
  (apps/tests/generic_loop_carrier_type_v0_numeric_min.hako, expected RC 4)
  as the third bounded pre-wrapper selection arm in the sole production
  root funnel; same canonical Main draft -> collector -> canonical
  finalize chain. BoxCount: one new admitted shape, no behavior change to
  the two landed profiles.
Source authority + canonical issuer: same installed package/batch one-shot
  App Main loan + ResolvedFunctionLoweringInputV1 Core; a bounded
  main0_derived_predicate syntax-Facts -> source-map -> LoopRecipeDraftV1
  co-seal mirroring compiler/main0_in_body_step_*; the portable producer
  alone issues Recipe keys; existing JoinSig/Completion owners retain
  control meaning.
Non-authority: fixture/backend names, legacy route IDs (the GenericLoopV0
  route tag is migration evidence only), VM output, MIR types/effects, the
  two landed Facts (they must decline, not co-admit), and any corpus-level
  producer claim.
Fail-fast boundary: missing/foreign/duplicate source, control, ABI or
  tail/result contracts reject before physical work; a source with
  If/Break/Continue/else/calls/non-unit steps/nested loops/strings or a
  different predicate derivation declines to the legacy path; disagreement
  after Facts admission is a hard contract failure; later failure discards
  the unpublished invocation.
Smallest next slice: static old-route classification of this fixture, then
  Facts -> map -> co-seal -> bounded lowerer reusing the common
  physicalizer, segment dispatcher, initialized-local materializer, and
  tail completion; the condition-read validator gains a sibling that
  traces compare.left through one Add to the carrier read; add a third
  selection arm beside the landed two; retire this profile's raw body/raw
  finish reachability; runtime acceptance RC 4 plus a new release-adopt
  gate (strict-shadow stays recorded baseline debt).
Non-claims: no If/Break/else/Return-in-loop arm coverage, no calls/print/
  strings, no LoopSimpleWhile/Generic-V0 family claim, no production
  route_loop switch, no M8-S6E/S6G/M9/M10b completion, no corpus census
  repeat.
```

Boundary: selected MIR source preparation -> same-package Main0 admission ->
whole-function portable lowering -> canonical finished root -> existing
module validation/publication. Includes that profile's raw body and raw
finish edges; excludes other Main profiles, widening of the physicalizer
boundary, VM restoration, B3/R7 reopening, and whole shared owner deletion.
This is a finite design target, not an all-family census.

## Target selection

The selected fixture is the only remaining App-Main0-shaped integer-only
corpus case (the `phase29ca`/`phase29cb` files and variants are admitted or
landed; `selfhost_trim` and `phase29bq_generic_loop_v1_*` carry calls,
strings, compound conditions, or nested loops and stay out of cohort):

```text
static box Main {
    main() {
        local j = 0
        local m = 0
        local n = 3
        loop(j + m <= n) {
            j = j + 1
        }
        return j
    }
}
```

Expected result `4`. No acceptance gate exists yet for this fixture; this
card adds one release-adopt VM gate mirroring
`tools/smokes/v2/profiles/integration/joinir/
generic_loop_in_body_step_release_adopt_vm.sh`. The strict-shadow lane is
known baseline-broken (`StringHelpers.to_i64/1` retired static-call
terminal, reproduced on the row-E parent commit); its red is recorded, not
repaired, in this card.

The profile is the smallest in-cohort widening left: integer locals only,
a zero-branch predicate loop, one body statement (`j = j + 1`), no
`if`/`break`/`else`/calls/strings/`print`/nested loop. Its single new
semantic surface is a derived predicate: the compare LHS is one `Add`
(`j + m`) over the carrier and a read-only local, and the operator is
`<=` (`SyntaxBinaryOperatorV1::LessEqual` -> `LoopCompareI64OpV1::
LessEqual`, both already in the shared vocabulary).

## Open items before implementation

1. Static old-route classification — the fixture comment claims
   GenericLoopV0 ("the add-expression condition keeps this shape on V0").
   Observe both lanes as the prior cards did: production callable lane
   (`--dump-mir` / `compile_normal`) and compatibility vm-keep lane; record
   the exact reject token or route tag.
2. Condition-read relation — `validate_main0_condition_read_relation_v1`
   requires `compare.left` to be a direct `ReadBinding` result. Here
   `compare.left` is the `Add` result; a sibling validator must trace the
   binary op's inputs to the carrier read and the read-only local read.
   Confirm the recipe draft composes `push_binary_i64(Add)` +
   `push_compare_i64(LessEqual)` in the condition block and that the
   segment dispatcher emits both block-agnostically.
3. Numeric contract — this is the first profile with arithmetic inside
   the predicate. Row E's shape-level representability proof does not
   cover `j + m`; record instead the i64-lane parity fact (dynamic
   Integer `+` and physical i64 `Add` coincide for all admitted
   operands) as the bounded contract. No value-dependent membership is
   introduced or needed.
4. Corpus TSV: update only this case's row in
   `docs/development/current/main/design/fixtures/
   generic-loop-legacy-disposition-v1.tsv`; do not rerun the census.

## Acceptance and retained scope

- Unchanged fixture: expected result `4`; a bound-2 variant (`n = 2`)
  must return `3` and a zero-iteration variant (`m > n`, e.g. `m = 9`)
  must return `0`.
- Renamed locals preserve the same membership. Any `if`, `break`, `else`,
  `continue`, call, `print`, string, non-unit step, or different predicate
  derivation is outside this profile with an explicit preselection
  decline.
- Success contains one physical Main and one outer publication. Selected
  source cannot reach raw body/raw finish. Real source-to-MIR and selected
  executable result evidence are required before closeout, same as the
  landed rows.

Shared raw body dispatch, other Main profiles, GenericV1/LoopCond/LoopTrue
consumers, registry/Composer/PlanLowerer and all nonselected callers
remain. This row does not authorize whole-symbol deletion or close the
wider M8/M9/M10b requirements.
