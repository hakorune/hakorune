---
Status: consultation required
Date: 2026-07-27
Decision: PRELOOP-LOCATED-ARGUMENT-REQUEST-BOUNDARY0-D0
Scope: S0-B semantics after the accepted located-ingress plan
Related:
  - preloop-located-argument-ingress0-d0-design-question-2026-07-27.md
  - src/mir/builder/calls/member_route.rs
  - src/mir/builder/calls/method_call_terminal.rs
---

# Pre-loop Located Argument Request Boundary D0

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

## Choice A — permit emitted Calls, but issue no receipt (recommended)

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

## Required decision

Choose A or B. Until then, S0-B may not add an ingress, selected-state repair,
or fixture caller.

## Invariants for either choice

```text
RawLocated -> RawLegacy conversion = 0
second member planner = 0
second ordered argument driver = 0
Builder clone/snapshot = 0
fallback/retry = 0
physical receipt producer in I0 = 0
type publication in I0 = 0
production caller = 0
```
