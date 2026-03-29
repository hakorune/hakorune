---
Status: SSOT
Decision: provisional
Date: 2026-03-30
Scope: stage axis と owner axis を混線させずに、kernel authority wave の実装フェーズ順を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - lang/README.md
  - lang/src/runtime/README.md
  - lang/src/runtime/collections/README.md
  - lang/src/runtime/kernel/README.md
  - lang/src/runtime/meta/README.md
---

# Kernel Implementation Phase Plan (SSOT)

## Summary

- stage reading:
  - `stage0` = Rust bootstrap / recovery keep
  - `stage1` = bridge / proof line
  - `stage2+` = final `.hako` mainline / distribution target
- owner reading:
  - `.hako` owns meaning / policy / route / acceptance / control
  - `.inc` is thin shim / boundary artifact (not a semantic owner noun)
  - native keeps metal / substrate (ABI/alloc/GC/TLS/atomic/backend emission)
- kernel authority wave is collection-first and proceeds domain-by-domain:
  1. `Array phase`
  2. `Map phase`
  3. `RuntimeData cleanup phase`
  4. return to `perf-kilo`

This SSOT is the canonical phase-plan entry for the collection-first kernel migration wave.

## Fixed Boundaries

- `lang/src/runtime/collections/**` is the current ring1 collection semantics owner frontier for this wave.
- `lang/src/runtime/kernel/**` is runtime behavior/policy owner (string search, numeric loops, etc).
- `lang/src/runtime/meta/**` owns compiler semantic tables only.
- `lang/src/runtime/host/**` stays transport only.

Rule:
- do not interpret "kernel .hako-ization" as "native zero" or "substrate wholesale rewrite".

## Phase Order

### 1. Array phase

Goal:
- visible `ArrayBox` method semantics are `.hako` owned (policy/contract/orchestration).
- Rust remains raw substrate/compat (slot load/store, reserve/grow, layout, handle/cache).

Stop line:
- docs/readmes/smokes describe `ArrayBox` semantics without naming Rust helpers as meaning owners.
- array path stays in `runtime/collections` (do not force-push into `runtime/kernel/array/` unless a concrete trigger fires).

### 2. Map phase

Goal:
- visible `MapBox` method semantics are `.hako` owned (policy/contract/orchestration).
- Rust remains raw substrate/compat (probe/rehash, slot load/store, layout, handle/cache).

Stop line:
- docs/readmes/smokes describe `MapBox` semantics without naming Rust helpers as meaning owners.
- `nyash.map.entry_count_h` and other transitional observers are treated as boundary-deepen tasks, not owner logic.

### 3. RuntimeData cleanup phase

Goal:
- `RuntimeDataBox` stays protocol/facade only.
- it must not become a collection semantics owner for array/map.

Stop line:
- runtime-data dispatch remains narrow and explicit.
- no doc suggests that `RuntimeDataBox` is the "collection owner".

### 4. Return to perf-kilo

Rule:
- do not reopen broad authority expansion while perf-kilo is active.
- any further owner migration requires a new exact blocker and a dedicated SSOT update.

## Acceptance Gates (Docs + Smokes)

The phase plan is considered "done-enough to return to perf-kilo" when:

1. stage docs agree on: `stage0 keep / stage1 bridge+proof / stage2+ final mainline`.
2. owner docs agree on: `.hako authority / .inc thin shim / native metal keep`.
3. collection docs agree on: `Array phase -> Map phase -> RuntimeData cleanup phase`.
4. daily proof locks remain green:
   - array provider smoke
   - map provider smoke
   - runtime-data dispatch e2e smoke

## Non-Goals

- no new public ABI
- no "native zero" claim
- no wholesale move of `array/map` into `runtime/kernel/{array,map}` without a trigger
- no perf work inside this authority wave (perf is the next lane after phase closeout)

