---
Status: Landed
Decision: accepted
Date: 2026-05-11
Scope: M97 allocator provider selection decision CLI surface.
Related:
  - docs/development/current/main/design/allocator-provider-selection-decision-cli-surface-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-v0.toml
  - src/cli/allocator_provider_selection_decision.rs
  - tools/checks/k2_wide_allocator_provider_selection_decision_cli_surface_guard.sh
---

# 293x-151 M97 Allocator Provider Selection Decision CLI Surface

## Result

M97 exposes the M96 selection decision diagnostic report through:

```text
hakorune --allocator-provider-selection-decision <SELECTION_DECISION_TOML>
```

The surface is explicit-path only. It reads caller-provided TOML, prints the
selection decision diagnostic report, and exits `0` only for
`selection_decision_status=ready_inactive`.

## Inactive Contract

The CLI keeps all allocator provider behavior inactive:

```text
active_registry_built=false
would_build_registry=false
would_select_provider=false
would_consume_proof=false
would_prepare_rollback=false
would_open_activation_gate=false
would_install_hook=false
would_replace_process_allocator=false
would_activate=false
```

The selected provider remains diagnostic-only:

```text
selected_provider_id=none_reserved
selected_provider_id_absent=true
```

No provider is selected, no proof is consumed, no rollback is prepared, no gate
opens, no hook is installed, no native allocator is activated, and the process
allocator is not replaced.

## Guard

M97 adds:

```text
tools/checks/k2_wide_allocator_provider_selection_decision_cli_surface_guard.sh
```

The guard checks CLI wiring, output shape, read-error and conflict diagnostics,
future-compatible M96 guard behavior, and the allocator provider inactive stop
line.
