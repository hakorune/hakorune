# Hako Alloc Provider Call Modeled Open Pilot

Status: accepted
Decision: accepted
Scope: MIMAP-386A provider-call modeled open pilot.

## Purpose

MIMAP-386A is the first row that opens provider-call readiness in model space.
It consumes the MIMAP-384A dry-run unsupported outcome and records a modeled
provider-call-open report.

This is not provider execution. It does not call a provider API, replace the
host allocator, install hooks, add backend matchers, or install
`#[global_allocator]`.

## Owner

```text
lang/src/hako_alloc/memory/provider_call_modeled_open_pilot_box.hako
```

## Input Contract

The modeled-open owner consumes:

```text
HakoAllocProviderCallDryRunUnsupportedBehaviorReport
```

The dry-run report must be accepted, preserve the provider-call capability gate
facts, and record an unsupported provider-call dry-run outcome. MIMAP-386A then
records `provider_call_modeled_open = 1`, `provider_call_model_active = 1`, and
`would_call_provider = 1`.

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
exe = deferred-to-modeled-open-closeout
```
