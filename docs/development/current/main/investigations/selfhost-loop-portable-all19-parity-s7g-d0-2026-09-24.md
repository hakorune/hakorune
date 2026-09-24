---
Status: landed__2026-09-24__all19_normalized_parity_closeout
Task: SELFHOST-LOOP-PORTABLE-ALL19-PARITY-S7G
Date: 2026-09-24
Parent: SELFHOST-LOOP-M8E-GENERIC-PARITY-S7B5 (landed)
PreviousCard: selfhost-loop-m8e-generic-parity-s7b5-d0-2026-09-24.md
NextCard: frontier-pause__family_scheduler_reselection
Implementation permission: landed with the D1 adjustment recorded
below; the closeout contract and all non-claims are unchanged.
---

# SELFHOST-LOOP-PORTABLE-ALL19-PARITY-S7G — D0 all19 normalized parity closeout design

## Decision

S7G is the closeout row of the S7 wire ladder: claim the strongest
legally reachable form of "all19 normalized parity" — for every one of
the 19 canonical routes, pin exactly one coverage class, and for every
attested recipe-backed route prove `.hako` wire emission + Rust-side
normalized parity against the real producer product.

Today the wire cohort covers 6 of the 8 attested-backed routes:
`AccumConstLoop` (S7A `direct_accum_v1` substrate emission) and the
five M8 cohorts (S7B1–S7B5). Two attested-backed routes remain unwired:
`NestedLoopMinimal` (`NestedPredicateV1`, cursor 11) and
`LoopTrueBreakContinue` (`LoopTrueBreakContinueV1`, cursor 12). Both
have real Rust producers and established issuer chains, so wire
coverage for them is legal and mechanical under the existing
caller-zero emitter pattern. Leaving them unwired would leave the
"all19" claim with two holes that no later row owns — S7G is the last
parity row before `GENERIC-M10B-DELETION-MANIFEST-S0`.

The bounded slice is therefore one closeout slice:

1. `.hako` wire coverage for the two unwired attested-backed routes,
   following the established caller-zero single-file emitter pattern
   (named string fragments, serde field order, one compact JSON line,
   checked-in fixture, byte-identical to the Rust-issuer artifact):
   - `emit_nested_predicate_wire.hako` -> canonical
     `LoopRecipeArtifactV1` with `nested_predicate_v1` provenance
     (golden `fixtures/nested_predicate_v1.json` already exists as the
     Rust-side canonical artifact; the bounded profile is
     `nested_function()` — `nested_loop_minimal` with root
     `loop(i < 3)` containing `local j; j = 0;` plus the child
     `loop(j < 3)` accumulation, first nested-parent/child wire
     surface: 2 loops, per-loop blocks).
   - `emit_loop_true_wire.hako` -> canonical `LoopRecipeArtifactV1`
     with `loop_true_break_continue_v1` provenance (bounded profile
     `positive_function()` — `loop(true)` with an `if` break/else
     continue branch; loop at `body_item(1)`; `if`/`exit` items plus
     break+continue exit rows, like M8D).
2. Parity arms in the wire harness for both routes, rebuilt through
   each producer's own issuer calls (not hand-assembled shapes), with
   `normalize_semantic` anchored on the real producer product:
   - Nested: `projection_for(nested_function())` ->
     `produce_nested_predicate_recipe_v1` (both already `pub(crate)`;
     zero visibility change) — artifact comparison against the
     checked-in golden and the `.hako` emission.
   - LoopTrue: `demand()` -> `VerifiedLoopTrueBreakContinuePolicyDemandV1`
     `into_parts` -> projection `into_parts` ->
     (`VerifiedLoopRootSourceV1`, shape, frame key) ->
     `loop_true_break_continue_recipe` (`pub(super)` promotion, same
     precedent as M8D/M8E) -> `verify` -> `into_root_claim` ->
     `LoopTrueBreakContinueV1` provenance; real
     `produce_loop_true_break_continue_recipe_v1` product as the
     semantic anchor via its own demand issuer (producer-test
     `demand` helper promoted to `pub(super)`).
