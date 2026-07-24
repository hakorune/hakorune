# RAW-PUBLIC-ADAPTER0-S0 execution task

Decision: `RAW-PUBLIC-ADAPTER-prime-r1`

Status: active implementation. This row closes the compatibility adapter only;
public Raw ingress remains the next disconnected row.

## Scope

```text
RawPublishedInvocationV1
  -> RawPublicationCompatibilityEnvelopeV1
  -> MirCompileResult
```

The adapter is one infallible consuming terminal. Before it can exist,
`RawPublishedModuleV1` is corrected from an installed marker to the opaque
`MirModule` owner published by the same publication terminal. The adapter
opens that owner exactly once and never re-reads the live Builder.

## Internal order

```text
ADAPTER-CARRIER0
  co-publish the opaque module owner and leave live Builder quiescent
ADAPTER-EVIDENCE0
  retain route/runtime/witness/parity/publication evidence without cloning
ADAPTER-I0
  issue the private compatibility envelope and its sole erasure terminal
ADAPTER-G0
  prove one opener, one adapter, no ingress, no JSON change, and <800 lines
```

## Explicit non-claims

```text
compile_raw_with_source
compile_with_source cutover
public ingress, executor, AST-JSON, Program(JSON v0)
legacy Raw finalizer retirement
MirCompileResult production caller beyond the internal adapter fixture
CUT0 activation
```

## Acceptance

```text
Script, App/NotSelected, App/Selected adapter fixtures
Raw pre-transform verifier Err moves once into MirCompileResult
published module is owned by the typed carrier
live Builder current_module remains None after publication
all pre-publication failures remain before the adapter
no fallible operation follows publication
compatibility erasure is one consuming terminal
```

## Sunset and proof budget

`sunset_id = RAW-PUBLICATION-SUNSET-001`.

The old Raw bridge and normal-entry cutover remain for a later measured
retirement row. Retirement requires explicit Raw ingress parity, old Raw
non-test caller zero, and a recorded normal-entry cutover decision.

`ceremony_tier = T2`; every modified or new source and check file stays below
800 lines.
