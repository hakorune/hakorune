# Hako Alloc Host Replacement Explicit Preflight Inventory

Status: accepted
Decision: accepted
Scope: MIMAP-420A host replacement explicit preflight inventory.

## Purpose

MIMAP-420A inventories the explicit preflight inputs that must exist before any
future optional host replacement work can proceed. It consumes the real
external provider API call first-pattern report and records whether the
replacement request, hook plan, rollback plan, and backend no-growth evidence
are present.

This row does not execute host replacement. It only records readiness in
model/scalar space.

## Input Contract

The owner consumes:

```text
HakoAllocRealExternalProviderApiCallFirstPatternPilotReport
```

Accepted inventory requires:

```text
first_pattern_present == 1
accepted == 1
real_external_provider_api_call_executed == 1
real_external_provider_api_result_present == 1
actual_external_provider_api_call_executed == 1
explicit_request_present == 1
hook_plan_present == 1
rollback_plan_present == 1
backend_no_growth_present == 1
```

The owner rejects if the prior report leaks any closed seam:

```text
would_replace_host_allocator != 0
would_install_hook != 0
would_add_backend_matcher != 0
would_run_thread != 0
```

## Report Contract

Accepted reports set:

```text
host_replacement_preflight_inventory_present = 1
host_replacement_preflight_ready = 1
```

Execution fields remain closed:

```text
host_replacement_executed = 0
hook_installed = 0
backend_matcher_added = 0
global_allocator_installed = 0
would_replace_host_allocator = 0
would_install_hook = 0
would_add_backend_matcher = 0
would_run_thread = 0
```

## Reject Reasons

```text
1 missing real external provider API call evidence
2 real external provider API call report rejected
3 real external provider API call result is not ready
4 missing explicit host replacement request
5 missing hook-install plan
6 missing rollback plan
7 missing backend no-growth proof
8 closed seam leak
```

## Still Closed

```text
hook installation
backend matcher additions
process allocator replacement
#[global_allocator]
worker/TLS or thread execution
hidden env / implicit discovery / process-global activation config
```

## Validation

```text
validation_profile = scalar-mir
exe = deferred-to-host-replacement-preflight-closeout
```
