# Brand Grammar Declaration Registry R0

Status: selected
Parent: `brand-declaration-namespace-and-result-contract-d1.md`
Row: `BRAND-GRAMMAR-DECLARATION-REGISTRY-R0`
Classification: behavior-neutral SSOT closeout

## Execution brief

Decision: Register only `brand IDENT : TYPE_REF` as the canonical Brand grammar
surface in both profiles.
Source authority + canonical issuer: The language-v1 registry and contract
corpus own spelling/profile normalization; both parsers already emit the same
`BrandDeclaration` capsule.
Non-authority: Constructor/unwrap call spelling, Brand catalogs, Stage1 maps,
Builder behavior, tests, EBNF prose, and semantic policy do not create another
grammar row.
Fail-fast boundary: Exact declaration syntax normalizes to BrandDeclaration;
missing name/colon/type rejects before semantic catalog issuance.
Smallest next slice: Add one registry row, two positive profile fixtures,
negative malformed witnesses, parser profile tests, and a reusable guard.
Non-claims: No constructor/unwrap semantic activation, duplicate check, catalog,
resolver relation, AST change, MIR, Stage1 bridge, raw cutover, or backend work.
