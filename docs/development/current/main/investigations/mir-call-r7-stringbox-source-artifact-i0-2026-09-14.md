Status: landed__bounded_parser_contract_i0__2026-09-14
Task: MIR-CALL-R7-STRINGBOX-SOURCE-ARTIFACT-I0
Date: 2026-09-14
Priority: issue the bounded StringBox source artifact and connect one typed handoff
Parent: mir-call-compatibility-retire-r7-d0-2026-09-11.md
NextCard: MIR-CALL-R7-STRINGBOX-CALLER-SWITCH-D0
Implementation permission: true for this parser/contract I0 only; no Hako caller switch
---

# StringBox source artifact and module handoff I0

## Six-line brief

```text
Decision: implement one source-owned artifact for the exact direct/New StringBox length|size cohort and validate it through the existing Program-v0 bridge.
Source authority + canonical issuer: SourceProgramJsonAuthority::for_source in the Rust Stage1 AST parse/lower transaction, co-sealing Program-v0, Stage1StringBoxSourceProductV1, and Stage1ProgramJsonCrosswalkV1.
Non-authority: Hako first-string scans, method names without exact receiver/arity, JSON/MIR index guesses, legacy boxcall/call spelling, MIR type inference, fallback, retry, and a second semantic issuer.
Fail-fast boundary: before Stage1FinalizedMirModule publication, reject missing/duplicate/unclassified source relations, product/crosswalk disagreement, and zero/multiple/unconsumed or shape-mismatched Call anchors.
Smallest next slice: add the typed source artifact, enrich the handoff, run the existing bridge, and admit exactly one lowering-issued anchor without rewriting the physical MIR.
Non-claims: no Hako registry/fallback cutover, no selected old-edge deletion, no indexOf migration, no shared schema retirement, no native Windows lifecycle, and no whole-MirBuilder completion.
```

## Bounded census

The boundary is one Rust source transaction from `SourceProgramJsonAuthority::for_source`
through `Stage1ProgramJsonModuleHandoff::from_source_artifact` and the existing
`json_v0_bridge::parse_json_v0_to_module` terminal. It includes the static Main
Program-v0 body, the selected direct/New StringBox source relations, the typed
crosswalk, one canonical admission, and typed rejection before finalization.
It excludes body-only Hako registry/fallback callers, `env.mirbuilder.emit`,
direct MIR schema 2.0/v0 call readers, `indexOf`, other producers, whole-selfhost
activation, backend parity, and native Windows lifecycle capability.

The selected membership is finite and exact:

| Source relation | Selected result |
| --- | --- |
| direct `Str` or `String` literal receiver, `length` or `size`, zero method args | source artifact and typed Call anchor |
| `New(StringBox)` with one direct string constructor arg, no field initializers, `length` or `size`, zero method args | source artifact and typed Call anchor |
| direct/New `indexOf` | compatibility/null; no artifact |
| extra constructor/method args, embedded or opaque/non-literal strings | compatibility/null; no artifact |
| malformed, missing, or unrecognized source relation | typed rejection or existing compatibility result according to the existing owner contract |

An empty string literal is valid. The old first-string scan is broader than this
membership and is retained only as compatibility behavior; it is not an issuer.

## Finite disposition table

| state | authority/issuer | pre-effect check | terminal/continuation | fallback |
| --- | --- | --- | --- | --- |
| Selected | Rust Stage1 source transaction issues product and anchor | validate exact receiver, selector, arity, and source relation before publication | continue through bridge admission and consume once | none |
| Compatibility / Unavailable | Hako compatibility owner; no source product | no selected artifact is issued | continue on existing compatibility/null path | existing registry/fallback only |
| NoSafeSlice / Rejected | no issuer for unsupported relation | reject before effect or artifact | terminal typed rejection; no retry | no fallback into selected route |

This finite table is the bounded inventory for this card. `Unavailable` and
`NoSafeSlice` are neutral or rejected outcomes, not hidden migration candidates.

## Authority and API contract

`src/stage1/program_json_v0/authority.rs` remains the only source issuer. The
implementation may place the product and crosswalk definitions in a new
stage1-owned module so the 760-line split trigger and 800-line hard stop remain
visible. The same AST parse/lower transaction must issue both Program-v0 and the
source product; a later JSON or MIR scan must not reconstruct provenance.

`Stage1StringBoxSourceProductV1` carries the source owner/path, MethodCall parent,
receiver and ordered arguments, selector and arity, the `New(StringBox)` site,
class and constructor arguments, field-initializer/type-argument classification,
and the selected result/effect/ABI contract borrowed from the existing
`CoreMethodContractBox` row. `Stage1ProgramJsonCrosswalkV1` carries one stable
source relation and one lowering-issued expected output shape. Duplicate, missing,
or unclassified relations fail before the artifact is returned.

The source authority returns one enriched artifact:

```text
SourceProgramJsonAuthority::for_source(source)
  -> (program_json, Stage1StringBoxSourceProductV1, Stage1ProgramJsonCrosswalkV1)
```

`Stage1ProgramJsonModuleHandoff::from_source_artifact(...)` is the selected
consumer. The existing `Stage1ProgramJsonModuleHandoff::parse(&str)` remains the
body-only compatibility seam until a later caller-switch card supplies the
source product. The terminal order is fixed:

