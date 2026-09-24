---
Status: landed__2026-09-24__wire_coverage_cohort
Task: SELFHOST-LOOP-M8A-RECURRENCE-PARITY-S7B1
Date: 2026-09-24
Parent: SELFHOST-LOOP-PORTABLE-WIRE-S7A (landed, D1 amendment)
PreviousCard: selfhost-loop-portable-wire-s7a-d0-2026-09-23.md
NextCard: frontier-pause__family_scheduler_reselection
Implementation permission: landed as designed; the wire-coverage cohort
contract and all non-claims are unchanged.
---

# SELFHOST-LOOP-M8A-RECURRENCE-PARITY-S7B1 — D0 wire-coverage cohort design

## Six-line brief

```text
Decision: one caller-zero wire-coverage slice for the M8A recurrence
cohort — (1) a second `.hako` stdout entry in the landed
`lang/src/mir/builder/loop_recipe/` subtree that emits the canonical
M8A `LoopRecipeArtifactV1` (provenance `variable_accum_recurrence_v1`)
as one compact JSON line; (2) the emission checked in as a fixture;
(3) the existing Rust parity harness extended so the `.hako` bytes are
decoded, verified, and normalized equal to the artifact the landed Rust
`produce_variable_accum_recurrence_recipe_v1` produces for the same
bounded source profile. This row delivers the emission+verification
half of the M9 contract for the M8A recipe family; it is not a
Facts/RoutePolicy/JoinSig producer port.
Source authority + canonical issuer: the Rust producer
`produce_variable_accum_recurrence_recipe_v1`
(`loop_recipe_contract/variable_accum_recurrence_producer.rs`) is the
sole authority for the canonical artifact shape; the Rust serde schema
(`loop_recipe_contract/schema.rs`) is the wire authority;
`LoopRecipeNormalizerV1` is the sole decode/verify/normalize owner; the
new `.hako` entry is the sole `.hako` wire producer for this family.
Non-authority: `lang/src/compiler/mirbuilder/` compat surface;
`lang/src/selfhost/mir_builder/` scaffold; ASTNode/synthetic AST;
borrowed Facts/frame; MirBuilder/CorePlan/ValueId/BasicBlockId/Frag;
callbacks/traits/lifetimes; route names as recipe inputs; Program/AST
JSON ingress on the `.hako` side (no executable input path exists);
retry/suffix/selector capability.
Fail-fast boundary: `.hako` bytes must `decode_and_verify` with zero
rejection and `normalize_artifact`/`normalize_semantic`/
`normalize_source_bound` equal to the Rust producer's artifact; a
schema/field/wire mismatch or provenance drift is a typed failure —
never a coerced decode or wildcard comparison.
Smallest next slice: one new `.hako` entry + one checked-in emission
fixture + parity-harness extension + subtree README/guard touch-ups +
focused positive/negative tests. One commit.
Non-claims: caller-zero; no `.hako` producer, Facts/RoutePolicy/JoinSig
port, verifier, CFG/PHI, physical MIR, production caller, hostbridge,
input reading, or V2 wire; the `variable_accum_recurrence_v1`
provenance names the claimed schema family, not a `.hako` production
receipt; no M9 parity claim (S7G); no M10/Row F unblock; no legacy
deletion.
```

## Row contract

From `joinir-loop-selfhost-recipe-pipeline-ssot.md` (M9 section): the
ordered ladder is `S7A -> SELFHOST-LOOP-M8*-PARITY-S7B1..S7B5 -> S7G`.
M9's Change clause: "Implement the same
StructuralFacts/RoutePolicy/Recipe/JoinSig producer in `.hako` over the
existing Program/AST JSON boundary." M9's Contract clause: "Rust and
`.hako` share the data contract and verifier expectations, not host
callbacks. The `.hako` side emits the owned recipe and Rust verifies
both normalized products." Stop: "Do not widen language semantics or
Rust source recognition to force parity. Do not claim `.hako` verifier,
CFG/PHI, physical MIR, or default authority."

`generic-loop-source-to-portable-recipe-ssot.md:1150-1157` names this
row `SELFHOST-LOOP-M8A-RECURRENCE-PARITY-S7B1`, "one producer cohort
per row".

## Why now (prerequisites satisfied)

- S7A landed caller-zero (`f6c1d71015`, `c539cb16f3`): the
  `lang/src/mir/builder/loop_recipe/` subtree exists, the wire
  transport `.hako` emit -> Rust `decode_and_verify` -> `normalize_*`
  is proven end-to-end, the no-hostbridge guard covers the subtree, and
  the parity harness convention (checked-in emission fixture +
  `wire_parity_tests.rs`) is established.
