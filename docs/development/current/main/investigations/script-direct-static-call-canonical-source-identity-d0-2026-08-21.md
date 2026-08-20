---
Status: Active design stop
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-SOURCE-IDENTITY-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-only-a-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: one parser-lineage/canonical-receipt identity handoff
Classification: BoxShape (transport existing identity only)
---

# SCRIPT-DIRECT-STATIC-SOURCE-IDENTITY-D0

## Six-line brief

Decision: Design one move-only identity handoff that co-seals the existing
parser lineage with the existing canonical source-plan receipt. Do not issue a
new semantic candidate, Resolver product, Recipe, Join, carrier, or physical
effect in this row.

Source authority + canonical issuer: the one-read/one-parse source receipt
and parser-backed postpass are the source authority. The existing
`CanonicalParserSourceHandoffV1::new` is the sole identity issuer: it creates
the parser-owned `NormalParserSourceLineageV1` once and co-seals it with the
front-door receipt. The future handoff validates that same product against
`NormalSourcePlanReceiptV1` once and lends the source/window identity to A.

Non-authority: display names, paths, filenames, AST addresses, statement
ordinals, pointer identity, digest equality without lineage, compatibility
success, `ValueId`/`MirType`, Builder/`comp_ctx`, and any canonical-side
reparse cannot issue or repair identity.

Fail-fast boundary: after the one parser read/parse and before normal source
classification or `prepare_script_recipe()`. Missing lineage, profile/read/
parse mismatch, empty identity, digest mismatch, or duplicate transport
rejects before Resolver, Recipe, Join, Builder, or physical effects.

Smallest next slice: `SCRIPT-DIRECT-STATIC-SOURCE-IDENTITY-D0` is a docs-only
contract for the handoff fields, finite states, one move, and A-facing loan.
An implementation I0 may add only thin transport/accessor wiring after this
contract is accepted.

Non-claims: no source-only A issuer, target/result catalog, Recipe/Join,
physical input/Call/publication/Return, source-admission change, canonical
production switch, raw/compat retirement, ABI/backend, or performance claim.

## Exhaustive identity disposition table

Identity transport must distinguish canonical authority from test and
compatibility paths. The following table is exhaustive for the handoff
boundary; no `None` or default identity may merge these states.

| state | issuer / authority | pre-effect behavior | terminal / continuation | fallback policy |
|---|---|---|---|---|
| `NotApplicable` | non-canonical profile or non-source-plan caller | no canonical identity claim | caller-owned non-canonical route | never fabricate canonical identity |
| `CanonicalSourceBacked` | parser lineage + canonical receipt co-seal | validate identity/profile/window before classification | lend one source identity to A | no display-name or digest-only substitution |
| `AstOnlyFixture` | explicit test-only `PreparedNormalSourcePlanInputV1::new` | no canonical A eligibility | test fixture terminal | never enter canonical production path |
| `CompatibilitySource` | parser disposition explicitly marked compatibility | retain reason and lineage boundary | compatibility owner or design stop | never become canonical `NonCandidate` or A input |
| `LineageUnavailable` | parser-backed identity was not issued or was dropped | typed `SourceAuthorityUnavailable` before effects | design stop until parser handoff is retained | no AST reconstruction or default identity |
| `IdentityInvalid` | issuer detects empty, foreign, duplicate, profile, receipt, digest, or window mismatch | typed reject before classification/Recipe | terminal source-plan discard | no retry, re-read, reparse, or re-pair |
| `Transported` | one handoff move into source plan/A input | source handoff cannot be cloned or replayed | one downstream loan/consumer | no second issuer or fallback route |

`CanonicalSourceBacked` is the only state eligible for a future Source-only A
loan. `AstOnlyFixture` remains test compatibility and is not a weaker form of
canonical identity. `CompatibilitySource` is not evidence of a complete
non-candidate observation. `LineageUnavailable` and `IdentityInvalid` stop
before any semantic or physical effect.

## Exhaustive transitions

