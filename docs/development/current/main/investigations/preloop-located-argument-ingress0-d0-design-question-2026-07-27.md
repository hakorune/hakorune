---
Status: accepted design
Date: 2026-07-27
Decision: PRELOOP-LOCATED-ARGUMENT-INGRESS0-prime-r1
Scope: the exact pre-loop proof-only ingress after PORT0
Related:
  - preloop-located-argument-descent0-d0-design-question-2026-07-27.md
  - src/mir/builder/calls/preloop_located_argument_port.rs
  - src/mir/source_instance_result_contract/preloop_located_argument.rs
---

# Pre-loop Located Argument Ingress

## Accepted decision

```text
Choice:
  A-prime — candidate-only located ingress

Builder transaction:
  no clone / snapshot
  no ingress-owned transaction
  one bounded proof fixture owns and discards its configured Builder

Route authority:
  existing member planner
  existing Me and Standard prepared-route owners
```

The source-sealed selected argument reaches one unified Method request only.
The request is not evidence of a successfully emitted physical Call, a final
destination, a nested-result receipt, or type publication.

## Why execution stopped

`PRELOOP-LOCATED-ARGUMENT-PORT0-S0` is closed. It keeps the exact selected
`CallArgument(1)` association in a candidate-only Port and retains the sole
ordered argument driver.

The I0 audit found two facts that invalidate the card's assumed direct wiring:

```text
Raw expression dispatcher:
  ExpressionInput = ASTNode
  MethodCallInput = RawLegacyMethodCallInputV1

CanonicalModuleLoweringSessionV1:
  opens a blank Builder
  does not retain the active function / me / static-box context
```

Connecting the candidate Port through the raw dispatcher therefore requires
the prohibited `RawLocated -> RawLegacy` conversion. Reusing the module
session as an isolated transaction loses the exact context required by the
pre-loop fixture. `LocatedLegacyLoweringSessionV1` is disconnected legacy
proof machinery and is not an alternative production or candidate ingress.

## Preserved facts

```text
source authority:
  PreparedPreloopLocatedArgumentV1
  = same catalog allocation
  + exact outer call
  + structural CallArgument(1)
  + exact inner located MethodCall

ordered argument authority:
  existing drive_call_arguments_v1

route authority:
  existing member-route and Me/Standard prepare/execute owners

physical Call authority:
  existing unified emitter

receipt / type publication:
  later rows only
```

## Considered choices

### A — candidate-only located ingress (accepted)

Add a small `calls/preloop_located_argument_ingress.rs`.

```text
selected tagged expression input
  -> candidate-only located ingress
  -> existing route preparation
  -> accept only:
       outer StaticReceiver
       inner Standard(UnifiedMethod)
  -> existing unified Method request terminal
```

The ingress does not implement the raw expression dispatcher, create a
second member planner, or convert the selected input to RawLegacy syntax.
Ordinary receiver and argument descent continue through the wrapped raw Port.

I0 is one bounded proof-fixture consumer with a real configured function/me
context. It does not introduce a general Builder snapshot or clone. Its
failure law is intentionally limited to the fixture owner:

```text
production caller = 0
publication = 0
receipt = 0
type publication = 0
candidate failure is discarded with its fixture Builder
fresh fixture candidate may run afterward
```

### B — general context-preserving Builder transaction

Create a snapshot/clone transaction for arbitrary active Builder context.

This is rejected unless a separate transaction D0 proves a complete state
inventory, restoration law, and publication boundary. The current module
session cannot be widened implicitly for this purpose.

### C — adapt through RawLegacy or located legacy lowering

Rejected. It either erases the selected source identity or activates an
unrelated disconnected legacy family.

## Non-negotiable constraints

```text
second raw source-navigation engine = 0
selected RawLocated -> RawLegacy conversion = 0
second ordered argument driver = 0
second member route planner = 0
callee-name / Box-name route policy = 0
Builder source-site map = 0
persistent source-site -> ValueId map = 0
physical Call receipt = 0
EmittedNestedInstanceCallV1 = 0
MirType / type_ctx publication = 0
loop-refresh activation = 0
fallback / retry / route reselection = 0
production caller = 0
```

## Executable series

```text
Decision:
  PRELOOP-LOCATED-ARGUMENT-INGRESS0-prime-r1

Choice:
  A-prime

First executable row:
  PRELOOP-LOCATED-ARGUMENT-INGRESS0-S0-A

Implementation:
  route prepare / execute seam
  -> candidate-only ingress and payload-retaining typestate
  -> bounded configured proof fixture
  -> G0
  -> UNIFIED-CALL-PHYSICAL-RECEIPT0-S0

Proof:
  selected Argument(1) reaches a unified Method request
  Argument(0) remains ordinary
  alternate route rejects before selected argument descent
  selected success retains its source association
  candidate rejection is isolated to its proof fixture
```

The selected state must retain the exact source association after either
success or failure. A payloadless `Consumed` or `Poisoned` state is forbidden.

Implementation is authorized only for the executable series above.

## S0-A closeout

Closed. `plan_member_call_route()` remains the sole member-route planner.
`build_member_method_call_v1()` now plans once and delegates the preselected
route to `execute_prepared_member_call_route_v1()` without a second probe.

`prepare_me_call_execution_v1()` is the source-neutral, effect-free Me
preparation seam. The ordinary Me policy delegates to it and retains its
existing execution owner. The focused test proves preparation emits no MIR
before the ordinary executor runs.

```text
ordinary route behavior = unchanged
candidate ingress = 0
physical receipt = 0
type publication = 0
production caller = 0
```

## Supersession note

`PRELOOP-LOCATED-ARGUMENT-REQUEST-BOUNDARY0-prime-r1` now selects A-prime:
fixture-owned inner/outer Call emission is allowed while typed physical receipt
authority remains zero. S0-B is split into B1 typestate repair and B2 located
ingress connection; the request-boundary card is their current task authority.
