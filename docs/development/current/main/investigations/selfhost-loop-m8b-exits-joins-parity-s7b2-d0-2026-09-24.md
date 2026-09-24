---
Status: design__2026-09-24__d0_wire_coverage_cohort
Task: SELFHOST-LOOP-M8B-EXITS-JOINS-PARITY-S7B2
Date: 2026-09-24
Parent: SELFHOST-LOOP-M8A-RECURRENCE-PARITY-S7B1 (landed)
PreviousCard: selfhost-loop-m8a-recurrence-parity-s7b1-d0-2026-09-24.md
NextCard: frontier-pause__family_scheduler_reselection
Implementation permission: false; name the owners to extend and fix the
bounded implementation slice only. No code, no fixture, no
route/caller change, no new semantic receipt from this card.
---

# SELFHOST-LOOP-M8B-EXITS-JOINS-PARITY-S7B2 — D0 wire-coverage cohort design

## Six-line brief

```text
Decision: one caller-zero wire-coverage slice for the M8B exits/joins
cohort — (1) a third `.hako` stdout entry in
`lang/src/mir/builder/loop_recipe/` that emits the canonical M8B
`LoopRecipeArtifactV1` (provenance `variable_accum_break_v1`) as one
compact JSON line; (2) the emission checked in as a fixture; (3) the
parity harness extended so the `.hako` bytes decode, verify, and
normalize equal to the artifact the landed Rust
`produce_variable_accum_break_recipe_v1` produces for the same bounded
source profile. Same cohort contract as S7B1: emission+verification
half of M9, not a Facts/RoutePolicy/JoinSig producer port.
Source authority + canonical issuer: the Rust producer
`produce_variable_accum_break_recipe_v1`
(`loop_recipe_contract/variable_accum_break_producer.rs`) is the sole
authority for the canonical artifact shape; the serde schema is the
wire authority; `LoopRecipeNormalizerV1` is the sole
decode/verify/normalize owner; the new `.hako` entry is the sole
`.hako` wire producer for this family.
Non-authority: `lang/src/compiler/mirbuilder/` compat surface;
`lang/src/selfhost/mir_builder/` scaffold; ASTNode/synthetic AST;
borrowed Facts/frame; MirBuilder/CorePlan/ValueId/BasicBlockId/Frag;
callbacks/traits/lifetimes; route names as recipe inputs; Program/AST
JSON ingress on the `.hako` side; retry/suffix/selector capability.
Fail-fast boundary: `.hako` bytes must `decode_and_verify` with zero
rejection and `normalize_artifact`/`normalize_semantic`/
`normalize_source_bound` equal to the Rust producer's artifact; a
schema/field/wire mismatch or provenance drift is a typed failure —
never a coerced decode or wildcard comparison.
Smallest next slice: one new `.hako` entry + one checked-in emission
fixture + parity-harness extension + subtree README/manifest sync +
focused positive/negative tests. One commit.
Non-claims: caller-zero; no `.hako` producer, Facts/RoutePolicy/JoinSig
port, verifier, CFG/PHI, physical MIR, production caller, hostbridge,
input reading, or V2 wire; the `variable_accum_break_v1` provenance
names the claimed schema family, not a `.hako` production receipt; no
M9 parity claim (S7G); no M10/Row F unblock; no legacy deletion.
```

## Row contract

Same M9 ladder position as S7B1 (`joinir-loop-selfhost-recipe-pipeline-ssot.md`
M9 section; `generic-loop-source-to-portable-recipe-ssot.md:1150-1157`
names this row `SELFHOST-LOOP-M8B-EXITS-JOINS-PARITY-S7B2`). The
wire-coverage cohort contract established by the S7B1 card applies
unchanged: each S7B row emits its family's canonical artifact from
`.hako` (the executable half of the M9 contract) and the producer half
stays deferred until a `.hako` execution-mechanism row lands in its
owning lane. The mechanism decision is not re-opened here; the S7B1
card's Option B selection and its deferred-claim clause are inherited.

## Why now (prerequisites satisfied)

- S7A + S7B1 landed caller-zero (`f6c1d71015`, `f4b1432929`): the
  subtree, transport, harness convention, and the wire-coverage cohort
  contract are all proven.
