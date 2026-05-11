---
Status: Landed
Decision: accepted
Date: 2026-05-11
Scope: M99 allocator provider proof bundle consumption CLI surface.
Related:
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-cli-surface-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml
  - src/cli/allocator_provider_proof_bundle_consumption.rs
  - tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_cli_surface_guard.sh
---

# 293x-155 M99 Allocator Provider Proof Bundle Consumption CLI Surface

## Result

M99 exposes the M98 proof bundle consumption diagnostic report through:

```text
hakorune --allocator-provider-proof-bundle-consumption <PROOF_BUNDLE_CONSUMPTION_TOML>
```

The surface is explicit-path only. It reads caller-provided TOML, prints the
proof bundle consumption diagnostic report, and exits `0` only for
`proof_bundle_consumption_status=ready_inactive`.

## Inactive Contract

The CLI keeps all allocator provider behavior inactive:

```text
proof_bundle_consumed=false
active_registry_built=false
would_build_registry=false
would_select_provider=false
would_consume_proof_bundle=false
would_prepare_rollback=false
would_open_activation_gate=false
would_install_hook=false
would_replace_process_allocator=false
would_activate=false
```

No provider is selected, no proof is consumed, no rollback is prepared, no gate
opens, no hook is installed, no native allocator is activated, and the process
allocator is not replaced.

## Guard

M99 adds:

```text
tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_cli_surface_guard.sh
```

The guard checks CLI wiring, output shape, read-error and conflict diagnostics,
future-compatible M98 guard behavior, and the allocator provider inactive stop
line.
