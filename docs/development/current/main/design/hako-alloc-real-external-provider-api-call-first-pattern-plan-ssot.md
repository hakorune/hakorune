# Hako Alloc Real External Provider API Call First-Pattern Plan

Status: accepted
Decision: accepted
Scope: MIMAP-414A real external provider API call first-pattern plan.

## Purpose

MIMAP-414A defines the first-pattern real external provider API call seam after
the MIMAP-410A/MIMAP-412A adapter execution preflight pack. It is a planning row
only and does not execute external provider APIs.

## First-Pattern Boundary

The first pilot may consume:

```text
HakoAllocRealExternalProviderApiAdapterExecutionPreflightReport
```

The input report must be accepted and must prove:

```text
real_external_preflight_present = 1
real_external_provider_api_execution_ready = 1
actual_external_provider_api_call_executed = 0
would_execute_real_external_provider_api = 1
```

## Required Pilot Report Shape

The eventual pilot must publish a report that keeps these boundaries visible:

```text
accepted
reason
real_external_preflight_present
real_external_provider_api_execution_ready
real_external_provider_api_call_executed
real_external_provider_api_result_present
real_external_provider_api_result_code
host_replacement_inactive
hooks_inactive
backend_matcher_inactive
would_replace_host_allocator
would_install_hook
would_add_backend_matcher
would_run_thread
```

## Still Closed In This Row

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
validation_profile = planning
exe = not-applicable
```
