---
Status: Completed
Date: 2026-05-11
Scope: M92 allocator provider activation implementation entry contract.
Related:
  - docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-v0.toml
  - tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh
---

# 293x-145 M92 Allocator Provider Activation Implementation Entry Contract

## Summary

M92 names the single future activation implementation owner and entry:

```text
src/runtime/allocator_provider_activation.rs
allocator_provider_activation_attempt
```

This keeps activation orchestration out of the diagnostic parser owners while
leaving the implementation itself for later rows.

## Boundary

M92 is docs/fixture/guard only. It does not add active provider registry code,
provider selection, proof consumption, rollback preparation, activation gate
opening, hook activation, environment discovery, `#[global_allocator]`,
`GlobalAlloc`, process allocator replacement, route widening, or `.inc`
provider/facade/policy name matching.

## Verification

```bash
bash -n tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh
bash tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
