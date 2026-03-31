---
Status: SSOT
Decision: provisional
Date: 2026-03-31
Scope: `stage` 軸を build/distribution 用語のまま固定しつつ、kernel の本当の置換進捗を `K0 / K1 / K2(core|wide)` replacement axis で読む。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/de-rust-zero-buildability-contract-ssot.md
  - docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - docs/development/current/main/design/stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/raw-array-substrate-ssot.md
  - docs/development/current/main/design/raw-map-substrate-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/final-metal-split-ssot.md
  - docs/development/current/main/design/hakoruneup-release-distribution-ssot.md
  - lang/src/runtime/host/README.md
---

# Kernel Replacement Axis (SSOT)

## Goal

- `stage0/stage1/stage2-mainline/stage2+` を置換進捗の語に流用しない。
- kernel の本当の置換進捗は、`K0 / K1 / K2(core|wide)` replacement axis で読む。
- 責務分割は変えず、`K2` を substrate era として昇格する。
- default target を `zero-rust` に寄せるが、bootstrap/recovery/reference/buildability keep と native metal keep は明示 keep にする。

## Fixed Reading

### Stage axis stays unchanged

- `stage0` = Rust bootstrap / recovery keep
- `stage1` = same-boundary swap proof line
- `stage2-mainline` = daily mainline / current distribution lane
- `stage2+` = umbrella / end-state label

Stage axis is for buildability, proof, and distribution reading only.
Replacement progress must not overload those names.

### Replacement axis

| Axis | Meaning | Success reading |
| --- | --- | --- |
| `K0` | Boundary Lock | `hako.abi` / `hako.value_repr` / ownership-layout / fail-fast contract が固定され、same-boundary swap の判定基準が docs で 1 枚に読める |
| `K1` | Semantic Owner Swap | method contract / route / acceptance / fallback / orchestration の daily owner が `.hako` に移る |
| `K2` | Substrate Era | `.hako substrate module` が daily owner に入り始め、widening と metal review がこの era の中で進む |

### `K2` internal states

- `K2-core`
  - first daily `.hako substrate`
  - first concrete pilot is `RawArray`
- `K2-wide`
  - `RawMap` second
  - capability widening packs
  - metal keep review

### Legacy mapping

- old `K3 capability widening` maps to `K2-wide`
- old `K4 metal split review` maps to `K2-wide`

## Implementation Flow

- the engineering line after `K0` can be read as one `K1 + K2` migration track.
- `K1` and `K2` remain separate gates / acceptance checkpoints.
- `K1` still means semantic owner swap.
- `K2` still means substrate era.
- the point of the compression is to make the build order feel like one post-`K0` line while keeping the gates distinct.

## Responsibility Split

### `.hako owner`

- meaning
- policy
- route
- acceptance
- control structure
- visible orchestration

### `.hako substrate`

- capability-backed low-level control
- `RawArray`
- `RawMap`
- future allocator/runtime policy state machine

### compiler/lowering owner

- compiler semantic tables
- lowering choice
- analyzer-heavy window probes
- builder seam

Do not mix this bucket into runtime kernel owner/substrate docs.

### `.inc thin shim`

- ABI shaping
- marshal
- fail-fast boundary checks
- backend entry glue

### native metal keep

- value/object layout
- handle registry / slot table
- GC barrier / root hooks
- final ABI entry stubs
- allocator backend
- OS/TLS/atomic fallback
- backend emission

## Naming Policy

- same-boundary daily swap targets are called:
  - `.hako kernel module`
  - `.hako substrate module`
- `plugin` is reserved for:
  - `runtime/host` loader lane
  - explicit cold dynamic lane

Do not use `plugin` as the noun for daily kernel/substrate replacement.

## Operational Order

### `K0` Boundary Lock

- boundary truth belongs to:
  - `hako.abi`
  - `hako.value_repr`
  - ownership/layout manifest
  - fail-fast / verifier contract
- `.inc` is not the boundary truth; it is a transitional thin artifact/shim.

### `K1` Semantic Owner Swap

- current collection-first wave is read as `K1`.
- `Array -> Map -> RuntimeData cleanup` is the visible semantic-owner wave.
- `RuntimeDataBox` stays facade-only throughout `K1`.

### `K2` Substrate Era

#### `K2-core`

- `K2-core` is the first real replacement milestone.
- entering `K2-core` means a capability-backed `.hako substrate module` becomes daily owner, not just a future note.
- first concrete `K2-core` pilot is `RawArray`.

#### `K2-wide`

- second concrete target is `RawMap`, but only after `RawArray` is operationally stable.
- widen only through capability modules.
- do not widen through ad hoc native escape hatches.
- keep `RawArray first / RawMap second / RuntimeData facade-only` while widening.
- review what still belongs to metal keep inside this era.
- this is not `native zero`.
- this is not `Rust source zero`.

## Zero-Rust Default

- default daily/distribution target is `zero-rust`.
- in this document, `zero-rust` means:
  - normal daily operation does not require Rust/Cargo as a user-facing dependency
  - standard distribution is read as self-contained delivery, not Cargo-first workflow
- `zero-rust` does not mean:
  - native zero
  - metal zero
  - bootstrap/recovery zero

### Explicit keep

- `stage0` bootstrap / recovery keep
- `stage1` proof / comparison keep
- Rust buildability contract
- native metal keep
- reference / archive / canary routes

## Current Reading

- current repo state is:
  - `K0` partially locked
  - `K1` active and done-enough on the collection semantic-owner wave
  - `K2-core` not yet entered as a daily owner replacement lane
- therefore the next structural step is not another broad owner rename.
- the next structural step is to make `RawArray` the first truthful `K2-core` pilot.

## Non-Goals

- redefining `stage0/stage1/stage2-mainline/stage2+`
- treating `K1` as sufficient proof of kernel replacement
- calling daily same-boundary swap code a `plugin`
- re-expanding `K2-wide` into public top-level milestones without a new architectural reason
- treating `zero-rust` as bootstrap/buildability deletion
- claiming `native zero` or `metal zero`
