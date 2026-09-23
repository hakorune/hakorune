---
Status: landed__2026-09-23
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

## Open items before implementation — resolved 2026-09-23

1. Static old-route classification — RESOLVED. Two lanes observed:
   - Production callable lane (`compile_normal` over the installed callable
     source, e.g. `--dump-mir`): the fixture **fails today** at
     `[callable-loop-handoff/incomplete-binding-coverage]`
     (`normal_callable_loop_handoff_validation.rs`): `tmp` is a
     `BodyRebind` with no `ConditionRead`/`BodyRead`, and the callable loop
     arm requires every rebound binding to be read. The callable source
     issuer lacks the write-only-rebind arm, so this is a migration target,
     not a re-implementation.
   - Compatibility/vm-keep lane (`--backend vm`): compiles and returns
     `3` through the retained raw path. Generic-facts static read:
     `LoopSimpleWhileFacts` rejects (`body.len() == 2` vs required `1`);
     `GenericLoopV0` skips (`body_writes_non_loop_vars` on `tmp = 1`);
     `GenericLoopV1` `validate_in_body_step_v1` admits `tmp = 1` as
     `is_simple_assignment` — consistent with the in-body-step route name.
2. Zero-branch lowering — RESOLVED safe to reuse. The recipe models the
   loop's natural backedge implicitly; `exits` only carry explicit
   Break/Continue/Return transfers (`LoopExitKindV1`). Zero declared exits
   is a valid recipe (`check_exits` only validates declared exits; the
   continue profile's normal body already falls through to the same
   backedge). `tmp` is admitted as a second `carrier_input`: the verifier
   requires no read for a carrier, and the seed preserves `tmp = 0`
   semantics for the zero-iteration case.
3. `LOOP-PHYSICAL-ALWAYS-COVERAGE-I0` — NOT REQUIRED. It names the
   `LoopConditionV1::Always` (`seal_condition_always`) coverage inside the
   V2/S6C neutral-envelope family. This fixture is a `Predicate` loop and
   its always-executed body is already covered by the shared body-segment
   to back-edge mechanism the landed profile exercises.
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

## Landed evidence — 2026-09-23

Implementation mirrors the row-E chain: profile Facts
(`main0_in_body_step_syntax_facts.rs`), source map + issue/ledger join,
Recipe co-seal (`LoopRecipeProducerIdV1::Main0InBodyStep`), semantic program
issuance, root selection, builder-free prepared operation, physical lowerer,
and a `SelectedMain0RootProductV1::InBodyStep` arm through the shared
canonical root admission (`complete_main0_root_draft_v1`, renamed
profile-neutral). The selected arm runs before the wrapper opens and seals
one canonical `main` draft under `FunctionDraftKeyV1::Main`.

Focused tests: 20 passed (Facts/map/co-seal rejects, semantic program,
selection positive/negative, lowerer canary, lifecycle positive,
post-Facts disagreement hard-reject). The lifecycle hard-reject covers a
source-map disagreement after admission (`main0-selection` error); a
decline-to-legacy loop near-miss was dropped because the retained raw loop
path overflows the test-harness stack — reproduced identically on parent
`44259c4b0d` (baseline debt, unrelated to this route).

Runtime acceptance (`--backend vm`, vm-reference build):

```text
phase29cb_generic_loop_in_body_step_min.hako   RC: 3  (expected 3)
bound-2 variant (/tmp/v_bound2.hako)           RC: 2  (expected 2)
zero-iteration variant (/tmp/v_zeroiter.hako)  RC: 0  (expected 0)
renamed-locals variant (/tmp/v_renamed.hako)   RC: 3  (expected 3)
tools/smokes/v2/.../generic_loop_in_body_step_release_adopt_vm.sh: PASS
```

`--dump-mir` on the selected fixture emits one `define i64 @main()`, a
loop-header PHI `[%1, bb0], [%8, bb2]` in bb1, `icmp Lt`, the in-body `Add`
in bb2 with its backedge to bb1, and a single `ret %3` in bb3 — the
callable lane that previously rejected this fixture at
`callable-loop-handoff/incomplete-binding-coverage` now compiles it
through the canonical route. The renamed variant emits the same canonical
shape (one `@main`, header PHI, one Return).

Caller-zero is unchanged from row E: selection happens inside the
`Installed` arm before the wrapper opens; `Compatibility` mode and the
`RawCompatibility`-fixed `lower_program_root_with_callable_port_v1` (zero
production callers, test seam only) cannot select. The fail-fast guard
rejects a product reaching any non-`VerifiedAppMain` terminal. Retained
users: non-selected profiles keep the wrapper/raw-finish path.

Red classification: the strict-shadow gate
(`generic_loop_in_body_step_strict_shadow_vm.sh`) fails with
`[freeze:contract][static-call/legacy-fallback-retired]
StringHelpers.to_i64/1` — the same baseline terminal recorded on row E and
the G0 premise-recheck card, unrelated to this route (the fixture contains
no `to_i64`; the strict derust lane emits it). The release-adopt gate
passes with `NYASH_BIN` pointed at the vm-reference debug build; its PASS
line prints `exit=9` but the checked condition is exit `3` (stale message
only).

Corpus manifest: the five `phase29cb`/in-body-step rows record
`accepted` (fixture + release-adopt rows) and `failed-before-loop`
(strict-shadow rows) observation states; P0 disposition/decision fields
stay inventory-only per the manifest contract. Seven `phase29ca` rows
were backfilled the same way (six row-E variant fixtures were missing
from the universe and are now registered; the min row records `accepted`).

Non-claims: no production route-loop selection beyond this bounded
profile, no GenericV1/LoopCond/LoopTrue migration, no raw-path deletion,
no strict-lane repair, and no M8/M9/M10b closure. Artifact admission
(`emit-exe`) remains unavailable for plain App Main roots — baseline
`root_completion` availability, unchanged.
