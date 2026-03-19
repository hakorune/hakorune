---
Status: Active
Decision: provisional
Date: 2026-03-19
Scope: `kernel-mainline`（`.hako` kernel）authority migration の fixed order と promotion trigger を 1 枚で固定する（中途半端な境界いじりを止める）。
Related:
  - CURRENT_TASK.md
  - lang/src/runtime/kernel/README.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/de-rust-zero-buildability-contract-ssot.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/build-lane-separation-ssot.md
  - docs/development/current/main/design/rep-mir-string-lowering-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
---

# Phase 29cm: Kernel Authority Migration (kernel-mainline)

## Goal

- kernel の「意味/contract/policy/control structure」の owner を `.hako` / docs 側へ寄せる。
- Rust/C は substrate（allocation / handle registry / GC / ABI / raw leaf）に固定し、meaning owner に戻さない。
- `0rust` は Rust meaning owner zero を意味するが、Rust ベースの build/bootstrap route を消すことではない。
- “境界だけいじって進まない” 状態を防ぐため、fixed order と promotion trigger を SSOT 化する。

## Non-Goals

- Rust substrate の wholesale delete はしない（authority migration と混ぜない）。
- Rust ベースの buildability を壊す slice は、この phase の mainline に入れない。
- perf/asm optimization を主線にしない（kernel authority migration 完了後の follow-up）。
- silent fallback を許可しない（`NYASH_VM_USE_FALLBACK=0` を前提）。
- `map` を早期に kernel lane に入れない（ring1 keep）。

## Fixed Order (Migration)

1. `string`
   - landed: `lang.runtime.kernel.string.search` (`find_index/contains/starts_with/ends_with/split_once_index`)
   - rule: further widening is paused until a new exact blocker appears

2. `array`
   - first owner: `lang/src/runtime/collections/` ring1
   - target: wrapper/router thin floor を先に達成し、promotion は trigger-based

3. `numeric`
   - landed: `MatI64.mul_naive` loop/body owner split (`lang/src/runtime/kernel/numeric/`)
   - rule: array が落ち着いた後にだけ次の narrow op を切る（必要が出た時だけ）

4. `map`
   - keep: `lang/src/runtime/collections/` ring1
   - rule: kernel lane へは入れない（最後寄り）

## Latest Inventory (2026-03-19)

- `array`
  - `lang/src/runtime/collections/array_core_box.hako` / `array_state_core_box.hako` / `crates/nyash_kernel/src/plugin/array*.rs` are already split at the natural seams.
  - no new dedicated `lang/src/runtime/kernel/array/` slice is justified yet.
  - keep defer until the promotion trigger is genuinely hit.
- `numeric`
  - `lang/src/runtime/kernel/numeric/matrix_i64.hako` plus `lang/src/runtime/numeric/{mat_i64_box.hako,intarray_core_box.hako}` are already thin enough.
  - no credible next narrow op was found in the inventory.
  - stop here until a new exact blocker appears.
- `map`
  - still ring1 keep / defer.
  - not part of the current kernel migration slice.

## Buildability Lock

- any migration slice:
  - Rust からの build/bootstrap route は常に再実行可能であること
  - owner cutover と buildability cutover を同じ slice で壊さないこと
  - `.hako` へ寄せる順番と Rust buildability の保持順番を混ぜないこと

## Promotion Trigger (array)

`lang/src/runtime/kernel/array/` への promotion は calendar-based ではなく trigger-based。

Promote してよい条件（どれか 1 つでも真になったら検討開始）:
- ring1 wrapper が wrapper-only の薄さを保てない:
  - owner-local policy / normalization / birth handling が ring1 wrapper に混入し始めた
- dedicated acceptance case + smoke が wrapper-only lane では表せない
- same policy が 2 箇所に増え、SSOT 化が ring1 では維持できない

Promote しない条件（defer 維持）:
- 変更が単なる “関数の移動/分割” で、policy difference が無い
- substrate（allocation/handles/GC）へ踏み込む必要がある

## Acceptance (minimum)

- gate:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh`
  - `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
- quick:
  - `bash tools/smokes/v2/profiles/quick/core/array/array_length_vm.sh`
- kernel pilots:
  - string: `bash tools/smokes/v2/profiles/integration/apps/phase29ck_string_kernel_search_min.sh`
  - numeric: `bash tools/smokes/v2/profiles/integration/apps/phase29ck_numeric_mat_i64_mul_naive_min.sh`

## Done Shape (phase closeout)

- `CURRENT_TASK.md` の fixed order が破れず、次の 1 手が常に 1 commit 単位で切れる
- `string`/`numeric` の landed pilots が smoke で固定されている
- `array` は ring1 thin floor を達成していて、promotion trigger の判定が「未発火」または「発火→promotion」どちらかに確定している
- `map` は ring1 keep のまま（kernel lane に混ぜない）
