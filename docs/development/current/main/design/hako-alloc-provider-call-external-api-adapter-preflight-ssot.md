# Hako Alloc Provider Call External API Adapter Preflight

Status: accepted
Decision: accepted
Scope: MIMAP-402A provider-call external API adapter preflight.

## Purpose

MIMAP-402A records preflight readiness for a future external provider API call
after the external API adapter inventory. It keeps external provider API
execution closed.

## Owner

```text
lang/src/hako_alloc/memory/provider_call_external_api_adapter_preflight_box.hako
```

## Input Contract

The adapter preflight consumes:

```text
HakoAllocProviderCallExternalApiAdapterInventoryReport
```

The inventory report must be accepted and must prove adapter presence/validity
with `external_provider_api_call_executed = 0`.

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
