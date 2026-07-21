# HEADERPORT0-ACCESS0: `me` method header ownership consultation

Status: **Candidate A-prime typed-source refinement selected; S0/P0/I0 closed;
G0 is next; production capture/commit and CUT0 remain disconnected**

Date: 2026-07-21

Parent:
`mirbuilder-headerport-i0-production-cutover-consultation-2026-07-21.md`

## Trigger

The disconnected header lookup seam now reaches the generic invocation method
terminal.  The next reader is `method_call_handlers.rs`, where `me.method(...)`
classifies a lowered function from its parameter list before lowering the
arguments.

That handler is shared by three surfaces:

```text
legacy raw port
invocation raw port
located legacy lowering session
```

The invocation port must read completed headers from its collector.  The
located session must not acquire an accidental collector fallback, and the
legacy facade must preserve its current `current_module` behavior.  Adding a
mandatory header trait to the shared handler therefore changes more than one
authority family.

## Concrete evidence

`MeCallPolicyBox::resolve_me_call` currently reads:

```rust
builder.current_module.functions[fname].signature.params
```

before it lowers arguments.  The same `MethodCallValueTerminalPortV1` surface
is also used by `build_method_call_from_input_v1`, including a located legacy
session that does not implement the invocation collector port.

The current standard terminal already has a short-lived header hook, but that
hook is too late for `me` classification: the parameter list determines
whether the receiver is prepended before the terminal is called.

## Non-negotiable laws

```text
explicit invocation header present:
  no current_module fallback

header loan lifetime:
  ends before argument descent and before terminal mutation

header observation:
  owns no Builder, ValueId, metadata, route, or retry authority

legacy raw facade:
  behavior and current_module parity remain unchanged

located route:
  no collector ownership is fabricated

me/static/instance policy:
  one classification owner; no second by-name dispatcher

production capture/commit and CUT0:
  remain disconnected
```

## Candidates

### Candidate A — owned parameter observation at the raw-port boundary

Add a raw-only observation capability whose result is an owned, immutable
parameter projection, for example:

```rust
RawFunctionHeaderObservationV1 {
    symbol: String,
    params: Box<[MirType]>,
}
```

`RawInvocationChildPortV1` obtains it through a short `with_headers` loan.
The legacy port snapshots the existing module signature.  The method handler
receives the optional projection and never reads `current_module` when it is
present.  Located sessions keep their existing route adapter and do not need a
collector capability.

Advantages:

```text
loan ends before argument lowering
shared me policy remains one owner
no Builder/TLS field
explicit invocation path has no fallback
```

Risk: the projection API must remain a temporary representation-only product,
not a second callable catalog or target authority.

### Candidate B — require header capability on every method port

Extend `MethodCallValueTerminalPortV1` or `MethodCallDescentPortV1` with a
header lookup method and implement `None` for legacy/located ports.

Rejected for now.  A `None` implementation in the shared handler makes it too
easy to reintroduce `current_module` as an implicit fallback, and it forces
located ports to implement an authority they do not own.

### Candidate C — duplicate an invocation-specific `me` handler

Rejected.  It creates a second receiver/arity policy and would drift from the
legacy route.  The shared handler must remain the sole policy owner.

### Candidate D — move `me` classification to the canonical callable catalog

Parked.  The first headerport profile still supports legacy raw syntax and
does not admit a general canonical callable resolver for this route.

## Recommended decision

Select **Candidate A-prime with the typed-source refinement** if and only if the projection is explicitly
construction-only, non-Clone, and contains only the parameter disposition
needed by `me` classification.  The method handler should consume it as an
optional input:

```text
Some(projection):
  use projection only

None:
  legacy/located facade keeps its already-owned behavior
```

The projection must not be persisted in `AssociatedMethodCallArgumentsV1`,
`MirBuilder`, `CompilationContext`, or a module cache.  It should live only
until the method-kind decision is complete.

## Proposed task order after decision

```text
ACCESS0-MEHEADER-CONSULT0
  this decision; code delta = 0

ACCESS0-MEHEADER-S0
  (closed) disconnected owned parameter observation and matrix

ACCESS0-MEHEADER-P0
  (closed) legacy/raw/located parity and no-fallback proof

ACCESS0-MEHEADER-I0
  (closed) shared `MeCallPolicyBox` consumes typed observation through all
  three route adapters; terminal lookup remains separate

ACCESS0-MEHEADER-G0
  next: one shared policy owner, no direct header fallback, and no long-lived
  observation loan
  shared me policy owner = 1
  invocation current_module fallback = 0
  located collector capability = 0

then:
  ACCESS0-REWRITE-KNOWN-P0 closeout
```