- The M8A Rust authority is landed: S6A's producer
  `produce_variable_accum_recurrence_recipe_v1` emits the canonical
  artifact with `VariableAccumRecurrenceV1` provenance, and the
  all-route attestation binds `LoopSimpleWhile` to that producer
  (`loop_route_policy/all_route_observation.rs:72-76`).
- The bounded source profile is fixed by the S6A fixture:
  `main() { local i = 0; local acc = 0; loop(i < 4) { acc = acc + i;
  i = i + 1 }; print(acc); return 0 }` — one predicate recurrence over
  two i64 carriers, bound 4, step +1
  (`src/mir/compiler/variable_accum_recurrence_projection_tests.rs:48-96`).

## Missing (the row's deliverable)

- A `.hako` wire entry for the M8A family — the S7A entry emits only
  the `direct_accum_v1` minimal artifact; zero `.hako` code emits
  `variable_accum_recurrence_v1` provenance today.
- The checked-in emission fixture and the harness arm that compares it
  against the Rust producer's artifact (S7A's harness compares against
  a Rust-assembled literal, not a producer product).
- Subtree README + guard sync for the second entry.

## The mechanism question (resolved in this card)

S7A's D1 amendment requires each S7B cohort to name its executable
mechanism instead of assuming the v0 transport runs. The scheduler
contract resolves missing internal facts inside the same card with
named competing choices and distinguishing evidence.

### Option A — input-driven `.hako` producer (the literal M9 Change)

A `.hako` program that reads Program/AST JSON, extracts Facts, applies
RoutePolicy, and emits the recipe — the "same producer" of the M9
Change clause. Rejected: no executable input path exists. `env.get`
and FileBox/string boxcalls are retired or unreachable on every
backend; Map/Array literals and all user method calls fail with
`[freeze:contract][static-call/legacy-fallback-retired]` /
`[vm-reference/legacy-call/global-stopped]` / NewBox
intrinsic-target-unsupported (S7A card D1 evidence). Restoring those
paths is the Call/R7 family lane, not an inventoried S7B row, and
would widen language semantics — M9's own Stop clause forbids that to
force parity.

### Option B — wire-coverage cohort (selected)

Each S7B row emits its family's canonical artifact from `.hako` — the
executable half of the M9 contract ("`.hako` emits the owned recipe and
Rust verifies both normalized products") — and defers the producer
half. For S7B1 the `.hako` entry emits the artifact identical in shape
and content to what `produce_variable_accum_recurrence_recipe_v1`
yields for the bounded profile above: `variable_accum_recurrence_v1`
provenance, the S6A source binding, and the 11-item/2-block/2-carrier
recurrence recipe (bound 4, `acc += i`, `i += 1`). The parity harness
asserts all three normalizations equal to the Rust producer's artifact
— stronger than S7A's Rust-assembled literal, since the comparison
target is the real producer product.

This is a bounded amendment of the cohort contract: S7B rows deliver
portable wire coverage per recipe family, not producer ports. It is
honest progress — the `.hako`-side emission and Rust-side verification
of every recipe family is required by M9 Done regardless of how Facts
extraction is later implemented.

### Option C — family-local NoSafeSlice on S7B

Defer the whole S7B series until a `.hako` execution-mechanism row
exists. Rejected: it halts the authorized ladder for the executable
half of the contract as well as the blocked half, and no inventoried
mechanism row exists to wait on — the deferral trigger would be
unbounded. Per the scheduler, a family-local stop is correct only when
no executable candidate remains; Option B is executable today.

### Deferred claim (named, not dropped)

The "same producer" half of the M9 Change clause — `.hako` Facts
extraction + RoutePolicy + recipe issuance over real input — stays
deferred for every S7B row until an executable `.hako` mechanism
(method calls + Map/Array or an equivalent substrate) is landed by the
lane that owns it (Call/R7 family or the selfhost-compiler lane). That
deferred claim reopens only when such a row lands; this card neither
invents the mechanism row nor claims its timeline. S7G's card must
re-examine whether all19 parity can be claimed from wire coverage plus
Rust-side producer products alone, or whether it requires the deferred
producer half.

## Design decisions (this card fixes them)

### 1. `.hako` entry — one file, S7A pattern

`lang/src/mir/builder/loop_recipe/emit_m8a_recurrence_wire.hako`: a
single `static box Main { main() }` that assembles the artifact from
named string-fragment locals in serde field order and prints one
compact JSON line — the D1 boundary pattern, no imports, no method
calls, no maps/arrays. The subtree README gains the entry row; the
module manifest gains `builder.loop_recipe.emit_m8a_recurrence_wire`.

### 2. Fixture + golden — producer-derived, not hand-written

The checked-in emission lands at
`src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_m8a_v1.json`.
The Rust comparison target is the artifact produced by
`produce_variable_accum_recurrence_recipe_v1` for the canonical bounded
profile — the harness builds that artifact through the existing
producer path (test-side reconstruction via the same issuer calls the
producer makes: `recurrence_recipe(4, 1)` shape +
`bind_resolved_loop_root_v1` source binding + `VariableAccumRecurrenceV1`
provenance) rather than duplicating recipe rows by hand.

### 3. Parity granularity — all three normalizers, same as S7A

`normalize_artifact` is the primary gate; `normalize_semantic` and
`normalize_source_bound` are asserted for diagnosability. The harness
also asserts the decoded provenance is `VariableAccumRecurrenceV1` —
provenance must round-trip, and a `direct_accum_v1` drift is a typed
failure. Typed rejects from S7A's harness (wrong version, unknown
field/key order/producer, missing field, truncation) apply unchanged.

