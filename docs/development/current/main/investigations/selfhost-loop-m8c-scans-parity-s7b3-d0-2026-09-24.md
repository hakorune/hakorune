---
Status: design__2026-09-24__d0_wire_coverage_cohort
Task: SELFHOST-LOOP-M8C-SCANS-PARITY-S7B3
Date: 2026-09-24
Parent: SELFHOST-LOOP-M8B-EXITS-JOINS-PARITY-S7B2 (landed)
PreviousCard: selfhost-loop-m8b-exits-joins-parity-s7b2-d0-2026-09-24.md
NextCard: frontier-pause__family_scheduler_reselection
Implementation permission: false; name the owners to extend and fix the
bounded implementation slice only. No code, no fixture, no
route/caller change, no new semantic receipt from this card.
---

# SELFHOST-LOOP-M8C-SCANS-PARITY-S7B3 — D0 wire-coverage cohort design

## Six-line brief

```text
Decision: one caller-zero wire-coverage slice for the M8C scans
cohort — the first `LoopRecipeArtifactV2` row. (1) a fourth `.hako`
stdout entry in `lang/src/mir/builder/loop_recipe/` that emits the
canonical M8C V2 artifact (provenance `scan_with_init_v2`) as one
compact JSON line; (2) the emission checked in as a fixture; (3) a
`LoopRecipeNormalizerV2` sibling named as the V2 decode/verify/
normalize owner; (4) the parity harness extended so the `.hako`
bytes decode, verify, and normalize equal to the artifact rebuilt
through the landed Rust producer's own issuer calls, plus a
`normalize_semantic` anchor against the real
`produce_s6c_scan_with_init_recipe_v2` product. Same cohort contract
as S7B1/S7B2: emission+verification half of M9, not a
Facts/RoutePolicy/JoinSig producer port.
Source authority + canonical issuer: the Rust producer
`produce_s6c_scan_with_init_recipe_v2`
(`loop_recipe_contract/s6c_scan_with_init.rs`) and its `build_recipe`
issuer are the sole authority for the canonical V2 recipe shape; the
V2 serde schema (`schema_v2.rs`) is the wire authority;
`LoopRecipeVerifierV2` is the sole verify owner;
`LoopRecipeNormalizerV2` (new sibling in `normalize.rs`) is the sole
decode/normalize owner; the resolver-owned
`bind_resolved_loop_root_v1` + `into_root_claim_v2` is the sole
wire source-claim issuer; the new `.hako` entry is the sole `.hako`
V2 wire producer for this family.
Non-authority: `lang/src/compiler/mirbuilder/` compat surface;
`lang/src/selfhost/mir_builder/` scaffold; ASTNode/synthetic AST;
borrowed Facts/frame; MirBuilder/CorePlan/ValueId/BasicBlockId/Frag;
callbacks/traits/lifetimes; route names as recipe inputs; Program/AST
JSON ingress on the `.hako` side; retry/suffix/selector capability;
`VerifiedS6CReturnSourceRecipeBindingV1` as wire data (internal
receipt, non-serde); `CallableSingleLoopV1` or any other existing
producer id as the artifact's provenance (false claim).
Fail-fast boundary: `.hako` bytes must `decode_and_verify` through
`LoopRecipeNormalizerV2` with zero rejection and normalize equal to
the Rust artifact under all three V2 normalizations; a schema/field/
wire mismatch, wrong `schema_version`, or provenance drift is a typed
failure — never a coerced decode or wildcard comparison.
Smallest next slice: one `LoopRecipeProducerIdV1` variant
(`ScanWithInitV2`) + `LoopRecipeNormalizerV2` + `build_recipe`
visibility + read-view `as_recipe` accessor + one `.hako` entry +
one checked-in fixture + parity-harness M8C arm + README/manifest/
reference sync + focused tests. One commit.
Non-claims: caller-zero; no `.hako` producer, Facts/RoutePolicy/
JoinSig port, verifier, CFG/PHI, physical MIR, production caller,
hostbridge, input reading, or V1 artifact for this family; the
`scan_with_init_v2` producer id names the claimed schema family,
not a `.hako` production receipt and not a route/selector input;
no M9 parity claim (S7G); no M10/Row F unblock; no legacy deletion.
```

## Row contract

