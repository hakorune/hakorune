# MIMAP-009 lifecycle integration pilot SSOT

Decision: accepted.

`MIMAP-009` adds an explicit page-local lifecycle state to `HakoAllocPageModel`.
It integrates the existing retired/reactivate behavior with decommit, recommit, and
reuse semantics without introducing OSVM or provider activation.

## State contract

The accepted sequence is:

```text
active -> retired -> decommitted -> recommitted -> reusable active
```

Represented fields:

- `retired`: page has no live blocks and is not active for acquire.
- `decommitted`: retired page is unavailable until recommit.
- `decommit_count`: accepted decommit transitions.
- `recommit_count`: accepted recommit transitions.
- `reuse_count`: accepted reuse/reactivation transitions.
- `lifecycle_reject_count`: rejected lifecycle transitions.

## Method contract

- `decommit()` accepts only retired, unused, non-decommitted pages.
- `recommit()` accepts only retired, decommitted pages.
- `canReuse()` observes reusable retired pages after recommit.
- `reuse()` calls the existing reactivation path only after `canReuse()` accepts.
- `reactivate()` rejects decommitted pages directly.

## Non-goals

- no OSVM reserve/commit/decommit call
- no segment ownership
- no remote-free or thread-local free ownership
- no allocator provider activation
- no host allocator replacement

## Proof and guard

```text
apps/mimalloc-lifecycle-integration-pilot-proof/main.hako
tools/checks/k2_wide_mimalloc_lifecycle_integration_pilot_guard.sh
```
