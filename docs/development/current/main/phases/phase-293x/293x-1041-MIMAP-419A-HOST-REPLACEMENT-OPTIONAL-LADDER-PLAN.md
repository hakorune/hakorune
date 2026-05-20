# 293x-1041 MIMAP-419A Host Replacement Optional Ladder Plan

Status: landed
Date: 2026-05-21

## Purpose

Plan the optional host replacement ladder after the real external provider API
call first-pattern closeout. This row fixes the order for future rows without
installing hooks, adding backend matchers, replacing the process allocator, or
installing a global allocator.

## Scope

- Define the optional host replacement ladder order.
- Keep host replacement behind explicit preflight and closeout rows.
- Keep hook installation, backend matcher additions, worker/thread execution,
  and global allocator install closed.
- Preserve the current goal: `hako_alloc` remains a comparable allocator
  implementation, not the default process allocator.

## Planned Ladder

```text
1. host replacement explicit preflight inventory
2. host replacement blocked-state diagnostics
3. host replacement preflight closeout
4. hook-install preflight plan
5. backend matcher no-growth closeout
6. optional process allocator replacement proposal
```

Each row must keep its own stop lines and must not silently activate the next
boundary.

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
bash tools/checks/k2_wide_hako_alloc_host_replacement_optional_ladder_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added the host replacement optional ladder plan SSOT.
- Added the planning guard.
- Selected host replacement explicit preflight inventory as the next row.
- Kept hook installation, backend matcher additions, process allocator
  replacement, worker/thread execution, and global allocator install closed.
