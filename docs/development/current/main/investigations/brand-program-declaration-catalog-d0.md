# Brand Program Declaration Catalog D0

Status: selected design stop
Parent: `brand-declaration-namespace-and-result-contract-d1.md`
Row: `BRAND-PROGRAM-DECLARATION-CATALOG-D0`

## Design brief

Decision: Design one AST-free, program-wide effective Brand declaration catalog;
do not copy the current Stage1 and MirBuilder name maps into a third owner.
Source authority + canonical issuer: Gate-pruned top-level BrandDeclaration
facts in the normal program declaration owner issue exact declaration identity,
name, underlying type, and duplicate disposition once.
Non-authority: `CompilationContext::brand_decls`, Stage1 `BTreeMap`, resolver
FreeStatic misses, raw name priority, Program JSON, and physical ValueId/type
cannot issue catalog membership.
Fail-fast boundary: Duplicate effective names reject with
`[brand/duplicate-declaration]` before callable resolution or argument effects;
foreign, missing, or re-paired rows publish no catalog and never default.
Smallest next slice: Census the declaration-facts lifetime and select one
move-only lending seam that can serve resolver relations and later physical
consumers without reinstalling membership into a mutable map.
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
