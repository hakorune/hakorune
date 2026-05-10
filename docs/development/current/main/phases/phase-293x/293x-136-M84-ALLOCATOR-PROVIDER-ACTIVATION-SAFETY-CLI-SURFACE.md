---
Status: Completed
Date: 2026-05-11
Scope: M84 allocator provider activation safety diagnostic CLI surface.
Related:
  - docs/development/current/main/design/allocator-provider-activation-safety-cli-surface-ssot.md
  - src/cli/allocator_provider_activation_safety.rs
  - src/runtime/allocator_provider_registry.rs
  - tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh
---

# 293x-136 M84 Allocator Provider Activation Safety CLI Surface

## Summary

M84 exposes the M83 activation safety report through an explicit diagnostic
CLI surface:

```text
hakorune --allocator-provider-activation-safety-gate <ACTIVATION_SAFETY_GATE_TOML>
```

The route reads only the caller-provided TOML path and prints stable key/value
diagnostics. A complete activation safety report exits `0` only while the gate
remains closed.

## Boundary

M84 does not add environment discovery, implicit manifest/proof/hook-plan
discovery, provider selection, proof consumption, rollback preparation,
activation gate opening, hook activation, `#[global_allocator]`, `GlobalAlloc`,
process allocator replacement, route widening, runner ownership, or `.inc`
name matching.

The CLI output keeps:

```text
activation_gate_open=false
would_open_activation_gate=false
would_activate_hook=false
would_activate=false
```

## Verification

```bash
bash -n tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh
bash tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh
cargo test -q allocator_provider_activation_safety -- --nocapture
git diff --check
```
