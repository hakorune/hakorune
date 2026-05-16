---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-049A read-only secure entropy/randomness inventory.
Related:
  - docs/development/current/main/design/mimalloc-hakorune-capability-surface-ssot.md
  - docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-530-MIMAP-049A-SECURE-ENTROPY-SOURCE-INVENTORY.md
---

# Mimalloc Secure Entropy Inventory SSOT

## Decision

`MIMAP-049A` adds a read-only inventory owner for secure entropy / randomness
requirements.

The owner may classify deterministic proof keys and rejected runtime entropy
requests. It must not source entropy, add random extern routes, mutate
secure-list behavior, or claim cryptographic hardening.

## Boundary

| Surface | State | Owner |
| --- | --- | --- |
| deterministic proof key classification | active | `HakoAllocSecureEntropyInventory` |
| runtime entropy source | inactive | future capability row only |
| random/entropy extern route | inactive | future substrate row only |
| secure-list encode/decode | unchanged | `HakoAllocSecureFreeListPolicy` |
| cryptographic hardening claim | inactive | future capability + audit row only |

## Guard Contract

The guard must prove:

```text
MIMAP-049A card is landed
this SSOT is accepted
hako_alloc exports secure_entropy_inventory_box.hako
proof app observes deterministic proof-key facts
runtime entropy / random / crypto hardening claims remain inactive
secure_free_list_policy_box.hako is not changed by this owner
no app/box matcher leaks into .inc
```

## Stop Lines

`MIMAP-049A` must not add:

```text
random or entropy extern routes
OS/provider/TLS/atomic random helpers
secure-list encode/decode behavior changes
cryptographic hardening claims
provider activation
hooks
host allocator replacement
#[global_allocator]
backend .inc app/name matchers
```

## Next Row

After this inventory, open:

```text
MIMAP-049B post-secure-entropy-inventory row selection
```

That row must choose one follow-up task and must not treat this inventory as
permission to implement entropy execution.
