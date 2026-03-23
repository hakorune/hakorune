---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `stage0/stage1/stage2/stage3` の bootstrap/distribution 軸と、`.hako core/alloc/std` の end-state layering を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md
  - docs/development/current/main/design/rune-v0-contract-rollout-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/hakoruneup-release-distribution-ssot.md
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
- release / distribution shape が必要な場合は `hakoruneup-release-distribution-ssot.md` を future reference として読む。

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
   - build lane re-emits Program/MIR payload snapshots from a known-good seed
   - identity / reproducibility confidence lane
   - helper: `tools/selfhost/stage3_same_result_check.sh`

### Artifact axis

1. `launcher-exe`
   - Stage1 run-oriented launcher artifact
2. `stage1-cli`
   - Stage1 bootstrap output artifact for the reduced Stage1 lane
3. These are artifact kinds, not stage numbers.
4. `stage2` is not a build script artifact kind in the current flow.
   - It is the future distribution target / compare label, not a separate artifact-kind family.

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
   - physical root reservation: `lang/src/hako_alloc/` (first wave: policy-plane helpers such as `ArcBox` / `RefCellBox`)
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
- `launcher-exe` / `stage1-cli` are Stage1 artifact kinds; they do not define Stage2 distribution packaging.
- current `.hako` kernel migration work lives on the owner/substrate axis and is allowed to proceed before Stage2 distribution packaging is active.
- collection owner growth belongs under `hako_alloc` / ring1 collection runtime, not ring0.
- `runtime/memory/**` is not the canonical home for alloc/policy helpers in the end-state layering.
- Rune is a contract layer that sits beside `hako_core` / `hako_alloc` / `hako_std`; it does not replace those implementation layers.

## Non-Goals

- This document does not activate packaging/distribution work.
- This document does not change current daily `phase-29ct` implementation order.
- This document does not declare Stage2 distribution complete.
- This document does not move LLVM / OS VM / GC / ABI metal into `.hako`.