3. Uniform live-producer anchor for `AccumConstLoop`: the S7A arm
   currently compares the `.hako` `direct_accum_v1` emission against
   a Rust-assembled literal plus the `accum_direct_v1.json` golden.
   Add `normalize_semantic` equality against
   `direct_accum_product_for_test()` (already `pub(crate)`) so every
   wire-covered route is anchored on its real producer product.
4. A new test-only all-route parity census module
   `src/mir/loop_recipe_contract/wire_route_parity_tests.rs`
   (a new file because `wire_parity_tests.rs` is already past the
   800-line source cap): for each route in
   `CANONICAL_LOOP_ROUTE_ORDER_V1`, pin exactly one coverage class —
   `WireBackedParity{fixture, producer_id}` for the eight
   attested-backed routes, or `TypedDeclined` for the eleven
   observation-only routes — and assert the census agrees with the
   sealed `ATTESTED_RECIPE_BACKED_V1` attestation table and the
   `producer_id_migration` `RECEIPTS` inventory. The census is a
   `#[cfg(test)]` aggregation over existing receipts; it issues no
   new `Verified*`/`Prepared*` semantic product.
5. Doc closeout: `lang/src/mir/builder/loop_recipe/README.md` (two
   new entry rows, fixture list, regeneration commands),
   `lang/src/mir/hako_module.toml` (two exports),
   `docs/reference/mir/loop-recipe-contract.md` (S7G receipt),
   `CURRENT_STATE.toml`, workstream row E. Heavy gate profile per
   the SSOT: S7G-class rows run release/quick gates plus the
   row-named parity gate.

## Source authority + canonical issuer

- Route inventory: `CANONICAL_LOOP_ROUTE_ORDER_V1` in
  `src/mir/loop_route_policy/schema.rs` (19 routes, canonical order).
- Per-route backing attestation: `ATTESTED_RECIPE_BACKED_V1` in
  `src/mir/loop_route_policy/all_route_observation.rs` (8 backed:
  cursors 0, 4, 7, 10, 11, 12, 13, 18; 11 declined).
- Migration inventory: `RECEIPTS` in
  `src/mir/loop_recipe_contract/producer_id_migration_tests.rs`
  (7 `portable_producer` + 1 `portable_v2_producer` + 11
  `legacy_only` — consistent with the attestation table).
- Nested issuer chain: `nested_predicate_producer_tests::nested_function`
  / `projection_for` -> `produce_nested_predicate_recipe_v1`
  (`src/mir/compiler/nested_predicate_producer.rs`), canonical
  artifact at `fixtures/nested_predicate_v1.json`.
- LoopTrue issuer chain: `loop_true_break_continue_producer_tests::demand`
  -> `produce_loop_true_break_continue_recipe_v1`
  (`src/mir/loop_recipe_contract/loop_true_break_continue_producer.rs`);
  recipe issuer `loop_true_break_continue_recipe` promoted to
  `pub(super)` for the harness rebuild arm.
- DirectAccum anchor: `direct_accum_product_for_test()` (already
  `pub(crate)`) in `direct_accum_producer_tests.rs`.

## Non-authority

- Route names/cursor ordinals are census keys only; they are never
  wire data, selector inputs, or provenance values.
- The `.hako` emitters are caller-zero string assemblers: no input
  reading, no Facts/RoutePolicy/JoinSig construction, no physical
  MIR, no verifier, no demand/schedule construction.
- The census is test-only aggregation; it does not mint a new
  `Verified*`/`Prepared*` product and is not a production receipt.

## Fail-fast boundary

- Exactly one coverage class per route; a route that is neither
  wire-backed nor typed-declined fails the census.
- A wire-backed route whose `.hako` emission fails
  `decode_and_verify`, diverges from the issuer-rebuilt artifact
  under `normalize_artifact`, or drifts from the real producer
  product under `normalize_semantic`/`normalize_source_bound`
  fails — no partial-credit class.
