---
Status: landed__2026-09-24__d1_wire_subset_amendment
Task: SELFHOST-LOOP-PORTABLE-WIRE-S7A
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-ALL19-CLOSEOUT-S6G (landed)
PreviousCard: joinir-loop-m8-all19-closeout-s6g-d0-2026-09-23.md
NextCard: frontier-pause__family_scheduler_reselection
Implementation permission: landed under D1 amendment below; the semantic
contract (caller-zero `.hako` V1 wire + stdout entry + Rust parity
harness) is unchanged; only the file layout was revised to the current
executable `.hako` subset.
---

# SELFHOST-LOOP-PORTABLE-WIRE-S7A — D0 portable-wire design

## Six-line brief

```text
Decision: one caller-zero portable-wire slice — (1) a new `.hako` subtree `lang/src/mir/builder/loop_recipe/` owning the `LoopRecipeArtifactV1` JSON wire DTO and emitter; (2) one `.hako` entry that emits one fixed minimal V1 artifact to stdout; (3) a Rust test-only comparison harness that decodes + verifies + normalizes the `.hako` emission and asserts equality with the same artifact assembled by Rust. The wire roundtrip is proven before any `.hako` producer cohort lands in S7B.
Source authority + canonical issuer: `LoopRecipeArtifactV1` wire (`schema_version`, `provenance.producer_id`, `source_binding`, `recipe`) is the sole comparison surface — the Rust serde definition in `loop_recipe_contract/schema.rs` is the schema authority; `LoopRecipeNormalizerV1` (`decode_and_verify`, `normalize_artifact`, `normalize_semantic`, `normalize_source_bound`) is the sole Rust decode/verify/normalize owner; the new `.hako` emitter is the sole wire producer on the `.hako` side.
Non-authority: `lang/src/compiler/mirbuilder/` compat surface and its `recipe/` PortSig oracle boxes; `lang/src/selfhost/mir_builder/` scaffold; ASTNode or synthetic AST on the wire; borrowed Facts/frame; MirBuilder/CorePlan/ValueId/BasicBlockId/Frag; callbacks/traits/lifetimes; route names or first-mutation profiles as recipe inputs; retry/suffix/selector capability.
Fail-fast boundary: the `.hako`-emitted bytes must decode through `LoopRecipeNormalizerV1::decode_and_verify` with zero rejection and normalize byte-equal to the Rust-assembled artifact; a schema/field/wire mismatch is a typed test failure, never a coerced decode. Program(JSON v0) ingress is not re-shaped for the wire row.
Smallest next slice: `lang/src/mir/builder/loop_recipe/` subtree (wire DTO + emitter + entry) + `loop_recipe_contract` test-only wire-parity harness + subtree README + focused positive/negative tests.
Non-claims: caller-zero only; no `.hako` producer cohort, Facts/RoutePolicy/JoinSig elaboration, Program(JSON v0) producer input, verifier, CFG/PHI, physical MIR, or default authority; no production caller or `route_loop`; no hostbridge; no language-semantics widening; M9 parity claim belongs to S7G, not this row.
```

## Row contract

From `joinir-loop-selfhost-recipe-pipeline-ssot.md` (M9 section):

> `SELFHOST-LOOP-PORTABLE-RECIPE-PARITY0-S7` — Change: "Implement the
> same StructuralFacts/RoutePolicy/Recipe/JoinSig producer in `.hako`
> over the existing Program/AST JSON boundary." Ordered rows:
> `SELFHOST-LOOP-PORTABLE-WIRE-S7A` -> five stable
> `SELFHOST-LOOP-M8*-PARITY-S7B1..S7B5` producer rows ->
> `SELFHOST-LOOP-PORTABLE-ALL19-PARITY-S7G`. Each cohort remains one
> commit.

> Contract: "Rust and `.hako` share the data contract and verifier
> expectations, not host callbacks. The `.hako` side emits the owned
> recipe and Rust verifies both normalized products. Rust remains the
> thin allocation/emission terminal until the later SH lane."

> Done (M9 overall): "Route ID, prefix reasons, logical roles, JoinSig,
> verifier result, and normalized recipe match for all 19 fixtures;
> selfhost quick, representative identity, and no-hostbridge gates are
> green."

> Stop: "Do not widen language semantics or Rust source recognition to
> force parity. Do not claim `.hako` verifier, CFG/PHI, physical MIR, or
> default authority. The no-hostbridge claim covers the portable
> producer subtree, not unrelated compatibility providers elsewhere in
> the canonical builder."

