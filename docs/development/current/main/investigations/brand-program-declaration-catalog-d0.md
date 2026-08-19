# Brand Program Declaration Catalog D0

Status: accepted
Parent: `brand-declaration-namespace-and-result-contract-d1.md`
Row: `BRAND-PROGRAM-DECLARATION-CATALOG-D0`

## Design brief

Decision: Issue one representation-neutral, AST-free Brand catalog and switch
both Stage1 and selected-normal MIR to it in the first catalog I0; do not leave
either private name map as a second issuer.
Source authority + canonical issuer: Gate-pruned effective top-level
BrandDeclaration rows feed one neutral catalog issuer containing exact source
site, name, underlying type, order, and duplicate disposition.
Non-authority: `CompilationContext::brand_decls`, Stage1 `BTreeMap`, resolver
FreeStatic misses, raw name priority, Program JSON, and physical ValueId/type
cannot issue catalog membership.
Fail-fast boundary: Duplicate effective names reject with
`[brand/duplicate-declaration]` before callable resolution or argument effects;
foreign, missing, or re-paired rows publish no catalog and never default.
Smallest next slice: First split the 773-line normal lifecycle test body into a
child without behavior change; then `BRAND-PROGRAM-DECLARATION-CATALOG-I0`
adds the neutral issuer and atomically removes Stage1's private collector.
Non-claims: No constructor/unwrap site relation, argument lowering, nominal MIR
type, Stage1 cutover, raw Brand deletion, runtime representation, or production
switch in this D0.

## Required acceptance before I0

- Effective declarations are collected after build-gate pruning and are
  program-wide rather than source-order visible.
- Duplicate names have one pre-resolution rejection owner.
- The catalog retains declaration identity, name, and underlying type; it is
  not a `contains_key` set.
- Resolver and later physical lowering borrow/project from the same product;
  neither reconstructs membership from AST, JSON, or spelling.
- Existing natural constructors such as `BlockId(7)` remain outside this first
  implementation slice until an exact site-relation row is separately selected.

## Accepted ownership and lifetime

- The catalog model/issuer lives in a neutral analysis owner usable by Stage1
  and MIR; neither pipeline defines its own Brand row or duplicate policy.
- Each mutually exclusive compiler entry issues one catalog instance from its
  already-pruned effective Program. Address identity across entries is neither
  possible nor required; schema and issuer identity are shared.
- In selected-normal lowering, `PreparedNormalProgramDeclarationFactsV1` owns
  the catalog. It lends it before Script resolver work, then moves the same
  owner through root lowering. A temporary `CompilationContext.brand_decls`
  projection is compatibility cache only, never a catalog issuer.
- Stage1 deletes `collect_brand_decl_index`; its checker, lowering context, and
  declaration JSON projection borrow the neutral catalog.
- Duplicate effective names reject with `[brand/duplicate-declaration]` before
  resolver, Program JSON lowering, Builder installation, or argument effects.

## Ordered task ladder

1. `NORMAL-DEFAULT-BRAND-CATALOG-LIFECYCLE-SPLIT-P0` — BoxShape: move the
   773-line lifecycle's test body to a bounded child before catalog wiring.
2. `BRAND-PROGRAM-DECLARATION-CATALOG-I0` — BoxCount: add the neutral catalog,
   switch Stage1 and selected-normal collection, and reject duplicates.
3. `BRAND-CONSTRUCTOR-SOURCE-RELATION-I0` — BoxCount: issue exact constructor
   and unwrap relations while excluding ordinary direct-call rows.
4. `BRAND-CONSTRUCTOR-CONSUMER-CUTOVER-R0` — BoxShape: consume only site
   relations and retire raw `is_brand_declared(name)` classification.
5. `BRAND-LEGACY-CACHE-RETIREMENT-R0` — BoxShape: remove caller-zero mutable
   compatibility maps after all consumers move.

## NoSafeSlice

- Stage1 or selected-normal retains an independently issued Brand name map.
- Catalog rows are re-paired to calls by name after resolution.
- Duplicate rejection happens after child traversal or Builder mutation.
- Constructor-site activation or raw-route retirement is mixed into catalog I0.
- The 773-line lifecycle is grown without the preceding behavior-neutral split.
