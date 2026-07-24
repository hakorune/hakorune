# RAW public cutover PARITY0 App body-return S0

Decision: `BODY-RETURN-APP-prime-r1`

Status: superseded before implementation.

This card is not executable. `FUNCTION-EXIT-SEMANTICS-prime-r1` rejects
LegacyAnyStatementValue as canonical function/Main semantics. No code from
this card landed; the repository still uses `AppFixedVoid`. The owner-chain
shape below is retained only as historical input to a test-only Legacy
observation/parity proof. It must not create a canonical or public production
consumer.

```text
canonical successor =
  FUNCTION-EXIT-F1-RETURN0-S0

compatibility-evidence successor =
  RAW-BODY-RETURN-COMPAT-P0

sunset =
  RAW-BODY-RETURN-COMPAT-SUNSET-001
```

## Historical compatibility proposal — not production authority

```text
route policy = AppLastValueOrVoid
grammar      = existing LinearScalar0; no widening
empty App    = NoValue completion, Void signature, synthetic Void Return
non-empty    = exact last lowered ValueId + exact Builder MirType
              -> completion Value(v), Return(v), AppValue witness
types        = Integer / Float / Bool / String / Void
Unknown/Box/Array/Future/WeakRef = typed BODY exit rejection
explicit Main return declaration = rejected by existing metadata preflight
```

This superseded row does not retire `AppFixedVoid` and does not introduce a
production `AppLastValueOrVoid` policy. A future disconnected compatibility
proof may observe the historical any-statement tail relation through the
test-only `LegacyObservationOracleV1`; it must not install that relation into
a recipe, BODY policy, runtime mode, or public ingress.

## Owner chain

```text
RawRootBodyRecipeV1(AppLastValueOrVoid)
-> InstalledRawRootEnvironmentV1::drive_root_body
-> RawLoweredRootTailV1
-> one PreparedRawRootBodyCommitV1
-> private infallible commit
   signature + Return + completion + exit witness
   tracker seal + draft extraction + function cleanup
-> CompletedRawRootBodyPhysicalV1
-> ROOTBATCH0 borrowed witness validation only
```

No second App finalizer, postprocess return repair, public adapter repair,
Legacy re-entry, retry, fallback, or partial publication is permitted.

## S0-A — route and recipe contract

Change the App exit policy only:

```rust
RawRootBodyRouteV1::AppMain0 { top_level_statement }
RawRootExitPolicyV1::AppLastValueOrVoid
```

Keep App metadata coverage unchanged: zero params, zero param declarations,
no declared return type, no uses, contracts, or attrs. Keep the admitted
LinearScalar0 grammar unchanged:

```text
Literal, Variable, Unary, ordinary Binary
Expr, Print, Local, Assignment, CompoundAssignment
```

Reject `And/Or`, `If`, `Loop`, `LoopRange`, `Return`, `Break`, `Continue`, and
`ScopeBox` at the existing source-facts/recipe boundary.

## S0-B — lowerer tail contract

Do not use `RootBodyResultV1` as both lowering output and tracker completion.
Introduce a Builder-private tail product:

```rust
enum RawLoweredRootTailV1 {
    Empty,
    Value {
        value: ValueId,
        provenance: RawRootTailProvenanceV1,
    },
}

struct RawRootTailProvenanceV1 {
    statement_ordinal: usize,
    kind: RawRootTailKindV1,
}
```

`RawRootTailKindV1` must distinguish:

```text
ExpressionValue
PrintedExpressionValue
AssignedPublishedValue
CompoundAssignedPublishedValue
LastDeclaredLocalValue { binding_ordinal }
```

The last statement owns the tail. The exact rules are:

```text
Expr                  -> expression ValueId
Print                 -> printed expression ValueId
Assignment            -> published assigned ValueId
CompoundAssignment   -> published combined-assignment ValueId
Local                 -> final fresh local binding ValueId
empty body            -> Empty / NoValue
non-empty body        -> Empty is an invariant error
```

An expression whose exact type is `Void` is still `Value(v, Void)`, not
`Empty`. A zero-variable Local is rejected as `EmptyLocalDeclaration` before
physical open. Tail provenance carries ordinal/kind only; the recipe remains
the source-site authority.

## S0-C — one prepared exit and completion commit

Share the existing type-check kernel for Script and App, but create distinct
route dispositions:

```rust
PreparedRawRootExitPlanV1::AppValue {
    block: BasicBlockId,
    value: ValueId,
    ty: MirType,
}

PreparedRawRootExitPlanV1::AppEmptyVoid {
    block: BasicBlockId,
}
```

`AppValue` co-seals:

```text
signature.return_type = ty
physical Return       = Return(value)
completion            = Value(value)
exit witness          = AppValue(block, value, ty)
```

