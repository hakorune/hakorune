---
Status: accepted design
Date: 2026-07-27
Decision: PRELOOP-LOCATED-ARGUMENT-REQUEST-BOUNDARY0-prime-r1
Scope: S0-B semantics after the accepted located-ingress plan
Related:
  - preloop-located-argument-ingress0-d0-design-question-2026-07-27.md
  - src/mir/builder/calls/member_route.rs
  - src/mir/builder/calls/method_call_terminal.rs
---

# Pre-loop Located Argument Request Boundary D0

## Decision

```text
Choice:
  A-prime — fixture-owned emitted Calls, receipt authority zero

Rejected:
  B — non-emitting placeholder / continuation
```

I0 may execute the existing standard and static terminals in its private
configured Builder. Both inner and outer Calls may therefore be emitted.
Emission is not receipt authority: I0 does not construct or export a typed
physical Call receipt, final physical destination, nested-result receipt, or
new nested Integer fact.

## Evidence

The accepted ingress plan requires the selected inner MethodCall to supply a
value as the second argument of the outer static `skip_ws` call.

```text
outer static terminal
  -> emits a Global Call with all lowered argument ValueIds

inner standard terminal
  -> allocates a destination
  -> invokes UnifiedCallEmitterBox
  -> emits a Method Call
```

There is no existing product meaning “prepared Method request with a usable
ValueId but no Call instruction.” Therefore the former wording,
`unified Method request` while not claiming physical Call success, has two
incompatible interpretations.

## Choice A — permit emitted Calls, but issue no receipt (accepted as A-prime)

I0 uses the existing static and standard terminals unchanged. The bounded
fixture may contain emitted Call instructions, but no new owner observes or
claims successful physical Call completion.

```text
I0 proof:
  selected located input reaches existing Standard(Unified) terminal
  + ordinary outer execution receives its returned ValueId

I0 non-claims:
  CompletedUnifiedValueCallEmissionV1 = 0
  final physical destination authority = 0
  nested result receipt = 0
  type publication = 0
```

Existing post-success signature/collection annotations remain unchanged, so
I0 does not claim repository-wide `type_ctx write = 0`. Its exact non-claim is:

```text
new nested Integer publication = 0
```

The following physical-receipt row remains the only owner that observes the
finalized Call destination after `emit_instruction` succeeds. I0 is not a
physical-receipt proof; it merely uses the existing terminal to make the
configured fixture executable.

## Choice B — require no Call emission in I0

This requires a new pre-emission continuation/value authority able to represent
an inner result before it exists physically, and a corresponding outer-call
continuation. It is not a small adapter: a raw `ValueId` without its defining
Call would be invalid MIR.

Reject unless a separate design selects the required continuation/SSA and
publication ownership. Do not fabricate a placeholder `ValueId`.

## Receipt upgrade law

The physical-receipt row upgrades the same generic emitter seam. It must not
attach a receipt retroactively or emit twice in one run.

```text
ordinary API:
  physical generic Call terminal
  -> receipt is internally discarded

receipt-required API:
  same physical generic Call terminal
  -> emit_instruction success
  -> existing post-success commit
  -> retain one source-neutral receipt
```

Special rewrite, BoxCall, legacy compatibility emission, no-destination Calls,
and failed instruction emission issue no value receipt. `Ok(ValueId)` and MIR
scans are not physical-success authority.

## Invariants for either choice

```text
RawLocated -> RawLegacy conversion = 0
second member planner = 0
second ordered argument driver = 0
Builder clone/snapshot = 0
fallback/retry = 0
physical receipt producer in I0 = 0
new nested Integer publication in I0 = 0
production caller = 0
```

## Failure law

```text
outer route mismatch:
  argument effects = 0

inner route mismatch:
  selected inner effects = 0
  earlier Argument(0) effects may exist in fixture Builder

inner/outer terminal failure:
  fixture Builder effects may exist
  exact source association is retained by rejection
  fixture Builder is discarded

inner success then outer failure:
  exact source association is retained by rejection
  fixture Builder is discarded

live compiler Builder mutation = 0
module publication = 0
fallback/retry = 0
```

## Executable task series

### 1. `PRELOOP-LOCATED-ARGUMENT-INGRESS0-S0-B1`

Repair the Port typestate only.

```text
Armed(source)
-> InFlight
-> Rejected(source + cause)
```

Remove payloadless `Consumed` / `Poisoned` and the separate route state. B1
materializes only the currently reachable fail-closed prefix. The concrete
reached/request owner and requested destination belong to B2, where the
success boundary first exists. Do not connect a fixture caller in B1.

#### B1 closeout

```text
Status:
  closed

reachable typestate:
  Armed(source)
  -> InFlight
  -> Rejected(source + CandidateIngressPending)

duplicate projection:
  restores the exact InFlight or Rejected state
  payload loss = 0

removed:
  payloadless Consumed
  payloadless Poisoned
  separate route-state field

deferred to B2:
  reached/request owner
  requested destination
  located ingress
```

