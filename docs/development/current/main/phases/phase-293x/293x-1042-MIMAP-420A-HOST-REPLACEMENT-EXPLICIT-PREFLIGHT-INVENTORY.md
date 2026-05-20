# 293x-1042 MIMAP-420A Host Replacement Explicit Preflight Inventory

Status: landed
Date: 2026-05-21

## Purpose

Inventory the explicit preflight inputs that would be required before any host
replacement work can proceed. This row must not install hooks, add backend
matchers, replace the process allocator, or install a global allocator.

## Scope

- Add a narrow host replacement explicit preflight inventory owner.
- Consume real external provider API call first-pattern evidence.
- Record the required-but-still-closed replacement inputs.
- Keep hook installation, backend matcher additions, process allocator
  replacement, worker/thread execution, and global allocator install closed.

## Stop Lines

- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Daily validation is L2:

```text
VM proof
MIR JSON emit
route preflight
```

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_host_replacement_explicit_preflight_inventory_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added the host replacement explicit preflight inventory owner.
- Added the proof app, design SSOT, guard, manifest row, and module export.
- Consumed real external provider API call first-pattern evidence and required
  explicit request / hook plan / rollback plan / backend no-growth inputs.
- Kept hook installation, backend matcher additions, process allocator
  replacement, worker/thread execution, and global allocator install closed.
