# HEADERPORT0-REENTRANT-TERM0: pending capture and commit-only terminal

Status: `HEADERPORT0-REENTRANT-TERM0-S0/P0` are closed; `HEADERPORT0-REENTRANT-TERM0-I0` is next
Date: 2026-07-21
Parent: `mirbuilder-headerport-reentrant-terminal-consultation-2026-07-21.md`
Decision: R-prime
docs_only_closeout: forbidden
code_or_artifact_delta_required: 1

## Selected boundary

Raw child lowering and collector mutation are separate phases. The one
`ModuleLoweringPortV1` remains the sole collector commit owner, while the raw
invocation port is reborrowable during child body lowering.

```text
capture child session
  -> port-aware draft/body lowering
  -> nested raw children use shorter reborrows
  -> terminal validation
  -> pending draft returns; all header/raw borrows end
  -> ModuleLoweringPort prepare -> seal -> infallible collect -> restore
```

The S0 row is disconnected: production child consumers remain zero. It adds
only private vocabulary and thin signatures needed by the later P0 proof.
The commit-only pending terminal, neutral signature lookup surface, and
port-aware draft/body/finalizer protocol are now present; no production route
uses the new commit methods.

P0 progress now includes capture-only lifetime seams, rejected-commit parent
restore fixtures, collector replacement preflight, and complete port-aware
body descent for nested static/instance/constructor, TaskScope, and FastMem
containers. The special `Main` root is handled by the same raw port as a
typed pre-effect rejection for invocation sessions; the legacy adapter alone
retains inline-main lowering.

The disconnected body and nested-child fixtures are green. The focused
primary-failure and root-only Main zero-delta fixtures now complement the
existing admission/panic terminal matrix. Remaining P0 work is explicit:
finish the header-loan matrix audit, then keep the existing invocation trait
consumers remain disconnected until the I0 capture/commit cutover.

## Authority

```text
ModuleLoweringInvocationV1
  owns one ModuleDraftCollectorV1

ModuleLoweringPortV1
  owns collector admission and commit only

RawInvocationChildPortV1
  reborrows the same ModuleLoweringPort during lowering

PendingFunctionSessionCloseV1
  owns one successful draft and captured parent state

LoweringHeaderPortV1
  exposes only signature/presence/count/visitor for short loans
```

No S0 product may own a Builder, `MirModule`, metadata, a second collector,
an AST clone authority, a fact session, a site id, or a fallback route.

## S0 products

Add a commit-only terminal operation whose input is an already captured
pending child and an owned identity request. The operation performs no body
lowering closure:

```rust
commit_legacy_pending(pending, request)
commit_resolved_pending(pending, request)
```

The existing closure-owning `complete_*` methods remain disconnected until the
port-aware P0 replaces their production consumers. The new port-aware draft
builder names are thin siblings, not duplicate lowering implementations:

```rust
build_static_method_draft_with_port_v1
build_instance_method_draft_with_port_v1
lower_function_body_with_port_v1
lower_method_body_with_port_v1
finalize_function_draft_with_headers
```

They reuse the existing skeleton, signature, parameter, rune, completion, and
verification owners. Only recursive body descent and signature lookup are
injected.

## Header loan law

The header view is borrowed only for an individual query or finalizer call.
It never spans child body lowering, nested collection, or collector commit.
Production port-aware finalization has no `current_module.functions` fallback.
The lookup surface is exactly:

```text
symbol -> FunctionSignature
symbol presence
symbol count
deterministic symbol visitor
```

No body, metadata, MirModule, type fact, or collector mutation is exposed.

## Required P0 proof

```text
outer raw child with nested static Box
outer raw child with nested instance Box
instance constructor plus ordinary method
invocation Main root rejected before root effects
nested header visible after inner collection
primary / cleanup / admission / panic
collector prefix unchanged on every rejected path
parent context restored exactly once
```

The constructor path must have zero direct
`lower_method_as_function` consumers in the invocation route. Legacy ports may
retain that call as their behavior-preserving adapter.

`Main` is a root-only entry. Invocation lowering must call the port-owned
`lower_static_main_box` decision and reject before instruction, metadata, or
collector mutation; only the legacy adapter may call
`build_static_main_box`.

## Forbidden in S0/P0

```text
production cutover
second collector or Builder-held port
module/header cache
post-collect assertion or fallible lookup
collector commit from RawInvocationChildPortV1
FACTSESSION0, PHI, TypePipeline, JoinIR, MODULETX, or CUT0 changes
```

## Task order

```text
HEADERPORT0-REENTRANT-TERM0-S0
  private vocabulary and port-aware signatures; consumers = 0
-> HEADERPORT0-REENTRANT-TERM0-P0
  nested/re-entry/failure matrix and collector preflight proof
  all disconnected P0 fixtures/guards green
-> HEADERPORT0-REENTRANT-TERM0-I0
  raw invocation capture/commit cutover
-> HEADERPORT0-REENTRANT-TERM0-G0
  direct bypass and closure-owner retirement guards
-> FINALIZE0-MODULEDRAFT0-HEADERPORT0-I0
```
