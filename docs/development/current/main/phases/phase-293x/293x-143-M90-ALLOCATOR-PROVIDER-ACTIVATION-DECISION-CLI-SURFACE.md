---
Status: Completed
Date: 2026-05-11
Scope: M90 allocator provider activation decision diagnostic CLI surface.
Related:
  - docs/development/current/main/design/allocator-provider-activation-decision-cli-surface-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-v0.toml
  - src/cli/allocator_provider_activation_decision.rs
  - src/runtime/allocator_provider_activation_decision.rs
  - tools/checks/k2_wide_allocator_provider_activation_decision_cli_surface_guard.sh
---

# 293x-143 M90 Allocator Provider Activation Decision CLI Surface

## Summary

M90 exposes the M89 activation decision report through an explicit diagnostic
CLI surface:

```text
hakorune --allocator-provider-activation-decision <ACTIVATION_DECISION_TOML>
```

The route reads only the caller-provided TOML path and prints stable key/value
diagnostics. A complete activation decision report exits `0` only while the
decision remains blocked.

## Boundary

M90 does not add environment discovery, implicit manifest/report/proof
discovery, provider selection, proof consumption, rollback preparation,
activation gate opening, hook activation, `#[global_allocator]`, `GlobalAlloc`,
process allocator replacement, route widening, runner ownership, or `.inc`
name matching.

The CLI output keeps:

```text
activation_decision_allowed=false
would_select_provider=false
would_consume_proof=false
would_prepare_rollback=false
would_open_activation_gate=false
would_install_hook=false
would_replace_process_allocator=false
would_activate=false
```

## Verification

```bash
bash -n tools/checks/k2_wide_allocator_provider_activation_decision_cli_surface_guard.sh
bash tools/checks/k2_wide_allocator_provider_activation_decision_cli_surface_guard.sh
cargo test -q allocator_provider_activation_decision -- --nocapture
git diff --check
```
