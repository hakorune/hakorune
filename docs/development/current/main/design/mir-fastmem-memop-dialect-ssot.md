---
Status: SSOT
Decision: accepted
Date: 2026-06-06
Scope: MIR representation boundary for `.hako` fastmem regions and memory dialect operations.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/phases/phase-296x/296x-437-MIRBUILDER-FASTMEM-MEMOP-DIALECT-DECISION.md
  - src/mir/builder/README.md
  - src/mir/contracts/README.md
---

# MIR FastMem MemOp Dialect

## Decision

`MIR-FMEM-001` accepts this representation:

```text
execution instruction:
  MirInstruction::MemOp

dialect vocabulary:
  MemOpKind

region truth:
  FastMemRegion side table / metadata

link:
  each MemOp carries region_id
```

Do not add `FastMemRegionBegin` / `FastMemRegionEnd` as normal MIR
instructions.

Short form:

```text
MemOp is execution.
MemOpKind is the memory dialect vocabulary.
FastMemRegion is contract metadata.
```

## Why Region Begin/End Are Rejected

`fastmem` is a contract-bound memory dialect, not a runtime instruction pair.
Begin/end markers would add non-executable instructions to CFG, JSON, VM, and
LLVM surfaces.

Rejected shape:

```text
FastMemRegionBegin instruction
...
FastMemRegionEnd instruction
```

Reasons:

```text
CFG noise:
  branches/loops/PHI make lexical begin/end ranges fragile

backend ambiguity:
  lowerers would need to ignore marker instructions, which weakens fail-fast

allowlist churn:
  VM / MIR JSON / LLVM allowlists would accept non-executable tags

wrong truth:
  verifier needs "which contract owns this MemOp", not a guessed instruction
  interval
```

The debug dump may print synthetic comments:

```text
# fastmem begin PageMapV0
  MemOp ...
# fastmem end PageMapV0
```

Those comments are presentation only. They are not MIR instructions.

## MIR Model

Canonical shape:

```rust
MirInstruction::MemOp {
    region: FastMemRegionId,
    kind: MemOpKind,
    dst: Option<ValueId>,
    operands: Vec<ValueId>,
    effects: EffectMask,
}
```

Region metadata:

```rust
struct FastMemRegion {
    id: FastMemRegionId,
    contract: FastMemContractId,
    source_span: Span,
    origin: FastMemOrigin,
    layout_ids: Vec<MemLayoutId>,
    pointer_classes: Vec<MemPointerClass>,
    flags: FastMemRegionFlags,
}
```

Truth lives here:

```text
region_id -> contract_id
```

Reports and JSON may redundantly print the contract id next to each MemOp for
diagnostics, but that copy is not the SSOT.

## V0 MemOpKind

`MIR-FMEM-001` keeps the first dialect narrow:

```text
AddrOf
LogicalShr
BitAnd
Add
Sub
TableIndex
FieldLoad
FieldStore
CurrentAllocOwnerId
OwnerEq
```

Atomic operations are intentionally a later slice:

```text
AtomicLoad
AtomicStore
AtomicCas
AtomicExchange
AtomicFetchAdd
```

Reason: the first MIR row should lock representation and contracts before
remote-free behavior or memory-order semantics are added.

## MemValueKind

Fast memory values must not silently become ordinary `i64`, `Box`, or raw
pointer values. The MIR layer should track a memory-value kind for Values
created by `MemOp`.

V0 vocabulary:

```text
UserPtr
Address
USize
PageKey
LayoutRef(MemLayoutId)
FieldValue(MemFieldId)
AllocOwnerId
Bool
```

Rules:

```text
MemValueKind values are no-escape by default.
They cannot be returned, stored into Box fields, stored into Arrays, captured
by closures, passed through Provider ABI, or used for Type ABI hot lookup.
Ordinary calls cannot receive them unless a verifier-approved intrinsic
descriptor allows it.
```

Exception:

