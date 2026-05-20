# Hako Alloc Provider Call No-Op Execution Seam Pilot

Status: accepted
Decision: accepted
Scope: MIMAP-390A provider-call no-op execution seam pilot.

## Purpose

MIMAP-390A opens the provider-call execution boundary as a no-op model seam. It
consumes the MIMAP-388A execution capability preflight and records that the
explicit execution seam was crossed without calling any provider API.

## Owner

```text
lang/src/hako_alloc/memory/provider_call_noop_execution_seam_pilot_box.hako
```

## Input Contract

The no-op seam owner consumes:

```text
HakoAllocProviderCallExecutionCapabilityPreflightReport
```

The preflight report must be accepted and execution-ready. MIMAP-390A then
records `provider_call_noop_execution_open = 1` and
`provider_call_noop_executed = 1` while keeping `provider_api_call_executed = 0`.

## Still Closed

The following remain closed and must stay inactive:

```text
actual provider API calls
host allocator replacement
hooks
backend matcher additions
worker/TLS or thread execution
process-global allocator install
```

## Validation

```text
validation_profile = scalar-mir
exe = deferred-to-real-provider-call-or-closeout
```
