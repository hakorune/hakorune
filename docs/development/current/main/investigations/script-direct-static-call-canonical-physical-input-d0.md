# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-PHYSICAL-INPUT-D0

Status: design stop; selected-normal physical bridge is closed, but the
canonical detached Script physical input has no implemented source-backed
consumer. The scalar operand Recipe authority is now designed; this card
defines the next detached input contract. No code, fixture, route switch, or
semantic receipt is authorized by this card.

Parent: `script-static-scalar-operand-recipe-d0.md`

## Current six-line brief

Decision: open one design-only BoxCount for the canonical Script physical
input. It must carry the already-issued direct-static source/Facts/Recipe/Join
meaning into a detached canonical session; it must not reuse the selected
claim ledger or widen the scalar Script recipe.

Source authority + canonical issuer: the existing
`VerifiedScriptDirectStaticJoinHandoffV1` owns call/target/terminal,
representation, and ordered argument sites; the dedicated scalar operand
Recipe owns each argument tree. A single physical-input producer must co-seal
those two products by the existing Recipe key, owner, source identity, and
site/cardinality contract. It is the only issuer of the detached input.

Non-authority: `RawScriptBodyRecipeV1`, retained AST/name/ordinal rescans,
callable-key conversion, selected `ScriptDirectStaticClaimLedgerV1`,
`ValueId`/`MirType`, generic Call receipts, `ScriptPhysicalExitCommitV1`,
backend markers, detached-session defaults, and raw or compatibility
publication cannot issue or repair the canonical input.

Fail-fast boundary: missing, foreign, duplicate, reordered, or drifted source
payload; absent terminal/exit relation; unsupported representation; or a
producer-to-handoff path that cannot preserve the exact sites must stop before
physical allocation/effects as `NoSafeSlice`. No fallback, retry, or inferred
empty row is permitted.

Smallest next slice: finish the physical-input design by naming the detached
consumer kernel and its source-backed scalar materialization/Call-receipt/
publication handoff. Keep source admission, existing Facts/Recipe/Join,
selected-normal lowering, and all physical emission unchanged. Only after this
card is accepted may a separate physical-input implementation row open.

Non-claims: no canonical consumer implementation, Script exit/Return or ABI
integration, production switch, raw/compatibility/Deferred retirement, MIR
Call representation cleanup, backend change, performance measurement, or
C-parity result.

## Evidence and owner census

The selected-normal bridge is already a complete bounded BoxShape:

```text
Bundle -> Recipe -> JoinHandoff -> claim
  -> ordered arguments -> existing generic Call receipt
  -> Script ExactI64 publication -> success-only scope finish
```

That path is not the canonical detached owner. The existing scalar
`RawScriptBodyRecipeV1` accepts scalar expressions and does not carry a
MethodCall with ordered argument sites. The callable-keyed static-result owner
accepts cataloged callers, not `ScriptRoot`. `ScriptPhysicalExitCommitV1` owns
final Return/signature commit only; it cannot infer a Call target or argument
payload. The selected claim ledger is session-local and cannot become a
second semantic source.

The production old edge therefore remains intentionally live:

```text
raw MethodCall AST entry
  -> StaticReceiver
  -> Absent/non-Script -> existing static handler
```

The canonical row is not eligible for retirement until every admitted
`StaticReceiver` has exact Bundle/Join coverage and the deferred/compatibility
families have an explicit canonical owner or an explicit non-production stop.

## Design boundary

The D0 must answer, without implementation:

1. Which existing source/Facts/Recipe/Join owner issues the detached input?
2. How are the exact argument expression sites carried without AST cloning or
   re-parsing?
3. How are `FinalSequence` and `RootReturn` represented without reconstructing
   completion from a `ValueId` or `MirType`?
4. Which single detached session consumes the input, and which existing
   receipt/exit owner does it call exactly once?
5. What complete caller census proves that the selected old edge can later be
   retired without fallback or route reselection?

No implementation row may open until all five answers share one source-bound
product and one fail-fast boundary. A missing answer is development
`NoSafeSlice`, not an empty/default product.

