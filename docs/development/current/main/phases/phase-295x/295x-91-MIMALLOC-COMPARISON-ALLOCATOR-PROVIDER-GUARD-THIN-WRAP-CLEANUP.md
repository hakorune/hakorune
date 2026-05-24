---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap allocator-provider guard families into root wrappers and impl/ entries.
Related:
  - tools/checks/dev_gate.sh
  - tools/checks/allocator/families/provider/core.steps
  - tools/checks/allocator/families/provider/diagnostics.steps
  - tools/checks/allocator/families/provider/activation.steps
---

# 295x-91 Allocator Provider Guard Thin-Wrap Cleanup

## Blocker

```text
MIMALLOC-COMPARISON-ALLOCATOR-PROVIDER-GUARD-THIN-WRAP-CLEANUP-295X-001
```

## Decision

Thin-wrap the allocator-provider guard families so the root scripts stay as
stable one-line wrappers and the real assertions live under `tools/checks/impl/`.
The cleanup keeps the current mimalloc comparison blocker parked and does not
open provider activation, provider/DLL packaging, or process allocator
replacement seams.

The families updated in this batch include the proof bundle consumption,
selection, diagnostics, manifest, core, and representative activation-related
guards that still had thick root implementations.

## Cleanup

- Keep each root `k2_wide_*` guard as a thin exec wrapper.
- Move the actual shell logic under `tools/checks/impl/`.
- Keep family membership in the `tools/checks/allocator/families/provider/*.steps`
  inventories.
- Keep `tools/checks/dev_gate.sh` aligned with the family inventories instead of
  duplicating the guard body.
- Keep `proof_validation` out of the allocator-wide gate inventory; it remains
  a separate guard, not a family-step member.

## Result

Allocator-provider guard ownership is now split more cleanly between stable root
entrypoints, family inventories, and impl scripts. The current mimalloc
comparison blocker remains unchanged.

## Stop Line

This row does not open provider activation, provider packages, DLL generation,
host allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned-heap stress, or winner claims.

## Validation

Representative thin-wrap wrappers were executed green, including the provider
proof bundle, diagnostics, selection, manifest, core, and activation cleanup
guards. The standard pointer guard and diff check also remained green.

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