And `generic-loop-source-to-portable-recipe-ssot.md` (row contract
table): "`.hako` portable wire, one stable producer cohort per row,
then all19 normalized parity | selfhost quick, identity, no-hostbridge;
no `.hako` physical/default claim".

Ordered ladder:

```text
S6A..S6E (landed) -> S6G (landed) <- M8 closed
  -> S7A <- this row -> S7B1..S7B5 -> S7G
  -> M10 seal series -> M10b -> R1/M11/M12 -> (SH1..SH5 reserved post-M12)
```

This is a Promote-class infrastructure row: it creates the data
contract plumbing the five `.hako` producer cohorts will ride on. It
emits no semantic product of its own — the wire artifact proves
transport + schema + verifier compatibility, not source admission.

## Why now (prerequisites satisfied)

- M8 coverage gate closed: S6G landed caller-zero — all 19 canonical
  routes carry a typed pre-effect decline or verified Recipe backing,
  `NoCandidate` is open on the S2 selector, and the 19-route parity
  receipt is extended (commits `b419b84c3d`, `630ab7a6f6`).
- The wire schema is already frozen and versioned:
  `LoopRecipeArtifactV1` = `{schema_version, provenance,
  source_binding, recipe}` (`loop_recipe_contract/schema.rs:15-20`);
  `LoopRecipeArtifactV2` exists separately for the S6C typed cohort.
- The Rust comparison owner exists and is exercised:
  `LoopRecipeNormalizerV1` (`loop_recipe_contract/normalize.rs`)
  decodes, verifies, and emits `normalize_artifact` /
  `normalize_semantic` / `normalize_source_bound` products.
- The `.hako` transport convention exists: Program(JSON v0) ingress
  and single-line stdout JSON emission are already proven by
  `lang/src/mir/builder/compat/emit_mir_json_v0.hako`; JSON
  cursor/frag utilities live in `lang/src/shared/json/`.
