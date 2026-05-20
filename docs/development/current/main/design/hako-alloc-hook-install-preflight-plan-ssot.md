# Hako Alloc Hook-Install Preflight Plan

Status: accepted
Decision: accepted
Scope: MIMAP-423A hook-install preflight plan.

## Purpose

MIMAP-423A names the next optional host-replacement ladder boundary after the
host replacement preflight closeout. It is planning-only: it does not install
hooks, add backend matchers, replace the process allocator, or install a global
allocator.

## Planned Input Boundary

The future hook-install preflight row must consume host replacement preflight
closeout evidence and require explicit inputs:

```text
host_replacement_preflight_closeout_present
explicit_hook_install_request_present
hook_target_symbol_present
hook_rollback_plan_present
backend_no_growth_evidence_present
```

The future row must reject if any closed seam would execute:

```text
would_install_hook != 0
would_replace_host_allocator != 0
would_add_backend_matcher != 0
would_install_global_allocator != 0
would_run_thread != 0
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

## Next Row

MIMAP-424A should validate backend matcher no-growth before any concrete
hook-install preflight owner is allowed to execute or emit backend-facing
behavior.

## Validation

```text
validation_profile = planning
exe = not-applicable
```
