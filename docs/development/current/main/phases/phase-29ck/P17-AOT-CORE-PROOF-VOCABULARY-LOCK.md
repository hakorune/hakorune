---
Status: Task Pack
Decision: accepted
Date: 2026-03-27
Scope: future `AOT-Core MIR` を新設せず、current MIR/lowering/manifest path に staged proof vocabulary を固定してから integer array fast lane へ戻る docs-first exact front。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P16-STAGE1-CANONICAL-MIR-CUTOVER.md
  - docs/development/current/main/design/stage2-aot-core-proof-vocabulary-ssot.md
  - docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/investigations/phase29ck-array-substrate-rejected-optimizations-2026-03-27.md
  - crates/nyash_kernel/src/plugin/array_slot_store.rs
  - crates/nyash_kernel/src/plugin/array_slot_load.rs
  - crates/nyash_kernel/src/plugin/handle_cache.rs
  - src/boxes/array/mod.rs
---

# P17: AOT-Core Proof Vocabulary Lock

## Purpose

- `P16` で Stage1 dialect split を retire したあと、perf lane を blind substrate tries に戻さない。
- future `AOT-Core MIR` が必要になりうることは先に固定する。
- ただし current wave では、新しい IR layer は入れず、current MIR/lowering/manifest path に narrow proof vocabulary を staged に入れる。
- first code consumer は integer-heavy `ArrayBox.get/set/len` fast lane に固定する。

## Preconditions

1. `P16-STAGE1-CANONICAL-MIR-CUTOVER.md` は landed
2. `kilo_kernel_small_hk` は `pure-first + compat_replay=none + aot_status=ok` に戻っている
3. current route mismatch は blocker ではなくなった
4. array substrate で ad hoc 候補を試したが、`WSL warmup=1 repeat=3` では mainline regressions が出た

## Fixed Reading

1. current wave は BoxCount ではなく BoxShape だよ
2. goal は「新 IR を増やすこと」ではなく「proof vocabulary を clean に置くこと」だよ
3. current proof vocabulary は次だけに固定する
   - `value_class`
   - `escape_kind`
   - `effect`
   - `cold_fallback`
4. `capture_kind` / `identity_required` は defer する

## First Carrying Seam

current phase で first carrying seam として扱うのは次だよ。

1. `value_class`
   - [`value-repr-and-abi-manifest-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md)
2. `cold_fallback`
   - [`stage2-fast-leaf-manifest-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md)
3. `escape_kind` / `effect`
   - lowering / verifier / route selection metadata

public MIR JSON syntax はこの wave では増やさない。

## First Consumer

first code consumer はこれで固定する。

- integer-heavy `ArrayBox.get/set/len` fast lane

対象 owner は次。

- `crates/nyash_kernel/src/plugin/array_slot_store.rs`
- `crates/nyash_kernel/src/plugin/array_slot_load.rs`
- `crates/nyash_kernel/src/plugin/handle_cache.rs`
- `src/boxes/array/mod.rs`

## Fixed Order

1. proof vocabulary SSOT を固定する
2. rejected optimization ledger を investigations に固定する
3. current mirrors (`README` / `CURRENT_TASK` / `10-Now` / indexes) を同期する
4. integer array fast lane へ proof vocabulary を 1 consumer だけ結ぶ
5. その exact hot path が動いてからだけ、次の perf leaf optimization を再開する

## Acceptance

- `phase-29ck` の current exact front が `P17` へ進んでいる
- future `AOT-Core MIR` は `future-needed but not now` と一意に読める
- rejected array-substrate tries が rolling investigation ledger に残っている
- next code front が integer-heavy `ArrayBox.get/set/len` fast lane へ固定されている

## Non-Goals

- broad generic `Box` optimization
- immediate distinct `AOT-Core MIR` layer
- parser/backend-active hint contract wave
- `llvmlite` keep lane への drift

## Exit Condition

- docs だけで proof vocabulary / carrying seam / first consumer / reject history が辿れる
- current perf wave が ad hoc substrate tweak ではなく structure-first exact front に戻っている
- next code patch を array integer fast lane に限定して再開できる
