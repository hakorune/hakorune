# Hako Alloc Provider Call Execution Capability Preflight

Status: accepted
Decision: accepted
Scope: MIMAP-388A provider-call execution capability preflight.

## Purpose

MIMAP-388A inventories the explicit capability preflight required before
provider API call execution can open. It consumes the MIMAP-386A modeled-open
report and records a readiness report in model space.

This is not provider execution. It does not call a provider API, replace the
host allocator, install hooks, add backend matchers, or install
`#[global_allocator]`.

## Owner

```text
lang/src/hako_alloc/memory/provider_call_execution_capability_preflight_box.hako
```

## Input Contract

The preflight owner consumes:

```text
HakoAllocProviderCallModeledOpenPilotReport
```

The modeled-open report must be accepted and must expose
`provider_call_modeled_open = 1`, `provider_call_model_active = 1`, and
`would_call_provider = 1` while the actual execution seam remains closed.

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
exe = deferred-to-execution-seam-or-closeout
```
