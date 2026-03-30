---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `stage0/stage1/stage2-mainline/stage2+/stage3` の bootstrap/distribution 軸と、`.hako core/alloc/std` の end-state layering を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md
  - docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md
  - docs/development/current/main/design/rune-v0-contract-rollout-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/hakoruneup-release-distribution-ssot.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/final-metal-split-ssot.md
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - lang/README.md
  - lang/src/runtime/collections/README.md
---

# Stage2 Selfhost And Hako-Alloc (SSOT)

## Goal

- `owner/substrate` 軸と `stage0/stage1/stage2-mainline/stage2+/stage3` 軸を混線させない。
- final end-state を `.hako` の `core/alloc/std` layering と thin native metal keep で固定する。
- current `lang/bin/hakorune` / `target/selfhost/hakorune` を stage1 snapshot/proof line として読み、final distribution target と混同しない。
- standard distribution shape is `hakoruneup + self-contained release bundle`; detailed packaging policy は `hakoruneup-release-distribution-ssot.md` を正本にする。
- shared artifact/lane vocabulary is owned by `execution-lanes-and-axis-separation-ssot.md`; this child doc owns stage/distribution layering and library layering.
- stage2-mainline entry order and the first optimization wave are owned by `stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md`.
- conduit note:
  - `stage1` has concrete build/invoke conduits today (`tools/selfhost/build_stage1.sh`, `tools/selfhost/run_stage1_cli.sh`)
  - `stage2-mainline` in this document is the daily mainline / target-mainline reading; `stage2+` is the umbrella end-state reading, not a current standalone build-script family
  - `stage3` remains a compare/sanity label only

## Fixed Reading

### Stage axis

1. `stage0`
   - Rust bootstrap keep
   - first-build / recovery / preservation lane
   - includes the maintained `llvmlite` compat/probe keep for stage0/bootstrap preservation
2. `stage1`
   - selfhost bridge/proof line
   - current dev/stable snapshot artifacts live here
  - collection cleanup may progress on the separate owner/substrate axis, but stage1 itself remains bridge/proof rather than stage2-mainline daily mainline
3. `stage2-mainline`
   - truly-current `.hako` compiler/runtime/stdlib/kernel/plugin
   - daily distribution lane
4. `stage2+`
   - umbrella / end-state distribution target
4. `stage3`
   - same-result sanity check
   - build lane re-emits Program/MIR payload snapshots from a known-good seed
   - identity / reproducibility confidence lane
   - helper: `tools/selfhost/stage3_same_result_check.sh`
   - not a dedicated artifact-kind family

### Artifact axis

1. `launcher-exe`
   - Stage1 run-oriented launcher artifact
2. `stage1-cli`
   - Stage1 bootstrap output artifact for the reduced Stage1 lane
3. These are artifact kinds, not stage numbers.
4. `stage2-mainline` is not a build script artifact kind in the current flow.
   - It is the daily distribution lane / compare label, not a separate artifact-kind family.
5. `stage2+` remains the umbrella / end-state reading and is not a build script artifact kind in the current flow.
6. first stage2-mainline optimization wave is `route/perf only`; this doc does not promote Rune/backend-active optimization as part of stage2-mainline entry.

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
- `launcher-exe` / `stage1-cli` are Stage1 artifact kinds; they do not define Stage2+ distribution packaging.
- standard distribution reading is `hakoruneup + self-contained release bundle`; a single stage artifact is not the default packaging truth.
- `stage2-aot-native-thin-path-design-note.md` owns the AOT fast-lane rule:
  - source layering stays
  - execution layering may collapse only inside `AOT/native`
  - daily/mainline AOT lane is `ny-llvm` / `ny-llvmc`
  - `llvmlite` remains stage0/compat/probe keep
  - this parent doc does not re-decide that route policy
- `hako-alloc-policy-state-contract-ssot.md` owns the first concrete allocator policy/state rows:
  - handle reuse policy
  - GC trigger threshold policy
  - stop-line for reserved-only `RawBuf / Layout / MaybeInit`
- boundary artifacts are not semantic owners:
  - headers/shims stay thin
  - owner truth is fixed separately by `hako.abi` / `hako.value_repr` / ownership-layout manifests
- current `.hako` kernel migration work lives on the owner/substrate axis and is allowed to proceed before Stage2 distribution packaging is active.
- `.hako complete` in this doc means authority/mainline completion, not native zero.
- collection owner growth belongs under `hako_alloc` / ring1 collection runtime, not ring0.
- `runtime/memory/**` is not the canonical home for alloc/policy helpers in the end-state layering.
- Rune is a contract layer that sits beside `hako_core` / `hako_alloc` / `hako_std`; it does not replace those implementation layers.

## Non-Goals

- This document does not activate packaging/distribution work.
- This document does not change current daily `phase-29ct` implementation order.
- This document does not declare Stage2 distribution complete.
- This document does not move LLVM / OS VM / GC / ABI metal into `.hako`.
