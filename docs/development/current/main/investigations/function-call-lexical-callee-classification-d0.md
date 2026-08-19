# FunctionCall Lexical Callee Classification D0

Status: closed NoSafeSlice
Scope: one source-site classification before argument effects
Parent: `function-call-direct-vs-value-call-compat-census-d0.md`
Row: `FUNCTION-CALL-LEXICAL-CALLEE-CLASSIFICATION-D0`

## Final decision

Decision: The general lexical/direct/special classifier is NoSafeSlice: current
lexical records prove binding identity but not callable-value membership, and
current special forms are raw preflight string branches rather than one registry.
Source authority + canonical issuer: `ResolvedLexicalRefV1` remains binding
authority and `VerifiedCallableIndexV1 -> ResolvedDirectCallTargetV1` remains the
sole direct FreeStatic authority; neither may issue the missing other meaning.
Non-authority: AST `FunctionCall`, binding kind, name/arity, `variable_map`, raw
preflight success, recovery/tail lookup, `ValueId`, MIR, tests, C, and ASM.
Fail-fast boundary: Do not issue a three-way classification until callable-value
membership and explicit-special identity each have a source owner; no defaults.
Smallest next slice: Design one source/context-owned registry for the existing
special namespace before a bounded Script FreeStatic callable-index handoff.
Non-claims: No general classifier, namespace precedence, parser rewrite, callable
type system, special registry, Builder retirement, production switch, or retry.

## Census result

- `ResolvedBindingRecordV1` carries diagnostic name, binding kind, scope, and
  origin. `ResolvedLexicalRefV1` carries `BindingRefV1`/`UpvarRefV1`; neither
  carries a callable-value semantic class.
- `VerifiedCallableIndexV1` already resolves exact FreeStatic source calls and
  the canonicalizer issues `ResolvedDirectCallTargetV1` without Builder state.
- Function-owner resolution has callable-index entry points, while Script forest
  resolution currently has only declaration views and seals no direct targets.
- raw special handling is distributed across name-shaped preflight branches;
  grammar/registry rows do not currently issue one exhaustive special identity.

## Handoff

The next card is `FUNCTION-CALL-SPECIAL-NAMESPACE-SOURCE-REGISTRY-D0`.
FreeStatic handoff follows only after special exclusion no longer depends on raw
string branches. Lexical callable values remain parked until their own source
membership issuer exists.
