# Generic raw structured current-owner static-call result publication D0

Status: `design stop opened 2026-08-07; implementation is not authorized`

Parent boundary:

- `docs/development/current/main/investigations/generic-raw-structured-generic-loop-carrier-representation-d0-task-2026-08-07.md`
- `src/mir/callable_result_representation/static_exact_i64_requirement.rs`
- `src/mir/source_call_target/current_owner.rs`
- `src/mir/builder/calls/method_call_terminal.rs`
- `src/mir/builder/stmts/variable_stmt.rs`

## Exact source boundary

The canonical strict probe reaches `StringHelpers.int_to_str/1` and observes:

```hako
local v = me.to_i64(n)
```

The source-site identity is:

```text
caller:  StringHelpers.int_to_str/1
site:    Body(0).Initializer(0)
receiver child: Body(0).Initializer(0).Receiver (AST Me)
argument child: Body(0).Initializer(0).Argument(0)
target:  CurrentOwnerStatic(StringHelpers.to_i64/1)
final local: ValueId(3)
```

`StringHelpers.to_i64/1` is exact-I64 only when the whole source target
inventory includes its current-owner `_digit_value` dependency.  That proof is
source-only; it is not itself a Builder `type_ctx` write.

## Current failure and authority split

The existing lowering order is:

```text
me.to_i64(n) ordinary lowered-global terminal
  -> call result destination (no source return annotation)
local materializer
  -> Copy(final local, call result)
  -> metadata::propagate(call result, final local)
  -> variable_map["v"] = final local
GenericLoop verifier
  -> type_ctx[ValueId(3)]
  -> MissingTransientType
```

The GenericLoop consumer is correct and remains verifier-only.  The local
materializer is transport-only and must not infer an Integer.  The missing
authority is a successful current-owner static-call result publication bridge.

## Recommended design

Reuse the existing source-only products instead of creating a GenericLoop
special case:

```text
VerifiedWholeSourceMethodCallInventory
  + VerifiedSourceStaticCallTargetCatalogV1
  + VerifiedSameModuleCallableResultCatalogV1
  -> VerifiedStaticExactI64RequirementV1
  -> source-site publication demand (non-Clone)
  -> successful generic physical Call receipt
  -> consume_once(Integer result publication)
  -> type_ctx[call_result_destination] = Integer
  -> existing local metadata::propagate
  -> existing GenericLoop verifier
```

The sole lowering-time type write must occur after the physical `Call` succeeds
and before the local copy transports the fact.  The call terminal's existing
`CompletedUnifiedValueCallEmissionV1` is the final destination authority; the
new publication receipt may not allocate or guess a `ValueId`.

Source-site transport must use the already located source context/activation
product.  A method name, Box name, route string, final metadata, runtime value,
or source annotation alone is not a selector.  The exact source contract and
the terminal receipt must be co-sealed before the publication plan is issued.

## Rejected alternatives

```text
GenericLoop backfill/default Integer              = reject
local materializer type inference                 = reject
final route metadata as lowering authority       = reject
source-name or callee-name branch                 = reject
stretch CALLABLE-RESULT-NESTED-REP0               = reject
new source annotation as the sole proof           = reject
retry through another route                       = reject
```

`CALLABLE-RESULT-NESTED-REP0` is for the separately parked nested required-
argument boundary.  This failure is a root static-call result in a Local
initializer and must use the general static-call activation/publication family
or a separately named narrow bridge.

## Failure and rollback contract

```text
source target/result proof unavailable   -> typed pre-effect reject
call takes an alternate route            -> no publication receipt
physical call fails                      -> no type fact
publication consumes twice               -> typed freeze
function-local transaction rollback      -> no leaked call-result/local type
```

The route-attempt transaction risk is adjacent but separate: current candidate
composition snapshots `variable_map` without snapshotting `type_ctx` or the
physical stream.  It must not be repaired by this row's type publication; it
gets its own bounded audit before production cutover.

## Minimum I0 after this D0

Only one natural source site is admitted: `int_to_str/1 Body(0).Initializer(0)`.
The I0 implementation must add one success-only publication receipt and focused
negative tests for foreign site, unavailable result, alternate route, failed
call, and duplicate consumption. It must not open Generic production,
physical cutover, legacy deletion, retry/fallback, broad unannotated-call
inference, or field/local type inference.

The implementation commit must update the exact `docs/reference/**` row,
immutable probe receipt, current mirrors, and this task's closeout together.
