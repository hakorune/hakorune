---
Status: Complete
Date: 2026-05-12
Scope: M182 secure free-list policy inventory.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/page_box.hako
  - lang/src/hako_alloc/memory/huge_release_seam_box.hako
---

# 293x-193 M182 Secure Free-List Policy Inventory

## Goal

Fix secure free-list responsibilities before diagnostics or encode/decode code
lands.

M182 is inventory only. It does not add secure-list behavior.

## Responsibility Split

| Owner | Responsibility |
| --- | --- |
| `HakoAllocPageModel` | owns page-local block identity, `free`, `local_free`, `block_used`, and capacity bounds |
| future `HakoAllocSecureFreeListDiagnostics` | observes duplicate, out-of-range, live-block, and count-mismatch conditions |
| future `HakoAllocSecureFreeListPolicy` | owns encode/decode/validate vocabulary after diagnostics are stable |
| page-map / huge owners | do not own small page free-list encoding |
| heap/facade owners | orchestrate only; they do not inspect encoded next values directly |

## Diagnostic Vocabulary For M183

M183 may add diagnostics-only observers for:

- `out_of_range_free_block`
- `duplicate_free_block`
- `live_block_in_free_list`
- `free_count_mismatch`
- `local_free_count_mismatch`

## Encode/Decode Vocabulary For M184

M184 may add the first encode/decode policy only after M183 diagnostics are
green:

- `encodeNext(block_index)`
- `decodeNext(encoded)`
- `validateDecodedIndex(index, capacity)`

## Stop Line

M182 does not implement secure-list diagnostics, encode/decode, cookies,
randomness, cryptographic hardening, native allocator replacement, provider
activation, hooks, OS release, or `.inc` allocator-name matching.

## Proof

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
