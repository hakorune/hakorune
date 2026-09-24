---
Status: landed__2026-09-24__wire_coverage_cohort
Task: SELFHOST-LOOP-M8D-LOOPCOND-PARITY-S7B4
Date: 2026-09-24
Parent: SELFHOST-LOOP-M8C-SCANS-PARITY-S7B3 (landed)
PreviousCard: selfhost-loop-m8c-scans-parity-s7b3-d0-2026-09-24.md
NextCard: frontier-pause__family_scheduler_reselection
Implementation permission: landed as designed; the wire-coverage cohort
contract and all non-claims are unchanged.
---

# SELFHOST-LOOP-M8D-LOOPCOND-PARITY-S7B4 — D0 wire-coverage cohort design

## Six-line brief

```text
Decision: one caller-zero wire-coverage slice for the M8D LoopCond
break/continue cohort — the fourth S7B row, back on
`LoopRecipeArtifactV1`. (1) a fifth `.hako` stdout entry in
`lang/src/mir/builder/loop_recipe/` that emits the canonical M8D
V1 artifact (provenance `loop_cond_break_continue_v1`) as one
compact JSON line; (2) the emission checked in as a fixture; (3)
the parity harness extended so the `.hako` bytes decode, verify,
and normalize equal to the artifact rebuilt through the landed
Rust producer's own issuer calls, plus a `normalize_semantic`
anchor against the real `produce_loop_cond_break_continue_recipe_v1`
product. Same cohort contract as S7B1/S7B2/S7B3: emission +
verification half of M9, not a Facts/RoutePolicy/JoinSig producer
port.
Source authority + canonical issuer: the Rust producer
`produce_loop_cond_break_continue_recipe_v1`
(`loop_recipe_contract/loop_cond_break_continue_producer.rs`) and
its `loop_cond_break_continue_recipe` issuer are the sole
authority for the canonical recipe shape; the V1 serde schema
(`schema.rs`) is the wire authority; `LoopRecipeVerifierV1` is the
sole verify owner; `LoopRecipeNormalizerV1` is the sole
decode/normalize owner; the resolver-issued
`VerifiedLoopRootSourceV1` carried by the typed source map +
`into_root_claim` is the sole wire source-claim issuer; the new
`.hako` entry is the sole `.hako` wire producer for this family.
Non-authority: `lang/src/compiler/mirbuilder/` compat surface;
`lang/src/selfhost/mir_builder/` scaffold; ASTNode/synthetic AST;
borrowed Facts/frame/demand; MirBuilder/CorePlan/ValueId/
BasicBlockId/Frag; callbacks/traits/lifetimes; route names as
recipe inputs; Program/AST JSON ingress on the `.hako` side;
retry/suffix/selector capability; any other existing producer id
as the artifact's provenance (false claim).
Fail-fast boundary: `.hako` bytes must `decode_and_verify` through
`LoopRecipeNormalizerV1` with zero rejection and normalize equal
to the Rust artifact under all three V1 normalizations; a
schema/field/wire mismatch, wrong `schema_version`, or provenance
drift is a typed failure — never a coerced decode or wildcard
comparison.
Smallest next slice: `loop_cond_break_continue_recipe` promoted
to `pub(super)` + `pub(super)` reuse of the producer-test issuer
chain helpers (`typed_map`, `schedule_with_winner`, `demand`) +
one `.hako` entry + one checked-in fixture + parity-harness M8D
arm + README/manifest/reference sync + focused tests. One commit.
Non-claims: caller-zero; no `.hako` producer, Facts/RoutePolicy/
JoinSig port, verifier, CFG/PHI, physical MIR, production caller,
hostbridge, input reading, or V2 artifact for this family; the
`loop_cond_break_continue_v1` producer id names the claimed
schema family, not a `.hako` production receipt and not a
route/selector input; no M9 parity claim (S7G); no M10/Row F
unblock; no legacy deletion.
```

## Row contract

Same M9 ladder position as S7B1–S7B3
(`generic-loop-source-to-portable-recipe-ssot.md:1154` names this
row `SELFHOST-LOOP-M8D-LOOPCOND-PARITY-S7B4`; it is the wire cohort
for the LoopCond exits producer the S6D row landed). The
wire-coverage cohort contract is inherited unchanged: the `.hako`
side emits the canonical artifact and Rust verifies both normalized
products; the Facts/RoutePolicy/JoinSig producer half stays
deferred until a `.hako` execution-mechanism row lands in its
owning lane.

## Why now (prerequisites satisfied)

- S7A + S7B1 + S7B2 + S7B3 landed caller-zero (`f6c1d71015`,
  `f4b1432929`, `e2a1e58eb8`, `9377b746df`): the subtree, transport,
  harness convention, and the wire-coverage cohort contract are
  proven on both V1 and V2 wires.
