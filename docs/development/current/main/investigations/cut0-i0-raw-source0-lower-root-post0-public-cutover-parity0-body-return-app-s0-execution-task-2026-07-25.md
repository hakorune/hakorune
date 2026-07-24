# RAW public cutover PARITY0 App body-return S0

Decision: `BODY-RETURN-APP-prime-r1`

Status: queued for implementation. The consultation is closed with Q1=A. This
task changes only the disconnected explicit Raw App route; normal entry and
all later publication/runtime cutovers remain closed.

## Selected contract

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

`AppFixedVoid` and production `discarded_tail` authority are retired by this
row. The route remains a distinct `AppLastValueOrVoid` variant; it must not
reuse Script policy by symbol/module inference.

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

## S0-F — promotion-blocking parity matrix

All rows below are required before App scalar parity is promoted:

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

## S0-G — structural guard

```text
AppLastValueOrVoid producer                         = 1
AppFixedVoid production producer                   = 0
discarded_tail production occurrence               = 0
RawLoweredRootTailV1 producer                      = 1
BODY exit prepare                                  = 1
PreparedRawRootBodyCommit consumer                 = 1
post-commit fallible tracker seal                  = 0
completion policy remap                            = 0
AppValue/AppEmptyVoid witness producers            = 1 each
ROOTBATCH witness validation                        = 1
ROOTBATCH signature rewrite/second Return          = 0
symbol/module route inference                      = 0
return type in ledger/collector identity           = 0
postprocess/adapter return repair                   = 0
compile_with_source/JSON/executor/CUT0 consumers    = 0
all modified/new source/check files                 < 800 lines
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
RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-CUTOVER-
PARITY0-BODY-RETURN-APP-S0
```
