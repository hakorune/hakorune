---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: allocator hook dry-run CLI surface before runtime activation.
Related:
  - docs/development/current/main/design/allocator-hook-activation-proof-validator-ssot.md
  - src/runtime/allocator_hook_dry_run.rs
  - src/cli/allocator_hook_dry_run.rs
---

# Allocator Hook Dry-Run CLI Surface (SSOT)

## Goal

Expose the allocator hook dry-run validator through an explicit diagnostic CLI
surface without adding activation, environment toggles, or implicit manifest
discovery.

## Decision

The accepted CLI shape is:

```text
hakorune --allocator-hook-dry-run \
  --allocator-hook-plan <PLAN_TOML> \
  --allocator-hook-proof <PROOF_TOML>
```

The CLI reads only the two paths explicitly passed by the user. It does not
search default locations, read environment variables, or install a process
allocator hook.

## Output Contract

The CLI prints stable key/value diagnostics:

```text
diagnostic=[allocator-hook/dry-run-ready]
hook_id=hako_alloc.production.v0
dry_run_status=ready
would_install=false
activation_proof_diagnostic=[allocator-hook/activation-proof-ready]
activation_proof_status=ready
would_activate=false
```

Exit code:

- `0`: dry-run and activation proof are both ready diagnostics.
- `2`: file read error, missing args, missing plan, missing proof, or invalid
  proof.

## Ownership

- `src/runtime/allocator_hook_dry_run.rs` owns validation facts.
- `src/cli/allocator_hook_dry_run.rs` owns explicit file input and CLI output.
- `src/main.rs` may early-exit after CLI parsing.
- `src/runner/**` does not own this surface.
- `.inc` remains a reader/emitter only and does not infer allocator hook
  behavior.

## Stop Line

M61 keeps these inactive:

- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- hook environment toggles;
- implicit runtime file-system manifest discovery;
- `.inc` hook/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_dry_run_cli_surface_guard.sh
bash tools/checks/k2_wide_allocator_hook_activation_proof_validator_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