`AppEmptyVoid` emits one synthetic Void constant and co-seals Void,
`NoValue`, and `AppEmptyVoid`. Remove the production `AppVoid` and
`discarded_tail` dispositions.

All checks happen before mutation:

```text
route/policy and provisional signature
current function/block and block openness
ValueId definition in current draft
Builder TypeContext type exists, is not Unknown, and is supported
tracker sealability
```

Introduce a consuming prepared tracker seal:

```rust
ActiveRootBodyCompletionTrackerV1::prepare_seal(result)
  -> PreparedRootBodyCompletionV1
PreparedRootBodyCompletionV1::commit(self)
  -> CompletedRootBodyV1
```

The outer `PreparedRawRootBodyCommitV1` is the only commit consumer. After it
is issued, no `Result`-returning operation remains. This removes the current
post-exit fallible tracker seal and the App completion remap.

## S0-D — witness and ROOTBATCH boundary

Use explicit witness variants:

```text
ScriptValue
ScriptEmptyVoid
AppValue
AppEmptyVoid
```

`RawRootBodyExitWitnessV1::validate` must prove route/disposition,
completion, signature return type, physical Return operand, and synthetic Void
constant where applicable. ROOTBATCH0 performs this borrowed validation before
collector/ledger preparation and retains the witness in
`RawInvocationRootWitnessV1`. It does not infer type, rewrite signature, add a
Return, or alter Main/condition identity.

Main remains key `Main`, symbol `main`, arity 0,
`LegacyReplaceWholePair`; condition remains `condition_fn/1`. Return type is
not added to collector or ledger identity.

## S0-E — typed failure and retention

Replace string-only lower failures with typed causes carrying error kind,
statement ordinal, source site when available, and assignment target where
relevant:

```text
EmptyLocalDeclaration
TailMissingFromNonEmptyBody
UndefinedVariable / UndefinedReturnValue
MissingReturnType / UnknownReturnType / UnsupportedReturnType
BlockAlreadyTerminated
ProvisionalSignatureMismatch
TrackerNotSealable
RouteMismatch / CompletionMismatch / SignatureMismatch / ReturnMismatch
ForeignBrand / ForeignFamily
```

Before physical open, retain the source-bound owner and exact recipe. During
BODY, retain the unpublished session, physical tracker, recipe, open function
state, lowered tail when available, and exact cause. ROOTBATCH rejection must
retain draft, completion, exit witness, session, physical state, and untouched
collector/ledger. Inspection plus `discard(self)` are the only exits.

## Historical parity matrix — input to private COMPAT-P0 only

These rows are historical inputs for a bounded parity observation. They do
not promote App scalar semantics:

```text
empty App
Integer / String / Float / Bool literal App
ordinary binary App
Void-valued tail (Value(v, Void), not Empty)
Print tail
Local tail (last fresh binding)
Assignment tail (published assigned value)
CompoundAssignment tail (published combined value)
helpers + empty main
helpers + scalar main
same MirCompiler reuse after success and typed rejection
callable-Main Omitted; internal Selected evidence unchanged
reportable pre-transform verifier Err retained as evidence
```

Each success compares signature return type, Return operand relation,
completion, exit witness, normalized module snapshot, observable return value,
verification evidence, and compiler reuse. ValueId equality is intra-invocation:
tail = completion = Return operand = witness value; numeric IDs need not match
Legacy across separate invocations.

## Supersession guard

```text
AppLastValueOrVoid canonical producer                  = 0
AppLastValueOrVoid public production consumer          = 0
LegacyAnyStatementValueOrUnit implicit selector        = 0
executable compatibility policy/profile                = 0
LegacyObservationOracleV1 production consumer          = 0
LegacyObservationOracleV1 normal-entry consumer        = 0
LegacyObservationOracleV1 test-only parity consumer    <= 1
current AppFixedVoid code rollback                     = 0
canonical replacement owner                            = FUNCTION-EXIT-F1-RETURN0-S0
compatibility-evidence replacement owner               = RAW-BODY-RETURN-COMPAT-P0
postprocess/adapter return repair                       = 0
Legacy fallback                                        = 0
compile_with_source/JSON/executor/CUT0 consumers        = 0
```

## Non-claims

```text
normal compile_with_source cutover
JSON / Program(JSON v0)
executor, selfhost, fastmem
old Raw-chain retirement
public adapter repair
CUT0 activation
grammar expansion beyond LinearScalar0
```

First executable row:

```text
none — superseded before implementation

Canonical successor:
  FUNCTION-EXIT-F1-RETURN0-S0

Parked compatibility evidence:
  RAW-BODY-RETURN-COMPAT-P0
```