The focused actual-ParserBox fixture proves that unselected `Argument(0)`
leaves the source owner armed, the first selected `Argument(1)` moves it to
`InFlight`, duplicate projection preserves that state, and the current
fail-closed terminal retains the exact selected site and index. No Builder
instruction, receipt, type fact, loop-refresh route, or production caller is
added by B1.

### 2. `PRELOOP-LOCATED-ARGUMENT-INGRESS0-S0-B2`

Add one candidate-only located ingress. Require existing prepared products:

```text
outer = StaticReceiver
inner = ReceiverNormalized(MeCall)
Me = Standard(Unified)
```

The selected inner MethodCall is handled directly from its located owner.
Its ordinary receiver/argument children use the wrapped ordinary Port. No raw
dispatcher re-entry or RawLegacy conversion is allowed.

#### B2 closeout

```text
Status:
  closed

source retention:
  Armed(source)
  -> InFlight(source)
  -> Reached(source + requested destination)
     or Rejected(source + typed cause)

selected projection:
  privately sealed one-shot token only
  caller-supplied source payload = 0

inner route:
  existing member planner
  -> ReceiverNormalized(MeCall)
  -> existing effect-free Me preparation
  -> Standard(Unified)
  -> existing ordered argument driver on wrapped ordinary Raw Port
  -> existing standard terminal

configured positive proof:
  outer existing StaticReceiver plan = 1
  inner Method Call = 1
  outer Global Call = 1
  retained requested destination = inner Call dst

still zero:
  typed physical receipt
  final physical destination authority
  nested result receipt
  new nested Integer publication
  Raw dispatcher re-entry for selected inner
  fallback / retry / production caller
```

The outer compatibility transport exists only inside the bounded configured
proof. It does not convert the selected inner located MethodCall. An outer
terminal failure after inner success moves the retained reached owner into the
typed rejection state before the fixture Builder is discarded.

### 3. `PRELOOP-LOCATED-ARGUMENT-INGRESS0-P0`

Use the existing real ParserBox source/catalog fixture and a private configured
Builder. Cover:

```text
Argument(0) ordinary descent
Argument(1) exact selected descent
inner Method Call emission
outer Global Call emission
source association retained
requested destination retained

outer/inner/Me/Standard alternate rejection
duplicate selected descent
foreign/parked source rejection
inner success -> outer failure retention
unified-disabled fail-fast
failure -> fresh fixture success
```

### 4. `PRELOOP-LOCATED-ARGUMENT-INGRESS0-G0`

Repair and consolidate the existing ARG0/ROUTE0/V0 guards. Do not add a new
row wrapper or extend the near-800-line catalog guard.

### 5. `UNIFIED-CALL-PHYSICAL-RECEIPT0-S0/P0/G0`

Add a small sibling `unified_emitter/physical_receipt.rs`. Move terminal code
out of the 724-line unified emitter as needed; do not grow that file past 800.

The sole receipt constructor is:

```text
finalized mir_call.dst
-> emit_instruction(Call) success
-> existing post-success commit
-> CompletedUnifiedValueCallEmissionV1
```

Existing APIs discard the source-neutral receipt. One receipt-required sibling
retains it. No source lookup or nested policy enters the emitter.

### 6. `CALLABLE-RESULT-NESTED-PRELOOP-REP0-S0/I0/P0/G0`

Consume the exact retained source association and one physical receipt to
produce `EmittedNestedInstanceCallV1(final_destination)`. No type write and no
loop-refresh activation occur here.

### 7. `CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-D0`

Select the one-shot Integer publication conflict law. This remains a semantic
stop. At minimum:

```text
None -> publish Integer once
Integer -> accept only with exact existing-authority pairing, otherwise conflict
Unknown/other -> typed conflict
overwrite -> forbidden
Call failure / receipt absence -> publication zero
```

### 8. Type I0/P0/G0 and post-type Stage-B guard

Publish outside GenericLoop. GenericLoop remains consumer-only. After the
actual Stage-B guard reruns, select exactly one next owner:

```text
ownership syntax frontier -> resume OWN-GRAM-REJECT0-HAKO0
loop-refresh frontier      -> GENERIC-LOOP-NESTED-RESULT-ACTIVATION0-D0
other frontier             -> exact new owner stop
```

Loop-refresh is parked, not a mandatory next row.

## File-size and guard constraints

```text
do not grow:
  calls/unified_emitter.rs                  724 lines
  calls/member_route_descent_tests.rs       near 800 and externally dirty
  callable_result_i64_catalog_s0.py         787 lines
  calls/lowering.rs                         near 800

new ingress/receipt/tests:
  separate sibling files, each < 800 lines

existing guards:
  ARG0 green
  ROUTE0/V0 stale and repaired at ingress G0
  catalog guard green and not extended
```

## Required closeout

```text
Decision:
  PRELOOP-LOCATED-ARGUMENT-REQUEST-BOUNDARY0-prime-r1

Status:
  accepted

Choice:
  A-prime

First executable row:
  PRELOOP-LOCATED-ARGUMENT-INGRESS0-S0-B1
```