- The canonical `.hako` home is fixed: `lang/src/mir/builder/` is the
  native MirBuilder home; `lang/src/compiler/mirbuilder/` is a compat
  surface that new authority must not enter (its README: "New
  authority behavior belongs under `lang/src/mir/builder/`").
- The selfhost boundary vocabulary is fixed (pipeline SSOT
  544-586): the portable boundary must not contain ASTNode, borrowed
  Facts/frame borrows, MirBuilder/CorePlan/ValueId/BasicBlockId/Frag,
  callbacks/traits/lifetimes, or retry/suffix/selector capability.

## Missing (the row's deliverable)

- `.hako` wire DTO + JSON emitter for `LoopRecipeArtifactV1` — zero
  `.hako` code emits this schema today.
- `.hako` wire entry — the stdout emission path for the artifact.
- Rust wire-parity harness — `decode_and_verify` +
  `normalize_artifact`/`normalize_semantic`/`normalize_source_bound`
  are `pub(super)`/crate-internal and exercised only by in-module
  tests; no cross-language comparison harness exists.
- Subtree README + focused tests.

## Design decisions (this card fixes them)

### 1. Wire scope — V1 only at S7A

The wire is `LoopRecipeArtifactV1` (`schema_version = 1`). V2
(`LoopRecipeArtifactV2`, the S6C typed-call cohort wire) joins when the
S7B ScanWithInit parity cohort's own card names it; S7A does not
pre-wire V2. The `.hako` DTO mirrors the Rust serde field names
exactly — `schema_version`, `provenance { producer_id }`,
`source_binding { owner, loops }`, `recipe` — because byte-level
normalized comparison, not structural similarity, is the parity gate.

### 2. `.hako` home — canonical native subtree

New files live under `lang/src/mir/builder/loop_recipe/`:

```text
loop_recipe_wire_box.hako      — artifact DTO + field access
loop_recipe_wire_emit_box.hako — deterministic JSON emitter
emit_loop_recipe_wire.hako     — stdout entry (same convention as
                                 compat/emit_mir_json_v0.hako)
README.md                      — owner boundary + non-goals
```

The subtree must not import `lang/src/compiler/mirbuilder/**` (compat
surface) or `lang/src/selfhost/mir_builder/**` (scaffold). It may
reuse `lang/src/shared/json/` string utilities only.

### 3. Input boundary — none at S7A

The wire row consumes no Program(JSON v0). Producer-cohort ingress
(facts extraction over the Program/AST JSON boundary) is per-cohort
design in S7B1..S7B5, where each cohort's bounded source profile
decides which JSON fields it reads. S7A emits one fixed minimal
artifact assembled from `.hako` data construction — the smallest V1
recipe the verifier accepts — purely to prove the wire roundtrip. It
is explicitly not a producer and claims no source admission.

### 4. Emission transport — stdout single-line JSON

`emit_loop_recipe_wire.hako` prints the artifact as one JSON line to
stdout, matching the existing `.hako` emit convention. The Rust test
harness captures the bytes (file redirect or process stdout); no
production caller, env hook, or registry wiring is added.

### 5. Rust comparison harness — test-only, crate-internal

A new focused test module beside `loop_recipe_contract/normalize.rs`
(e.g. `wire_parity_tests.rs` or an `s7a` test file) that:

```text
Rust-built minimal artifact
  -> serde JSON (Rust wire form)
  -> [ .hako emitter produces the same bytes ]
  -> LoopRecipeNormalizerV1::decode_and_verify(.hako bytes)
  -> normalize_artifact / normalize_semantic / normalize_source_bound
  -> assert equality with the Rust-normalized products
```

The harness consumes `.hako` output captured by the test (fixture
file or `Command` spawn — whichever the repo's existing cross-language
test convention uses); it stays `#[cfg(test)]`-scoped and adds no
public API. Negative coverage: a `.hako` emission with a
malformed/unknown field or wrong schema_version is a typed decode
reject, not a coercion.

### 6. Parity granularity at S7A

Full `normalize_artifact` equality (all four wire fields) is the
primary gate — strongest proof the `.hako` wire is byte-compatible.
`normalize_semantic` and `normalize_source_bound` equality are also
asserted so the per-field failure location stays diagnosable.
`JoinSig` is not on the wire: Rust elaborates JoinSig from the
verified recipe on both sides, so JoinSig parity is derived, not
transported. `producer_id` provenance travels on the wire and must
round-trip (the `generic_residual_v1` and newer keys are already in
the roundtrip receipt).

### 7. Gate mapping

- selfhost quick → the existing quick selfhost smoke path used by the
  `.hako` builder suite; the wire row adds its focused Rust tests to
  the normal `cargo test` surface only — it does not require a new
  smoke profile.
- representative identity → `tools/selfhost_identity_check.sh`
  remains the identity gate; S7A adds no identity surface of its own.
- no-hostbridge → `tools/checks/hako_mirbuilder_no_hostbridge.sh`
  currently greps `lang/src/compiler/mirbuilder`; the new subtree is
  in scope for the same claim — extend the guard's grep roots to
  include `lang/src/mir/builder/loop_recipe/` (the Stop clause scopes
  the claim to the portable producer subtree).
- per generic SSOT, no per-row shell guard; the shared
  `mirbuilder_inplace_replacement_guard.sh` is extended only if new
  vocabulary lands.

### 8. S7A's own Done

```text
lang/src/mir/builder/loop_recipe/ subtree lands (DTO + emitter + entry)
.hako emits one fixed minimal LoopRecipeArtifactV1 to stdout
Rust test harness decodes + verifies + normalizes it
normalize_artifact equality holds vs the Rust-assembled artifact
negative wire shapes are typed rejects
subtree README + focused tests green; pointer guard green
```

One bounded implementation slice, one commit family. Producer cohorts
(S7B1..S7B5) remain separate cards.

## Fail-fast boundary

- `.hako` emission bytes decode through `decode_and_verify` with zero
  rejection; a malformed field, wrong `schema_version`, unknown wire
  key, or missing required field is a typed decode reject — never a
  coerced or defaulted decode.
- The `.hako` DTO contains no ASTNode, borrowed Facts, MIR physical
  IDs, callbacks, or selector capability; any such field is a
  contract error at the subtree boundary.
- The harness compares normalized products byte-for-byte; a
  difference fails the test loudly — no subset/wildcard comparison.
- No `Option`/skip/retry/fallback anywhere in the slice.

## Non-claims (stop conditions)

- Caller-zero only: no production caller, no `route_loop`/registry/
  handler connection, no production switch, no caller-zero-retirement
  claim from green tests.
- No `.hako` producer cohort, Facts/RoutePolicy/JoinSig elaboration,
  Program(JSON v0) producer input, verifier, CFG/PHI, physical MIR,
  or default authority — M9's producer parity belongs to S7B/S7G.
- No `LoopRecipeArtifactV2` wiring at this row.
- No language-semantics or Rust source-recognition widening to force
  parity (M9 Stop clause).
- No hostbridge import or host-callback dependency; the no-hostbridge
  claim is scoped to the new subtree.
- No Rust fallback from the `.hako` path, and no `.hako` path
  becoming a fallback for Rust — the wire row proves one direction
  only (`.hako` emits, Rust verifies).
- No M10/M10b/Row F unblock claim; no legacy deletion; no corpus
  re-census.
- Not complete selfhosting: M9 "proves only the portable `.hako`
  producer" (pipeline SSOT:582); SH1..SH5 lanes stay reserved
  post-M12.

## Exit

When this card is accepted, `work_mode` moves to `fast` for the
bounded implementation slice named above (one wire slice, one commit
family): `.hako` wire subtree + stdout entry + Rust wire-parity
harness + focused tests + subtree README + doc closeout.

## D1 amendment — executable `.hako` subset boundary (landed 2026-09-24)

Implementation found that the D0 premise "the `.hako` transport
convention exists and is proven by `emit_mir_json_v0.hako`" is stale on
current HEAD. Measured on `target/debug/hakorune` (2026-09-24) and
`hakorune-compat`:

- `me.*` calls, `OwnerBox.method(...)`, instance-box calls, and
  top-level `fn` calls all fail compilation with
  `[freeze:contract][static-call/legacy-fallback-retired]` or fail the
  VM with `[vm-reference/legacy-call/global-stopped]` /
  `[vm-reference/canonical-call]`.
- `%{}`/`[]` literals and `new MapBox()` lower to `NewBox`
  `IntrinsicMap`/`IntrinsicArray`, unimplemented in the VM.
- `env.get` is retired the same way, so the Phase-0 compat entry
  `lang/src/mir/builder/compat/emit_mir_json_v0.hako` itself no longer
  runs (`phase29bq_hako_mirbuilder_phase0_pin_vm.sh` is baseline red).
- The executable subset is `static box Main { main() }` with scalar /
  string locals, `+` concat, loop/if, print, return.

Revised layout (same semantic contract): the subtree keeps
`emit_loop_recipe_wire.hako` + `README.md`; the artifact is assembled
from named string-fragment locals in fixed serde order inside
`Main.main()`. The DTO-box/emitter-box split sketched in §2 is deferred
to the first row whose card names an executable mechanism for `.hako`
method calls — every S7B producer cohort that needs maps, arrays, or
method calls hits this same boundary and must name its mechanism rather
than assuming the v0 transport still runs.

The `.hako` emission was captured by running
`./target/debug/hakorune --backend vm lang/src/mir/builder/loop_recipe/emit_loop_recipe_wire.hako`
and is checked in at
`src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_v1.json`.

## Landed evidence (2026-09-24)

```text
lang/src/mir/builder/loop_recipe/emit_loop_recipe_wire.hako — single-file
  caller-zero entry; emits the fixed minimal LoopRecipeArtifactV1 as one
  compact JSON line (schema_version/provenance/source_binding/recipe in
  serde field order, tagged kinds in snake_case)
lang/src/mir/builder/loop_recipe/README.md — owner boundary, non-goals,
  executable-subset boundary, regeneration command
src/mir/loop_recipe_contract/fixtures/hako_loop_recipe_wire_v1.json —
  checked-in stdout emission (one line; structurally equal to
  fixtures/accum_direct_v1.json)
src/mir/loop_recipe_contract/wire_parity_tests.rs — 10 tests:
  decode_and_verify + normalize_artifact/normalize_semantic/
  normalize_source_bound equality vs a Rust-assembled artifact;
  single-line/V1/producer round-trip; deterministic normalization;
  typed rejects (wrong schema_version, unknown field, missing field,
  unknown producer_id, noncanonical binding order, truncated JSON)
tools/checks/hako_mirbuilder_no_hostbridge.sh — grep roots extended to
  lang/src/mir/builder/loop_recipe
```

Gates: `cargo test --lib mir::loop_recipe_contract` 198/198 green
(includes the 10 wire-parity tests);
`bash tools/checks/hako_mirbuilder_no_hostbridge.sh` OK;
`bash tools/checks/current_state_pointer_guard.sh` OK.

Non-claims retained: caller-zero only; no `.hako` producer cohort,
Facts, verifier, CFG/PHI, physical MIR, production routing, hostbridge,
or V2 wire; the wire fixture's `direct_accum_v1` provenance is the
claimed schema family, not a Rust DirectAccum production receipt; no M9
parity claim (S7G owns that).