### 4. No input, no producer claim

The entry emits one fixed artifact; it reads nothing and is not a
producer. The card's amendment scope is wire coverage only: no Facts
shape, RoutePolicy vocabulary, or JoinSig payload is claimed on the
`.hako` side — Rust still derives JoinSig from the verified recipe.

## Fail-fast boundary

- `.hako` emission decodes through `decode_and_verify` with zero
  rejection; malformed/version/unknown-key/unknown-producer/missing
  field/non-canonical order/truncation stay typed rejects.
- Normalized products compare byte-for-byte in all three
  normalizations — no subset/wildcard comparison.
- Provenance is exactly `variable_accum_recurrence_v1`; any other
  producer id or a missing provenance is a typed failure.
- No `Option`/skip/retry/fallback; no `.hako` input path is simulated
  by string-flag dispatch (that would be trial-selection, not a
  producer).

## Non-claims (stop conditions)

- Caller-zero only; no production caller, routing, `route_loop`,
  registry, or production switch.
- No `.hako` producer, Facts/RoutePolicy/JoinSig port, verifier,
  CFG/PHI, physical MIR, or default authority — the deferred producer
  half is named above, not silently dropped.
- No Program/AST JSON ingress, no input reading, no hostbridge, no
  host callback.
- No `LoopRecipeArtifactV2` (S7B3's card owns V2).
- No M9/S7G parity claim; no M10/M10b/Row F unblock; no legacy
  deletion; no corpus re-census; no language-semantics widening.

## Exit

When this card is accepted, `work_mode` moves to `fast` for the bounded
implementation slice named above (one wire-coverage slice, one commit):
`.hako` M8A entry + checked-in emission fixture + parity-harness
extension + README/manifest sync + focused tests + doc closeout.

## Landed evidence (2026-09-24)

```text
lang/src/mir/builder/loop_recipe/emit_m8a_recurrence_wire.hako —
  single-file caller-zero entry; emits the canonical M8A
  LoopRecipeArtifactV1 (provenance variable_accum_recurrence_v1,
  source path body_item(2), bound 4, acc += i, i += 1) as one compact
  JSON line in serde field order
src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_m8a_v1.json —
  checked-in stdout emission (one line)
src/mir/loop_recipe_contract/wire_parity_tests.rs — 5 new M8A tests
  (15 total): decode_and_verify + all three normalizations equal vs the
  artifact assembled through the real producer's issuer calls
  (resolver facts -> bind_resolved_loop_root_v1 ->
  recurrence_recipe(bound, delta) -> VariableAccumRecurrenceV1
  provenance); reconstructed artifact matches the real
  produce_variable_accum_recurrence_recipe_v1 product under
  normalize_semantic; provenance round-trip; deterministic
  normalization; foreign-provenance drift assert_ne
src/mir/loop_recipe_contract/variable_accum_recurrence_producer.rs —
  recurrence_recipe promoted to pub(super) so the harness reuses the
  producer's own recipe issuer (no row duplication)
lang/src/mir/hako_module.toml — exports
  builder.loop_recipe.emit_m8a_recurrence_wire
lang/src/mir/builder/loop_recipe/README.md — M8A entry row, deferred
  producer half, regeneration commands for both fixtures
```

Gates: `cargo test --lib mir::loop_recipe_contract` 203/203 green;
`cargo test --lib variable_accum_recurrence` 9/9 green;
`bash tools/checks/hako_mirbuilder_no_hostbridge.sh` OK;
`bash tools/checks/current_state_pointer_guard.sh` OK.

Non-claims retained: caller-zero; no `.hako` producer, Facts/
RoutePolicy/JoinSig port, verifier, CFG/PHI, physical MIR, production
caller, hostbridge, input reading, or V2 wire; the
`variable_accum_recurrence_v1` provenance names the claimed schema
family, not a `.hako` production receipt; no M9 parity claim (S7G); no
Row F unblock; no legacy deletion.
