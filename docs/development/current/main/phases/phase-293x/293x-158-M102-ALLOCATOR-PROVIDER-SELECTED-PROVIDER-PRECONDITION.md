---
Status: Landed
Decision: accepted
Date: 2026-05-11
Scope: M102 allocator provider selected-provider precondition.
Related:
  - docs/development/current/main/design/allocator-provider-selected-provider-precondition-ssot.md
  - docs/development/current/main/design/allocator-provider-post-m101-implementation-ladder-ssot.md
  - src/runtime/allocator_provider_activation.rs
  - tools/checks/k2_wide_allocator_provider_selected_provider_precondition_guard.sh
---

# 293x-158 M102 Allocator Provider Selected-Provider Precondition

## Result

M102 adds a caller-provided selected-provider precondition runtime entry:

```text
src/runtime/allocator_provider_activation.rs
allocator_provider_selected_provider_precondition_attempt(...)
```

The entry accepts an existing proof-bundle consumption diagnostic report plus an
explicit selected provider id. It verifies that the selected provider is real,
matches `requested_provider_id`, and has a proof entry in the report.

## Inactive Contract

M102 still keeps all allocator activation behavior inactive:

```text
proof_bundle_consumed=false
would_select_provider=false
would_consume_proof_bundle=false
would_prepare_rollback=false
would_open_activation_gate=false
would_install_hook=false
would_replace_process_allocator=false
would_activate=false
```

No provider is selected by runtime code, no proof is consumed, no rollback is
prepared, no gate opens, no hook is installed, no native allocator is activated,
and the process allocator is not replaced.

## Guard

M102 adds:

```text
tools/checks/k2_wide_allocator_provider_selected_provider_precondition_guard.sh
```

The guard checks the docs, runtime entry, focused unit tests, integration into
the allocator guard group, and the inactive stop line.
