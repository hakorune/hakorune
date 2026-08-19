# FunctionCall Explicit Externcall Source Identity D0

Status: selected design stop
Scope: one canonical explicit `externcall` source-site identity
Parent: `function-call-special-namespace-source-registry-d0.md`
Row: `FUNCTION-CALL-EXPLICIT-EXTERNCALL-SOURCE-IDENTITY-D0`

## Current execution brief

Decision: Design one resolver-owned source-site identity for canonical explicit
`externcall` before its target and ordinary arguments are evaluated.
Source authority + canonical issuer: The language registry/profile plus exact
`FunctionCall` source site and its first string-literal target operand are inputs;
resolver canonicalization must issue the identity once.
Non-authority: Raw name comparison, Builder preflight success, MIR extern symbol,
return-type heuristic, AST adjacency after resolution, tests, C, and ASM.
Fail-fast boundary: Wrong spelling/profile, absent or non-string target, foreign
site, or ambiguous ownership rejects before lowering remaining arguments and
before Builder/MIR effect; it must not fall through to FreeStatic or Ordinary.
Smallest next slice: Decide the grammar row/normalized source shape, exact
site-keyed product placement, target-literal ownership, and one bounded I0 that
replaces only the raw `externcall` classifier branch.
Non-claims: No FFI target validation, return-type redesign, lexical/FreeStatic
classification, other special routes, new syntax, fallback, or production switch.

## Questions to close

1. Should canonical parsing normalize this spelling to a dedicated AST shape, or
   may the resolver issue a site-keyed special identity over existing FunctionCall?
2. Which product already owns the exact call site without cloning arguments?
3. Is `StringBox("...")` a canonical target spelling or only legacy extraction?
4. Does malformed `externcall` reject in parser/resolver or remain a stable
   pre-effect semantic diagnostic?
5. Which raw branch and tests become caller-zero in the future I0?

## Future I0 acceptance

- One exact source site receives one explicit-extern identity.
- The target spelling and ordered remaining arguments retain source ownership.
- Malformed target cases reject before child effects.
- A FreeStatic declaration named `externcall` cannot shadow the explicit form.
- The selected raw name classifier branch becomes caller-zero without fallback.
- Resolver and raw route files remain below the 760-line split threshold.
