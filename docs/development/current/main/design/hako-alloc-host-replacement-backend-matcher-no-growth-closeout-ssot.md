# Hako Alloc Host Replacement Backend Matcher No-Growth Closeout

Status: accepted
Decision: accepted
Scope: MIMAP-424A backend matcher no-growth closeout.

## Purpose

MIMAP-424A reconfirms that the optional host replacement ladder has not added
backend `.inc` matchers by app name, owner name, hook name, replacement name,
or row name before any hook-install preflight owner is introduced.

Backend lowering must continue to consume route metadata. It must not grow
host replacement or hook-specific recognizers.

## Checked Names

The guard checks the current host-replacement preflight chain:

- host replacement explicit preflight inventory
- host replacement blocked-state diagnostics
- host replacement preflight closeout
- hook-install preflight plan

## Stop Lines

- No backend `.inc` matcher by app, box, owner, hook, replacement, or row name.
- No hook installation.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_host_replacement_backend_matcher_no_growth_closeout_guard.sh
```
