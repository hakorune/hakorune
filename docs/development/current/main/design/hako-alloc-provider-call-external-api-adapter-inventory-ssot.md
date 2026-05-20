# Hako Alloc Provider Call External API Adapter Inventory

Status: accepted
Decision: accepted
Scope: MIMAP-400A provider-call external API adapter inventory.

## Purpose

MIMAP-400A inventories the external provider API adapter boundary after the
provider-call real API stub execution closeout. It records adapter
presence/readiness and keeps external provider API execution closed.

## Owner

```text
lang/src/hako_alloc/memory/provider_call_external_api_adapter_inventory_box.hako
```

## Input Contract

The adapter inventory consumes:

```text
HakoAllocProviderCallRealApiStubExecutionPilotReport
```

The stub report must be accepted, present, and must record stub/model-space
provider API call execution with `actual_provider_api_call_executed = 0`.

## Still Closed

The following remain closed:

```text
external provider API calls
host allocator replacement
hooks
backend matcher additions
worker/TLS or thread execution
process-global allocator install
```

## Validation

```text
validation_profile = scalar-mir
exe = deferred-to-external-provider-api-call-pilot-or-closeout
```
