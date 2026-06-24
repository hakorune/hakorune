---
Status: Done
Date: 2026-06-05
Scope: add safe capability wrapper plan evidence over existing FastMemory MemOps without opening RawPtr or product activation.
Blocker: MIM-FMEM-015
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - tools/hako_check/fastmem_capability_inventory.py
  - tools/hako_check/fastmem_check.py
  - tools/hako_check/fastmem_capability_inventory_smoke.sh
---

# 296x-428 Safe Capability Wrapper Plan

## Purpose

`MIM-FMEM-014` made remote `AtomicRemoteHead` push/drain evidence visible from
the non-activating cross-thread smoke pack. This row fixes the safe wrapper
surface above the already-observed FastMemory MemOps.

## Decision

Safe wrappers are a readable surface over the same MemOps, not a second hot
path:

```text
AddressToken
PageKey
PageMapBridge
PageMetaHandle
AllocOwnerId
AtomicRemoteHead
```

Accepted route:

```text
safe_capability_wrapper_route=fastmem_memop_alias
safe_capability_wrapper_lowering_route=fastmem_memop_alias
safe_capability_wrapper_memop_equivalence=1
safe_capability_wrapper_count=6
safe_capability_wrapper_missing_count=0
```

Closed surfaces:

```text
safe_capability_wrapper_rawptr_surface=0
safe_capability_wrapper_deref_surface=0
safe_capability_wrapper_escape_count=0
address_token_deref_allowed=0
address_token_pointer_arithmetic_allowed=0
```

## Boundary

```text
source_rewrite=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
type_abi_hot_path_lookup_count=0
provider_dispatch_hot_path=0
```

No general `RawPtr<T>`, pointer arithmetic outside `fastmem`, or address
dereference syntax is introduced.

## Smoke Growth Brake

```text
new_smoke_script_added=0
existing_fastmem_capability_inventory_smoke_extended=1
new_fixture_added=safe_capability_wrapper_report.kv
bad_fixture_added=bad_safe_wrapper_inventory.kv
```

The positive fixture proves wrapper coverage. The bad inventory proves that a
non-alias wrapper route fails `fastmem-check`.

## Acceptance

```text
safe_capability_wrapper_plan=1
safe_capability_wrapper_route=fastmem_memop_alias
safe_capability_wrapper_lowering_route=fastmem_memop_alias
safe_capability_wrapper_memop_equivalence=1
safe_capability_wrapper_count=6
safe_capability_wrapper_missing_count=0
safe_capability_wrapper_rawptr_surface=0
safe_capability_wrapper_deref_surface=0
safe_capability_wrapper_escape_count=0
summary=ok
```

Proof:

```bash
python3 -m py_compile tools/hako_check/fastmem_capability_inventory.py tools/hako_check/fastmem_check.py
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed Evidence

```text
safe_capability_wrapper_plan_evidence=1
wrapper_route_aliases_fastmem_memops=1
rawptr_surface=0
deref_surface=0
source_rewrite=0
product_activation=0
```

Next row:

```text
MIM-FMEM-016 Mimalloc shape coverage score
```

## Stop Line

- do not add a general raw pointer type
- do not open pointer arithmetic outside `fastmem`
- do not add product allocator activation, hooks, global allocator claim, or
  winner claim
- do not treat safe wrappers as a Type ABI or Provider ABI hot-path dispatch