```text
UserPtr may be an ABI endpoint for allocator malloc/free/realloc/usable_size
surfaces, but it still cannot become a general dereferenceable source pointer.
```

## Contracts Model

`mir/contracts` remains the backend acceptance SSOT.

Two layers are required:

```text
instruction vocabulary:
  MemOp

dialect vocabulary:
  MemOpKind allowlist
```

Implementation placement:

```text
src/mir/contracts/backend_core_ops.rs:
  add the `MemOp` instruction tag and kept-tag membership

src/mir/contracts/fastmem_ops.rs:
  define FastMemBackend and MemOpKind support tables
```

Backends must not maintain hidden MemOpKind allowlists.

Unknown instruction tags or unknown MemOpKind values must fail fast. Silent
drop is forbidden.

## JSON Shape

MIR JSON should use a region table plus MemOp payloads:

```json
{
  "fastmem_regions": [
    {
      "id": 1,
      "contract": "PageMapV0",
      "source_span": "...",
      "layouts": ["PageMetaLayoutV0"],
      "flags": ["no_alloc", "no_safepoint", "no_escape"]
    }
  ],
  "instructions": [
    {
      "op": "memop",
      "region": 1,
      "kind": "addr_of",
      "dst": 10,
      "operands": [1]
    }
  ]
}
```

Do not serialize begin/end instructions.

## MIRBuilder Boundary

MIRBuilder must:

```text
preserve source span, origin, region id, and contract id
emit MemOp instructions for classified fastmem operations
assign MemValueKind facts for memory values
fail fast on malformed region nesting or missing contract ids
avoid ghost ValueIds
```

MIRBuilder must not:

```text
choose page-map strategy
choose side_table vs two_level vs page_base_mask
choose C vs LLVM backend
choose fast vs slow route
choose Type ABI or Provider ABI route
claim product activation or keeper status
```

## Planner / Verifier / Lowering Boundary

Planner chooses:

```text
selected_mem_route
selected_producer
selected_lowering_backend
page_map_bridge_strategy
fast/slow route
```

Verifier enforces:

```text
contract exists
MemOpKind is supported for the selected profile
layout fields are verified
MemValueKind values do not escape
no allocation
no safepoint
no arbitrary calls
no Type ABI hot lookup
no Provider ABI hot dispatch
```

Lowering consumes a selected plan:

```text
LLVM primary:
  ptrtoint / lshr / and / gep / load / store / icmp

C artifact optional:
  uintptr_t / >> / & / field access
```

C may remain as a backend artifact. Python-template C must not remain a second
semantic SSOT.

## Capability Wrapper Relation

`fastmem` and future safe wrappers lower to the same MemOps.

```text
fastmem PageMapV0 { ... }
  -> MemOp AddrOf / LogicalShr / BitAnd / TableIndex

AddressToken -> PageKey -> PageMapBridge -> PageMetaHandle
  -> same MemOp sequence
```

Wrapper lowering must be descriptor-driven, not name-pattern matching:

```text
accepted:
  explicit MemIntrinsic / capability descriptor registry

rejected:
  lowering that searches for string names like PageKey.from
```

## Future FastPath Generalization

`fastmem ContractName { ... }` is the accepted near-term spelling for the
memory-profile pilot. It should not force the language to grow one keyword per
future low-level domain.

Future generalization candidate:

```hako
fastpath PageMapV0 { ... }
fastpath SocketBufferV0 { ... }
fastpath VectorOpV0 { ... }
```

Conceptual model:

```text
left side:
  one common contract-bound fast-path region syntax

right side:
  contract id / profile selector
```

Examples:

```text
PageMapV0:
  profile=memory

SocketBufferV0:
  profile=io

VectorOpV0:
  profile=simd
```

MIR direction if this opens:

```rust
struct ContractRegion {
    id: ContractRegionId,
    profile: ContractProfile,
    contract: ContractId,
    source_span: Span,
    origin: ContractRegionOrigin,
    flags: ContractRegionFlags,
}

MirInstruction::ContractOp {
    region: ContractRegionId,
    profile: ContractProfile,
    kind: ContractOpKind,
    dst: Option<ValueId>,
    operands: Vec<ValueId>,
    effects: EffectMask,
}
```

