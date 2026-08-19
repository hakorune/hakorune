# Brand Grammar Declaration Capsule I0

Status: landed
Parent: `brand-declaration-namespace-and-result-contract-d1.md`
Row: `BRAND-GRAMMAR-DECLARATION-CAPSULE-I0`
Classification: BoxCount

## Execution brief

Decision: Register `brand IDENT : TYPE_REF` in both profiles and add the missing
metadata-only Hako grammar witness capsule.
Source authority + canonical issuer: The language-v1 registry/corpus own the
spelling and normalized BrandDeclaration form; Rust and Hako parsers only
project evidence from that exact declaration.
Non-authority: Constructor/unwrap calls, Brand catalogs, Stage1 maps, Builder,
tests, EBNF prose, and semantic policy cannot add grammar meaning.
Fail-fast boundary: Exact declaration syntax produces BrandDeclaration;
missing name/colon/type rejects with `parser/brand_declaration_invalid` before
any semantic catalog or declaration effect.
Smallest next slice: One registry row, two positive and malformed fixtures,
Rust witness projection, Hako declaration parser child, focused tests, and a
reusable guard.
Non-claims: No constructor/unwrap activation, duplicate check, catalog,
resolver relation, semantic publication, MIR, Stage1 bridge, raw cutover, or
backend work.

## Acceptance

- Canonical and Compat2025 normalize `brand PageId: i64` identically.
- Rust and Hako recursive witnesses match the corpus form.
- Missing name, colon, or type rejects before semantic publication.
- Hako evidence retains `semantic_publication_allowed=false`, MIR/runtime/
  backend permission false.
- No constructor or unwrap grammar row is introduced.

## Receipt

- The focused capsule guard is green: generated projection check, 10 Python
  witness tests, and the exact Rust grammar-profile test all pass.
- The exhaustive Rust matrix reports all four Brand fixtures supported.
- The repository-wide Hako adapter remains blocked before fixture execution by
  `MissingTransientType { init: ValueId(113) }`. The identical command and
  error reproduce on parent `cb3a7cd50d`, so this is classified as known
  baseline debt rather than a current-change failure.
- The full gate also sees the pre-existing `explicit_externcall` Rust witness
  drift and naming-charter debt; neither is reclassified or repaired here.
