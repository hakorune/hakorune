---
Status: Active design stop
Date: 2026-08-20
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-PARSER-SOURCE-HANDOFF-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-disposition-d0-2026-08-20.md
ProductionCaller: none; design only
ReplacementCell: parser postpass source handoff into canonical Script source planning
Classification: BoxCount
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-PARSER-SOURCE-HANDOFF-D0

## Six-line brief

Decision: The canonical frontdoor must retain one parser-issued source handoff
before source-only A can open. The current `AST + digest` request is not enough;
do not reconstruct parser Facts or re-run the postpass in A.

Source authority + canonical issuer: the parser's one-shot postpass is the
sole issuer of parser brand, constructor/parameter, build-gate, and retained
source evidence. A new move-only carrier co-seals that product with the
`read_once` digest/profile and transports it to the canonical source-plan
consumer without reissuing parser meaning.

Non-authority: AST-only `parse_once`, `ParsedNormalCallableProgramV1::ast()`,
source paths/display names, digest equality by itself,
`parse_from_string_with_resolver_source_handoff` as a box-only projection,
and any AST reconstruction of `VerifiedFinalCallableProgramSourceV1` cannot
issue the canonical parser handoff.

Fail-fast boundary: issue the parser product once at the parse boundary and
retain it through source-plan classification. If the product is dropped,
reparsed, foreign, incomplete, or digest/profile-mismatched, terminate as
`NoSafeSlice`/`IntegrityInvalid` before A, C, B, Recipe, or physical work;
never coerce the source to compatibility or Raw.

Smallest next slice: design the carrier from
`normal_file_vm_frontdoor::parse_once` to canonical Script source planning,
including exact parser brand, constructor/parameter/postpass evidence and
digest/profile identity. Do not change resolver, Bundle, Recipe, physical,
Builder, or production route behavior in this D0.

Non-claims: no source-only A producer, three-state disposition, canonical
request carrier, physical Call/publication/Return, source-admission expansion,
selected-normal switch, raw/compat retirement, ABI/backend, or performance.

## Evidence of the missing boundary

The current canonical path is:

```text
normal_file_canonical_core_vm::run
  -> PreparedNormalFileRequest::read_once
  -> LoadedNormalFileSource::parse_once
  -> PreparedNormalFileSource { AST, profile, receipt }
  -> PreparedNormalSourcePlanInput { AST, display identity }
  -> CanonicalCoreSourcePlanCompileRequest { plan, admission, digest }
```

`normal_file_vm_frontdoor.rs:304-357` moves only the parsed `ASTNode`, profile,
and read/parse receipt. `source_plan_input.rs:63-142` retains only AST and
display identity while issuing the canonical request. The parser-owned
`VerifiedFinalCallableProgramSourceV1` and its source seals are therefore
already gone before canonical dispatch.

The parser does have a separate rich API:

```text
NyashParser::parse_from_string_with_source_seal
  -> ParsedProgramWithSourceV1
NyashParser::parse_from_string_with_resolver_source_handoff
  -> (AST, ParserBoxResolverSourceHandoffV1)
```

These are parser-owned products, not a canonical Script carrier. The resolver
handoff is a box-source projection and does not replace the complete parser
postpass product. Calling either API again after `parse_once` would be a
second parse/postpass authority and would break the one-read/one-parse receipt.

## Required carrier shape (design only)

```text
CanonicalParserSourceHandoffV1 (non-Clone, move-only)
  parser_brand / postpass invocation identity
  retained source-backed parser product
  build-gate and selected-source evidence
  constructor + parameter source evidence
  source identity and canonical profile
  read_once digest + UTF-8 cardinality
  seal: one parser/digest/profile co-seal
```

The carrier is transport, not a second semantic issuer. It may expose
higher-ranked loans to source-plan classification, but it must not expose an
AST-only escape that permits later reclassification. A source-only A producer
can consume this carrier only after the parser product has been retained and
its identity is verified.

## Required source cohorts

The carrier must declare its cohort explicitly. A source-backed ordinary Box
product and a canonical Script/no-Box source are not interchangeable merely
because both contain an `ASTNode::Program`. If the parser postpass cannot
issue complete evidence for a cohort, that cohort is `NoSafeSlice` here;
`NonCandidate`, compatibility, or Raw is not a substitute.

At minimum the design must resolve:

- canonical Script/no-Box source with a complete ProgramBody window;
- ordinary source-backed Box source, if the same canonical plan can reach it;
- static, record, interface, mixed, build-gate, and non-Program cohorts;
- constructor and parameter evidence cardinality, including empty cohorts;
- one source/profile/digest identity across all retained products.

## Later I0 acceptance matrix

Positive:

- one `parse_once` issues one parser handoff and the canonical plan consumes
  that same handoff without reparsing;
- parser brand, postpass, constructor, parameter, gate, profile, and digest
  evidence remain available at the A boundary;
- one-read/one-parse counts remain exact and the carrier is non-Clone;
- the canonical source-plan consumer can borrow source evidence without
  creating a second parser/resolver authority;
- unsupported cohorts terminate explicitly before A rather than becoming an
  empty source product.

Negative:

- AST-only parse followed by a second source-sealed parse or resolver pass;
- source path/name/pointer used to pair parser evidence with the digest;
- missing parser brand, constructor/parameter row, gate evidence, or cohort
  coverage treated as zero rows;
- foreign, duplicate, stale, or mismatched parser product;
- compatibility/deferred/raw AST presented as a complete source handoff;
- carrier cloned, replayed, or replaced by a fresh AST-derived `Verified*`;
- any resolver/Bundle/Recipe/physical/Builder effect before carrier validity.

## NoSafeSlice conditions

Keep this D0 open if the canonical frontdoor cannot retain the parser product
from its one parse, if a new carrier would need to reissue parser meaning, if
Script/no-Box evidence has no complete parser issuer, or if source identity can
only be joined through name/path/pointer inference. Do not open source-only A,
disposition C, or request carrier B until this parser boundary is closed.

## Evidence anchors

- `src/runner/reference/normal_file_vm_frontdoor.rs:304-357`
- `src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs:63-142`
- `src/parser/mod.rs:288-321`
- `src/parser/normal_callable_program_source/model.rs:112-140`
- `src/parser/source_resolver_handoff.rs:180-220`
