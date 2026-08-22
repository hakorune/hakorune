# Brand Constructor Source Relation I0

Status: landed
Parent: `brand-constructor-source-relation-d0.md`
Row: `BRAND-CONSTRUCTOR-SOURCE-RELATION-I0`
Classification: BoxCount

## Execution brief

Decision: Issue one exact semantic relation batch for catalog-owned constructor
and unwrap sites; do not change MIRBuilder physical consumption in this row.
Source authority + canonical issuer: The same effective Brand catalog is lent
to shared resolved-semantics traversal, which owns exact owner/site paths and
co-seals declaration identity, kind, receiver, and operand site.
Non-authority: Mutable Brand maps, names after traversal, callable misses,
deferred Script indices, AST spans, Stage1 JSON, and MIR values cannot issue or
repair a relation.
Fail-fast boundary: Before child traversal, reject wrong arity, unsupported
Brand static methods, foreign/duplicate sites, catalog drift, and malformed
receiver/operand relations; recognized sites never enter generic resolution.
Smallest next slice: Add one focused relation model/issuer, move declaration
facts collection before callable resolution, lend the catalog to callable and
Script traversal, and project the sealed rows in verified semantic products.
Non-claims: No raw constructor consumer switch, unwrap physical consumer,
nominal MIR Brand value, mismatch checking, cache deletion, runtime, or backend.

## Bounded implementation

- Put the relation model in a focused resolved-semantics child module.
- Reuse `BrandDeclarationSourceIdV1`, owner-local `SourceExprSiteV1`, and exact
  `Argument(0)` paths; add no span-, name-, or ValueId-derived identity.
- Collect declaration facts once after gate pruning and before resolver/package
  issuance. Lend the same catalog to callable and Script traversal and retain
  the same moved facts owner for root installation.
- Constructor rows are absent from the ordinary direct-call ledger.
- Exact `Variable(BrandName).unwrap(arg)` issues Unwrap; another selector on a
  declared Brand rejects before child traversal.
- Compatibility/macro outcomes without the same source-backed catalog and
  resolver traversal do not claim exact relation parity.

## Acceptance

Positive:

- constructor calls before and after their declaration bind the same exact row;
- callable owners and Script root both issue owner-local relations;
- same-name sites remain distinct and cannot cross owners;
- Brand collisions with FreeStatic, Math, TypeOp, and `str` issue only Brand;
- constructor and unwrap retain the exact declaration and `Argument(0)` site;
- classic and TokenCursor normalized forms yield the same inventory.

Negative:

- arity zero/two and unsupported Brand selector reject before child traversal;
- missing, duplicate, swapped, foreign-owner, or foreign-catalog rows reject;
- `obj.unwrap`, `pkg.Brand.unwrap`, and `(Brand)(value)` remain non-Brand forms;
- no recognized Brand constructor remains in generic direct-call products;
- no missing relation falls back to raw name classification;
- no production owner reaches 760 lines without an explicit split.

Focused tests and a reusable guard must cover relation inventory, effect order,
collision exclusion, both owner families, catalog lifetime, and caller census.

## Landed evidence

- `VerifiedBrandCallSourceRelationV1` is keyed by the resolver-owned function
  owner and exact `SourceExprSiteV1`; it retains the catalog declaration ID,
  underlying type, relation kind, receiver site when required, and the exact
  `Argument(0)` operand site.
- Callable and Script shadow traversal borrow the same effective Brand catalog
  before child descent. Recognized constructors are absent from the ordinary
  direct-call ledger; exact Brand `unwrap` is absent from generic method
  resolution.
- Declaration facts are collected once after gate pruning, lent before callable
  and Script resolution, then moved unchanged into the existing root install.
- `cargo check --profile quick --lib` is green.
- The three focused relation tests and
  `tools/checks/brand_constructor_source_relation_guard.sh` are green.
- The broader `normal_default_root_catalog_lifecycle` suite remains 5 green / 3
  red. The identical three failures reproduce on parent `2b48087466` and are
  classified as known baseline debt; they are not caused by this relation row.
- Production owners remain below the 760-line split trigger. Physical raw
  consumption still uses the explicitly retained compatibility probe and is
  the subject of a separate BoxShape cutover row.
