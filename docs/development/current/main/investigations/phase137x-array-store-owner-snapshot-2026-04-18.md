---
Status: Active
Date: 2026-04-18
Scope: `phase-137x` current owner snapshot after the trusted direct-emit keeper; freeze the current measurements so `CURRENT_TASK.md` can stay thin.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-137x/README.md
---

# Phase 137x Array-Store Owner Snapshot

## Purpose

- current broad owner family を 1 枚で読む
- `Stage A` exact reread 後と rejected slot-store boundary probe 後の current truth を固定する
- `substring` keeper / `indexOf` diagnostic / `whole-kilo` guard を混同しない

## Measured Facts

### Whole-Kilo

- `kilo_kernel_small_hk`
  - `C: 80 ms`
  - `Ny AOT: 724 ms`
- current top report:
  - `__memmove_avx512_unaligned_erms: 21.61%`
  - `nyash.string.concat_hs: 10.71%`
  - `execute_store_array_str_contract closure: 5.59%`
  - `_int_malloc: 5.05%`
  - `array_get_index_encoded_i64 closure: 3.10%`
  - `insert_const_mid_fallback closure: 2.38%`
  - `array_string_indexof_by_index closure: 1.09%`

### Exact Fronts

- closed keeper:
  - `kilo_micro_substring_concat`
    - `C: 2 ms`
    - `Ny AOT: 3 ms`
  - `kilo_micro_substring_only`
    - `C: 3 ms`
    - `Ny AOT: 3 ms`
- current broad owner candidate:
  - `kilo_micro_array_string_store`
    - `C: 10 ms`
    - `Ny AOT: 126 ms`
- diagnostic leaf:
  - `kilo_leaf_array_string_indexof_const`
    - `C: 4 ms`
    - `Ny AOT: 224 ms`
- broader `indexOf` family sanity check:
  - `kilo_micro_indexof_line`
    - `C: 5 ms`
    - `Ny AOT: 11 ms`

## Current Owner Reading

- current broad owner family is `array/string-store`, not `substring`
- trusted direct MIR no longer shows duplicated producer birth on this front:
  - `text + "xy"` is shared across `set(...)` and trailing `substring(...)`
- current plain-release baseline after the compiler-known-length keeper:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 126 ms`
  - `kilo_kernel_small_hk = C 80 ms / Ny AOT 724 ms`
- current compiler-side keeper on this front is:
  - known string-length propagation across const / substring-window / same-length string `phi`
  - active AOT entry IR now folds `len_h` to integer constants and no longer emits `nyash.string.len_h` in `ny_main`
- latest locked exact counter snapshot on this front remains:
  - `store.array.str total=800000`
  - `cache_hit=800000`
  - `plan.action_retarget_alias=800000`
  - `plan.action_store_from_source=0`
  - `plan.action_need_stable_object=0`
  - `carrier_kind.source_keep=0`
  - `carrier_kind.owned_bytes=1600000`
  - `carrier_kind.stable_box=1600000`
  - `carrier_kind.handle=1600000`
  - `publish_reason.generic_fallback=1600000`
- active AOT lowering fact is now pinned separately:
  - direct MIR still contains generic `RuntimeDataBox.set(...)`
  - the built AOT object/entry IR still calls `nyash.array.set_his`
  - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh`
- latest `perf-observe` reread on `kilo_micro_array_string_store` still ranks publication/capture first:
  - `issue_fresh_handle: 15.39%`
  - `freeze_owned_bytes: 15.34%`
  - `capture_store_array_str_source: 13.51%`
  - `StringBox::perf_observe_from_owned: 11.10%`
  - `string_concat_hh_export_impl: 10.43%`
  - `LocalKey::with: 6.90%`
  - `execute_store_array_str_slot_boundary: 5.96%`
  - `string_substring_concat_hhii_export_impl: 5.61%`
  - `host_handles::with_text_read_session closure: 5.23%`
  - `execute_store_array_str_contract: 4.47%`
- current reading stays:
  - dominant cost is still upstream birth/publication plus source capture
  - the compiler-known-length keeper removed `string_len_export_slow_path` from the active top report, but did not change the live owner family
  - slot mutation itself is not the first owner once source is already published
  - trusted direct MIR still carries generic `RuntimeDataBox.set(...)` / `substring(...)` calls
  - the landed `.hako` owner-side pilot is therefore still VM/reference-lane only today
  - active AOT already reaches the current concrete `store.array.str` lowering without that pilot
  - the exact-front owner is still publication/source-capture around the string births before/after `nyash.array.set_his`

## Rejected Slot-Store Boundary Probe

- active slot route v1:
  - `kilo_micro_array_string_store = 252 ms`
  - `kilo_kernel_small_hk = 765 ms`
- active slot route v2:
  - `kilo_micro_array_string_store = 211 ms`
  - `kilo_kernel_small_hk = 1807 ms`
- keeper from that card:
  - helper-only infra is landed as `b35382cf9 feat: add kernel text slot store helpers`
- rejected reading:
  - the bad cut was the array-store boundary itself
  - the probe bypassed the existing `set_his` fast path / alias-retarget behavior
  - publication sink remains the right family, but not at the array-store boundary

## `indexOf` Separation

- `kilo_leaf_array_string_indexof_const` is a dedicated seed-route diagnostic lane
- both current seed bundles still lower directly to `nyash.string.indexOf_ss`
- runtime leaf helper A/B was tried and reverted:
  - attempted swap: `h.find(n)` -> `find_substr_byte_index(...)`
  - exact rereads after rebuild stayed in the same bad band:
    - `216 ms`
    - `220 ms`
    - `224 ms`
- therefore:
  - do not treat `indexOf_ss` helper swapping as the next broad optimization card
  - keep `indexOf` as side diagnosis while the main cut stays on `array-store placement`

## Bench / Code Pointers

- bench shape:
  - `benchmarks/bench_kilo_micro_array_string_store.hako`
  - `benchmarks/bench_kilo_leaf_array_string_indexof_const.hako`
- runtime owner area:
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`
  - `crates/nyash_kernel/src/plugin/value_codec/string_store.rs`
- design anchor:
  - `docs/development/current/main/design/string-birth-sink-ssot.md`

## Current Rule

- next proof is not kilo-name keyed
- next cut is no longer owner-route diagnosis
- park Stage A as VM/reference-only and keep exact-front work on publication/source-capture
- keep the compiler-known-length lane fixed; the next first slice is publication/source-capture reopen, not another `len_h` card
- preserve the existing `set_his` fast path while testing any unpublished outcome cut
- no generic slot API widening
- no public ABI changes