## ACCESS0-MEHEADER-S0 closeout

The disconnected S0 vocabulary owns one typed-source observation and one pure
receiver/arity preparation product:

```text
MeCallHeaderSourceV1
  ModuleCompatibility | InvocationCollector

MeCallParameterObservationV1
  Missing { source, symbol }
  Present { source, symbol, parameter_count, first_parameter }

PreparedMeLoweredCallV1
  expected_params + PreparedMeReceiverV1
```

The observation is owned and non-Clone. It is created from one explicit
`FunctionSignatureLookupV1` source and carries no return type, effects, body,
metadata, Builder, collector borrow, ValueId, route, or retry authority.
`prepare_me_lowered_call_v1` consumes it and returns `None` for a
source-branded miss without providing a fallback lookup.

The method handler and every production method port remain disconnected:

```text
MeCallPolicyBox consumers = 0
MeCallHeaderObservationPortV1 implementations = 0
MethodCallLoweringPortV1 consumers = 0
current_module fallback changes = 0
```

Three focused fixtures cover source-branded miss, first-parameter `Box`
instance preparation, and empty/non-`Box` static preparation. The reusable
S0 guard is `tools/checks/lib/me_call_header_observation_guard.py`.
`ACCESS0-MEHEADER-P0` is now closed as a disconnected proof; I0 must still
prove the same laws while connecting only the shared `MeCallPolicyBox`.

## ACCESS0-MEHEADER-P0 closeout

The P0 matrix keeps the three route families on their selected header source:

```text
legacy compatibility:
  ModuleCompatibility

located compatibility:
  ModuleCompatibility, with no collector capability

invocation raw:
  InvocationCollector only
```

The disconnected fixtures prove that compatibility and invocation signatures
do not cross-contaminate, an invocation miss remains `Missing` even when a
stale module signature exists, and the owned observation can be followed by a
header mutation before preparation. The source guard also proves that existing
strict/warning arity diagnostics remain after `lower_all`, so P0 does not move
error timing ahead of argument descent.

```text
source miss fallback = 0
long-lived header loan = 0
MeCallPolicyBox consumers = 0
production method-port consumers = 0
```

The P0 row is now closed. `ACCESS0-MEHEADER-I0` connects only the shared
policy; terminal-time lookup/result annotation and production
MODULEDRAFT/FACTSESSION capture remain outside this row.

## ACCESS0-MEHEADER-I0 closeout

The shared `MeCallPolicyBox` now consumes one owned typed observation before
argument descent through all three route adapters:

```text
RawLegacyChildLoweringPortV1:
  ModuleCompatibility

RawInvocationChildPortV1:
  InvocationCollector only, including an explicit miss

LocatedLegacyLoweringSessionV1:
  ModuleCompatibility, with no collector capability
```

The header loan ends before `lower_all`, and the existing strict/warning arity
diagnostics, missing-receiver timing, and terminal-time result lookup remain
unchanged. The handler has no direct module-map read and an invocation miss
cannot fall back to `current_module`.

```text
MeCallPolicyBox consumers = 1
MeCallHeaderObservationPortV1 route adapters = 3
direct method-handler module reads = 0
invocation miss -> current_module fallback = 0
header loan across argument descent = 0
observation persistence/cache = 0
terminal result-annotation redesign = 0
```

`ACCESS0-MEHEADER-G0` is now the sole next row. `ACCESS0-REWRITE-KNOWN-P0`
and the production capture/commit/CUT0 rows remain outside this closeout.

## Required fixtures

```text
missing header
static method params == source arity
instance method params == source arity + receiver
arity mismatch strict error
arity mismatch legacy warning parity
invocation header present while current_module is stale
legacy header snapshot parity
located route remains collector-free
argument descent occurs only after the header loan ends
```

## Stop conditions

Stop and reopen consultation if any candidate requires:

```text
header lookup stored in Builder/TLS
current_module fallback after an explicit invocation lookup
AST/name reconstruction of parameter shape
second me/static receiver policy
long-lived borrow across argument descent
located session borrowing the invocation collector
production root cutover in this row
```
