# Hako Alloc Provider Activation Modeled Open Pilot

Status: accepted
Decision: accepted
Scope: MIMAP-380A provider activation modeled open pilot.

## Purpose

MIMAP-380A is the first row that opens provider activation in model space. It
consumes the MIMAP-378A dry-run unsupported outcome and records a modeled
activation-open report.

This is not provider execution. It does not call a provider API, replace the
host allocator, install hooks, add backend matchers, or install
`#[global_allocator]`.

## Owner

```text
lang/src/hako_alloc/memory/provider_activation_modeled_open_pilot_box.hako
```

## Input Contract

The modeled-open owner consumes:

```text
HakoAllocProviderActivationDryRunUnsupportedBehaviorReport
```

The dry-run report must be accepted and must preserve the explicit input bundle
facts. MIMAP-380A then records `provider_activation_modeled_open = 1`,
`provider_activation_model_active = 1`, and `would_activate_provider = 1`.

## Still Closed

The following remain closed and must stay zero/inactive:

```text
provider API calls
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
