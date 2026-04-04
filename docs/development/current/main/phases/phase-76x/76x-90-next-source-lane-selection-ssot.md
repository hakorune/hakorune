---
Status: Landed
Date: 2026-04-04
Scope: rank the next source cleanup lane after phase-75x selfhost alias canonicalization.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-75x/README.md
---

# 76x-90 Next Source Lane Selection SSOT

## Intent

- choose the next structural cleanup lane after selfhost top-level aliases were canonicalized
- prefer tree-shape work over prose-only work
- keep current top-level wrappers explicit and thin

## Initial Candidate Read

1. `.hako` top-level alias canonicalization
   - thin top-level wrappers still exist under `lang/src/runner/`
   - likely target files:
     - `lang/src/runner/launcher.hako`
     - `lang/src/runner/stage1_cli_env.hako`
     - `lang/src/runner/stage1_cli.hako`
2. current root-pointer thinning
   - `CURRENT_TASK.md` / current mirrors still carry long landed ledgers
   - useful, but lower leverage than source-tree cleanup
3. caller-zero archive rerun
   - possible follow-up after another alias/facade thinning wave

## Decision Rule

- prefer the next lane that changes tree reading directly
- do not reopen rust-vm retirement prose
- keep proof scope small: `cargo check --bin hakorune` plus one focused smoke/probe

## Current Read

- `76xA1` landed:
  - `lang/src/runner/stage1_cli.hako` is a thin top-level wrapper but still has visible caller pressure in tests/smokes/probes
  - `lang/src/runner/runner_facade.hako`, `launcher_native_entry.hako`, and `stage1_cli_env_entry.hako` are already low-pressure compatibility wrappers
  - `lang/src/runner/launcher.hako` and `lang/src/runner/stage1_cli_env.hako` are canonical owners, not alias-cleanup targets
- `76xA2` landed ranking:
  1. `runner top-level pressure thinning`
  2. `root/current pointer thinning`
  3. `caller-zero archive rerun`
- `76xB1` landed decision:
  - thin wrapper canonicalization is already mostly done
  - next lane should target the remaining top-level pressure in `launcher.hako` and `stage1_cli_env.hako`
