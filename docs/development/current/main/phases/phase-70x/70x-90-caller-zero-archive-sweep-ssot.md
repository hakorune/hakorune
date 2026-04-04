---
Status: Active
Date: 2026-04-04
Scope: sweep caller-zero aliases/docs/wrappers after the selfhost, `.hako` runner, and rust runner folder recuts landed.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-67x/README.md
  - docs/development/current/main/phases/phase-68x/README.md
  - docs/development/current/main/phases/phase-69x/README.md
---

# 70x-90 Caller-Zero Archive Sweep SSOT

## Intent

- archive only surfaces whose live callers have actually drained to zero
- do not archive proof-only keep
- do not archive reference/conformance surfaces

## Starting Read

- `tools/selfhost/` is folder-split with top-level wrappers still present
- `lang/src/runner/` is recut with top-level wrappers still present
- `src/runner/modes/mod.rs` is now a compatibility re-export surface

## Candidate Sweep Surfaces

### Selfhost top-level wrappers

- `tools/selfhost/*.sh` wrappers whose callers now point at `mainline/`, `proof/`, or `compat/`

### `.hako` top-level wrappers

- `lang/src/runner/stage1_cli.hako`
- `lang/src/runner/runner_facade.hako`
- `lang/src/runner/launcher_native_entry.hako`
- `lang/src/runner/stage1_cli_env_entry.hako`

### Rust compatibility surfaces

- `src/runner/modes/mod.rs` re-export status

## Decision Rule

- archive only when caller-zero is proven
- keep live aliases when current callers still name them
- keep reference/proof/current paths live even if they are no longer mainline
