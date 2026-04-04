---
Status: Active
Date: 2026-04-04
Scope: thin `lang/src/runner` top-level owner files after wrapper canonicalization settled.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-76x/README.md
  - lang/src/runner/README.md
---

# 77x-90 Runner Top-Level Pressure Thinning SSOT

## Intent

- keep top-level wrappers thin
- treat `launcher.hako` and `stage1_cli_env.hako` as the main remaining top-level pressure points
- push internal body logic down into canonical same-cluster helpers instead of widening new facades

## Initial Read

- already-thin wrappers:
  - `lang/src/runner/runner_facade.hako`
  - `lang/src/runner/stage1_cli.hako`
  - `lang/src/runner/launcher_native_entry.hako`
  - `lang/src/runner/stage1_cli_env_entry.hako`
- remaining top-level pressure:
  - `lang/src/runner/launcher.hako`
  - `lang/src/runner/stage1_cli_env.hako`

## Decision Rule

- keep `launcher.hako` as the top-level CLI facade, but thin the body further
- keep `stage1_cli_env.hako` as current authority entry, but reduce inline authority bulk where possible
- do not reopen wrapper path churn that phase-68x and phase-75x already settled

## Current Read

- `77xA1` landed:
  - thin wrapper keep:
    - `lang/src/runner/runner_facade.hako`
    - `lang/src/runner/stage1_cli.hako`
    - `lang/src/runner/launcher_native_entry.hako`
    - `lang/src/runner/stage1_cli_env_entry.hako`
  - remaining top-level pressure:
    - `lang/src/runner/launcher.hako`
    - `lang/src/runner/stage1_cli_env.hako`
  - worker rerun agrees that wrapper canonicalization is largely done and the next leverage is body thinning in those two files
- `77xB1` in progress:
  - `lang/src/runner/launcher.hako` now delegates command bodies through `launcher/command_dispatch.hako`, `launcher/bootstrap.hako`, and `launcher/build_exe.hako`
  - `lang/src/runner/stage1_cli_env.hako` now delegates mode/input authority to `stage1_cli_env/mode_contract.hako` and `stage1_cli_env/input_contract.hako`
  - probe note: `tools/hakorune_emit_mir_mainline.sh lang/src/runner/launcher.hako ...` still trips the known `Main._emit_mir_checked/1` residual red; keep it tracked, do not reopen wrapper canonicalization
