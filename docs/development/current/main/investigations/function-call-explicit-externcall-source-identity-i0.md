# FunctionCall Explicit Externcall Source Identity I0

Status: landed
Parent: `function-call-explicit-externcall-source-identity-d0.md`
Row: `FUNCTION-CALL-EXPLICIT-EXTERNCALL-SOURCE-IDENTITY-I0`
Classification: BoxCount

## Execution brief

Decision: Replace raw Builder externcall name classification with one dedicated
parser-issued `ExplicitExternCall` and resolver-sealed site-keyed identity.
Source authority + canonical issuer: Grammar profile, exact contextual token
sequence, source site, decoded target, and resolver canonicalization issue the row.
Non-authority: Ordinary FunctionCall name/first argument, Builder state, runtime
StringBox, raw extraction, MIR symbol, callable misses, tests, C, and ASM.
Fail-fast boundary: Malformed, foreign, duplicate, or untagged forms reject before
remaining child effects and never fall through to Ordinary/FreeStatic.
Smallest next slice: Land AST/parser/grammar transport, resolved source relation
and verification child, then consume it at the exact raw call route and remove the old
`name == "externcall"` branch in the same bounded series.
Non-claims: No FFI authorization/return-type change, other special route migration,
generic-call retirement, StringBox target, or broad production cutover.

## Implementation order

1. Add dedicated AST vocabulary and update both parser paths plus grammar witnesses.
2. Extend the resolved source product and source-site inventory.
3. Thread the exact row through the selected Script raw-expression port.
4. Remove the raw explicit-extern name classifier and retain lowering semantics.
5. Add focused positive/negative tests, reusable guard, and README/reference receipt.

Every commit must leave files below 760 lines; split before touching an owner near
that boundary. No new semantic `Verified*` or `Prepared*` product is authorized.

## Landed receipt

- Both parser paths issue `ExplicitExternCall`; generic parentheses remain
  `FunctionCall`.
- The resolver and source inventory own one site-keyed decoded symbol row.
- The raw consumer requires that row and rejects source/relation drift before
  argument lowering. The old raw `name == "externcall"` classifier is gone.
- Grammar registry, corpus, EBNF, AST JSON transport, owner READMEs, focused
  parser/resolver/preflight tests, and the reusable guard are synchronized.
- `cargo check --profile quick --workspace`, library test compilation, focused
  guard, pointer guard, formatting, and diff checks are green. The broader
  grammar-substrate guard remains a pre-existing naming-charter baseline red.
