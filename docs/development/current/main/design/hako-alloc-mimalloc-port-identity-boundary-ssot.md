---
Status: Active
Date: 2026-06-06
Scope: hako_alloc / mimalloc port identity, temporary replacement-front producer, bootstrap/application allocator split.
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# Hako Alloc Mimalloc Port Identity Boundary SSOT

## Decision

`hako_alloc` is the `.hako` body and source truth of the mimalloc port. It is
not a separate allocator family competing with the mimalloc migration.

The current replacement-front C shim is a temporary execution bridge for the
same mimalloc port. It is allowed only while product allocator activation is
closed and while bridge evidence proves that the duplicated C surface is tied
back to `.hako` source truth.

Long-term source truth is:

```text
.hako hako_alloc / fastmem / capability surface
  -> MIR FastMemRegion metadata + MemOp instructions
  -> verifier
  -> LLVM/object primary producer
```

C may remain as a MIR-to-C backend artifact for debug/bootstrap/diff work, but
Python-template C must not remain the allocator semantic producer.

## Current State

Today the mimalloc port has two active surfaces:

```text
mimalloc port
  .hako hako_alloc
    lang/src/hako_alloc/memory/page_box.hako
    lang/src/hako_alloc/memory/size_class_box.hako
    lang/src/hako_alloc/memory/worker_tls_cache_box.hako
    ...
    role: source/model/semantic truth

  python_template_c_bridge replacement front
    generated benchmark-only malloc/free/realloc front
    role: temporary execution bridge
```

This is intentional double management during migration, not the desired final
architecture.

Bridge evidence exists to keep this temporary split honest:

```text
size_class_bridge evidence:
  C size-class mirror remains tied to .hako SizeClassBox policy.

page_local_bridge evidence:
  C page-local metadata/same-owner evidence remains tied to .hako PageModel.

producer taxonomy:
  report.kv names the producer so hako_check can distinguish the bridge from
  MIR-to-C and MIR-to-LLVM producers.
```

Bridge evidence is a migration guard. It is not a permanent abstraction goal.

## Target State

The target state removes semantic duplication:

```text
source truth:
  .hako hako_alloc / fastmem / capability surface

canonical representation:
  MIR FastMemRegion side-table metadata
  MIR MemOp instruction dialect

acceptance:
  verifier gates for escape/layout/safepoint/allocation/ABI boundaries

primary product producer:
  MIR -> LLVM/object

optional artifact producer:
  MIR -> C

retired bridge:
  Python-template C replacement front
```

The report/check contract stays producer-neutral while producers change:

```text
replacement_front_producer=
  python_template_c_bridge
  | mir_to_c_lowering
  | mir_to_llvm_lowering
```

The same `report.kv` and `hako_check` fields should remain meaningful across
all producer values.

## Allocator Role Split

The compiler/runtime bootstrap allocator and application/product allocator are
different roles.

```text
runtime/bootstrap allocator:
  used by Hakorune compiler, runtime, runner, backend, and tooling
  may use Rust/std/system allocator or a small bootstrap arena
  must not depend on hako_alloc being available
  must not recurse through the product allocator under construction

application/product allocator:
  hako_alloc mimalloc port
  may eventually serve malloc/free for a target application
  can be exposed through LD_PRELOAD / provider / global allocator only by an
  explicit activation row
```

Therefore ordinary Hakorune applications do not currently imply:

```text
hako_alloc_product_activation=1
hook_installed=1
global_allocator_claim=1
winner_claim=1
```

Those remain closed until a dedicated product activation ladder reopens them.

## Naming Contract

Use these names consistently:

```text
hako_alloc:
  .hako body/source truth for the mimalloc port.

replacement_front C shim:
  temporary benchmark/product-shaped execution bridge for the same port.

Python-template C bridge:
  current temporary producer that duplicates selected .hako allocator logic.

MIR-to-C lowering:
  future backend artifact producer; C is output, not semantic truth.

MIR-to-LLVM lowering:
  primary final producer.

runtime/bootstrap allocator:
  allocator used to run/build Hakorune itself.

application/product allocator:
  allocator offered to target programs.
```

Avoid saying "hako_alloc versus mimalloc" as if they were separate products.
The accurate phrasing is "`hako_alloc` is the `.hako` representation of the
mimalloc port".

## Report Fields

Producer-neutral reports should keep these meanings visible:

```text
hako_alloc_mimalloc_port_identity=hako_alloc_is_mimalloc_hako_body

replacement_front_producer=
  python_template_c_bridge
  | mir_to_c_lowering
  | mir_to_llvm_lowering

replacement_front_source_truth=
  hako_fastmem
  | hako_alloc.size_class_box
  | hako_alloc.page_box
  | unknown

replacement_front_python_template_c_semantic_ssot=0
replacement_front_python_template_c_retirement_required=1

runtime_allocator_role=bootstrap_host_allocator
application_allocator_role=hako_alloc_mimalloc_port

hako_alloc_product_activation=0
hook_installed=0
global_allocator_claim=0
winner_claim=0
```

## Task Order

This identity clarification does not change the active blocker. Continue the
MIR-FMEM producer-transition path:

```text
MIR-FMEM-004:
  verifier gates for fastmem escape/layout/safepoint/allocation/ABI boundaries.

MIR-FMEM-005:
  MIR -> LLVM/object primary producer.
  C is not required on the primary path.
  The Python-template C bridge remains only as comparison baseline.

MIR-FMEM-006:
  producer-neutral parity against the current python_template_c_bridge.

MIR-FMEM-007:
  retire python_template_c_bridge after producer-neutral parity is proven.
  No hidden fallback to the Python-template C bridge may remain.

MIR-FMEM-C-ARTIFACT:
  optional MIR -> C debug/diff/bootstrap artifact producer.
  C is generated from MIR MemOps, not hand-maintained semantic truth.
```

Do not use these rows to open:

```text
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
runtime self-allocation through hako_alloc
```
