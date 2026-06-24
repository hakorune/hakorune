---
Status: Done
Date: 2026-06-06
Scope: connect parsed `fastmem ContractName { ... }` source to MIR FastMemory metadata without opening backend execution.
Blocker: MIR-FMEM-003
Related:
  - docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - src/mir/builder/fastmem.rs
  - src/mir/function/types.rs
  - src/mir/instruction.rs
---

# 296x-439 MIRBuilder FastMem Source Lowering

## Purpose

`MIR-FMEM-002` added the `MemOp` vocabulary and contract allowlist. This row
connects parsed source fastmem blocks to that MIR-side representation:

```text
fastmem PageMapV0 { ... }
  -> FunctionMetadata.fastmem_regions[]
  -> MirInstruction::MemOp { region, kind, dst, operands, effects }
```

This is still representation only. JSON, VM, LLVM, C artifact lowering, product
allocator activation, and replacement-front producer changes remain closed.

## Decision

```text
fastmem_region_metadata_table=1
fastmem_region_instruction_markers=0
mir_builder_fastmem_source_lowering=1
mir_builder_fastmem_route_selection=0
mir_builder_fastmem_backend_selection=0
mir_json_memop_supported=0
vm_memop_supported=0
llvm_memop_supported=0
c_artifact_memop_supported=0
```

`FastMemRegion` remains side-table metadata. MIRBuilder emits only executable
`MemOp` instructions and points each one back to the side-table region id.

## Implemented V0 Source Shapes

The first lowering surface is intentionally narrow:

```text
mem.addr(ptr) / mem.addr method form -> AddrOf
addr >> shift                       -> LogicalShr
value & mask                        -> BitAnd
value + delta                       -> Add
value - delta                       -> Sub
table[index]                        -> TableIndex
page.field                          -> FieldLoad
page.field = value                  -> FieldStore
mem.currentAllocOwnerId()           -> CurrentAllocOwnerId
mem.ownerEq(a, b)                   -> OwnerEq
```

Unsupported calls, allocation-like expressions, arbitrary method receivers, and
unhandled operators fail fast instead of falling back to normal expression
lowering.

## Acceptance

```bash
cargo check -q
cargo test -q fastmem_source_lowers_to_region_metadata_and_memops
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

The unit proof fixes the minimal source route:

```text
fastmem PageMapV0 {
  local addr = mem.addr(ptr)
  local key = (addr >> 12) & 255
}

metadata.fastmem_regions.len=1
metadata.fastmem_regions[0].contract=PageMapV0
metadata.fastmem_regions[0].emitted_memop_count=3
memop_kinds=AddrOf,LogicalShr,BitAnd
```

## Stop Line

- do not serialize `MemOp` to MIR JSON in this row
- do not execute `MemOp` in VM in this row
- do not lower `MemOp` to LLVM/C in this row
- do not add `FastMemRegionBegin` / `FastMemRegionEnd` MIR instructions
- do not choose page-map strategy in MIRBuilder
- do not open Type ABI hot lookup or Provider ABI dispatch
- do not open product allocator activation, hooks, globals, or winner claims

## Follow-Up

```text
MIR-FMEM-004:
  verifier gates for fastmem escape/layout/safepoint/allocation/ABI boundaries.
```
