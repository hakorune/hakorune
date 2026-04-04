---
Status: Active
Date: 2026-04-04
Scope: select the next source lane after phase-70x ended as a no-op caller-zero sweep.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-70x/README.md
  - docs/development/current/main/phases/phase-69x/README.md
---

# 71x-90 Next Source Lane Selection SSOT

## Intent

- choose the next source lane without reopening `70x`
- prefer tree/code movement over new phase prose
- keep explicit proof/compat/reference routes stable while selecting the next active source target

## Starting Read

- `tools/selfhost/` already reads `mainline / proof / compat / lib`
- `lang/src/runner/` now has explicit `compat / facade / entry`
- `src/runner/` now reads `product / keep / reference`
- `phase-70x` proved that the first caller-zero archive pass is a no-op

## Candidate Direction

- folder-top-level facade thinning
- current source owner cleanup around the new folder layout
- focused blocker follow-up around `emit_mir_mainline`

## 71xA1 Inventory Result

### Highest Pressure

- `tools/selfhost/run.sh`
- `tools/selfhost/selfhost_build.sh`
- `tools/selfhost/build_stage1.sh`
- `lang/src/runner/stage1_cli_env.hako`
- `lang/src/runner/stage1_cli.hako`
- `lang/src/runner/launcher.hako`

### Read

- top-level selfhost facades still carry the heaviest live reference count after the folder split
- top-level `.hako` runner wrappers still have broad live references after the authority/compat/facade recut
- the focused `emit_mir_mainline` blocker remains real, but it is still narrower than the facade-thinning pressure

## Decision Rule

- pick the lane that creates visible tree/code progress
- do not reopen rust-vm retirement discussion unless a new caller-zero fact appears
