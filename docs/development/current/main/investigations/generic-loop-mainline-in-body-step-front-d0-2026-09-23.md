---
Status: fast__in_body_step_static_classification_open__2026-09-23
Task: GENERIC-LOOP-MAINLINE-IN-BODY-STEP-SOURCE-FRONT-D0
Date: 2026-09-23
Parent: generic-loop-mainline-source-front-d0-2026-09-23.md
PreviousCard: generic-loop-mainline-source-front-d0-2026-09-23.md
NextCard: selected by CURRENT_STATE after this card closes
Implementation permission: true for this card's exact Main0 in-body-step
profile, its canonical Main publication, and removal of that profile's raw
body/raw finish edge in the same bounded series.
---

# Main0 in-body-step source-to-portable migration

## Six-line brief

```text
Decision: select the existing Main0 in-body-step source profile
  (apps/tests/phase29cb_generic_loop_in_body_step_min.hako, expected RC 3) as
  the second bounded pre-wrapper selection arm in the sole production root
  funnel; same canonical Main draft -> collector -> canonical finalize chain.
  BoxCount: one new admitted shape, no behavior change to the landed profile.
Source authority + canonical issuer: same installed package/batch one-shot
  App Main loan + ResolvedFunctionLoweringInputV1 Core; a bounded
  main0_in_body_step syntax-Facts -> source-map -> LoopRecipeDraftV1 co-seal
  mirroring compiler/main0_continue_*; the portable producer alone issues
  Recipe keys; existing JoinSig/Completion owners retain control meaning.
Non-authority: fixture/backend names, legacy route IDs (the GenericLoopV0
  in-body-step route is migration evidence only), VM output, MIR
  types/effects, the LoopCondContinueOnly Facts (they must decline, not
  co-admit), and any corpus-level producer claim.
Fail-fast boundary: missing/foreign/duplicate source, control, ABI or
  tail/result contracts reject before physical work; a source with
  If/Break/Continue/else/calls/non-unit steps/nested loops declines to the
  legacy path; disagreement after Facts admission is a hard contract
  failure; later failure discards the unpublished invocation.
Smallest next slice: static old-route classification of this fixture, then
  Facts -> map -> co-seal -> bounded lowerer reusing the common
  physicalizer, segment dispatcher, initialized-local materializer, and
  tail completion; verify a zero-branch predicate loop lowers through the
  existing normal-continuation path; add a second selection arm beside
  program_root_lowering/main0_continue_route; retire this profile's raw
  body/raw finish reachability; runtime acceptance RC 3 plus both existing
  smoke gates.
Non-claims: no If/Break/else/Return-in-loop arm coverage, no calls/print/
  strings, no LoopSimpleWhile/Generic-V0 family claim, no production
  route_loop switch, no M8-S6E/S6G/M9/M10b completion, no corpus census
  repeat.
```

Boundary: selected MIR source preparation -> same-package Main0 admission ->
whole-function portable lowering -> canonical finished root -> existing
module validation/publication. Includes that profile's raw body and raw
finish edges; excludes other Main profiles, If/Continue widening of the
physicalizer boundary, VM restoration, B3/R7 reopening, and whole shared
owner deletion. This is a finite design target, not an all-family census.

## Target selection

The selected fixture is the staged 29cb sibling of the landed 29ca profile
(both share the smoke-retirement SSOT):

```text
static box Main {
    main() {
        local i = 0
        local tmp = 0
        loop(i < 3) {
            i = i + 1
            tmp = 1
        }
        return i
    }
}
```

Expected result `3`. Both acceptance gates already exist:

```text
tools/smokes/v2/profiles/integration/joinir/generic_loop_in_body_step_release_adopt_vm.sh
tools/smokes/v2/profiles/integration/joinir/generic_loop_in_body_step_strict_shadow_vm.sh
```

The release gate requires exit `3` and rejects FlowBox tags; the strict gate
requires exit `3` plus a shadow/planner-first tag. The strict lane is known
baseline-broken (`StringHelpers.to_i64/1` retired static-call terminal,
reproduced on the row-E parent commit); its red is recorded, not repaired,
in this card.

The profile is the smallest in-cohort widening: integer locals only, a
zero-branch predicate loop, two body statements (`i = i + 1`, `tmp = 1`),
no `if`/`break`/`else`/calls/strings/`print`/nested loop. It reuses the
landed canonical chain end to end; the only new semantic surface is the
Facts/map/co-seal product and one extra selection arm.

## Open items before implementation

1. Static old-route classification: resolve which legacy route this shape
   takes through the existing Facts/registry (expected `LoopSimpleWhile`
   with an in-body-step observation; the Generic V0/V1 source issuer must
   lack that arm for this to be a migration target, not a re-implementation).
2. Zero-branch lowering verification: the landed lowerer is
   profile-specific; confirm the common physicalizer plus the existing
   normal-continuation path covers a branch-free loop body, or record the
   exact gap as `NoSafeSlice` instead of patching Layout.
3. `LOOP-PHYSICAL-ALWAYS-COVERAGE-I0` was deferred "unless the unchanged
   source actually requires it": if 29cb's always-executed body requires
   that row, name the dependency rather than smuggling coverage in.
4. Corpus TSV: update only this case's rows in
   `docs/development/current/main/design/fixtures/
   generic-loop-legacy-disposition-v1.tsv`; do not rerun the census.

## Acceptance and retained scope

- Unchanged fixture: expected result `3`; a bound `2` variant must return
  `2` and a zero-iteration variant `0`.
- Renamed locals preserve the same membership. Any `if`, `break`, `else`,
  `continue`, call, `print`, string, or non-unit step is outside this
  profile with an explicit preselection decline.
- Success contains one physical Main and one outer publication. Selected
  source cannot reach raw body/raw finish. Real source-to-MIR and selected
  executable result evidence are required before closeout, same as row E.

Shared raw body dispatch, other Main profiles, GenericV1/LoopCond/LoopTrue
consumers, registry/Composer/PlanLowerer and all nonselected callers
remain. This row does not authorize whole-symbol deletion or close the
wider M8/M9/M10b requirements.
