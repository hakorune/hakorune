---
Status: Landed
Date: 2026-04-05
Scope: top-level `.hako` wrapper policy after the runner recut; phase is now landed and handed off.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-90x/README.md
  - lang/src/runner/README.md
---

# 91x-90 Top-Level .hako Wrapper Policy Review SSOT

## Purpose

- freeze the policy for top-level `.hako` wrappers after the runner/selfhost recut
- keep the top-level wrappers public/front-door only
- avoid turning thin wrappers into archive/delete targets without a caller-zero proof

## In Scope

- `lang/src/runner/launcher.hako`
- `lang/src/runner/runner_facade.hako`
- `lang/src/runner/launcher_native_entry.hako`
- `lang/src/runner/stage1_cli.hako`
- `lang/src/runner/stage1_cli_env.hako`
- `lang/src/runner/stage1_cli_env_entry.hako`

## Canonical Homes

- `lang/src/runner/compat/stage1_cli.hako`
- `lang/src/runner/facade/runner_facade.hako`
- `lang/src/runner/entry/launcher_native_entry.hako`
- `lang/src/runner/entry/stage1_cli_env_entry.hako`

## Policy

- top-level wrappers stay thin
- top-level wrappers stay public/front-door keeps unless a canonical replacement and caller-zero proof exists
- wrapper policy cleanup should not expand wrapper bodies or reintroduce orchestration logic
- archive/deletion remains out of scope for thin public wrappers unless a separate no-op rerun proves they are truly caller-zero

## Out of Scope

- rust-vm retirement
- vm-hako reference/conformance lane
- proof/compat caller rerun
- archive/deletion sweeps
