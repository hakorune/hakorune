---
Status: Landed
Decision: accepted
Date: 2026-05-11
Scope: M101 allocator provider proof consumption fail-fast entry.
Related:
  - docs/development/current/main/design/allocator-provider-proof-consumption-failfast-entry-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-entry-contract-ssot.md
  - src/runtime/allocator_provider_activation.rs
  - tools/checks/k2_wide_allocator_provider_proof_consumption_failfast_entry_guard.sh
---

# 293x-157 M101 Allocator Provider Proof Consumption Fail-Fast Entry

## Result

M101 creates the reserved M100 runtime entry:

```text
src/runtime/allocator_provider_activation.rs
allocator_provider_proof_bundle_consumption_attempt(...)
```

The entry is fail-fast only. It accepts an existing proof-bundle consumption
diagnostic report and blocks when no real selected provider exists.

## Inactive Contract

M101 keeps all allocator activation behavior inactive:

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

No provider is selected, no proof is consumed, no rollback is prepared, no gate
opens, no hook is installed, no native allocator is activated, and the process
allocator is not replaced.

## Guard

M101 adds:

```text
tools/checks/k2_wide_allocator_provider_proof_consumption_failfast_entry_guard.sh
```

The guard checks the runtime entry, focused unit tests, future-compatible M100
guard behavior, and the inactive stop line.
