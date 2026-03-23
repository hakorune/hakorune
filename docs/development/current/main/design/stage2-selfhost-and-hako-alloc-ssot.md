---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `stage0/stage1/stage2/stage3` の bootstrap/distribution 軸と、`.hako core/alloc/std` の end-state layering を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/final-metal-split-ssot.md
  - lang/README.md
  - lang/src/runtime/collections/README.md
---

# Stage2 Selfhost And Hako-Alloc (SSOT)

## Goal

- `owner/substrate` 軸と `stage0/stage1/stage2/stage3` 軸を混線させない。
- final end-state を `.hako` の `core/alloc/std` layering と thin native metal keep で固定する。
- current `lang/bin/hakorune` / `target/selfhost/hakorune` を stage1 snapshot/proof line として読み、final distribution target と混同しない。

## Fixed Reading

### Stage axis

1. `stage0`
   - Rust bootstrap keep
   - first-build / recovery / preservation lane
2. `stage1`
   - selfhost bridge line
   - current dev/stable snapshot artifacts live here
3. `stage2`
   - truly-current `.hako` compiler/runtime/stdlib/kernel/plugin
   - future distribution target
4. `stage3`
   - same-result sanity check
   - identity / reproducibility confidence lane

### Library layering axis

1. `hako_core`
   - base types
   - slice/span-like vocabulary
   - minimum capability declarations
2. `hako_alloc`
   - `RawBuf`
   - `Layout`
   - `MaybeInit`
   - `Vec/Map`-class collection substrate
   - future allocator policy / hakozuna policy owner
3. `hako_std`
   - process / env / fs / time / net / plugin-host / C ABI convenience

### Native keep

Native keep remains below those layers:

- LLVM
- OS virtual memory / page commit
- final allocator syscall/backend calls
- exact GC plumbing
- final ABI entry stubs
- platform TLS fallback
- platform atomic fallback

## Current Rule

- `lang/bin/hakorune` is a Stage1 stable snapshot, not the final distribution truth.
- `target/selfhost/hakorune` is the current Stage1 dev line, not the final distribution truth.
- current `.hako` kernel migration work lives on the owner/substrate axis and is allowed to proceed before Stage2 distribution packaging is active.
- collection owner growth belongs under future `hako_alloc` / ring1 collection runtime, not ring0.

## Non-Goals

- This document does not activate packaging/distribution work.
- This document does not change current daily `phase-29ct` implementation order.
- This document does not declare Stage2 distribution complete.
- This document does not move LLVM / OS VM / GC / ABI metal into `.hako`.
