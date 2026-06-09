---
Status: Landed
Date: 2026-06-09
Scope: select the next read-only secure-entropy-backed free-list seam for the unified production allocator API lane.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-66-HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY.md
  - docs/development/current/main/design/mimalloc-secure-entropy-inventory-ssot.md
  - docs/development/current/main/design/mimalloc-port-remaining-inventory-ssot.md
---

# 296x-643 HAKO Mimalloc Secure Entropy Backed Free List Integration

## Purpose

Select the next remaining read-only allocator seam that composes secure
entropy inventory facts with the deterministic encoded-next free-list policy.

This row does not add entropy sourcing, provider activation, hooks, allocator
replacement, or page mutation. It only selects the narrow secure entropy backed
free-list surface and keeps the policy read-only.

## Required Input

```text
output_contract=hako-mimalloc-port-feature-gap-inventory-v0
selected_feature=secure_entropy_backed_free_list
missing_feature_count=7
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-unified-production-allocator-api-selection-v0
selected_feature=secure_entropy_backed_free_list
selector_ready=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-secure-entropy-backed-free-list-v0
inventory_eligible=1
policy_ready=1
encoded_next=262181
decoded_next=3
entropy_sourcing_closed=1
provider_activation_closed=1
hook_install_closed=1
replacement_closed=1
summary=ok
```

## Guard

```text
tools/checks/k2_wide_hako_alloc_secure_entropy_backed_free_list_guard.sh
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mimalloc-current.md
docs/development/current/main/phases/phase-296x/phase-296x-90-mimalloc-benchmark-taskboard.md
```

## Stop Line

Do not source entropy, select a provider, install a hook, replace the host
allocator, or mutate page state in this row.
