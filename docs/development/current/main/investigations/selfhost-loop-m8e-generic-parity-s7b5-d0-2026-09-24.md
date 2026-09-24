---
Status: landed__2026-09-24__wire_coverage_cohort
Task: SELFHOST-LOOP-M8E-GENERIC-PARITY-S7B5
Date: 2026-09-24
Parent: SELFHOST-LOOP-M8D-LOOPCOND-PARITY-S7B4 (landed)
PreviousCard: selfhost-loop-m8d-loopcond-parity-s7b4-d0-2026-09-24.md
NextCard: frontier-pause__family_scheduler_reselection
Implementation permission: landed as designed; the wire-coverage cohort
contract and all non-claims are unchanged.
---

# SELFHOST-LOOP-M8E-GENERIC-PARITY-S7B5 — D0 wire-coverage cohort design

## Six-line brief

```text
Decision: one caller-zero wire-coverage slice for the M8E Generic
residual cohort — the fifth and last S7B producer row, on
`LoopRecipeArtifactV1`. (1) a sixth `.hako` stdout entry in
`lang/src/mir/builder/loop_recipe/` that emits the canonical M8E
V1 artifact (provenance `generic_residual_v1`) as one compact JSON
line; (2) the emission checked in as a fixture; (3) the parity
harness extended so the `.hako` bytes decode, verify, and
normalize equal to the artifact rebuilt through the landed Rust
producer's own issuer calls, plus a `normalize_semantic` anchor
against the real `produce_generic_residual_recipe_v1` product.
Same cohort contract as S7B1–S7B4: emission + verification half
of M9, not a Facts/RoutePolicy/JoinSig producer port.
Source authority + canonical issuer: the Rust producer
`produce_generic_residual_recipe_v1`
(`loop_recipe_contract/generic_residual_producer.rs`) and its
`generic_residual_recipe` issuer are the sole authority for the
canonical recipe shape; the V1 serde schema (`schema.rs`) is the
wire authority; `LoopRecipeVerifierV1` is the sole verify owner;
`LoopRecipeNormalizerV1` is the sole decode/normalize owner; the
resolver-issued `VerifiedLoopRootSourceV1` carried by the typed
source map + `into_root_claim` is the sole wire source-claim
issuer; the new `.hako` entry is the sole `.hako` wire producer
for this family.
Non-authority: `lang/src/compiler/mirbuilder/` compat surface;
`lang/src/selfhost/mir_builder/` scaffold; ASTNode/synthetic AST;
borrowed Facts/frame/demand; MirBuilder/CorePlan/ValueId/
BasicBlockId/Frag; callbacks/traits/lifetimes; route names as
recipe inputs; Program/AST JSON ingress on the `.hako` side;
retry/suffix/selector capability; any other existing producer id
as the artifact's provenance (false claim); `GenericLoopV0` /
`GenericLoopV1` route names as wire data.
Fail-fast boundary: `.hako` bytes must `decode_and_verify` through
`LoopRecipeNormalizerV1` with zero rejection and normalize equal
to the Rust artifact under all three V1 normalizations; a
schema/field/wire mismatch, wrong `schema_version`, or provenance
drift is a typed failure — never a coerced decode or wildcard
comparison.
Smallest next slice: `generic_residual_recipe` promoted to
`pub(super)` + `pub(super)` reuse of the producer-test issuer
chain helpers (`typed_map`, `schedule_with_winner`, `demand`) +
one `.hako` entry + one checked-in fixture + parity-harness M8E
arm + README/manifest/reference sync + focused tests. One commit.
Non-claims: caller-zero; no `.hako` producer, Facts/RoutePolicy/
JoinSig port, verifier, CFG/PHI, physical MIR, production caller,
hostbridge, input reading, or V2 artifact for this family; the
`generic_residual_v1` producer id names the claimed schema
family, not a `.hako` production receipt and not a route/selector
input; no M9 parity claim (S7G); no M10/Row F unblock; no legacy
deletion; no Generic residual language-semantics widening.
```

## Row contract

Same M9 ladder position as S7B1–S7B4
(`generic-loop-source-to-portable-recipe-ssot.md:1155` names this
row `SELFHOST-LOOP-M8E-GENERIC-PARITY-S7B5`; it is the wire cohort
for the Generic residual producer). The wire-coverage cohort
contract is inherited unchanged: the `.hako` side emits the
canonical artifact and Rust verifies both normalized products; the
Facts/RoutePolicy/JoinSig producer half stays deferred until a
`.hako` execution-mechanism row lands in its owning lane.

