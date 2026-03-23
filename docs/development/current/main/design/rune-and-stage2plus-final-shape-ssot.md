---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `stage2+` final shape と Rune の役割を current truth / clean end-state に分けて固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/thread-and-tls-capability-ssot.md
  - docs/development/current/main/design/final-metal-split-ssot.md
  - docs/development/current/main/design/rune-v0-contract-rollout-ssot.md
  - docs/development/current/main/phases/phase-29cu/README.md
  - lang/README.md
  - lang/src/runtime/collections/README.md
  - docs/private/roadmap/phases/phase-23-type-system/rune-design.md
  - docs/private/roadmap/language-evolution/macro-rune-tag-strategy.md
---

# Rune And Stage2+ Final Shape (SSOT)

## Goal

- current public-main truth と clean end-state を混ぜずに読む。
- `stage0/stage1/stage2+` 軸と `owner/substrate` 軸の分離を Rune lane にも適用する。
- Rune を `.hako` owner growth のための contract layer として固定し、実装本体や metal/substrate と混線させない。

## 1. Current Truth

- `lang/` は current でも `Rust-less Kernel (C ABI)` の line として読む。
- current hot path の目標は `Hakorune -> LLVM -> C ABI` であり、daily backend convergence は `ny-llvmc` line を優先する。
- `lang/src/runtime/collections/**` はまだ thin `.hako` wrapper + Rust/native primitive seam を含む。
- collection semantics / runtime policy の owner growth は `.hako` ring1 に向かうが、primitive storage / raw substrate はまだ native keep が残る。
- `ring0` は OS-facing capability のみで、collection/runtime semantics owner にはしない。

## 2. Clean End-State

### 2.1 Stage axis

| Stage | Reading | Keep / Target |
| --- | --- | --- |
| `stage0` | Rust bootstrap / recovery / first-build lane | explicit keep |
| `stage1` | selfhost bridge / proof / migration artifact lane | keep while migrating |
| `stage2+` | daily selfhost mainline / distribution target | target |

### 2.2 Owner / substrate split

At `stage2+`, the target reading is:

- compiler / kernel / collection / plugin mainline: `.hako` owner
- native metal keep only:
  - LLVM backend
  - final C ABI stubs
  - OS virtual memory / dynamic loader
  - GC barrier / root / pin hooks
  - platform TLS / atomic fallback
  - final Box/Type metal leaf

`stage1` success does not mean this owner split is complete. It remains a bridge/proof line.

## 3. Ring Split

| Ring | Responsibility | Examples |
| --- | --- | --- |
| `ring0` | OS-facing capability only | `mem`, `io`, `fs`, `time`, `log`, `thread`, `abi`, `osvm`, `gc bridge` |
| `ring1` | language/runtime owner | `array`, `map`, `string`, `runtime_data` facade, allocator policy, plugin mainline, kernel semantics |

The exact owner frontier remains `ring1 semantic owner -> algorithm substrate -> capability substrate -> native metal keep`.

## 4. Rune Role

Rune is a declaration-attached contract layer.

- Rune marks capability / visibility / ownership / ABI-facing lowering metadata on declarations.
- Rune does not execute logic.
- Rune does not replace ordinary `.hako` code, intrinsic bodies, or native helpers.
- `hako_module.toml` remains the file/module export boundary SSOT; Rune visibility is declaration-level only.

### 4.1 Rune families

| Family | Examples | Reading |
| --- | --- | --- |
| stable semantic rune | `Public`, `Internal`, `Ownership(...)`, `FfiSafe` | compiler/verifier readable contract |
| restricted lowering rune | `Symbol("...")`, `CallConv("c")`, `ReturnsOwned`, `FreeWith("...")` | ABI-facing lowering metadata; compiler-owned and tightly scoped |
| future protocol/capability rune | `Serializable`, `Deterministic`, `ValueLike` | deferred beyond v0 |

Optimization-promise runes such as `Inline`, `NoAlias`, `InBounds`, and `TlsModel(...)` are not part of public Rune v0. They stay deferred or restricted until verifier strength exists.

## 5. What Rune Does Not Replace

Rune does not replace:

- `memcpy`, CAS, slot load/store, queue operations
- allocator state machine implementation
- TLS implementation details
- GC bridge implementation details
- raw pointer / span / layout / init primitives

Those belong to `hako_core`, `hako_alloc`, `hako_std`, and the substrate capability lanes.

## 6. Required Layering Beside Rune

Rune only works if the implementation layers exist beside it:

- `hako_core`
  - base types
  - `Ptr` / `Span`
  - minimum capability declarations
- `hako_alloc`
  - `Layout`
  - `MaybeInit`
  - `RawBuf`
  - collection substrate / allocator policy
- `hako_std`
  - process / env / fs / time / net / abi / plugin host convenience

## 7. Fixed Reading

1. Rune growth belongs to the `.hako` compiler authority axis, not to the native substrate axis.
2. Adding Rune metadata does not reopen or weaken the final metal split.
3. Grammar activation requires both the Rust parser and the `.hako` parser; docs-only lock may land first.
4. V0 backend consumption is `ny-llvmc` only; `llvmlite` stays compat/noop keep.
