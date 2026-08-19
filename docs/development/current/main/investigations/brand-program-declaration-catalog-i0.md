# Brand Program Declaration Catalog I0

Status: selected
Parent: `brand-program-declaration-catalog-d0.md`
Row: `BRAND-PROGRAM-DECLARATION-CATALOG-I0`
Classification: BoxCount

## Execution brief

Decision: Add one neutral AST-free effective Brand declaration catalog and
switch both Stage1 and selected-normal MIR collection to its sole issuer.
Source authority + canonical issuer: Each gate-pruned effective Program feeds
ordered top-level Brand rows into one neutral draft/seal owner; the sealed row
retains exact source site, name, and underlying type.
Non-authority: Stage1 `known_brands`, `collect_brand_decl_index`, mutable
`CompilationContext.brand_decls`, Program JSON, raw call priority, and physical
ValueId/type cannot issue membership or duplicate disposition.
Fail-fast boundary: A second effective declaration of one name rejects with
`[brand/duplicate-declaration]` before checker/resolver, Program JSON lowering,
Builder installation, or child effects; inactive pruned declarations are absent.
Smallest next slice: Add the neutral model/issuer child, make normal declaration
facts fallible and catalog-owning, migrate Stage1 checker/context/JSON projection,
and retain only a catalog-derived temporary MirBuilder compatibility cache.
Non-claims: No constructor/unwrap site relation, precedence change, nominal MIR
value, raw consumer cutover, cache retirement, runtime/ABI, or backend work.

## Acceptance

- Empty, singleton, and ordered multi-Brand Programs seal deterministic rows.
- Earlier and later declarations have program-wide catalog visibility.
- Effective duplicate rejects with the stable tag in Stage1 and selected-normal.
- A duplicate in an inactive pruned gate does not enter the issuer.
- Stage1 private Brand collector has zero callers and is deleted.
- Selected-normal facts lend then move the same catalog instance; no AST rescan
  or name-based re-pairing occurs inside one invocation.
- Existing unique-Brand Program JSON and MIR behavior remain unchanged.
- `CompilationContext.brand_decls`, while still needed by the raw consumer, is
  populated only from the sealed catalog and documented as compatibility cache.
- Every touched source stays below 760 lines; 800 is a hard stop.