## Why now (prerequisites satisfied)

- S7A + S7B1–S7B4 landed caller-zero (`f6c1d71015`, `f4b1432929`,
  `e2a1e58eb8`, `9377b746df`, `5be1808a3b`): the subtree, transport,
  harness convention, and the wire-coverage cohort contract are
  proven on both V1 and V2 wires.
- The M8E Rust authority is landed:
  `produce_generic_residual_recipe_v1`
  (`generic_residual_producer.rs`) emits the canonical recipe for
  the bounded profile (1 loop / 2 blocks / 8 items / 2 bindings /
  2 inputs / 9 values / 2 carriers / 0 exits) via
  `generic_residual_recipe` and its dense-key `RecipeEmitterV1`;
  `LoopRecipeProducerIdV1::GenericResidualV1` already exists (wire
  key `generic_residual_v1`) — no provenance vocabulary change.
- The bounded source profile is fixed by the canonical fixture
  `generic_residual_function_for_test` (re-export of
  `generic_residual_projection_tests::positive_function`):
  `generic_residual_projection(i, limit) { loop(i < limit)
  { local tmp = 0; i = i + 1 } }` — loop at body item index 0, so
  the wire source claim is `body_item(0)` on `function_body(0, 0)`.
- First wire family whose bound is a second carrier binding
  (`i < limit` — `limit` is a boundary slot, not a constant): the
  recipe carries two bindings, two inputs, and two carriers, and a
  body-local declaration (`tmp = 0`) that is a per-iteration SSA
  constant with no boundary write — new wire surface inside the
  existing V1 serde vocabulary.

## Missing (the row's deliverable)

- A `.hako` wire entry for the M8E family — zero `.hako` code emits
  the `generic_residual_v1` artifact today.
- The checked-in emission fixture
  (`fixtures/hako_loop_recipe_wire_m8e_v1.json`) and the harness
  arm.
- `pub(super)` visibility on `generic_residual_recipe` (same
  precedent as `recurrence_recipe` / `break_recipe` /
  `build_recipe` / `loop_cond_break_continue_recipe`) so the
  harness rebuilds the artifact through the producer's own recipe
  issuer.
- `pub(super)` reuse of the producer-test issuer chain
  (`generic_residual_producer_tests.rs` helpers) so the
  demand/typed-map path is the producer's own chain.
- Subtree README + manifest + reference sync for the sixth entry.

## Design decisions (this card fixes them)

### 1. Artifact-rebuild chain — same typed-map ownership as M8D

`VerifiedGenericResidualTypedSourceMapV1::into_parts` carries the
resolver-bound `VerifiedLoopRootSourceV1` plus the typed parts the
recipe issuer consumes. The harness rebuilds the canonical
artifact through the producer's own issuer calls:

```text
generic_residual_function_for_test() AST fixture
  -> VerifiedResolvedSourceUnitV1::resolve_function
  -> root_function_input -> root_body -> body_stmt(&body, 0)
  -> resolved_loop_source(loop_stmt.site())
  -> issue_generic_residual_source_projection_v1
  -> issue_generic_residual_typed_source_map_v1
  -> map.into_parts() -> (source_root, _projection, carrier,
       condition, body_rows, carrier_step, _frame)
  -> generic_residual_recipe(carrier, &condition, &body_rows,
       &carrier_step)                            [pub(super)]
  -> LoopRecipeVerifierV1::verify
  -> source_root.into_root_claim(&verified_recipe)
  -> LoopRecipeArtifactV1::new(GenericResidualV1, ..)
```

### 2. Real-product anchor — the producer's own demand chain

Same as S7B4: `typed_map()` + `schedule_with_winner(Some(
GenericLoopV1 cursor))` -> `issue_generic_residual_policy_demand_v1`
-> `produce_generic_residual_recipe_v1` -> `product.recipe()` ->
`normalize_semantic`. The producer-test helpers are promoted to
`pub(super)`; the schedule is policy evidence for the demand
issuer only and never a recipe or wire input.

### 3. Bounded profile and wire values

The canonical recipe fixes: two bindings (`generic_residual_carrier`
key 0, `generic_residual_bound` key 1 — `limit` is a distinct
boundary variable); inputs `[0, 1]`; two carriers (key 0 -> binding
0 entry 0; key 1 -> binding 1 entry 1); condition block items
`[0,1,2]` = read carrier -> v2, read bound -> v3, `less` compare ->
v4 (the predicate); body block items `[3,4,5,6,7]` = const `0` ->
v5 (`tmp` declaration, body-local SSA), read carrier -> v6, const
`1` -> v7 (step delta), `add` -> v8, write binding 0; no exits.
The `.hako` entry emits exactly this in serde field order as one
line.

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
`GenericLoopV0` and `GenericLoopV1` remain route names used only
inside the Rust demand issuer's schedule evidence — they do not
appear on the wire.