```text
from_source_artifact
  -> json_v0_bridge::parse_json_v0_to_module
  -> admit_stringbox_source
  -> Stage1FinalizedMirModule
```

Admission validates the product and crosswalk against exactly one existing
`MirInstruction::Call`, checks receiver/class, selector, arity, constructor
arguments, and source relation, consumes the anchor once, and rejects zero,
multiple, unconsumed, or mismatched anchors. It proves the bridge output; it does
not mutate or physically rewrite that output.

## Authorized task order

1. **Define the source product and crosswalk.** Add source-owned typed definitions
   with stable source relation keys and expected output shape. Keep Facts free of
   Recipe keys and physical IDs; only the Recipe/issuer side may issue the
   crosswalk anchor.
2. **Emit during lowering.** Thread a small lowering accumulator through the
   existing Program-v0 transaction. Record only the selected direct/New
   StringBox length/size relations and their expected canonical Call shape.
   Scope flattening and multi-local expansion must not be handled with guessed
   JSON indices.
3. **Enrich the source handoff.** Change `for_source` to return the artifact and
   add `from_source_artifact`. Preserve the existing body-only `parse` and all
   existing v0/v1 rejection precedence.
4. **Add canonical admission.** Run the bridge once, validate and consume each
   lowering-issued anchor exactly once, and fail before finalization on every
   mismatch. No retry, fallback, schema spelling substitution, or physical MIR
   rewrite is allowed.
5. **Focused evidence and owner docs.** Add positive and negative tests beside
   the owning Rust modules, update the MirBuilder handoff README and the relevant
   reference contract, record the source-to-terminal route, and classify any
   environment-only red. Keep source files below the split limits.

## Acceptance

```text
direct Str/String literal length and size, zero args: one product and one Call anchor
New(StringBox) with one direct string arg, no field initializer, length and size: same
empty literal: accepted as a valid selected relation
indexOf, extra args, embedded/opaque/non-literal strings: no selected artifact; compatibility/null remains
malformed/missing/unrecognized relation: typed reject or existing compatibility result, with no publication
product/crosswalk source, receiver/class, selector, arity, or constructor relation mismatch: typed reject before finalization
zero, multiple, or unconsumed expected Call anchors: typed reject; no retry and no physical rewrite
existing Stage1ProgramJsonModuleHandoff::parse(&str): unchanged body-only compatibility behavior
phase14/phase17 semantic results: 3/1 remain unchanged through existing compatibility callers
```

The focused Rust gate is one Cargo process with the quick profile and bounded
jobs. A local zero-test filter is not evidence; use the complete test path and
`--exact` after confirming its name. CI is follow-up evidence, not a prerequisite
for independent design or focused implementation progress.

## Caller and retirement boundary

This I0 deliberately does not switch `registry_authority_box.hako` or
`fallback_authority_box.hako`: both callers currently receive body-only JSON and
cannot supply the co-sealed product. They remain compatibility capsules, as do
`env.mirbuilder.emit` and the selected `indexOf` edges. The next D0 must name the
source-owning caller that can pass the enriched artifact; only its successor I0
may switch both callers and remove the exact selected
`_emit_length_mir`, `_emit_size_mir`, and `_emit_new_stringbox_boxcall0` edges.
No caller-zero is manufactured here, and R7 shared-schema deletion remains
blocked until that later cutover has its own production evidence.

## Worker consultation and closeout rule

The read-only handoff worker confirmed the terminal order, typed-anchor
requirements, and zero/multiple/unconsumed rejection set. The prior source and
crosswalk audits confirmed that AST-to-JSON indices are unstable because scope
flattening and multi-local expansion occur during lowering. This card therefore
uses one lowering-issued crosswalk and one consumer owner.

The card may close only after code, focused positive/negative/guard evidence,
README/reference updates, and pointer synchronization are observable. Closure
does not claim a production caller switch. Any need to widen membership, add a
second issuer, infer provenance after lowering, or change a compatibility caller
reopens `MIR-CALL-R7-STRINGBOX-CALLER-SWITCH-D0` instead of expanding this row.

## Implementation evidence (2026-09-14)

- `src/stage1/program_json_v0/source_artifact.rs` now issues the source product
  and crosswalk during the Rust AST parse/lower transaction. Strict source
  lowering carries a private `source_anchor` on selected Method expressions;
  relaxed compatibility lowering keeps the body-only shape.
- `src/runner/json_v0_bridge` returns an ephemeral
  `(function, block, instruction, dst)` receipt while lowering that anchor. The
  receipt is not serialized and is shared across bridge environment clones.
- `src/host_providers/mir_builder/handoff.rs` consumes the artifact through one
  bridge pass and validates exact cardinality, source/crosswalk identity,
  receiver origin, `RuntimeDataBox` Call shape, selector, and arity before
  finalization. No MIR rewrite or post-hoc index scan is used.
- Focused evidence: `source_stringbox_literal_uses_source_anchor_admission`,
  `source_stringbox_new_uses_source_anchor_admission`, and
  `source_stringbox_anchor_missing_from_program_is_rejected` pass in the
  `host_providers::mir_builder` test binary. `cargo check --profile quick -j1`
  also passes. The pointer guard is recorded with the closeout update. This
  bounded parser/contract I0 is landed; it does not claim a production caller
  switch, selected old-edge deletion, or shared-schema retirement.
