---
Status: closeout; parser-only Box declaration syntax I0 implemented
Date: 2026-08-23
Decision: NORMAL-GENERAL-PROGRAM-PARSER-BOX-DECLARATION-SYNTAX-D0
ParentDecision: NORMAL-GENERAL-PROGRAM-PARSER-MODULE-ROWS-D0
Candidate: extend existing ParserBoxSourceSealV1
ProductionCaller: 0
ProductionEdit: parser source-seal only; module ingress and Builder remain forbidden
ExecutionRow: NORMAL-GENERAL-PROGRAM-PARSER-BOX-DECLARATION-SYNTAX-I0
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

`ParserBoxDeclarationKindV1` is closed to the currently admitted parser
source-seal cohort:

```text
Ordinary
```

`Static`, `Interface`, and `Record` are explicit outside/compatibility cases
for this slice, not variants silently represented by a default or empty kind.

The name is syntax/diagnostic payload only. Identity remains the opaque parser
brand plus `SourceBoxDeclarationSiteV1`; no consumer may pair rows by name,
statement ordinal, or AST pointer.

The sole syntax co-seal point is the existing parser finalizer boundary at
`PreparedBoxSourceSealV1::finalize_against`. This I0 does not add a second
issuer type: the finalizer validates the prepared syntax against the final AST
inventory and moves it into `ParserBoxSourceSealV1`. No second source scan is
introduced downstream.

The capture and validation points are intentionally separate:

```text
parse_box_declaration_after_box_keyword
  -> capture declaration name + admitted kind + sync flag
  -> OpenBoxMethodSourceTransactionV1::finish
  -> PreparedBoxSourceSealV1
  -> finalize_against(final AST shape)
  -> ParserBoxSourceSealV1
```

The parser header is the only place that may capture the declaration syntax;
the finalizer may compare it with final AST declaration flags, but may not
issue the name by looking up an AST node through a statement ordinal. The
existing `box_site` remains the only source-site identity. The syntax name may
later be used by the bounded module issuer as the explicit predicate
`name == "Main"`, but never as a row pairing key or identity.

This D0 only admits the source-seal cohort that exists today: an ordinary Box
with an explicit `is_sync` flag. `Static`, `Interface`, and `Record` are not
variants of this new seal yet because their parser lanes do not issue the same
`PreparedBoxSourceSealV1`. They remain explicit outside/compatibility cases
until a separate parser authority is designed for them. No broad kind enum is
introduced merely to name unsupported lanes.

The implementation must respect the source-size rule. `source_authority.rs`
is already near the 760-line split threshold, so declaration-syntax types and
capture helpers belong in a parser-private submodule (for example
`source_authority/declaration_syntax.rs`), rather than being appended to the
large authority file. A behavior-neutral split is part of the implementation
preflight if needed; no compression or unrelated cleanup is allowed.

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
kind is the admitted ordinary source-seal kind
sync flag matches the final AST declaration flags
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

## Acceptance contract

The I0 acceptance contract is:

```text
PreparedBoxSourceSealV1 finalizer syntax co-seal    = 1
ParserBoxSourceSealV1 constructor/finalizer         = 1
parser-header syntax capture                       = 1
final-AST syntax validation                        = 1
declaration syntax source scan below parser        = 0
name/ordinal/pointer pairing outside parser        = 0
foreign/duplicate/changed row typed rejection      = exact
AST/request/Builder effect in this slice           = 0
normal production caller                            = 0
fallback/retry/reselection                          = 0
admitted declaration kinds in this slice            = Ordinary only
```

The selected implementation slice was parser-only: extend the prepared/final
Box seal, add focused source-seal positives and negative coverage, and add one
reusable structural guard. It does not connect the module aggregate or normal
ingress.

## I0 implementation receipt (2026-08-23)

```text
parser-header syntax capture                         = 1
PreparedBoxSourceSealV1 transport                    = 1
finalizer syntax co-seal                             = 1
ordinary kind variants                               = 1
downstream AST/name syntax issuer                    = 0
resolver/module/Builder/Recipe/MIR effect            = 0
focused source-seal tests                            = 7 passed
parser source-authority tests                        = 15 passed
cargo check                                          = passed
current-state pointer guard                          = passed
frontend syntax I0 guard                             = passed
changed Rust rustfmt check                           = passed
git diff --check                                     = passed
```

The workspace-wide `cargo fmt --all -- --check` remains red in unrelated
baseline files; every changed Rust file passes the targeted rustfmt check, so
this is recorded as known baseline debt rather than a current-change failure.
The module aggregate, normal ingress, fallback, publication, and production
caller remain deliberately closed after this parser-only I0.

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
