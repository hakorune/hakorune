---
Status: Completed
Date: 2026-05-11
Scope: M87 allocator provider activation decision fixture contract.
Related:
  - docs/development/current/main/design/allocator-provider-activation-decision-surface-proposal-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-v0.toml
  - docs/development/current/main/design/allocator-provider-lightweight-doc-sync-policy-ssot.md
  - tools/checks/k2_wide_allocator_provider_activation_decision_fixture_contract_guard.sh
---

# 293x-140 M87 Allocator Provider Activation Decision Fixture Contract

## Summary

M87 turns the M86 activation decision proposal into a reserved fixture contract:
`allocator-provider-activation-decision-v0.toml`.

The fixture names a caller-provided activation decision bundle and its five
explicit diagnostic inputs. The decision remains blocked:

```text
activation_decision_allowed = false
would_select_provider = false
would_consume_proof = false
would_prepare_rollback = false
would_open_activation_gate = false
would_install_hook = false
would_replace_process_allocator = false
would_activate = false
```

## Boundary

This is docs/fixture/guard only. It does not add runtime parsing, CLI routing,
provider selection, proof consumption, rollback preparation, activation gate
opening, hook activation, `#[global_allocator]`, process allocator replacement,
environment discovery, route widening, or `.inc` name matching.

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_activation_decision_fixture_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