## Non-claims

- Caller-zero: no production caller, route, registry, or selection
  row is touched; `route_loop` wiring stays absent.
- No `.hako` Facts extractor, RoutePolicy/JoinSig port, verifier,
  CFG/PHI, physical MIR, or demand/schedule construction on the
  `.hako` side — the demand exists only in the Rust anchor.
- No input reading, hostbridge, Program/AST JSON ingress, retry,
  suffix state, or selector capability.
- No V2 artifact for this family (M8E is a V1 producer).
- No Generic residual coverage widening: the bounded profile stays
  the canonical `i < limit` + body-local declaration + `i = i + 1`
  shape; no new body-row vocabulary, no exit coverage, no corpus
  re-census.
- No M9/S7G parity claim; no M10/M10b/Row F unblock; no legacy
  deletion; no language-semantics widening.

## Exit

When this card is accepted, `work_mode` moves to `fast` for the
bounded implementation slice named above (one wire-coverage slice,
one commit): `generic_residual_recipe` visibility + producer-test
helper reuse + `.hako` M8E entry + checked-in emission fixture +
parity-harness extension + README/manifest/reference sync +
focused tests + doc closeout.

## Landed evidence (2026-09-24)

```text
lang/src/mir/builder/loop_recipe/emit_m8e_generic_wire.hako —
  single-file caller-zero entry; emits the canonical M8E
  LoopRecipeArtifactV1 (schema_version 1, provenance
  generic_residual_v1, source path body_item(0); 8 items incl.
  two boundary-binding reads in the condition block, a body-local
  const declaration, and the carrier step + write; 2 blocks;
  9 values; 2 bindings; inputs [0,1]; 2 carriers; no exits) as
  one compact JSON line in serde field order — verified
  byte-identical to the Rust-issuer artifact
src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_m8e_v1.json —
  checked-in stdout emission (one line; byte-identical to a fresh
  ./target/debug/hakorune --backend vm run)
src/mir/loop_recipe_contract/generic_residual_producer.rs —
  generic_residual_recipe promoted to pub(super) (same precedent
  as recurrence_recipe / break_recipe / build_recipe /
  loop_cond_break_continue_recipe)
src/mir/loop_recipe_contract/generic_residual_producer_tests.rs —
  typed_map / target_cursor / schedule_with_winner / demand
  promoted to pub(super) so the harness reuses the producer's
  own issuer chain (resolver unit -> projection -> typed map ->
  policy demand)
src/mir/loop_recipe_contract/wire_parity_tests.rs — 5 new M8E
  tests (36 wire-parity total): decode_and_verify + all three V1
  normalizations equal vs the artifact rebuilt through the
  producer's own issuer calls (typed_map into_parts ->
  VerifiedLoopRootSourceV1 + carrier/condition/body_rows/
  carrier_step -> generic_residual_recipe -> verify ->
  into_root_claim -> GenericResidualV1 provenance); real
  produce_generic_residual_recipe_v1 product normalize_semantic
  anchor via its own demand issuer; V1 provenance/schema
  round-trip; two-binding/two-carrier coverage asserts;
  determinism; foreign-provenance drift
lang/src/mir/hako_module.toml — exports
  builder.loop_recipe.emit_m8e_generic_wire
lang/src/mir/builder/loop_recipe/README.md — M8E entry row,
  fixture list, regeneration command
```

Gates: `cargo test --lib mir::loop_recipe_contract` 224/224 green;
`cargo test --lib generic_residual` 18/18 green;
`cargo test --lib loop_route_policy` 91/91 green;
`bash tools/checks/hako_mirbuilder_no_hostbridge.sh` OK;
`bash tools/checks/current_state_pointer_guard.sh` OK.

Non-claims retained: caller-zero; no `.hako` producer, Facts/
RoutePolicy/JoinSig port, verifier, CFG/PHI, physical MIR,
production caller, hostbridge, input reading, demand/schedule
construction on the `.hako` side, or V2 artifact for this family;
`generic_residual_v1` names the claimed schema family, not a
`.hako` production receipt or selector input; no Generic residual
coverage widening; no M9 parity claim (S7G); no Row F unblock; no
legacy deletion.
