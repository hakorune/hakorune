---
Status: Done
Date: 2026-06-05
Scope: make `fastmem ContractName { ... }` the only canonical FastMemory region boundary.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-416-FASTMEM-PARSER-PARITY-CATCHUP.md
  - docs/development/current/main/phases/phase-296x/296x-417-FASTMEM-SOURCE-SYNTAX-PILOT.md
---

# 296x-422 Fastmem Named Block Canonical Boundary

## Purpose

FastMemory had two visible spellings in docs:

```text
fastmem ContractName { ... }
@rune FastMemory(ContractName)
```

Only the first one should be a region boundary. This card removes the ambiguity
from current docs.

## Decision

```text
canonical_fastmem_boundary=fastmem ContractName { ... }
contract_name_required=1
contractless_fastmem_allowed=0
unsafe_block_allowed=0

fastmemory_rune_region_boundary=0
fastmemory_rune_metadata_only=1
method_wide_fastmem_by_annotation=0
```

The contract name stays mandatory even before per-contract allowlist branching
exists. In v0 it is a stable report/discovery id and future allowlist key. It
must not be inferred from the enclosing method name.

## Stop Line

- no `fastmem { ... }`
- no `unsafe { ... }`
- no rune-only method-wide fastmem region
- no method-name-derived contract id
- no execution or lowering change

## Proof

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