- The M8D Rust authority is landed:
  `produce_loop_cond_break_continue_recipe_v1`
  (`loop_cond_break_continue_producer.rs`) emits the canonical
  recipe (1 loop / 4 blocks / 9 items / 1 binding / 1 input /
  7 values / 1 carrier / 2 exits — `break` then `continue`) and the
  shared JoinSig elaborator proves the `If`'s else arm;
  `LoopRecipeProducerIdV1::LoopCondBreakContinueV1` already exists
  (wire key `loop_cond_break_continue_v1`) — no provenance
  vocabulary change is needed.
- The bounded source profile is fixed by the canonical fixture
  `loop_cond_function_for_test` (re-export of
  `loop_cond_break_continue_projection_tests::positive_function`):
  `local flag = 1; loop(flag < 2) { if flag == 1 { break }
  else { continue } }` — loop at body item index 1, so the wire
  source claim is `body_item(1)` on `function_body(0, 0)`.
- First wire family carrying a `continue` exit kind and an `If`
  item with a populated `else_block` — both already inside the V1
  serde vocabulary, so no schema change.

## Missing (the row's deliverable)

- A `.hako` wire entry for the M8D family — zero `.hako` code emits
  the `loop_cond_break_continue_v1` artifact today.
- The checked-in emission fixture
  (`fixtures/hako_loop_recipe_wire_m8d_v1.json`) and the harness
  arm.
- `pub(super)` visibility on `loop_cond_break_continue_recipe`
  (same precedent as `recurrence_recipe` / `break_recipe` /
  `build_recipe`) so the harness rebuilds the artifact through the
  producer's own recipe issuer rather than duplicating the shape.
- `pub(super)` reuse of the producer-test issuer chain
  (`loop_cond_break_continue_producer_tests.rs` helpers) so the
  demand/typed-map path is the producer's own chain, not a
  test-only shape.
- Subtree README + manifest + reference sync for the fifth entry.

## Design decisions (this card fixes them)

### 1. Artifact-rebuild chain — typed map is the source-claim owner

Unlike M8A/M8B (facts -> `bind_resolved_loop_root_v1`), the M8D
issuer chain carries the resolver-bound `VerifiedLoopRootSourceV1`
inside `VerifiedLoopCondBreakContinueTypedSourceMapV1`. The harness
rebuilds the canonical artifact through the producer's own issuer
calls:

```text
loop_cond_function_for_test() AST fixture
  -> VerifiedResolvedSourceUnitV1::resolve_function
  -> root_function_input -> root_body -> body_stmt(&body, 1)
  -> resolved_loop_source(loop_stmt.site())
  -> issue_loop_cond_break_continue_source_projection_v1
  -> issue_loop_cond_break_continue_typed_source_map_v1
  -> map.into_parts() -> (source_root, .., loop_condition,
       branch_condition, ..)
  -> loop_cond_break_continue_recipe(&loop_condition,
       &branch_condition)                       [pub(super)]
  -> LoopRecipeVerifierV1::verify
  -> source_root.into_root_claim(&verified_recipe)
  -> LoopRecipeArtifactV1::new(LoopCondBreakContinueV1, ..)
```

No `bind_resolved_loop_root_v1` call and no AST re-inspection: the
typed map is the sole owner that co-seals source root, conditions,
carrier, and frame.

### 2. Real-product anchor — the producer's own demand chain

The semantic anchor consumes the real producer product, built
through the producer's own demand issuer:

```text
typed_map() + schedule_with_winner(Some(LoopCondBreakContinue
cursor)) -> issue_loop_cond_break_continue_policy_demand_v1
  -> produce_loop_cond_break_continue_recipe_v1
  -> product.recipe() -> normalize_semantic
```

The schedule construction (`freeze_loop_route_schedule_v1` over the
19 canonical rows with `Candidate` evidence only at the LoopCond
cursor) is what the landed producer tests already build; the
helpers are promoted to `pub(super)` rather than re-implemented in
the harness. The schedule is policy evidence for the demand issuer
only — it is never an input to the recipe or the wire.

### 3. Bounded profile and wire values

The canonical recipe fixes: loop condition `Less` bound `2`
(`flag < 2`); branch condition `Equal` bound `1` (`flag == 1`);
binding `loop_cond_carrier` (I64); input value `0`; carrier `0`
entering at value `0`; exits `[break -> loop 0, continue -> loop 0]`;
`If` at item key `6` with `then_block` `2` and `else_block` `3`;
`Exit` items at keys `7` (exit `0`) and `8` (exit `1`). The `.hako`
entry emits exactly this in serde field order as one line.

### 4. Mechanism — no new decision

The D1 executable-subset boundary recorded in the S7A card stands:
`.hako` emits a fixed artifact; no input reading, method calls, or
Map/Array construction. The M9 `emit -> verify` half advances; the
`same producer` half stays deferred behind a named `.hako`
execution-mechanism row in its owning lane.

