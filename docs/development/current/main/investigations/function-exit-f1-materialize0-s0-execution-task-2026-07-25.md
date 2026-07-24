# Function-exit F1 MATERIALIZE0 S0

Decision authority: `FUNCTION-EXIT-SEMANTICS-prime-r1`

Status: next executable task after `FUNCTION-EXIT-F1-RETURN0-S0`.

First executable row:

```text
FUNCTION-EXIT-F1-MATERIALIZE0-S0
```

Normative SSOT:

- `docs/reference/language/function-exit-and-entry-result.md`
- `docs/reference/language/semantic-kernel.md`
- `docs/reference/language/types.md`
- `docs/development/current/main/investigations/function-exit-f1-return0-s0-execution-task-2026-07-25.md`

## Objective

Consume the sealed F1 source completion contract at the existing Builder
completion owner. Materialization may create the physical signature,
terminator, and completion draft, but it must not re-read source AST, infer a
return from the last lowered `ValueId`, or create a second completion owner.

The input is the exact S0 topology only:

```text
zero root-body Return exits
or one terminal root-body Return exit
```

Nested, multiple, cleanup-bearing, and all-path CFG coverage remain outside
this row and must fail at their existing typed boundary.

## Authority lock

`VerifiedFunctionCompletionV1::function_exit_contract()` is the sole source
exit authority. The existing `ResolvedFunctionCompletionConsumptionV1` /
`finalize_ready_function_completion` family is the only physical completion
consumer to be narrowed for this row.

The materializer must consume, not reconstruct:

```text
SealedFunctionExitDispositionV1
FunctionExitCoverageV1
DeclaredFunctionResultContractV1
existing ReturnExitContract relation
```

Forbidden:

```text
second AST walk
last-lowered ValueId as signature authority
MIR terminator scan as source completion authority
postprocess return repair
fallback to Legacy build_module
new physical Return writer
second signature writer
```

## Required prepare/commit boundary

Preparation is borrow-only and must validate:

```text
completion owner/target matches current function
exact 0/1 coverage matches the open Builder function
explicit site is consumed exactly once
implicit Unit has the exact body site/end
current block is open before synthetic Unit Return
explicit Return operand exists when the sealed disposition requires one
declared contract relation is not silently weakened
```

After the prepared materialization product is issued, the private commit is
infallible and performs the one physical transition:

```text
sealed disposition
  -> signature return type / physical Return / completion draft
```

There is no fallible operation after the commit begins. A failure before the
commit retains the complete unpublished Builder owner and the typed cause;
retry, repair, fallback, and partial publication are forbidden.

## Exact S0 matrix

```text
ImplicitUnit / EmptyBody
  -> one synthetic Void value + Return(Void)
  -> completed physical signature is Void

ImplicitUnit / ImplicitFallthrough
  -> same Unit materialization, with fallthrough provenance retained

ExplicitUnit / ExplicitVoid or ExplicitNull or BareReturn
  -> Unit materialization only when the current physical route admits it

ExplicitValue
  -> consume the already-lowered exact value operand
  -> do not infer its source type from position or symbol
```

`ScriptLastExpressionOrUnit`, physical Main/source-entry transport, process
exit projection, dynamic result carriers, and public activation are later
rows. `AppLastValueOrVoid` remains compatibility evidence only.

## Acceptance gates

```text
existing completion consumers green
explicit/implicit S0 materialization fixtures green
no second source walk or ReturnExitContract producer
Builder/runtime/public behavior delta = 0 outside this row
all modified source/check files < 800 lines
```

Non-claims: nested/multiple/all-path completion, Script tail activation,
process exit, JSON, executor, normal-entry cutover, Raw compatibility
execution, old-chain retirement, and CUT0.