- Schema-version drift (V2 rows under V1 decode and vice versa)
  rejects; no coercive decode.

## Smallest next slice

One closeout commit: two `.hako` emitters + two checked-in fixtures +
two parity arms + DirectAccum live-producer anchor + all19 census
module + manifest/README/reference sync + focused gates + doc
closeout. Frontier pause follows for family-scheduler reselection.

## Deferred claim (explicit, not silent)

The "same producer" half of the M9 Change clause — `.hako` Facts
extraction + RoutePolicy + recipe issuance over real input — stays
deferred; it requires an executable `.hako` mechanism that the
Call/R7 family or selfhost-compiler lane owns. M9 Done's literal
route-ID / prefix-reason / logical-role / JoinSig parity fields are
also unreachable on the wire today: the artifact schema carries
`{schema_version, provenance.producer_id, source_binding, recipe}`
only, route names are forbidden as wire data, and JoinSig parity is
derived not transported. S7G therefore claims *normalized wire
parity for all 19 routes' coverage classes*, not literal M9 Done.
Reopen trigger: a named `.hako` execution-mechanism row landing
real-input producer ports.

## Rejected interpretations

- **Census-only closeout** (no new emitters): leaves the two
  attested-backed unwired routes as a named gap that no later row
  owns; weaker than the achievable claim at near-zero marginal cost.
- **Emitters for declined routes**: illegal — declined routes have
  no Recipe producer, so there is no canonical artifact to emit;
  synthesizing one would mint an unbacked receipt.
- **New sealed `Verified*` census product**: unnecessary — no
  consumer exists; test-only co-sealing of existing receipts is the
  honest shape (S6G needed products only because `NoCandidate`
  consumed them).
- **Literal M9 Done**: unreachable without the deferred producer
  half; claimed only via the explicit deferred record above.

## Non-claims

- Caller-zero only; no `.hako` producer/Facts/RoutePolicy/JoinSig
  port, verifier, CFG/PHI, physical MIR, production caller,
  hostbridge, input reading, demand/schedule construction, or
  route-name semantic input.
- No M9 Done claim; no production selection; no selector/retry/
  fallback; no Row F unblock; no legacy deletion; no coverage or
  language-semantics widening; no V2 emission for Nested/LoopTrue
  (V1 provenance variants already exist for both).
- The census adds no new semantic authority; it only pins the
  existing attestation/migration inventory against wire coverage.

## Exit

When this card is accepted, `work_mode` moves to `fast` for the
bounded closeout slice named above: two emitters + two fixtures +
two parity arms + DirectAccum anchor + all19 census + doc closeout,
one commit, then frontier pause.

## D1 implementation finding (role labels / older-era goldens)

The D0 card assumed the `nested_predicate_v1` and `accum_direct_v1`
goldens equal the live producers' products. They do not: both goldens
are older-era witnesses carrying *source-name* labels (`i`/`sum`/`j`)
and `body_item(0)` paths, while the live producers emit *role* labels
(`root_0`/`root_1`/`child_0`, `induction`/`accumulator`) and
`body_item(1)` paths for their bounded profiles. `normalize_semantic`
preserves labels, so a golden-vs-product equality claim fails.

Adjustment landed within the same contract: the `.hako` cohort
emissions carry the **live-producer artifacts** (role labels,
`body_item(1)`), rebuilt through each producer's own issuer chain in
the harness (`nested_recipe` promoted to `pub(crate)`;
`direct_accum_recipe` and `direct_accum_producer_tests::demand`
promoted to `pub(super)`). The S7A substrate emission stays pinned to
the older `accum_direct_v1` golden as the wire-transport witness —
its landed claim is transport, not producer parity — and is not in
the census; `AccumConstLoop` instead gets its own cohort fixture
(`hako_loop_recipe_wire_accum_direct_v1.json` +
`emit_direct_accum_wire.hako`, a third emitter beyond the two the
card estimated). The `nested_predicate_v1` golden remains a
decode-and-verify witness. This preserves every landed S7A claim
while giving all eight attested-backed routes uniform
live-producer-anchored parity.

