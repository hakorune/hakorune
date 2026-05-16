# 293x-530 MIMAP-049A Secure Entropy Source Inventory

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-049A` is the row selected by `MIMAP-048B`.

It adds a read-only `.hako` inventory owner for the secure entropy/randomness
boundary. The row may document deterministic proof keys and capability gaps, but
it must not source entropy, add random externs, change secure-list behavior, or
claim cryptographic hardening.

## Scope

- Add a small `HakoAllocSecureEntropyInventory` owner.
- Add a proof app that observes:
  - entropy capability is not present;
  - deterministic proof key use is allowed only for proof models;
  - cryptographic / runtime random claims remain false.
- Add a focused guard for docs, proof, exports, and stop lines.
- Select one follow-up planning row after the inventory lands.

## Non-Goals

- Do not add random/entropy extern routes.
- Do not call OS, provider, hook, TLS, atomic, or backend random helpers.
- Do not change `HakoAllocSecureFreeListPolicy` encode/decode behavior.
- Do not claim cryptographic strength.
- Do not add provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- Do not add backend `.inc` app/box-name matchers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `049A.1` | Add secure entropy inventory owner. | owner reports deterministic/proof-only and inactive entropy facts. | no entropy source |
| `049A.2` | Add proof app. | VM proof observes inactive random/hardening surfaces. | no secure-list behavior change |
| `049A.3` | Add focused guard and docs index row. | guard proves stop lines and no `.inc` matcher leak. | no backend route |
| `049A.4` | Update current pointers and select follow-up. | current pointer guard passes. | no multi-row bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_secure_entropy_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Return Condition

This row closes when secure entropy remains an explicit inactive capability
boundary with a read-only owner and guard.

## Inventory Result

`MIMAP-049A` adds:

```text
owner:
  HakoAllocSecureEntropyInventory
proof app:
  apps/hako-alloc-secure-entropy-inventory-proof/main.hako
guard:
  tools/checks/k2_wide_hako_alloc_secure_entropy_inventory_guard.sh
```

The owner classifies deterministic proof-key facts and rejected runtime entropy
requests. It does not source entropy, call random/OS/provider helpers, mutate
secure-list behavior, or claim cryptographic hardening.

## Evidence

```text
bash tools/checks/k2_wide_hako_alloc_secure_entropy_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection Result

`MIMAP-049A` selects `MIMAP-049B`.

```text
row:
  MIMAP-049B post-secure-entropy-inventory row selection
classification:
  planning-only row
why now:
  secure entropy remains inactive after MIMAP-049A; the lane needs a single-row
  selection step before allocator/compiler/language implementation resumes.
stop lines:
  no entropy/random execution
  no secure-list behavior change
  no provider activation
  no cleanup bundle
```

Closeout:

```text
current blocker moves to MIMAP-049B.
```
