---
Status: active design stop; Box declaration syntax prerequisite
Date: 2026-08-23
Decision: NORMAL-GENERAL-PROGRAM-PARSER-BOX-DECLARATION-SYNTAX-D0
ParentDecision: NORMAL-GENERAL-PROGRAM-PARSER-MODULE-ROWS-D0
Candidate: extend existing ParserBoxSourceSealV1
ProductionCaller: 0
ProductionEdit: forbidden during D0
---

# NORMAL-GENERAL-PROGRAM-PARSER-BOX-DECLARATION-SYNTAX-D0

## Six-line brief

```text
Decision:
  extend the existing parser Box source seal with exact declaration syntax;
  do not create a resolver-owned or downstream Box-name authority.
Source authority + canonical issuer:
  parser Box declaration transaction + PreparedBoxSourceSealV1;
  the existing finalizer is the sole syntax co-seal point.
Non-authority:
  AST/name lookup after parsing, SourceResolverHandoff, source-plan classifier,
  Builder catalog, Main expansion, runtime entry policy, MIR.
Fail-fast boundary:
  final parser source-seal validation, before ParsedProgramWith* construction;
  foreign, missing, duplicate, or changed declaration syntax rejects there.
Smallest next slice:
  one ordinary Box declaration syntax row: parser brand/site, diagnostic name,
  and closed Box kind; no module admission or normal production caller.
Non-claims:
  no method arity, Main selection, imports, resolver semantics, Recipe/Join,
  request transport, Builder effect, fallback, or physical lowering.
```

## Authority decision

The current `ParserBoxSourceSealV1` already owns the correct Box-level source
site, parser brand, method relations, generated-delegate coverage, and
constructor relations. The missing fact is the declaration-local syntax needed
to distinguish `Main` from another ordinary Box without a downstream AST name
lookup.

Add one parser-owned syntax payload to that existing seal, conceptually:

```text
ParserBoxDeclarationSyntaxV1
  = SourceBoxDeclarationSiteV1
  + diagnostic name
  + ParserBoxDeclarationKindV1
```

`ParserBoxDeclarationKindV1` is closed to the parser grammar cohort:

```text
Ordinary | Static | Interface | Record
```

The name is syntax/diagnostic payload only. Identity remains the opaque parser
brand plus `SourceBoxDeclarationSiteV1`; no consumer may pair rows by name,
statement ordinal, or AST pointer.

The sole issuer is the existing parser finalizer boundary, represented by the
future named operation `ParserBoxDeclarationSyntaxIssuerV1` inside
`PreparedBoxSourceSealV1::finalize_against`. It must validate the prepared
syntax against the final AST inventory and move it into
`ParserBoxSourceSealV1`. No second source scan is introduced downstream.

This is a source syntax extension, not a semantic module product. The later
`ParserNormalModuleSourceAuthorityIssuerV1` may consume the extended seal to
recognize a `Main` declaration, but it may not issue the Box declaration row
again.

## Finite disposition

| State | Sole owner | Pre-effect behavior | Fallback |
| --- | --- | --- | --- |
| `Ready` | parser finalizer | move one validated syntax row | none |
| `Outside` | parser cohort classifier | explicit static/interface/record cohort | no normal retry |
| `Incomplete` | source-seal validator | missing syntax/coverage terminal | no empty name/kind |
| `IntegrityInvalid` | finalizer co-seal validator | foreign/duplicate/changed terminal | no AST repair |
| `CompatibilityOutOfScope` | total postpass compatibility arm | preserve compatibility provenance | no ordinary promotion |

`Ready` requires:

```text
brand matches every existing relation in the seal
site is unique and final-placement coverage is exact
name is present as parser syntax (not an identity key)
kind matches the final AST declaration flags
ordinary source rows remain ordinary, never silently static/interface/record
```

The syntax row is allowed to be retained in a non-`Clone` seal. It must not be
published as a standalone public catalog or projected into a `String` map.

## Boundary and non-authority

```text
parser declaration transaction
  -> PreparedBoxSourceSealV1
  -> final AST/source coverage validation
  -> ParserBoxSourceSealV1 { declaration syntax + existing relations }
  -> later parser module-source aggregate
```

The following remain downstream and cannot issue this row:

```text
ParserBoxResolverSourceHandoffV1
NormalSourcePlanClassifierV1
PreparedNormalDefaultProgramRootV1
VerifiedRawRootExpansionV1
PreparedNormalProgramDeclarationFactsV1
Main expansion or runtime entry selection
Box name/ordinal lookup after postpass
```

The resolver handoff may remain a transport consumer of the seal, but it is
not the source authority and must not be promoted to the normal module owner.

## NoSafeSlice conditions

Stop and keep `design_stop` if any of these is true:

```text
the seal cannot retain name/kind while preserving opaque site identity
finalizer validation needs a downstream AST/name relookup
the new row can be constructed independently of PreparedBoxSourceSealV1
the same declaration syntax has two production issuers
static/interface/record states collapse into an empty/default kind
foreign parser brands or duplicate sites are not rejected before move
the change requires NormalCompileRequest, Builder, Recipe, or physical code
```

## Acceptance packet

Before implementation permission, record:

```text
ParserBoxDeclarationSyntaxIssuerV1 call site       = 1
ParserBoxSourceSealV1 constructor/finalizer        = 1
declaration syntax source scan below parser        = 0
name/ordinal/pointer pairing outside parser        = 0
foreign/duplicate/changed row typed rejection      = exact
AST/request/Builder effect in this slice           = 0
normal production caller                            = 0
fallback/retry/reselection                          = 0
```

The next implementation slice, after this D0 is accepted, is parser-only:
extend the prepared/final Box seal, add focused source-seal positives and
negative coverage, and add one reusable structural guard. It must not connect
the module aggregate or normal ingress in the same change.

## Explicit non-claims

```text
ParserNormalModuleSourceAuthorityV1 implementation
Main.main source observation
method arity or callable selection
imports/config snapshot
NormalCompileRequest transport
Builder catalog/effect
resolver semantic owner forest
Recipe/Join/MIR/publication
production switch, fallback retirement, backend, performance
```