## Acceptance for this D0

- The proposed input is AST-free and identity-bound to the existing
  source/Facts/Recipe/Join rows.
- Producer, handoff, and intended consumer are each unique and named.
- Call, receiver, ordered argument, target, representation, terminal, and
  source-owner cardinality are exhaustive and drift-checked.
- Missing/foreign/duplicate/order/site/representation/exit cases are rejected
  before physical effects.
- `RawScriptBodyRecipeV1`, selected claim state, generic callable publication,
  and `ScriptPhysicalExitCommitV1` are not promoted to new authority.
- Compatibility, Deferred, RawLegacy, StaticThis, typeop, and reserved routes
  remain explicit non-claims.
- No Rust source/check file crosses the 760-line split trigger or 800-line
  hard boundary; this D0 adds no production code.

## Future order (not authorized here)

```text
SCRIPT-DIRECT-STATIC-CALL-CANONICAL-PHYSICAL-INPUT-D0
  -> canonical physical input I0
  -> canonical consumer I0
  -> one production cutover
  -> raw/compat caller-zero and old-edge retirement
```

MIR Call dual-representation retirement, metadata consumer census, builder
root-tail cleanup, main integration, and branch protection remain separate
ordered lanes.

## D0 audit closeout — scalar operand issuer is now named

Four read-only audits plus a focused transport follow-up inspected the actual
owners. The initial `NoSafeSlice` was specifically the missing ordered operand
recipe, not a missing resolver source. The Script root can reach the resolver
facts without reopening the AST:

```text
VerifiedScriptSemanticSourceV1::forest()
  -> Script root semantic owner
  -> VerifiedResolvedScriptV1::core().data()
  -> expression_source + method_calls
```

`VerifiedResolvedMethodCallSourceV1::arguments()` supplies ordered
`(ordinal, SourceExprSiteV1)` rows, and the existing Join supplies the same
sites plus target, representation, and terminal. A future dedicated producer
can therefore co-seal one complete scalar operand recipe from existing
source/Facts/Join products. It must not use the retained `source()` AST.

The only missing accessor work is mechanical and belongs to the later
implementation row: a Script-specific read-only view and
`ResolvedExpressionSourceInventoryV1::binary(site)`. These accessors issue no
new meaning. The producer is the sole issuer of the new AST-free tree:

```text
VerifiedScriptDirectStaticScalarOperandRecipeV1::issue(
  existing Join row,
  Script resolver source view,
) -> Result<AST-free scalar operand recipe>
```

It reuses the existing `ScriptDirectStaticRecipeKeyV1`; it issues no new
callable key, physical ID, or source identity. Each argument row stores its
existing ordinal/site and one recursive tree:

```text
ScalarLiteral(i64)
ScalarUnary { site, operator, operand }
ScalarBinary { site, operator, left, right }
```

The first cohort is deliberately integer-only: unary `Minus | BitNot` and
binary `Add | Subtract | Multiply | BitAnd | BitOr | BitXor`. Comparisons,
logical operators, `Weak`, shifts, division, modulo, typed-integer payloads,
variables, calls, fields, indexes, blocks, await/qmark, and unknown literal
payloads remain rejected until a separate source-backed contract exists.

This closes the scalar operand D0 as design-only. It does not authorize a
`Verified*`/`Prepared*` implementation receipt, a `RawScriptBodyRecipe`
extension, AST clone/reparse, callable-key conversion, `ValueId`/`MirType`
inference, claim-ledger promotion, physical effect, fallback, retry,
production switch, exit integration, or performance claim. The current
canonical physical-input D0 is the sole next design row.

## Canonical input contract to close in this card

The future AST-free input is conceptually:

```text
VerifiedScriptDirectStaticPhysicalInputV1 {
  source_identity
  source_owner
  existing ScriptDirectStaticRecipeKeyV1
  call_site / receiver_site / result_site
  existing FinalSequence | RootReturn terminal
  existing canonical static target
  existing ExactI64 representation
  ordered ScalarOperandRecipe[0..N)
}
```

