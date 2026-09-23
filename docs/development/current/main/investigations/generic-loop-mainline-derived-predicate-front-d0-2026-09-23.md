---
Status: landed__2026-09-23
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

## Open items before implementation — resolved 2026-09-23

1. Static old-route classification — RESOLVED 2026-09-23. Two lanes
   observed:
   - Production callable lane (`--dump-mir` / `compile_normal`): the
     fixture **compiles today** through the retained legacy raw path,
     producing `define void @main()` with a loop-header PHI, the `j + m`
     `Add`, `icmp Le`, the body backedge, and `ret void` — the declared
     `return j` is dropped. Unlike the in-body-step fixture there is no
     handoff reject; the legacy physicalizer lowers the loop shape but
     cannot publish the value-returning Main.
   - Compatibility/vm-keep lane (`--backend vm`): runs and returns `4`
     through the retained raw path. The fixture comment's GenericLoopV0
     claim matches the add-expression condition; the Generic V0/V1 source
     issuer lacks this exact arm on the canonical route.
2. Condition-read relation — RESOLVED. The recipe draft composes
   `push_binary_i64(Add)` + `push_compare_i64(LessEqual)` in the one
   condition block; the shared segment dispatcher emits both
   block-agnostically, and the new sibling
   `validate_main0_derived_condition_read_relation_v1` traces
   `compare.left` through the `Add`'s two read inputs: exactly one must
   target the binding the body writes (the carrier), the other is the
   read-only operand.
3. Numeric contract — RESOLVED. Bounded contract recorded below in the
   landed evidence; no value-dependent membership was introduced.
4. Corpus TSV — RESOLVED. The base row's `observation_state` moved to
   `accepted`; the three new variant fixtures were added as
   fixture-inventory rows with the same observation state. All other P0
   fields stay inventory-only.

## Landed evidence — 2026-09-23

### Implementation path

Third arm in the same canonical root funnel, identical chain to the two
landed profiles:

- `compiler/main0_derived_predicate_syntax_facts.rs` — AST-free bounded
  observer; admits exactly `[Local, Local, Local, Loop, Return]` with
  `<add-binary> <= <var>` predicate and one carrier step statement.
- `compiler/main0_derived_predicate_source_map{,_issue}.rs` — 13-role
  join; assigns carrier (the Add LHS), operand (Add RHS, read-only), and
  bound (compare RHS) to three *distinct* declared locals, enforces
  `Add`/`LessEqual`, carrier-only rebind, carrier tail read, and the
  residual boundary (no calls/exits/foreign refs).
- `compiler/main0_derived_predicate_recipe_coseal.rs` — one co-sealed
  admission: Recipe draft (3 carriers, 2 blocks, 9 items, 0 declared
  exits), verifier, JoinSig, source-bound Core, initialized inputs,
  operation evidence, continuation contract, tail and derived-add
  control receipts. Producer id `Main0DerivedPredicateV1`.
- `compiler/main0_derived_predicate_semantic_program.rs` +
  `main0_derived_predicate_root_selection.rs` — one-shot semantic
  program + pre-wrapper observation; `Unselected` on facts decline
  without consuming the loan; post-admission disagreement hard-rejects.
- `builder/normal_main0_derived_predicate_prepared_operation.rs` —
  Builder-free ingress; verifies owner/origin/kind/loop-site/frame/
  scope and issues the full operation demand.
- `builder/resolved_lowering/loop_recipe_physicalizer/
  main0_derived_predicate_lowerer.rs` — physical consumer; derived
  condition-read sibling validator, profile close counts `(9,4,4,1)`,
  single tail Completion.
- Wiring: `SelectedMain0RootProductV1::DerivedPredicate` third arm in
  `program_root_lowering/main0_root_route.rs` (order: continue ->
  in-body-step -> derived-predicate -> decline), dispatch arm in
  `decls.rs`, raw-port default reject in `module_lifecycle.rs`, adapter
  method + `main0_root::lower_app_main0_derived_predicate_root_v1`
  canonical handoff, `complete_main0_root_draft_v1` shared admission.

### Focused tests — 22 PASS

`cargo test --features vm-reference --lib main0_derived_predicate`:
16 co-seal/facts/map (positive co-seal recipe shape, product survives
unit drop, 7 facts declines, 6 map rejects, foreign ledger, missing-row
co-seal), 2 root-selection (select + loan preservation, decline + loan
preservation), 1 lowerer canary (two Adds, header-PHI backedge, single
value Return), 3 lifecycle (canonical root publishes one `main` +
entry point + one Return + PHI; bound-2 variant stays canonical;
post-facts tail/operand disagreement hard-rejects with
`main0-selection`).

### Runtime and MIR evidence

- `apps/tests/generic_loop_carrier_type_v0_numeric_min.hako` —
  `--backend vm` (vm-reference debug binary): **RC 4**.
- Variants: `n = 2` bound variant RC 3; `m = 9` zero-iteration RC 0;
  renamed locals RC 4.
- New release-adopt gate
  `tools/smokes/v2/profiles/integration/joinir/
  generic_loop_carrier_type_release_adopt_vm.sh`: PASS (exit=4, no
  FlowBox tags).
- `--dump-mir` witness: `define i64 @main() effects(0x0010)`; bb0
  seeds three locals; bb1 header carries three PHIs, `%6 = %4 Add %5`
  (derived predicate), `icmp Le %6, %7`, conditional branch; bb2 body
  PHIs + `%11 = %9 Add %10` step, backedge to bb1; bb3 single
  `ret %4`. One function, one Return, one entry point.
- Raw reachability zero for the selected source: the sole production
  funnel `lower_prepared_program_root_with_callable_mode_v1` selects
  before wrapper opening; the canonical adapter never opens the raw
  wrapper, and the raw/compatibility port default rejects the
  `DerivedPredicate` product
  (`[mir/main0-derived-predicate-root/raw-port]`).

### Baseline reds — recorded, not repaired

- `generic_loop_carrier_type_strict_shadow_vm.sh`: fails at the known
  baseline `[freeze:contract][static-call/legacy-fallback-retired]
  owner=StringHelpers method=to_i64 arity=1` — the same strict-lane
  derust debt reproduced on the row-E/F parent commits; unrelated to
  this route.
- `tools/checks/mirbuilder_inplace_replacement_guard.sh`: fails on
  `raw Loop callers must share one JoinIR route/freeze owner` —
  `lower_loop_or_freeze_v1` moved from `recursive_child_lowering.rs`
  into `raw_loop_child_entry.rs` by `da4fbf7a89` (pre-cohort refactor)
  and the 3-file count check was never updated. Reproduced on the
  parent commit `f803b2bf7e` with this slice's changes stashed.

### Bounded numeric contract

The i64 lane parity fact holds for all admitted operands: dynamic
Integer `+` and physical i64 `Add` coincide on this profile's integer
locals; membership carries no literal values (initializer and delta
values are not membership). No new physical primitive was added —
`LoopBinaryI64OpV1::Add` and `LoopCompareI64OpV1::LessEqual` were
already in the shared vocabulary.

### Non-claims retained

No all-family Loop migration, no GenericV1/LoopCond/LoopTrue consumer
migration, no `route_loop` production switch, no registry/Composer/
PlanLowerer migration, no shared raw-body dispatch or retained
wrapper-symbol deletion, no strict-lane `to_i64` repair, no guard
re-baseline for the moved loop entry, no M8/M9/M10b/S6E closure, no
artifact/LLVM/AOT acceptance.

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
