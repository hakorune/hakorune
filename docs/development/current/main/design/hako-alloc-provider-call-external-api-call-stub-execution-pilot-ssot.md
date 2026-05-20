# Hako Alloc Provider Call External API Call Stub Execution Pilot

Status: accepted
Decision: accepted
Scope: MIMAP-406A provider-call external API call stub execution pilot.

## Purpose

MIMAP-406A opens only a model-space external provider API call stub execution
seam after the external API adapter closeout. It records a stub result while
actual external provider API calls remain closed.

## Owner

```text
lang/src/hako_alloc/memory/provider_call_external_api_call_stub_execution_pilot_box.hako
```

## Input Contract

The stub execution pilot consumes:

```text
HakoAllocProviderCallExternalApiAdapterPreflightReport
```

The preflight report must be accepted and ready, and must prove
`external_provider_api_call_executed = 0`.

## Execution Boundary

This row opens only the stub/model-space external provider API call execution
marker:

```text
external_provider_api_stub_call_executed = 1
external_provider_api_stub_result_present = 1
actual_external_provider_api_call_executed = 0
```

## Still Closed

The following remain closed:

```text
actual external provider API calls
host allocator replacement
hooks
backend matcher additions
worker/TLS or thread execution
process-global allocator install
```

## Validation

```text
validation_profile = scalar-mir
exe = deferred-to-external-provider-api-call-closeout
```
