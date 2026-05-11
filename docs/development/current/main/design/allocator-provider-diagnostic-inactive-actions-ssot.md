---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: M93B allocator provider diagnostic inactive action cleanup.
Related:
  - src/runtime/allocator_provider_diagnostic_inactive.rs
  - src/runtime/allocator_provider_registry.rs
  - src/runtime/allocator_provider_activation_decision.rs
  - tools/checks/k2_wide_allocator_provider_diagnostic_inactive_actions_guard.sh
---

# Allocator Provider Diagnostic Inactive Actions (SSOT)

## Goal

Keep allocator provider diagnostic report outputs structurally inactive from one
code-side source.

M83, M89, and M93 all produce diagnostic-only reports. They may parse
caller-provided TOML and return stable diagnostics, but they must not build an
active runtime registry, select providers, consume proofs, prepare rollback,
open the activation gate, install hooks, or replace the process allocator.

## Runtime Owner

```text
src/runtime/allocator_provider_diagnostic_inactive.rs
```

Owned constants:

```text
DIAGNOSTIC_INACTIVE_ACTIONS
REGISTRY_SNAPSHOT_INACTIVE_ACTIONS
SAFETY_GATE_INACTIVE_ACTIONS
```

Report owners consume these constants:

```text
src/runtime/allocator_provider_registry.rs
src/runtime/allocator_provider_activation_decision.rs
```

## Contract

The inactive action constants are the code-side SSOT for these false outputs:

```text
active_registry_built = false
would_build_registry = false
would_select_provider = false
would_consume_proof = false
would_prepare_rollback = false
would_open_activation_gate = false
would_install_hook = false
would_replace_process_allocator = false
would_activate_hook = false
would_activate = false
```

Report structs keep their existing public fields. This cleanup changes only
where those false values are sourced from.

## Guard Hygiene

M93B also removes the M93 guard's current-card pin. Once M93 has landed, its
guard must behave like a past guard: it proves its own artifacts and forbidden
activation behavior, but it must not require `CURRENT_STATE.latest_card` to
remain on M93.

The M93B guard deliberately does not add a new latest-card pin. The generic
`current_state_pointer_guard` remains the stable check for pointer shape.

## Non-Goals

- no CLI surface;
- no provider selection;
- no proof consumption;
- no rollback preparation;
- no activation gate opening;
- no hook activation;
- no process allocator replacement;
- no report field rename.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_diagnostic_inactive_actions_guard.sh
cargo test -q allocator_provider_inactive -- --nocapture
cargo test -q activation_safety -- --nocapture
cargo test -q activation_decision -- --nocapture
cargo test -q registry_snapshot -- --nocapture
git diff --check
```
