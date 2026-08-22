# Brand Constructor Source Relation D0

Status: accepted
Parent: `brand-program-declaration-catalog-i0.md`
Row: `BRAND-CONSTRUCTOR-SOURCE-RELATION-D0`

## Design brief

Decision: Add one catalog-backed, owner/site-keyed Brand source relation for
constructor and unwrap; exclude recognized constructor sites from the ordinary
direct-call ledger without changing physical lowering.
Source authority + canonical issuer: `VerifiedBrandProgramDeclarationCatalogV1`
and the shared resolved-semantics traversal at exact `SourceExprSiteV1` issue
the declaration identity, relation kind, and exact `Argument(0)` site once.
Non-authority: Raw `is_brand_declared(name)`, FreeStatic miss, AST/JSON names,
Stage1 nodes, argument ValueIds, spans, and the compatibility cache cannot issue
or repair constructor/unwrap membership.
Fail-fast boundary: Catalog membership, exact syntax, method, arity, owner, and
child-site parity close before argument descent; foreign, duplicate, missing,
unsupported-method, or wrong-arity rows reject with no ordinary/raw fallback.
Smallest next slice: `BRAND-CONSTRUCTOR-SOURCE-RELATION-I0`, one BoxCount that
issues and projects the semantic relation for callable and Script owners and
removes recognized constructor sites from generic direct-call resolution.
Non-claims: No constructor consumer cutover, unwrap MIR activation, nominal
Brand MIR representation, mismatch verifier, legacy-cache retirement,
runtime/ABI, backend, fallback, or retry.

## Audit acceptance

- Name the exact source site carried through resolver output for natural
  `Brand(value)` and `Brand.unwrap(value)` forms.
- Prove catalog membership and the site relation meet before child traversal.
- Preserve current Brand-first collisions and exactly-one-argument timing, or
  stop for a separate language decision.
- Name the later consumer cutover and the exact raw authority deleted there;
  do not mix that BoxShape retirement into the relation BoxCount.

## Accepted relation

`VerifiedBrandCallSourceRelationBatchV1` is non-Clone and owner-local. Each row
is keyed by exact `SourceExprSiteV1` and contains:

- `Constructor` or `Unwrap`;
- exact `BrandDeclarationSourceIdV1`, Brand name, and underlying type;
- exact call site and `Argument(0)` operand site;
- for unwrap, the exact receiver site.

The effective catalog must be collected once before callable-package
resolution, lent to both callable and Script traversal, then moved with the
same declaration-facts owner into root lowering. Re-collecting or pairing a
sealed call name to the catalog later is forbidden.

For a bare call, a declared Brand owns the site ahead of FreeStatic, TypeOp,
Math, `str`, and compatibility routes. For unwrap, only exact
`Variable(BrandName).unwrap(value)` is eligible. A declared Brand receiver with
another selector rejects `[brand/unsupported-static-method]`; it does not fall
through to ordinary method resolution. This is the program-wide namespace
rule already fixed by `brand-constructor-unwrap-policy-ssot.md`.

## Ordered follow-ups

1. `BRAND-CONSTRUCTOR-SOURCE-RELATION-I0` — BoxCount relation issuance only.
2. `BRAND-CONSTRUCTOR-CONSUMER-CUTOVER-R0` — BoxShape switch from the mutable
   name probe to the exact constructor relation.
3. `BRAND-UNWRAP-PHYSICAL-ACTIVATION-I0` — separate BoxCount because no current
   MIRBuilder unwrap consumer exists.
4. Nominal Brand result identity/mismatch verification — separate design row.
5. `BRAND-LEGACY-CACHE-RETIREMENT-R0` — BoxShape after every caller is zero.
