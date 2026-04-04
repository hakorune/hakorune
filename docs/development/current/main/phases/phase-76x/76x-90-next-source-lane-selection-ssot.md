---
Status: Active
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
