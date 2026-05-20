# 293x-1045 MIMAP-423A Hook-Install Preflight Plan

Status: landed
Date: 2026-05-21

## Purpose

Plan the explicit hook-install preflight row after the host replacement
preflight closeout. This row should name the next narrow hook-install
preflight boundary without installing hooks, adding backend matchers, replacing
the process allocator, or installing a global allocator.

## Scope

- Define the hook-install preflight input boundary.
- Keep hook installation and process replacement closed.
- Keep backend matcher additions closed unless a later no-growth closeout
  explicitly validates them.

## Stop Lines

- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Planning validation is L0:

```text
current state pointer guard
git diff --check
```

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_hook_install_preflight_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added the hook-install preflight plan SSOT and guard.
- Named the future hook-install preflight input boundary.
- Selected backend matcher no-growth closeout as the next row.
- Kept hook installation, backend matcher additions, process allocator
  replacement, worker/thread execution, and global allocator install closed.