The producer must compare the scalar Recipe row against the Join row rather
than reconstructing either side. The key is borrowed from the existing Recipe
producer; this D0 may not issue a second key or pair rows by names, statement
ordinals, or argument count alone.

The detached direct-static kernel is a sibling helper of the existing Script
physical entry, not a second session and not a widening of
`RawScriptBodyRecipeV1`:

```text
VerifiedScriptDirectStaticPhysicalInputV1
  -> existing OpenScriptPhysicalEntrySessionV1
       -> direct_static_entry_kernel.rs
  -> scalar operands materialized left-to-right
  -> existing unified generic Call receipt issuer
  -> Script ExactI64 publication sibling
  -> typed FinalSequence | RootReturn handoff to the existing exit owner
```

The consumer may use the existing scalar instruction-lowering kernel, but it
must not create a second operator meaning or Call receipt producer. The Call
target comes only from the input's canonical target. The Script publication
sibling writes the already-verified `ExactI64` result once; it does not infer
from a `ValueId` or wait for `finalize_module()` to repair a missing type.
`OpenScriptPhysicalEntrySessionV1` remains the sole candidate `open`/`finish`
owner. A narrow parent-owned finalization seam may accept only
`LoweredScriptTerminalV1::Value { value }`, prepare/commit the existing
`PreparedScriptPhysicalExitCoreV1`/`ScriptPhysicalExitCommitV1`, verify, and
finish the same session. The source `FinalSequence | RootReturn` is validated
before effects and retained as a typed witness until that adapter; it is never
reconstructed from a `ValueId` or used to create a second Return writer.

After the input is claimed, the only outcomes are `Completed` or candidate
discard. Argument failure, Call-receipt failure, publication failure, or
completion-handoff failure must not retry, rollback, reinsert, or enter the
legacy Script route.

The source-to-exit conversion is fixed and deliberately narrow:

```text
Join terminal: FinalSequence | RootReturn
  + completed ExactI64 Call ValueId
  -> LoweredScriptTerminalV1::Value { value }
  -> PreparedScriptPhysicalExitCoreV1
  -> ScriptPhysicalExitCommitV1
  -> existing OpenScriptPhysicalEntrySessionV1::finish
```

Both accepted terminal variants carry a value for this cohort. The terminal
variant is checked against the Join row before operand descent; it does not
select a second exit owner. If the existing entry session cannot expose this
one borrowed finalization seam without duplicating candidate verification or
finish ownership, the physical-input I0 remains `NoSafeSlice`.

The card closes only when the producer and detached consumer are named with
exact future file owners, the same input/Join/operand identity and cardinality
are checked before effects, and the `Literal(7)` plus recursive accepted
integer-expression positives and all foreign/unsupported/drift negatives are
specified. If the consumer would need AST, MIR inference, generic Recipe
widening, a second Call receipt, partial publication, or rollback, this card
remains `NoSafeSlice`.

## Future owner map (design contract only)

The eventual implementation row must stay within these sibling owners:

```text
src/mir/builder/normal_script_direct_static_join_handoff/scalar_operand_recipe.rs
  - Script resolver source view + one scalar operand Recipe issuer

src/mir/builder/normal_script_direct_static_join_handoff/physical_input.rs
  - Join row + scalar Recipe co-seal; no AST or physical effect

src/mir/builder/script_physical_exit/direct_static_entry_kernel.rs
  - helper only; receives the already-open session, delegates scalar lowering,
    the existing unified Call receipt, and the existing Script publication
    owner; it never opens or finishes a session

src/mir/builder/script_physical_exit/entry_session.rs
  - sole open/finish owner; adds only the narrow terminal-finalization seam
    used by the helper

focused tests/guard siblings
  - identity/cardinality, left-to-right operands, failure discard, and line
    count/second-authority checks
```

The first module may add only the Script-specific product accessors and the
`binary(site)` lookup needed by the producer. The second module may only
compose already-issued rows. The detached session must not add a second
`ASTNode` matcher or a second Call emitter. If any owner would cross the 760
line split trigger, split that responsibility before implementation; never
compress code or move authority into a facade to satisfy the line budget.