## Landed evidence (2026-09-24)

```text
lang/src/mir/builder/loop_recipe/emit_direct_accum_wire.hako —
  live-producer AccumConstLoop artifact (direct_accum_v1,
  body_item(1), role labels induction/accumulator; 11 items,
  2 blocks, 11 values, 2 carriers)
lang/src/mir/builder/loop_recipe/emit_nested_predicate_wire.hako —
  live-producer NestedLoopMinimal artifact (nested_predicate_v1;
  2 loops, parent/child; role labels root_0/root_1/child_0;
  body_item(1) + loop_body_item(2) paths; 20 items, 4 blocks,
  inputs [0,3], 3 carriers)
lang/src/mir/builder/loop_recipe/emit_loop_true_wire.hako —
  live-producer LoopTrueBreakContinue artifact
  (loop_true_break_continue_v1; `always` condition; 3 blocks;
  if + break/continue exits; body_item(1))
fixtures/hako_loop_recipe_wire_{accum_direct,nested,loop_true}_v1.json —
  checked-in emissions, each byte-identical to a fresh
  ./target/debug/hakorune --backend vm run
src/mir/loop_recipe_contract/wire_route_parity_tests.rs (new,
  15 tests) — three parity arms (each: emission decodes+verifies
  and matches the issuer-rebuilt artifact under all three V1
  normalizations; rebuilt artifact's normalize_semantic equals
  the real producer product issued through its own demand/
  projection chain; single-line provenance/shape asserts), two
  witness pins (S7A substrate emission still equals the
  accum_direct golden; nested golden still decodes+verifies),
  and the all19 census: every canonical route pinned exactly
  once in canonical order as V1Parity/V2Parity (8 wire-backed)
  or TypedDeclined (11), cross-checked against
  ATTESTED_RECIPE_BACKED_V1 (now pub(crate), re-exported) and
  the migration RECEIPTS (now pub(crate))
src/mir/compiler/nested_predicate_producer.rs — nested_recipe
  promoted to pub(crate)
src/mir/loop_recipe_contract/{direct_accum,loop_true_break_continue}_
  producer{,_tests}.rs — recipe issuers / demand helpers promoted
  to pub(super) (same precedent as the S7B arms)
src/mir/compiler/module_registry.in.rs — re-export
  nested_projection_for_test alongside nested_function_for_p3_test
lang/src/mir/hako_module.toml — three new exports
lang/src/mir/builder/loop_recipe/README.md — S7G section,
  fixture list, regeneration commands, role-label boundary note
```

Gates: `cargo test --lib mir::loop_recipe_contract` 239/239 green
(15 S7G tests included); `cargo test --lib loop_route_policy`
91/91; `cargo test --lib producer_id_migration` 4/4;
`cargo test --release --lib mir::loop_recipe_contract` green
(S7G-class release gate);
`bash tools/checks/hako_mirbuilder_no_hostbridge.sh` OK;
`bash tools/checks/current_state_pointer_guard.sh` OK;
`git diff --check` clean. Baseline debt observed, not caused by
this slice: `dev_gate.sh quick` fails inside the naming-charter
guard on pre-existing `stage_a_route.rs` "Stage-A" wording (file
untouched by this change; `tools/bin/hako` exec bit also missing
in the worktree), and `mirbuilder_inplace_replacement_guard.sh`
fails on the clean tree expecting a removed
`recursive_child_lowering.rs`.

Non-claims retained: caller-zero; no `.hako` producer/Facts/
RoutePolicy/JoinSig port/verifier/CFG-PHI/physical MIR/production
caller/hostbridge/input reading; route names stay census keys,
never wire data; no new `Verified*`/`Prepared*` product (census is
test-only); no literal M9 Done claim (route-ID/prefix-reason/
JoinSig wire parity stays deferred behind a named `.hako`
execution-mechanism row); no production selection; no Row F
unblock; no legacy deletion.
