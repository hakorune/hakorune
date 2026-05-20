# Hako Alloc Provider Call Real API Stub Execution Pilot

Status: accepted
Decision: accepted
Scope: MIMAP-396A provider-call real API stub execution pilot.

## Purpose

MIMAP-396A opens the first stubbed provider API call execution seam after the
real API execution preflight. It records model-space provider API call
execution evidence and a stub result while actual provider API calls remain
closed.

## Owner

```text
lang/src/hako_alloc/memory/provider_call_real_api_stub_execution_pilot_box.hako
```

## Input Contract

The stub execution pilot consumes:

```text
HakoAllocProviderCallRealApiExecutionPreflightReport
```

The preflight report must be accepted, present, capability-valid, and ready. It
must still prove that host replacement, hooks, backend matcher additions, and
worker/thread behavior are inactive.

## Execution Boundary

This row opens only the stub/model-space provider API call execution marker:

```text
provider_call_stub_execution_open = 1
provider_api_stub_call_executed = 1
provider_api_call_result_present = 1
actual_provider_api_call_executed = 0
```

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
exe = deferred-to-provider-call-closeout
```