Same M9 ladder position as S7B1/S7B2
(`generic-loop-source-to-portable-recipe-ssot.md:1150-1157` names this
row `SELFHOST-LOOP-M8C-SCANS-PARITY-S7B3`). This is the row the V2
wire was reserved for: the S7B1 card's non-claims state "`no V2 wire
… `LoopRecipeArtifactV2` (S7B3's card owns V2)" and the subtree README
carries the same reservation. The wire-coverage cohort contract is
inherited unchanged: the `.hako` side emits the canonical artifact
and Rust verifies both normalized products; the Facts/RoutePolicy/
JoinSig producer half stays deferred until a `.hako`
execution-mechanism row lands in its owning lane.

## Why now (prerequisites satisfied)

- S7A + S7B1 + S7B2 landed caller-zero (`f6c1d71015`, `f4b1432929`,
  `e2a1e58eb8`): the subtree, transport, harness convention, and the
  wire-coverage cohort contract are proven.
- The M8C Rust authority is landed: the V2 schema/verifier landed at
  `f15056f903`; `produce_s6c_scan_with_init_recipe_v2` emits the
  canonical V2 recipe (1 loop / 3 blocks / 15 items / 1 binding /
  3 inputs / 15 values / 1 carrier / 1 `return` exit);
  `LoopRecipeVerifierV2::verify_artifact` and
  `bind_verified_artifact` exist; the all-route attestation binds
  `ScanWithInit` to `LoopRouteRecipeBackingV1::ScanWithInitV2`
  (`loop_route_policy/all_route_observation.rs:77-80`).
- The bounded source profile is fixed by the S6C fixture
  `apps/tests/scan_with_init_typed_ok_min.hako`:
  `find_ok(s: StringBox, ch: StringBox): i64 { local i: i64 = 0;
  loop(i < s.length()) { if s.substring(i, i + 1) == ch { return i };
  i = i + 1 }; return -1 }` — loop at body item index 1.

## Missing (the row's deliverable)

- A `.hako` wire entry for the M8C family — zero `.hako` code emits a
  `LoopRecipeArtifactV2` today; the subtree README explicitly parks
  V2 for this row.
- A `LoopRecipeProducerIdV1` variant for this cohort — the wire
  provenance record requires a producer id and no `ScanWithInit`
  variant exists (the migration receipt pins `producer_id: None` +
  `portable_v2_producer` for this route).
- A V2 decode/normalize owner — `LoopRecipeNormalizerV1` is V1-only;
  no `decode_and_verify`/normalize path exists for V2.
- The checked-in emission fixture and the harness arm — first wire
  family carrying `call_slot`/`text_eq`/`const`-free `Text` values
  and a `return` exit kind.
- Subtree README + manifest + reference sync for the fourth entry.

## Design decisions (this card fixes them)

### 1. Provenance — one `LoopRecipeProducerIdV1` variant

Add `ScanWithInitV2` to `LoopRecipeProducerIdV1`
(`producer_id.rs`), serialized `"scan_with_init_v2"`. The provenance
record `LoopRecipeProvenanceV1` is shared by the V1 and V2 wire
schemas (`schema_v2.rs:21`), so the variant belongs in the shared
vocabulary. It names the claimed schema family — a diagnostic receipt
only; it must not be used to select a family, schedule a route, or
dispatch a physicalizer (existing enum contract). The migration
receipt row for `LoopRouteId::ScanWithInit` is updated from
`producer_id: None` to `Some(ScanWithInitV2)` while keeping
disposition `"portable_v2_producer"`; the
`LoopRouteRecipeBackingV1::ScanWithInitV2` backing vocabulary and its
doc comment are refreshed to note the provenance variant now exists.
Rejected: reusing `CallableSingleLoopV1` or any existing variant —
a false provenance claim; rejected: a parallel `ProducerIdV2` enum —
the wire already names one shared provenance record.

### 2. `LoopRecipeNormalizerV2` — named V2 decode/normalize owner

`normalize.rs` gains a `LoopRecipeNormalizerV2` sibling mirroring the
V1 contract exactly: `decode_and_verify(&str)` -> serde
`LoopRecipeArtifactV2` -> `LoopRecipeVerifierV2::verify_artifact`;
`normalize_artifact(&VerifiedLoopRecipeArtifactV2)`;
`normalize_semantic(&VerifiedLoopRecipeV2)`;
`normalize_source_bound(&VerifiedLoopRecipeArtifactV2)` through a
`LoopRecipeSourceBoundViewV2` (`schema_version: 2`, source_binding,
recipe). `LoopRecipeDecodeErrorV2` wraps `serde_json::Error` +
`LoopRecipeV2RejectReason`. No V1 path is widened; V1 continues to
reject V2 bytes on `schema_version`/`deny_unknown_fields`.

### 3. `.hako` entry — one file, S7A/S7B1/S7B2 pattern

`lang/src/mir/builder/loop_recipe/emit_m8c_scans_wire.hako`: a single
`static box Main { main() }` assembling the canonical artifact from
named string-fragment locals in serde field order, one compact JSON
line. New wire vocabulary exercised: `"schema_version":2`,
`call_slot` (`receiver`/`args`/`result`, nullable), `text_eq`,
`"text"` value class, `{"kind":"return","value":11}` exit kind.
Manifest export `builder.loop_recipe.emit_m8c_scans_wire`.

### 4. Rust comparison target — producer's own issuer calls

`build_recipe` is promoted to `pub(super)` (same precedent as
`recurrence_recipe`/`break_recipe`). The harness assembles the
comparison artifact as: `LoopRecipeVerifierV2::verify(build_recipe())`
-> `bind_resolved_loop_root_v1(resolved_loop_source)` +
`into_root_claim_v2(&verified_recipe)` ->
`LoopRecipeVerifierV2::bind_verified_artifact(ScanWithInitV2
provenance, source_binding, verified_recipe)` — the
`dynamic_full_body_recipe/mod.rs:185-196` issuance shape. The
`resolved_loop_source` token is obtained inside the same
`with_declaration_semantics` issuance chain the S6C facts helper
already runs (`ledger.resolved_loop_source(&loop_site).into_parts()`);
the existing `issue_facts` helper gains a sibling that returns the
resolved loop source alongside the facts. The real-producer anchor
asserts `normalize_semantic` equality between the rebuilt artifact's
recipe and the `produce_s6c_scan_with_init_recipe_v2` product's
recipe; the product read view
(`S6CVerifiedRecipeReadViewV2`) gains an `as_recipe()` accessor so the
anchor reads the producer's own recipe — no row duplication.

### 5. Wire source claim — `body_item(1)`, resolver ordinal

The bounded profile's loop is `find_ok` body item index 1. The wire
source claim is issued by the resolver projection
(`FunctionOriginV1` -> `function_body(cu_ordinal, 0)`,
`Body(1)` -> `body_item(1)`), not hand-written — same issuer path as
M8A/M8B, V2 claim variant. The `.hako` emission carries whatever
`compilation_unit_ordinal` the canonical issuance produces; the
fixture is generated by running the entry, and the harness arm uses
the same session ordinal, so the two agree deterministically.

## Fail-fast boundary

- `.hako` emission decodes through `LoopRecipeNormalizerV2::
  decode_and_verify` with zero rejection; malformed/version/
  unknown-key/unknown-producer/missing field/non-canonical
  order/truncation stay typed rejects (V2 rejects unknown fields and
  wrong `schema_version` before recipe use).
- `call_slot`/`text_eq`/`text`/`return` vocabulary must round-trip
  exactly — wrong kind tags, missing `args`, non-`Option` result
  coercion, or missing `target`/`value` payload are typed failures.
- Provenance is exactly `scan_with_init_v2`.
- No `Option`/skip/retry/fallback; no simulated input path.

## Non-claims (stop conditions)

- Caller-zero only; no production caller, routing, `route_loop`,
  registry, or production switch; the producer-id variant is
  diagnostic, not a selector.
- No `.hako` producer, Facts/RoutePolicy/JoinSig port, verifier,
  CFG/PHI, physical MIR, or default authority — the deferred producer
  half of M9 is inherited from the S7B1 card unchanged.
- No Program/AST JSON ingress, no input reading, no hostbridge, no
  host callback; the `.hako` entry cannot run `find_ok`/`s.length()`/
  `substring` — it emits the canonical artifact only.
- No V1 artifact for this family; no V2-to-V1 coercion; no
  `VerifiedS6CReturnSourceRecipeBindingV1` on the wire.
- No M9/S7G parity claim; no M10/M10b/Row F unblock; no legacy
  deletion; no corpus re-census; no language-semantics widening.

## Exit

When this card is accepted, `work_mode` moves to `fast` for the
bounded implementation slice named above (one wire-coverage slice,
one commit): `ScanWithInitV2` producer-id variant +
`LoopRecipeNormalizerV2` + `build_recipe`/`as_recipe` visibility +
`.hako` M8C entry + checked-in emission fixture + parity-harness
extension + README/manifest/reference sync + focused tests + doc
closeout.