Profile-specific dialects stay separate:

```text
profile=memory:
  MemOpKind

profile=simd:
  SimdOpKind

profile=io:
  IoOpKind
```

This is not accepted as active syntax in `MIR-FMEM-001`. The current accepted
MIR row remains:

```text
FastMemRegion + MemOp + MemOpKind
```

The general form is a planned abstraction path, not a reason to widen the v0
memory dialect before it is implemented and verified.

### Multiple Contract Idea

Future syntax idea:

```hako
fastpath PageMapV0, VectorOpV0 {
    ...
}
```

This is possible as syntax, but it is not accepted for v0.

Reason:

```text
comma syntax would make the source grammar own contract composition semantics:
  contract ordering
  profile conflict resolution
  effective flags when contracts disagree
  cross-profile value bridges
  report region_id ownership
  verifier blame assignment
```

Preferred future shape if a combined region is needed:

```hako
fastpath PageMapVectorProbeV0 {
    ...
}
```

The composite contract descriptor owns the merge:

```text
PageMapVectorProbeV0:
  includes:
    PageMapV0
    VectorOpV0

  effective_flags:
    stricter-wins / intersection

  allowed_bridges:
    memory.load_to_simd_vector
```

Guidance:

```text
v0:
  use adjacent or nested regions
  reject `fastpath A, B { ... }`

future:
  prefer named composite contracts over comma-list source semantics
```

## Selfhost Timing

Do not rename or generalize the source spelling before the current
selfhosting-sensitive parser/MIRBuilder lanes are stable.

Current order:

```text
before selfhost stabilization:
  keep `fastmem ContractName { ... }` as the memory-profile pilot
  document `fastpath` as the future general spelling only
  do not add fastio / fastsimd keywords
  do not add `fastpath` parser behavior

after selfhost parser/MIRBuilder stability:
  consider `fastpath ContractName { ... }` as the canonical general spelling
  decide whether `fastmem` remains a memory-profile alias or migrates through a
  compatibility window
```

Early exception:

```text
Only open `fastpath` before selfhost stabilization if a non-memory profile
becomes an active blocker and a phase card explicitly accepts the grammar and
dual-parser parity work.
```

## Report Fields

Producer-neutral evidence should include:

```text
fastmem_region_metadata_table=1
fastmem_region_instruction_markers=0
fastmem_memop_instruction_enabled=1
fastmem_memop_kind_allowlist_v0=1
fastmem_region_count
fastmem_memop_count
fastmem_unknown_memop_kind_count=0
fastmem_escape_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
replacement_front_producer=python_template_c_bridge|mir_to_c_lowering|mir_to_llvm_lowering
```

## Task Split

```text
MIR-FMEM-001:
  this design decision and docs only

MIR-FMEM-002:
  add mir/contracts instruction tag, MemOpKind allowlist, and JSON vocabulary

MIR-FMEM-003:
  lower parsed fastmem source to FastMemRegion metadata + MemOp instructions

MIR-FMEM-004:
  verifier gates for MemValueKind escape, layout, safepoint, allocation, and
  ABI boundaries

MIR-FMEM-005:
  MIR -> C backend artifact producer

MIR-FMEM-006:
  MIR -> LLVM/object primary producer

MIR-FMEM-007:
  retire python_template_c_bridge once producer-neutral parity is proven
```

## Acceptance For MIR-FMEM-001

This row is complete when docs say:

```text
Decision: MemOp single instruction
Decision: MemOpKind dialect
Decision: FastMemRegion side table / metadata
Rejected: FastMemRegionBegin/End as normal MIR instructions
Rejected: MIRBuilder route/backend/product decisions
```

No MIR enum, JSON, verifier, or lowering code changes are required in
`MIR-FMEM-001`.
