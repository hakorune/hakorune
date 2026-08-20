---
Status: Active design stop
Date: 2026-08-20
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-ONLY-A-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-parser-source-handoff-d0-2026-08-20.md
ProductionCaller: none; design only
ReplacementCell: Builder-free source/Facts/Recipe/Join prefix for canonical Script
Classification: BoxCount
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-ONLY-A-D0

## Six-line brief

Decision: Define one Builder-free source-only A prefix that issues or lends
the already-owned Script source/Facts/Recipe/Join and physical-input products
before the canonical request enters `compile_script`. Do not open a Builder,
install a mutable catalog, lower MIR, or finalize a module in this row.

Source authority + canonical issuer: the retained parser/source handoff plus
resolver-owned declaration Facts, Script window, target inventory, result
bundle, Recipe, Join, and physical-input issuers are the source authority. A
new move-only `CanonicalScriptSourceOnlyAHandoffV1` is the single transport
issuer; it co-seals the landed source digest/profile with those existing
products and never reissues their meaning.

Non-authority: `ModuleBuilderInvocationSessionV1`, `builder.comp_ctx`, mutable
catalog installation, selected work-plan ordinals, AST/name/path re-scans,
digest-only inference, `RawScriptBodyRecipeV1`, selected claim ledgers, MIR
instructions, `ValueId`, and the later three-state disposition are not A
issuers.

Fail-fast boundary: before any Builder or physical effect, require one retained
source identity, canonical digest/profile, complete Script window, resolver
forest/catalog/brand view, target/result/Recipe/Join/operand coverage, and
matching owner/site/cardinality. Missing, foreign, duplicate, stale, or
non-source-backed input terminates as `NoSafeSlice`/`IntegrityInvalid`; it may
not become `NonCandidate` or Raw fallback.

Smallest next slice: design the source-only prefix and its shared producer
contract, then make the selected-normal lifecycle and the future canonical
frontdoor consume the same handoff rather than issuing parallel products.
Disposition C and canonical request carrier B remain later rows.

Non-claims: no three-state disposition, canonical physical Call/publication/
Return, source admission expansion, selected-normal production switch, raw or
compat retirement, ABI/backend/performance change, or Builder cleanup.

## Why the existing lifecycle is not A

`normal_default_root_catalog_lifecycle.rs` is evidence of the desired source
products, but it is not a source-only caller. Its current receiver is
`ModuleBuilderInvocationSessionV1` and it performs, in one lifecycle:

```text
root expansion / declaration Facts / resolver
  -> Builder module preparation
  -> mutable catalog installation
  -> Script target/result/Recipe/Join issuance
  -> Builder lowering
  -> finalize_module
```

The canonical frontdoor at `normal_file_canonical_core_vm.rs` does not enter
that lifecycle; it makes a source-plan request and `canonical_core_dispatch`
still prepares `RawScriptBodyRecipeV1`. Calling the existing lifecycle from
the canonical route would open a second physical pipeline and make A depend on
Builder effects. Copying its AST or re-running resolver logic on the
canonical side would create a second semantic authority.

## Closed prerequisite: parser handoff is retained at the canonical frontdoor

The parser-backed source-handoff I0 is now closed. The canonical frontdoor's
`parse_once` retains the existing `CompletedParserPostpassV1` inside a
non-Clone `CanonicalParserSourceHandoffV1`; source-plan classification moves
that same opaque postpass into `PreparedNormalSourcePlanInputV1::ParserBacked`
and exposes it read-only at the sealed Script boundary. The AST-only source
constructor remains test compatibility only.

This is not repaired by calling `parse_from_string_with_source_seal` or
`parse_from_string_with_resolver_source_handoff` again: those are parser-owned
postpass/projection issuers, and a second call would violate the one-read/
one-parse receipt and create a second authority. The missing boundary is now
tracked by
`SCRIPT-DIRECT-STATIC-CALL-CANONICAL-PARSER-SOURCE-HANDOFF-D0`.

The carrier closeout removes this transport blocker but does not open A
implementation. A remains a design contract until its Builder-free producer,
complete source/Facts/Recipe/Join coverage, and shared-consumer identity are
accepted. No canonical Script source may enter A from AST plus digest alone.

## Required source-only handoff shape

```text
CanonicalScriptSourceOnlyAHandoffV1 (non-Clone, move-only)
  source_identity: retained source-backed identity
  source_digest/profile: exact frontdoor pair
  script_window: complete ProgramBody coverage
  semantic_forest/catalog: resolver-issued owner and declaration views
  target_inventory: existing source-call target product
  result_bundle: existing result/representation product
  recipe/join: existing source-owned products
  physical_input: existing verified input plan (no ValueId/MIR effect)
  seal: one co-seal binding all identities and cardinalities
```

The handoff may borrow existing products while the source-only issuer is
running, but the published handoff owns or linearly moves every product needed
by C and by the selected-normal consumer. It must not contain a Builder,
`CompilationContext`, mutable registry, physical block, `ValueId`, or signature.

The source-only producer must be shared. The selected-normal lifecycle may no
longer independently issue a second target inventory, result bundle,
Recipe/Join, or physical-input product after this row is implemented. Until
that replacement is proved, the existing lifecycle remains unchanged and the
new handoff is design-only.

## Source eligibility and rejection

Only a retained source-backed program with an exact parser/resolver handoff may
enter A. A raw AST that lacks the source identity, resolver owner/window, or
complete Facts is not an explicit zero-candidate observation. It is
`NoSafeSlice`/`IntegrityInvalid` before any Raw recipe is selected.

The digest is carried from `read_once` and is compared/co-sealed with the same
retained source context; it is never compared to an AST pointer or recreated
from a path. Compatibility, Deferred, RawLegacy, and selected-normal products
without the exact source handoff remain outside this A row.

## Later I0 acceptance matrix

Positive:

- canonical source-only entry opens no `ModuleBuilderInvocationSessionV1` and
  performs no catalog installation, MIR lowering, or module finalization;
- one producer issues the complete handoff once, with digest/profile, window,
  owner/site, target/result/Recipe/Join, operand, and physical-input identity
  all matching;
- selected-normal and canonical consumers receive the same source product
  identity, with no duplicate issuer or re-resolution;
- missing candidate coverage remains a terminal handoff error rather than an
  implicit zero-candidate result.

Negative:

- digest/profile drift or source/Facts identity mismatch;
- missing, duplicate, foreign, stale, or partially covered window/owner/site;
- target/result/Recipe/Join/physical-input cardinality or operand mismatch;
- compatibility/deferred/raw source presented as a complete A handoff;
- Builder/session/catalog/MIR/ValueId appears in the source-only producer;
- canonical-side AST/name/path scan or resolver re-issuance;
- a selected-normal lifecycle creates a second product set instead of consuming
  the shared handoff;
- failure converted to `NonCandidate` or retried through Raw.

## NoSafeSlice conditions

Keep this row parked if a source-only owner cannot be separated from Builder
effects, if the parser handoff is not retained at the canonical caller, if
one producer cannot serve both canonical and
selected-normal consumers without duplicate issuance, or if digest/profile can
only be paired through pointer/name/path inference. Do not open disposition C,
carrier B, or physical bridge I0 until A is closed.