```text
front-door input
  -> NotApplicable | CanonicalSourceBacked | AstOnlyFixture
  -> CompatibilitySource | LineageUnavailable | IdentityInvalid
CanonicalSourceBacked -> Transported | IdentityInvalid
Transported            -> one A/source-plan loan only; no replay
AstOnlyFixture         -> test terminal; never canonical
CompatibilitySource    -> compatibility terminal/design stop; never A
LineageUnavailable     -> design stop only
IdentityInvalid        -> terminal discard only
```

The table intentionally has no `Unknown`, `Pending`, or `EmptyIdentity`
wildcard. If a new source disposition is discovered, return to design stop
and extend the table before implementation evidence is accepted.

## Ownership and proposed implementation seam

```text
CanonicalParserSourceHandoffV1::new
  is the sole front-door issuer of the parser-owned lineage plus receipt.

PreparedNormalSourcePlanInputV1 / SealedNormalScriptSourceV1
  retain the same non-Clone handoff and expose a read-only identity loan;
  they do not issue a new identity or semantic disposition.

CanonicalScriptDirectStaticSourceOnlyIssuerV1 (later A)
  consumes the loan plus the complete Script window and co-seals A products.

CanonicalCoreSourcePlanCompileRequestV1 (later B)
  receives the same identity through a typed source-plan owner; it does not
  re-resolve AST or join by display path.
```

The likely thin implementation seam is an accessor/loan from the existing
parser-backed `NormalParserCallableSourceHandoffV1` through
`PreparedNormalSourcePlanInputV1`, plus one equality validator against the
already retained `NormalSourcePlanReceiptV1`. Do not grow the 760-line
canonical dispatcher with identity logic; use a focused sibling if the
validator needs more than a few lines. Do not clone `NormalParserSourceLineageV1`.

The validator must compare the issuer-owned identity fields (source identity,
digest, grammar profile, UTF-8 length, and one-read/one-parse counts) and the
sealed Script window. A pointer, display path, AST ordinal, or equal digest
alone is not a proof of source identity.

## Evidence and current gap

- `src/parser/normal_callable_program_source/model.rs:33-104` defines and
  validates the parser-owned `NormalParserSourceLineageV1` shape; it must not
  be re-issued by a later source-plan or A consumer.
- `src/runner/reference/normal_file_vm_frontdoor/parser_source_handoff.rs:20-64`
  creates that lineage exactly once in `CanonicalParserSourceHandoffV1::new`.
- `src/mir/compiler/normal_source_plan/product.rs:24-83` retains a separate
  display identity and exposes AST/postpass, but not the parser lineage.
- `src/mir/compiler/canonical_core_dispatch.rs:33-73` owns the canonical
  receipt independently; the request at `:92-130` has no shared identity
  product.
- `src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs:67-141`
  moves the parser handoff into source planning while projecting display name
  separately.

This row is therefore a transport/design boundary, not an excuse to create a
guessed `Verified*` product. Once the handoff is closed, Source-only A still
requires its own source-owned catalog/result/forest issuer and remains a
separate design decision.

## Acceptance for this design stop

- one named issuer provides every identity field;
- canonical parser-backed and receipt-backed identities are co-sealed once;
- `AstOnlyFixture`, `CompatibilitySource`, `LineageUnavailable`, and
  `IdentityInvalid` remain distinct;
- no identity is reconstructed from path, name, pointer, ordinal, or digest
  alone;
- one move/loan reaches Source-only A without cloning, replay, or reparse;
- missing/foreign/duplicate/profile/window drift fails before effects;
- all touched implementation owners can remain below the 760/800-line limits;
- no A, Recipe, Join, physical carrier, production switch, or raw retirement
  is opened by this D0.

## NoSafeSlice conditions

Remain at design stop if any of these holds:

1. parser lineage cannot be lent without cloning or reissuing it;
2. canonical receipt and parser lineage have no common issuer/validator;
3. a compatibility or AST-only fixture must masquerade as canonical;
4. the validator needs AST/path/name/pointer re-pairing;
5. the source plan drops identity before A can borrow it;
6. identity wiring requires semantic Resolver/Recipe/Join or physical edits;
7. `LineageUnavailable` or `IdentityInvalid` can fall through to raw;
8. the implementation would grow a 760-line owner instead of splitting by
   identity responsibility.

Until these are closed, only documentation and read-only audits are allowed.
