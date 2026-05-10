---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: allocator hook activation preflight boundary before process allocator replacement.
Related:
  - docs/development/current/main/design/allocator-hook-dry-run-cli-surface-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-proof-validator-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-proof-v0.toml
---

# Allocator Hook Activation Preflight (SSOT)

## Goal

Name the proof handoff required before any allocator hook can replace the
process allocator.

## Decision

Activation remains inactive until a later implementation row introduces a
runtime-owned preflight object that proves every activation hazard explicitly.

The preflight must not be inferred from:

- CLI flag presence;
- symbol names;
- facade names;
- `.inc` matches;
- environment variables;
- successful dry-run diagnostics alone.

## Required Preflight Facts

The first activation row must consume named facts for:

- reentrancy guard;
- bootstrap allocation path;
- no-allocation / no-safepoint contract;
- rollback condition;
- fail-fast diagnostic;
- active hook plan row;
- active activation proof row.

The reserved activation-proof TOML vocabulary already names the first five
facts, but M62 does not make them executable.

## Ownership

- Runtime owns the preflight object and fail-fast diagnostics.
- `hako_alloc` owns allocator policy and page-source behavior.
- CLI owns explicit diagnostic input only.
- `.inc` remains a reader/emitter and must not infer activation behavior.
- App code must not hide allocator activation blockers.

## Activation Stop Line

M62 keeps these inactive:

- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- hook environment toggles;
- implicit runtime file-system manifest discovery;
- `.inc` hook/facade/policy name matching;
- route widening for allocator activation.

## Next Allowed Row

The next row may add a diagnostic-only `AllocatorHookActivationPreflight` data
shape, but it must still return `would_activate = false` until a separate
activation row proves process allocator replacement safety.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_activation_preflight_guard.sh
bash tools/checks/k2_wide_allocator_hook_dry_run_cli_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
