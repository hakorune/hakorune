# HEADERPORT0-REENTRANT-TERM0: pending capture and commit-only terminal

Status: `HEADERPORT0-REENTRANT-TERM0-S0` is closed; `HEADERPORT0-REENTRANT-TERM0-P0` is active
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
restore fixtures, and collector replacement preflight. Full port-aware body
descent, nested constructor coverage, and the complete header-loan matrix are
still required before P0 closes.

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
nested header visible after inner collection
primary / cleanup / admission / panic
collector prefix unchanged on every rejected path
parent context restored exactly once
```

The constructor path must have zero direct
`lower_method_as_function` consumers in the invocation route. Legacy ports may
retain that call as their behavior-preserving adapter.

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
-> HEADERPORT0-REENTRANT-TERM0-I0
  raw invocation capture/commit cutover
-> HEADERPORT0-REENTRANT-TERM0-G0
  direct bypass and closure-owner retirement guards
-> FINALIZE0-MODULEDRAFT0-HEADERPORT0-I0
```