## Fail-fast boundary

- `decode_and_verify` through `LoopRecipeNormalizerV1` must accept
  the `.hako` bytes; all three normalizations must equal the
  producer-issuer artifact.
- Provenance drift (`producer_id` swapped to a foreign key) must be
  observable as normalized-artifact inequality.
- Wrong `schema_version` must be a typed `UnsupportedVersion`
  reject, not coercion.
- The emission must be byte-identical across runs and checked in;
  any drift regenerates the fixture explicitly.
- The `.hako` entry performs no input read, no hostbridge, no
  producer call, and emits no second artifact.

## Non-authority

As listed in the brief; additionally the parity harness itself is
test-only and never a production verifier, selector, or caller.

## Non-claims

- Caller-zero: no production caller, route, registry, or selection
  row is touched; `route_loop` wiring stays absent.
- No `.hako` Facts extractor, RoutePolicy/JoinSig port, verifier,
  CFG/PHI, physical MIR, or demand/schedule construction on the
  `.hako` side — the demand exists only in the Rust anchor.
- No input reading, hostbridge, Program/AST JSON ingress, retry,
  suffix state, or selector capability.
- No V2 artifact for this family (M8D is a V1 producer).
- No M9/S7G parity claim; no M10/M10b/Row F unblock; no legacy
  deletion; no corpus re-census; no language-semantics widening.

## Exit

When this card is accepted, `work_mode` moves to `fast` for the
bounded implementation slice named above (one wire-coverage slice,
one commit): `loop_cond_break_continue_recipe` visibility +
producer-test helper reuse + `.hako` M8D entry + checked-in
emission fixture + parity-harness extension + README/manifest/
reference sync + focused tests + doc closeout.

## Landed evidence (2026-09-24)

```text
lang/src/mir/builder/loop_recipe/emit_m8d_loopcond_wire.hako —
  single-file caller-zero entry; emits the canonical M8D
  LoopRecipeArtifactV1 (schema_version 1, provenance
  loop_cond_break_continue_v1, source path body_item(1); 9 items
  incl. two compares (less/equal), If at key 6 with else_block 3,
  Exit items at keys 7-8; 4 blocks; 7 values; 1 binding; 1 input;
  1 carrier; two exit rows — break then continue, both targeting
  loop 0) as one compact JSON line in serde field order — verified
  byte-identical to the Rust-issuer artifact
src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_m8d_v1.json —
  checked-in stdout emission (one line; byte-identical to a fresh
  ./target/debug/hakorune --backend vm run)
src/mir/loop_recipe_contract/loop_cond_break_continue_producer.rs —
  loop_cond_break_continue_recipe promoted to pub(super) (same
  precedent as recurrence_recipe / break_recipe / build_recipe)
src/mir/loop_recipe_contract/loop_cond_break_continue_producer_tests.rs —
  typed_map / target_cursor / schedule_with_winner / demand promoted
  to pub(super) so the harness reuses the producer's own issuer
  chain (resolver unit -> projection -> typed map -> policy demand)
  rather than re-implementing policy evidence
src/mir/loop_recipe_contract/wire_parity_tests.rs — 5 new M8D tests
  (31 wire-parity total): decode_and_verify + all three V1
  normalizations equal vs the artifact rebuilt through the
  producer's own issuer calls (typed_map into_parts ->
  VerifiedLoopRootSourceV1 + loop_cond_break_continue_recipe ->
  verify -> into_root_claim -> LoopCondBreakContinueV1 provenance);
  real produce_loop_cond_break_continue_recipe_v1 product
  normalize_semantic anchor via its own demand issuer; V1
  provenance/schema round-trip; continue-exit + else_block coverage
  asserts; determinism; foreign-provenance drift
lang/src/mir/hako_module.toml — exports
  builder.loop_recipe.emit_m8d_loopcond_wire
lang/src/mir/builder/loop_recipe/README.md — M8D entry row, fixture
  list, regeneration command
```

Gates: `cargo test --lib mir::loop_recipe_contract` 219/219 green;
`cargo test --lib loop_cond_break_continue` 20/20 green;
`cargo test --lib loop_route_policy` 91/91 green;
`bash tools/checks/hako_mirbuilder_no_hostbridge.sh` OK;
`bash tools/checks/current_state_pointer_guard.sh` OK.

Non-claims retained: caller-zero; no `.hako` producer, Facts/
RoutePolicy/JoinSig port, verifier, CFG/PHI, physical MIR,
production caller, hostbridge, input reading, demand/schedule
construction on the `.hako` side, or V2 artifact for this family;
`loop_cond_break_continue_v1` names the claimed schema family,
not a `.hako` production receipt or selector input; no M9 parity
claim (S7G); no Row F unblock; no legacy deletion.
