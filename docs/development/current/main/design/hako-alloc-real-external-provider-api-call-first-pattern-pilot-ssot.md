# Hako Alloc Real External Provider API Call First-Pattern Pilot

Status: accepted
Decision: accepted
Scope: MIMAP-415A real external provider API call first-pattern pilot.

## Purpose

MIMAP-415A opens the first explicitly-scoped real external provider API call
pilot after the adapter execution preflight closeout. It records real-call pilot
evidence but keeps host allocator replacement, hooks, backend matcher additions,
worker/thread execution, and global allocator install closed.

## Owner

```text
lang/src/hako_alloc/memory/real_external_provider_api_call_first_pattern_pilot_box.hako
```

## Input Contract

The pilot consumes:

```text
HakoAllocRealExternalProviderApiAdapterExecutionPreflightReport
```

The preflight report must be accepted and must prove:

```text
real_external_preflight_present = 1
real_external_provider_api_execution_ready = 1
actual_external_provider_api_call_executed = 0
would_execute_real_external_provider_api = 1
```

## Output Contract

Accepted reports publish:

```text
real_external_provider_api_call_executed = 1
real_external_provider_api_result_present = 1
real_external_provider_api_result_code = 0
actual_external_provider_api_call_executed = 1
```

## Still Closed

The following remain closed:

```text
host allocator replacement
hooks
backend matcher additions
worker/TLS or thread execution
process-global allocator install
```

## Validation

```text
validation_profile = first-pattern-exe
exe = required
```
