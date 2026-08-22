# SCRIPT-STATIC-SCALAR-OPERAND-RECIPE-D0

Status: design stop; the canonical detached-input audit found no operand
issuer. This card narrows the missing authority to one source-backed scalar
cohort. It authorizes no code, fixture, physical effect, fallback, or
production switch.

Parent: `script-direct-static-call-canonical-physical-input-d0.md`

The long worker label `SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SCALAR-OPERAND-
RECIPE-D0` is intentionally shortened here to the repository's
family/slice/stage task-token rule.

## Current six-line brief

Decision: design one new `BoxCount` for a direct-static call whose ordered
arguments are complete resolver-issued scalar expression recipes. The first
cohort is recursive integer `Literal | Unary | Binary` only, with literal
leaves; variables, calls, fields, indexes, control expressions, and unknown
payloads remain rejected.

Source authority + canonical issuer: `VerifiedResolvedMethodCallSourceV1`
issues owner/call/receiver/ordered argument sites;
`ResolvedExpressionSourceInventoryV1` issues the exact scalar expression facts;
and the existing `VerifiedScriptDirectStaticJoinHandoffV1` supplies target,
terminal, representation, and the same argument sites. A dedicated Recipe
producer must co-seal these three existing products once and issue the
operand recipe; no AST or physical value is an issuer.

Non-authority: `RawScriptBodyRecipeV1`, AST lookup/reparse, names/ordinals,
`ValueId`/`MirType`, claim-ledger rows, generic Call receipts, callable-key
conversion, runtime constants, and detached-session state cannot issue or
complete an operand recipe.

Fail-fast boundary: before any detached physical effect, reject missing,
foreign, duplicate, reordered, or drifted argument sites; non-contiguous
ordinals; absent expression facts; unsupported expression nodes; incomplete
literal payload; and any operand whose representation is not exact integer.
The whole direct-static row becomes ineligible; no partial argument rows,
fallback, retry, or inferred default is allowed.

Smallest next slice: design the AST-free scalar operand Recipe shape, its one
source/Facts/Join producer, and the exact detached-session consumer contract.
Do not implement it until the producer can prove total coverage and the
consumer can lower it without re-reading AST or inferring from MIR.

Non-claims: no arbitrary scalar values, variables/SSA bindings, strings,
floats/bools, nested calls, fields/indexes, general Script Recipe widening,
canonical physical input implementation, Call emission, Script exit/ABI,
production cutover, raw retirement, MIR Call cleanup, or performance result.

## Authority and proposed shape

The existing Join row already owns the Call-level relation:

```text
call site + receiver site + ordered argument sites
  + canonical static target
  + ExactI64 representation
  + FinalSequence | RootReturn terminal
```

This D0 adds no Call-level meaning. It designs a nested, AST-free operand
tree keyed by each exact argument site. The proposed scalar vocabulary is:

```text
ScalarLiteral(i64)
ScalarUnary { op, operand }
ScalarBinary { op, left, right }
```

Every node owns its resolver-issued `SourceExprSiteV1`; child sites must match
the resolver expression inventory exactly. The producer must reject a tree
that contains a variable, MethodCall, generic Call, FieldAccess, Index,
Block, QMark, Await, unknown literal payload, or a non-integer representation.
The tree is a source recipe only: it owns no `ValueId`, block, `MirType`,
effect mask, ABI class, or runtime lookup.

The future canonical physical input would then be composed as:

```text
JoinHandoff(call/target/terminal)
  + ScalarOperandRecipe(argument 0..N)
  -> one detached direct-static physical input
```

The selected-normal bridge remains unchanged while this design is open.

## Required design answers

1. Which resolver traversal call issues the complete scalar expression facts,
   including every child site and literal payload, before source retention is
   dropped?
2. How does one dedicated Recipe producer bind the operand tree to the exact
   Join argument ordinal without reconstructing from AST or names?
3. Which single detached physical kernel consumes the tree, and how does it
   preserve left-to-right argument order and the existing Call receipt owner?