- The M8B Rust authority is landed: S6B's producer
  `produce_variable_accum_break_recipe_v1` emits the canonical artifact
  with `VariableAccumBreakV1` provenance; the all-route attestation
  binds `LoopBreakRecipe` to it
  (`loop_route_policy/all_route_observation.rs:67-70`).
- The bounded source profile is fixed by the S6B fixture:
  `main() { local sum = 0; local i = 0; loop(i < 10) { if(i == 5) {
  sum = sum + 10; break }; sum = sum + 1; i = i + 1 }; return sum }` —
  loop at root body item index 2, loop bound 10, branch bound 5,
  one `If` item, one `Exit` item, one `break` exit row
  (`src/mir/compiler/variable_accum_break_projection_tests.rs:44-103`).

## Missing (the row's deliverable)

- A `.hako` wire entry for the M8B family — zero `.hako` code emits
  `variable_accum_break_v1` provenance today.
- The checked-in emission fixture and the harness arm comparing it
  against the Rust producer's artifact — this is the first wire family
  carrying `If`/`Exit` item kinds and a non-empty `exits` array, so it
  extends wire coverage to exit/join vocabulary.
- Subtree README + manifest sync for the third entry.

## Design decisions (this card fixes them)

### 1. `.hako` entry — one file, S7A/S7B1 pattern

`lang/src/mir/builder/loop_recipe/emit_m8b_break_wire.hako`: a single
`static box Main { main() }` assembling the artifact from named
string-fragment locals in serde field order, one compact JSON line.
New wire vocabulary exercised: `LoopRecipeItemV1::If`
(`"kind":"if"`), `LoopRecipeItemV1::Exit` (`"kind":"exit"`), and the
`exits` array with `LoopExitKindV1::Break`. Manifest export
`builder.loop_recipe.emit_m8b_break_wire`.

### 2. Fixture + golden — producer-derived

Checked-in emission at
`src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_m8b_v1.json`.
The Rust comparison target is built through the same issuer calls the
producer makes: `issue_variable_accum_break_source_attempt_v1` facts
over the canonical fixture ->
`bind_resolved_loop_root_v1` -> `break_recipe(loop_bound, branch_bound)`
-> `VariableAccumBreakV1` provenance. `break_recipe` is promoted to
`pub(super)` so the harness reuses the producer's recipe issuer — same
precedent as `recurrence_recipe` in S7B1.

### 3. Parity granularity — all three normalizers + real-producer anchor

`normalize_artifact` primary; `normalize_semantic` and
`normalize_source_bound` asserted; decoded provenance must be
`VariableAccumBreakV1`; the real producer product's
`normalize_semantic` is asserted equal to the reconstructed artifact's
(same double-anchor as S7B1). Foreign-provenance drift stays a
normalized-inequality assertion.

## Fail-fast boundary

- `.hako` emission decodes through `decode_and_verify` with zero
  rejection; malformed/version/unknown-key/unknown-producer/missing
  field/non-canonical order/truncation stay typed rejects.
- `If`/`Exit` item kinds and the `break` exit row must round-trip
  exactly — wrong kind tags, missing `target_loop`, or non-canonical
  exit keys are typed failures.
- Provenance is exactly `variable_accum_break_v1`.
- No `Option`/skip/retry/fallback; no simulated input path.

## Non-claims (stop conditions)

- Caller-zero only; no production caller, routing, `route_loop`,
  registry, or production switch.
- No `.hako` producer, Facts/RoutePolicy/JoinSig port, verifier,
  CFG/PHI, physical MIR, or default authority — the deferred producer
  half of M9 is inherited from the S7B1 card unchanged.
- No Program/AST JSON ingress, no input reading, no hostbridge, no
  host callback.
- No `LoopRecipeArtifactV2` (S7B3's card owns V2).
- No M9/S7G parity claim; no M10/M10b/Row F unblock; no legacy
  deletion; no corpus re-census; no language-semantics widening.

## Exit

When this card is accepted, `work_mode` moves to `fast` for the bounded
implementation slice named above (one wire-coverage slice, one commit):
`.hako` M8B entry + checked-in emission fixture + parity-harness
extension + README/manifest sync + focused tests + doc closeout.
