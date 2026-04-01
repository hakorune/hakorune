---
Status: SSOT
Decision: provisional
Date: 2026-03-31
Scope: `stage` 軸を build/distribution 用語のまま固定しつつ、`K0 / K1 / K2` を hakorune の build/runtime stage axis として読み、`K2-core` / `K2-wide` を `K2` 内 task pack として扱う。
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
- `K-axis` は hakorune の build/runtime stage axis として読む。
- `K-axis` は task ledger ではない。
- task packs (`boundary lock`, semantic owner swap, `RawArray`, `RawMap`, capability widening, metal keep shrink) は別軸で追う。
- current active order is `stage / docs / naming` -> `K1 done-enough stop-line` -> `K2-core acceptance lock` -> `K2-wide deferred` -> `zero-rust default`.
- 責務分割は変えず、`K2` を `.hako kernel` mainline / `zero-rust` daily-distribution stage として読む。
- default target を `zero-rust` に寄せるが、bootstrap/recovery/reference/buildability keep と native metal keep は明示 keep にする。

## Fixed Reading

### Stage axis stays unchanged

- `stage0` = Rust bootstrap / recovery keep
- `stage1` = same-boundary swap proof line
- `stage2-mainline` = daily mainline / current distribution lane
- `stage2+` = umbrella / end-state label

Stage axis is for buildability, proof, and distribution reading only.
`K-axis` must not overload those names.

### `K-axis`

| Axis | Meaning | Success reading |
| --- | --- | --- |
| `K0` | all-Rust hakorune | Rust-built hakorune が baseline / bootstrap reference / comparison keep として読める |
| `K1` | `.hako kernel` migration stage | semantic kernel の daily owner が `.hako` 側へ移り始め、Rust keep と並走しながら current migration wave が回る |
| `K2` | `.hako kernel` mainline / `zero-rust` daily-distribution stage | daily/distribution の normal path が `.hako kernel` mainline で読み取れ、`K2-core` / `K2-wide` task packs がこの stage の中で進む |

### Task packs stay separate

- boundary lock
- semantic owner swap
- `RawArray`
- `RawMap`
- capability widening
- metal keep shrink

### `K2` internal task packs

- `K2-core`
  - first daily `.hako substrate`
  - first concrete pilot is `RawArray`
  - acceptance lock:
    - Rust/kernel RawArray acceptance tests:
      - `runtime_data_invalid_handle_returns_zero`
      - `runtime_data_array_round_trip_keeps_rawarray_contract`
      - `runtime_data_array_has_keeps_runtime_facade_fail_safe_contract`
      - `runtime_data_array_non_i64_keys_keep_fail_safe_fallback_contract`
      - `runtime_data_scalar_handle_keeps_facade_only_contract`
      - `legacy_set_h_returns_zero_but_applies_value`
      - `hi_hii_aliases_keep_fail_safe_contract`
      - `slot_load_store_raw_aliases_keep_contract`
      - `slot_append_raw_alias_keeps_contract`
      - `slot_reserve_and_grow_raw_aliases_keep_length_and_expand_capacity`
    - lowering/manifest drift pack:
      - `test_runtime_data_dispatch_policy`
      - `test_collection_method_call`
      - `test_boxcall_collection_policy`
      - `test_rawarray_manifest_lock`
- `K2-wide`
  - `RawMap` second
  - `RawMapCoreBox` first live substrate slice is the narrow map entry point (`entry_count / cap / probe / slot_load / slot_store`)
  - capability widening packs:
    - `hako.atomic`
    - `hako.tls`
    - `hako.gc`
    - `hako.osvm`
  - `hako_alloc` policy/state rows and allocator/TLS/GC policy-owner widening
  - metal keep review is limited to truthful seam inventory and boundary-shrink planning
  - keep `RuntimeDataBox` facade-only and do not widen through ad hoc native escape hatches

## Implementation Flow

- `Rune` is the canonical primitive control plane and is landed/keep, not the current blocker lane.
- current stage progression is `K0 -> K1 -> K2`.
- `K2-core` and `K2-wide` are separate task packs / acceptance checkpoints inside `K2`.
- `K0` is the all-Rust baseline, `K1` is the migration stage, and `K2` is the mainline/daily stage.
- the point of the current reshaping is to keep stage progression, task packs, and stage/build vocabulary separate.

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
- widening surface inside `K2-wide`:
  - `hako.atomic`
  - `hako.tls`
  - `hako.gc`
  - `hako.osvm`
  - `hako_alloc` policy/state rows

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

## Artifact Contract

### Current repo reality

- Cargo/Rust artifacts still appear as:
  - `target/release/hakorune`
  - `target/selfhost/hakorune`
  - `lang/bin/hakorune`

### Target contract

- `K0` / `K1` are read primarily as binaries:
  - dev/current outputs live under `target/k0/` and `target/k1/`
  - promoted snapshots live under `artifacts/k0/` and `artifacts/k1/`
- `K2` is read primarily as a distribution bundle:
  - `dist/k2/<channel>/<triple>/bundle/`
- `K2` bundle reading does not imply `native zero`; it fixes the distribution unit, not metal deletion.

## Operational Order

### `K0` all-Rust hakorune

- Rust implementation remains the kernel owner.
- use `K0` as the all-Rust baseline / bootstrap reference / comparison keep.
- boundary lock belongs to the task axis, not to the `K0` stage definition.

### `K1` `.hako kernel` migration stage

- current collection-first wave is read as the current migration slice inside `K1`.
- `Array -> Map -> RuntimeData cleanup` is the visible current done-enough stop-line inside `K1`.
- `RuntimeDataBox` stays facade-only throughout `K1`.

### `K2` `.hako kernel` mainline / `zero-rust` daily-distribution stage

#### `K2-core`

- `K2-core` is the first task pack inside `K2`.
- entering `K2-core` means a capability-backed `.hako substrate module` becomes the first truthful `K2` substrate owner, not just a future note.
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

- default daily/distribution target inside `K2` is `zero-rust`.
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
  - `K0` is the all-Rust baseline / bootstrap reference reading
  - `K1` is active and done-enough on the collection migration wave
  - `K2` is not yet entered as the daily owner stage
- visible order therefore reads as:
  - `K0`
  - `K1`
  - `K2-core acceptance lock`
  - `RawMap` deferred in `K2-wide`
- current active order is:
  - `stage / docs / naming` fixation
  - `K1 done-enough` stop-line fixation
  - `K2-core acceptance lock`
  - `K2-wide` deferred follow-up
  - `zero-rust` default operationalization
- therefore the next structural step is not another broad stage rename.
- the next structural step is to make `RawArray` the first truthful `K2-core` pilot inside `K2`.

## Non-Goals

- redefining `stage0/stage1/stage2-mainline/stage2+`
- treating `K1` as sufficient proof of kernel replacement
- calling daily same-boundary swap code a `plugin`
- re-expanding `K2-wide` into public top-level milestones without a new architectural reason
- treating `zero-rust` as bootstrap/buildability deletion
- claiming `native zero` or `metal zero`
