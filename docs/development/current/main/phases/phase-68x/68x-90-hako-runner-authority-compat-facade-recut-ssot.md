---
Status: Landed
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

### Compat-facing

- `lang/src/runner/compat/stage1_cli.hako`
- `lang/src/runner/stage1_cli/`

### Facade / entry-facing

- `lang/src/runner/launcher.hako`
- `lang/src/runner/facade/runner_facade.hako`
- `lang/src/runner/entry/launcher_native_entry.hako`
- `lang/src/runner/entry/stage1_cli_env_entry.hako`

## Current Inventory

### Authority / env cluster

- `lang/src/runner/stage1_cli_env.hako`
- `lang/src/runner/stage1_cli_env_entry.hako`

### Compat / raw subcommand cluster

- `lang/src/runner/stage1_cli/**`
- `lang/src/runner/compat/stage1_cli.hako`
- `lang/src/runner/stage1_cli.hako` (wrapper)

### Facade / launcher cluster

- `lang/src/runner/launcher.hako`
- `lang/src/runner/launcher/**`
- `lang/src/runner/facade/runner_facade.hako`
- `lang/src/runner/runner_facade.hako` (wrapper)

### Entry / bootstrap stubs

- `lang/src/runner/entry/launcher_native_entry.hako`
- `lang/src/runner/entry/stage1_cli_env_entry.hako`
- `lang/src/runner/launcher_native_entry.hako` (wrapper)
- `lang/src/runner/stage1_cli_env_entry.hako` (wrapper)
- `lang/src/runner/hako_module.toml`

## Target Layout

```text
lang/src/runner/
  authority/
  compat/
  facade/
  entry/
```

## Ranking

1. `68xB1 facade/entry split`
   - low-blast move: `runner_facade.hako` and bootstrap entry stubs are wired mostly through `hako_module.toml` and thin wrappers
   - compatibility wrappers can preserve old paths while the canonical tree becomes visible
2. `68xB2 authority/compat split`
   - higher blast: `stage1_cli_env.hako`, `stage1_cli.hako`, and `stage1_cli/**` participate in direct `using` chains and should move after the low-blast slice lands
   - safe first slice: move the top-level `stage1_cli.hako` compat owner behind a wrapper, but keep `stage1_cli_env.hako` at top level until focused probes stop depending on its full same-file box set

## Known Blocker Read

- focused `emit_mir_mainline` probes against both the moved entry stub and the top-level wrapper still hit the existing selfhost-first parse red
- current origin remains `lang/src/compiler/build/build_box.hako`
- current error remains `Unexpected token BOX, expected LBRACE`
- read this as inherited blocker, not as a new path-regression from the `68xB1` move

## Decision Rule

- authority-heavy owners go under `authority/`
- compat/raw-subcommand keep goes under `compat/`
- thin orchestration shells go under `facade/`
- thin bootstrap entry stubs go under `entry/`

## Big Tasks

1. `68xA1` runner folder inventory lock
2. `68xA2` target layout ranking
3. `68xB1` facade/entry split
4. `68xB2` authority/compat split
5. `68xC1` alias/readme cleanup
6. `68xD1` proof / closeout

## Current Progress

- `68xB1` landed: facade and entry stubs have canonical homes under `facade/` and `entry/`
- `68xB2` landed: top-level compat owner is wrapped while `stage1_cli_env.hako` stays authority-top-level for now
- `68xC1` landed: live readmes now read canonical paths first and wrappers as aliases only
- `68xD1` landed: `cargo check --bin hakorune`, `git diff --check`, and `tools/selfhost/mainline/stage1_mainline_smoke.sh` stayed green; focused `emit_mir_mainline` red remains inherited from `build_box.hako`
