---
Status: Landed
Decision: accepted
Date: 2026-05-11
Scope: M100 allocator provider proof bundle consumption implementation entry contract.
Related:
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-entry-contract-v0.toml
  - docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-ssot.md
  - tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_entry_contract_guard.sh
---

# 293x-156 M100 Allocator Provider Proof Bundle Consumption Entry Contract

## Result

M100 reserves the future proof-bundle consumption behavior owner and entry:

```text
owner = src/runtime/allocator_provider_activation.rs
entry = allocator_provider_proof_bundle_consumption_attempt
```

The row keeps diagnostic owners diagnostic-only. The M99 CLI remains an
explicit-path reporting surface and does not become a behavior entry.

## Inactive Contract

M100 keeps all allocator provider behavior inactive:

```text
proof_bundle_consumed=false
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

M100 adds:

```text
tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_entry_contract_guard.sh
```

The guard checks the SSOT, reserved fixture, current-state pointer, check-index
wiring, and the inactive stop line.
