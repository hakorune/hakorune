# FunctionCall Explicit Externcall Source Identity D0

Status: accepted
Scope: one canonical explicit `externcall` source-site identity
Parent: `function-call-special-namespace-source-registry-d0.md`
Row: `FUNCTION-CALL-EXPLICIT-EXTERNCALL-SOURCE-IDENTITY-D0`

## Final decision

Decision: Add one dedicated `ASTNode::ExplicitExternCall { target, arguments }`
for canonical `externcall "symbol"(args)`; generic `externcall("symbol", args)`
remains an ordinary, shadowable `FunctionCall`.
Source authority + canonical issuer: The language grammar/profile row and exact
parser token sequence issue the source shape; resolver canonicalization co-seals
its exact site, decoded target symbol, and ordered argument relations once.
Non-authority: Ordinary FunctionCall name/first argument, runtime `StringBox`,
raw literal extraction, Builder preflight, MIR symbol/type hints, tests, C, ASM.
Fail-fast boundary: Canonical-looking malformed target/parentheses and
missing/foreign/duplicate resolved relations reject before argument traversal and
Builder/MIR effects, with no Ordinary/FreeStatic fallback.
Smallest next slice: `FUNCTION-CALL-EXPLICIT-EXTERNCALL-SOURCE-IDENTITY-I0`
lands the grammar row, dedicated AST/source relation, exact consumer, and raw
branch retirement as one bounded BoxCount.
Non-claims: No FFI symbol authorization, return-type redesign, other special
forms, lexical/value calls, general direct-call cutover, or compat retirement.

## Accepted syntax and product

- Canonical spelling: `externcall "symbol"(args)`.
- `externcall("symbol", args)` is an ordinary identifier call and may resolve to a
  lexical/direct callable named `externcall`; it is not compatibility syntax.
- `StringBox` construction is not a target spelling for the dedicated form.
- `ExplicitExternCall` stores decoded target text separately from its ordered
  runtime arguments, so target extraction is not an argument effect.
- The existing resolved product gains one site-keyed explicit-extern relation;
  no parallel `Verified*`/`Prepared*` receipt and no ordinary-call enum pollution.

## I0 acceptance

- each canonical site receives exactly one identity and direct decoded symbol;
- generic parenthesized calls remain ordinary and shadowable;
- missing/non-string/duplicate/foreign/spoofed forms reject before child effects;
- Brand/local/FreeStatic declarations named `externcall` do not shadow the
  dedicated form but may own the generic ordinary form;
- remaining arguments resolve/lower exactly once from left to right;
- the raw `ExplicitExtern` classifier branch becomes caller-zero with no fallback;
- classic and TokenCursor parsers issue the same dedicated shape;
- verifier logic lives in a focused child rather than growing the 743-line owner.

Classification: `BoxCount`. One dedicated normalized source shape is added and
the accidental raw-name interpretation of generic FunctionCall is removed.
