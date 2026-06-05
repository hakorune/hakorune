---
Status: Current
Date: 2026-06-05
Scope: connect `fastmem ContractName { ... }` parser output to FastMemory inventory/check metadata without opening execution.
Blocker: MIM-FMEM-008
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-416-FASTMEM-PARSER-PARITY-CATCHUP.md
  - tools/hako_check/README.md
  - tools/hako_check/fastmem_capability_inventory.py
  - tools/hako_check/fastmem_check.py
---

# 296x-417 Fastmem Source Syntax Pilot

## Purpose

`PARSER-FMEM-001` through `PARSER-FMEM-006` proved that the Rust parser and
the `.hako` parser agree on the narrow parse-only `fastmem ContractName { ... }`
surface. This row reopens the original fastmem source-syntax pilot by wiring
that parse result into FastMemory inventory/check metadata.

The pilot is intentionally not an allocator execution row. It gives
`hako_check` a source-facing FastMemRegion input so later rows can build
PageMapBridge, PageMetaHandle, WorkerId/TLS, and AtomicRemoteHead work against
visible MemOp counts instead of prose.

## Decision

```text
fastmem_source_syntax_active=1
fastmem_source_inventory_input=1
fastmem_contract_name_required=1
fastmem_contractless_region_allowed=0
unsafe_block_allowed=0

fastmem_execution_open=0
fastmem_product_lowering_open=0
provider_activation=0
replacement_activation=0
hook_installed=0
global_allocator_claim=0
winner_claim=0
```

## Scope

Accepted in this row:

```text
source/AST or Program(JSON v0) input to hako_check fastmem inventory
FastMemRegion begin/end counts from parsed source
contract id / contract family metadata from parsed source
basic MemOp classification for source expressions inside fastmem:
  mem.addr(...) -> MemAddrOf
  >>            -> MemLogicalShr
  &             -> MemAnd
forbidden call count for non-mem fastmem calls
fastmem-check consumption of source-derived inventory
```

Left for later rows:

```text
MIM-FMEM-009 PageMapBridge benchmark-front pilot
MIM-FMEM-010 TypedPageMetaHandle plan
MIM-FMEM-011 WorkerId / TLS arena owner state
MIM-FMEM-012 AtomicRemoteHead plan
MIR/backend execution lowering
replacement-front malloc/free behavior changes
product allocator activation
```

`page_table[key]` may remain unclassified or unavailable in the source JSON
transport for this row. Table/index lowering belongs to the PageMapBridge row,
not the initial syntax pilot.

## Acceptance

Proof stays light and source-facing:

```text
fastmem_region_count=1
fastmem_contract_count=1
fastmem_contract_id=PageMapV0
fastmem_contract_family=allocator.page_map
fastmem_memop_region_begin_count=1
fastmem_memop_region_end_count=1
fastmem_memop_unbalanced_region_count=0
fastmem_memop_addr_of_count=1
fastmem_memop_logical_shr_count=1
fastmem_memop_and_count=1
fastmem_forbidden_call_count=0
fastmem_type_abi_hot_lookup_count=0
fastmem_provider_abi_crossing_count=0
summary=ok
```

Negative source proof:

```text
ordinary_call_inside_fastmem:
  fastmem_forbidden_call_count>0
  fastmem-check fails
```

Required commands:

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/smokes/v2/profiles/integration/parser/fastmem_parser_parity_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
```

Add a dedicated source-syntax smoke in this row and include it in the tools
index when the implementation lands.

## Stop Line

- no broad `unsafe {}`
- no `RawPtr<T>`
- no pointer arithmetic outside `fastmem`
- no runtime contract lookup
- no Type ABI hot lookup
- no Provider ABI hot dispatch
- no lowering to allocator execution
- no benchmark winner or product readiness claim
