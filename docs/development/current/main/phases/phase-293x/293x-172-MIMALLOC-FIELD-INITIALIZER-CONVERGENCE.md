---
Status: Complete
Date: 2026-05-12
Scope: mimalloc/hako_alloc stored field initializer convergence.
Related:
  - lang/src/hako_alloc/memory/page_box.hako
  - lang/src/hako_alloc/memory/page_queue_box.hako
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/allocator_facade_box.hako
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
---

# 293x-172 Mimalloc Field Initializer Convergence

## Goal

Apply the 293x-171 parser/lowering result to the active mimalloc `.hako` port
code before continuing the allocator algorithm ladder.

Fixed defaults and owner construction now live at the stored field declaration
site:

```hako
free: ArrayBox = new ArrayBox()
used: IntegerBox = 0
heap: HakoAllocHeap = new HakoAllocHeap()
```

Constructor parameters remain in `birth(...)`, so the source still has a clear
boundary between caller-provided identity/configuration and local default state.

## Changes

- M165 `HakoAllocPageModel`: moved arrays and counters to stored field
  initializers.
- M166 `HakoAllocPageQueue`: moved queue/default counters and direct-page cache
  default to stored field initializers.
- Legacy compatibility `HakoAllocPage` / `HakoAllocHeap` / `HakoAllocHandle`:
  removed `init { ... }` slot lists in favor of direct stored declarations and
  initializers where defaults are fixed.
- `HakoAllocProductionFacade`: moved heap construction and counters to stored
  field initializers.

## Non-goals

- No algorithm row advancement: this does not add M167 allocation fast path
  behavior.
- No OSVM, TLS, atomic, remote-free, provider, hook, or process allocator
  replacement behavior.
- No field initializer that depends on a constructor parameter.

## Proof

```bash
cargo build --release --bin hakorune
bash tools/checks/k2_wide_mimalloc_page_model_guard.sh
bash tools/checks/k2_wide_mimalloc_page_queue_guard.sh
bash tools/checks/k2_wide_mimalloc_layout_migration_guard.sh
cargo test -q stored_field_initializers_generate_birth_prologue
```
