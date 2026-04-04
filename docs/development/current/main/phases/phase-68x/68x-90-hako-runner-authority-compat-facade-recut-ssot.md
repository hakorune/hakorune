---
Status: Active
Date: 2026-04-04
Scope: split `lang/src/runner` into authority / compat / facade / entry lanes after the selfhost folder split landed.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-67x/67x-90-selfhost-folder-split-ssot.md
---

# 68x-90 `.hako` Runner Authority/Compat/Facade Recut SSOT

## Intent

- make `.hako` runner ownership readable from paths instead of comments alone
- separate authority, compat, facade, and entry surfaces without changing the mainline contract
- keep the deliverable as tree shape plus narrow caller cleanup

## Starting Read

- `lang/src/runner/` already has partial structure
- `stage1_cli_env.hako` is the authority-heavy owner cluster
- `stage1_cli.hako` is compat/raw-subcommand keep
- launcher/facade and entry stubs are still mixed enough that the tree is not self-explanatory

## Candidate Reading

### Authority-facing

- `lang/src/runner/stage1_cli_env.hako`
- `lang/src/runner/stage1_cli_env_entry.hako`

### Compat-facing

- `lang/src/runner/stage1_cli.hako`
- `lang/src/runner/stage1_cli/`

### Facade / entry-facing

- `lang/src/runner/launcher.hako`
- `lang/src/runner/runner_facade.hako`
- `lang/src/runner/launcher_native_entry.hako`

## Current Inventory

### Authority / env cluster

- `lang/src/runner/stage1_cli_env.hako`
- `lang/src/runner/stage1_cli_env_entry.hako`

### Compat / raw subcommand cluster

- `lang/src/runner/stage1_cli.hako`
- `lang/src/runner/stage1_cli/**`

### Facade / launcher cluster

- `lang/src/runner/launcher.hako`
- `lang/src/runner/launcher/**`
- `lang/src/runner/runner_facade.hako`

### Entry / bootstrap stubs

- `lang/src/runner/launcher_native_entry.hako`
- `lang/src/runner/stage1_cli_env_entry.hako`
- `lang/src/runner/hako_module.toml`

## Target Layout

```text
lang/src/runner/
  authority/
  compat/
  facade/
  entry/
```

## Decision Rule

- authority-heavy owners go under `authority/`
- compat/raw-subcommand keep goes under `compat/`
- thin orchestration shells go under `facade/`
- thin bootstrap entry stubs go under `entry/`

## Big Tasks

1. `68xA1` runner folder inventory lock
2. `68xA2` target layout ranking
3. `68xB1` authority/compat split
4. `68xB2` facade/entry split
5. `68xC1` alias/readme cleanup
6. `68xD1` proof / closeout
