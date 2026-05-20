# Hako Alloc Provider Call Real API Execution Preflight

Status: accepted
Decision: accepted
Scope: MIMAP-392A provider-call real API execution preflight.

## Purpose

MIMAP-392A inventories readiness for a future real provider API call after the
no-op execution seam. It keeps actual provider API execution closed.

## Owner

```text
lang/src/hako_alloc/memory/provider_call_real_api_execution_preflight_box.hako
```

## Input Contract

The real API preflight owner consumes:

```text
HakoAllocProviderCallNoopExecutionSeamPilotReport
```

The no-op seam report must be accepted and must prove the explicit no-op
execution boundary crossed without a provider API call. MIMAP-392A records
`provider_api_call_ready = 1` and `would_execute_provider_api = 1` while keeping
`provider_api_call_executed = 0`.

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
exe = deferred-to-real-provider-api-call-pilot-or-closeout
```
