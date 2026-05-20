# Hako Alloc Host Replacement Blocked-State Diagnostics

Status: accepted
Decision: accepted
Scope: MIMAP-421A host replacement blocked-state diagnostics.

## Purpose

MIMAP-421A observes MIMAP-420A explicit preflight inventory reports and
classifies why host replacement remains blocked. The row is diagnostic-only:
it does not install hooks, add backend matchers, replace the process allocator,
or install a global allocator.

## Input Contract

The owner consumes:

```text
HakoAllocHostReplacementExplicitPreflightInventoryReport
```

The diagnostic maps MIMAP-420A reasons directly:

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

## Report Contract

Accepted-ready input produces:

```text
diagnostic_present = 1
preflight_inventory_present = 1
preflight_ready = 1
blocked_state_present = 0
reason = 0
```

Blocked input produces exactly one blocked-state flag for the reason family.

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