4. How are malformed/unsupported trees rejected before any argument effect or
   partial row publication?
5. What focused positive/negative gate proves integer literal and recursive
   unary/binary closure while rejecting variables and nested calls?

If any answer requires AST cloning, a second expression matcher, a new
callable-key authority, or `ValueId`/`MirType` inference, close this D0 as
`NoSafeSlice` and do not open an implementation row.

## Acceptance for this design stop

- The producer and consumer are named, unique, and source-backed.
- Every accepted operand node and child is covered exactly once by resolver
  facts; argument ordinals are contiguous and preserve source order.
- A missing/foreign/duplicate/drifted site or unsupported node rejects the
  whole direct-static demand before physical effects.
- No existing scalar Recipe, selected claim ledger, raw AST path, generic Call
  receipt, or `ScriptPhysicalExitCommitV1` becomes a second authority.
- The design stays below the 760-line split trigger and does not alter source
  admission, selected-normal lowering, compatibility/Deferred behavior, or
  production caller counts.

## D0 audit closeout

The four read-only audits closed the authority question. The Script root can
reach the resolver-owned expression inventory without reopening the AST:

```text
VerifiedScriptSemanticSourceV1::forest()
  -> Script root semantic owner
  -> VerifiedResolvedScriptV1::core().data()
  -> expression_source + method_calls
```

The existing method-call rows provide the ordered `(ordinal, site)` argument
relations, and the existing Join row provides the same sites together with
target, representation, and terminal. The producer therefore has one
source-backed input boundary; it must not use the retained `source()` AST.

The implementation-facing source view may add only read-only accessors for
the Script product and `ResolvedExpressionSourceInventoryV1::binary(site)`.
Those accessors issue no new meaning. The new sibling producer is the sole
issuer of the operand recipe:

```text
VerifiedScriptDirectStaticScalarOperandRecipeV1::issue(
  existing Join row,
  Script resolver source view,
) -> Result<AST-free scalar operand recipe>
```

The recipe reuses the existing `ScriptDirectStaticRecipeKeyV1`; it issues no
new callable key, physical ID, or source identity. Each argument row stores
its existing ordinal/site and one recursive tree:

```text
ScalarLiteral(i64)
ScalarUnary { site, operator, operand }
ScalarBinary { site, operator, left, right }
```

The first accepted operator cohort is deliberately small and integer-only:
unary `Minus | BitNot` and binary `Add | Subtract | Multiply | BitAnd |
BitOr | BitXor`. Comparisons, logical operators, `Weak`, shifts, division,
modulo, typed-integer payloads, variables, calls, fields, indexes, blocks,
await/qmark, and unknown literal payloads remain rejected until a separate
source-backed contract exists. This avoids silently assigning physical or
effect semantics in the Recipe issuer.

The future detached consumer is a separate direct-static kernel, not an
extension of `RawScriptBodyRecipeV1`:

```text
JoinHandoff
  + ScalarOperandRecipe[0..N)      (same key/owner/site/cardinality)
  -> VerifiedScriptDirectStaticPhysicalInputV1
  -> OpenScriptDirectStaticPhysicalEntrySessionV1
```

That kernel must materialize operands left-to-right, delegate Call emission to
the existing unified receipt issuer, publish the already-verified ExactI64
result once, and preserve `FinalSequence | RootReturn` for the existing exit
owner. It may not reread the AST, infer from `ValueId`/`MirType`, widen the
generic Script Recipe, or retry another route. Any failure discards the whole
candidate with no partial row/publication.

This closes `SCRIPT-STATIC-SCALAR-OPERAND-RECIPE-D0` as design-only. No code,
fixture, physical effect, production switch, raw retirement, or performance
claim is authorized by this closeout.

## Future order (not authorized here)

```text
SCRIPT-STATIC-SCALAR-OPERAND-RECIPE-D0
  -> SCRIPT-DIRECT-STATIC-CALL-CANONICAL-PHYSICAL-INPUT-D0
  -> canonical consumer I0
  -> production cutover
  -> raw/compat caller-zero and old-edge retirement
```
