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
- `Stage A` exact reread 後の current truth を固定する
- `substring` keeper / `indexOf` diagnostic / `whole-kilo` guard を混同しない

## Measured Facts

### Whole-Kilo

- `kilo_kernel_small_hk`
  - `C: 85 ms`
  - `Ny AOT: 786 ms`
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
    - `Ny AOT: 153 ms`
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
- `Stage A` exact reread on the active AOT front is now closed:
  - plain release:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 153 ms`
    - `kilo_kernel_small_hk = C 85 ms / Ny AOT 786 ms`
  - `perf-observe` counter facts:
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
- `perf-observe` on `kilo_micro_array_string_store` still ranks publication/capture first:
  - `freeze_owned_bytes: 15.76%`
  - `issue_fresh_handle: 14.54%`
  - `StringBox::perf_observe_from_owned: 11.70%`
  - `capture_store_array_str_source: 8.53%`
  - `string_concat_hh_export_impl: 7.23%`
  - `string_len_export_slow_path: 6.74%`
  - `LocalKey::with: 5.72%`
  - `__memmove_avx512_unaligned_erms: 4.63%`
- current reading stays:
  - dominant cost is still upstream birth/publication plus source capture
  - slot mutation itself is not the first owner once source is already published
  - trusted direct MIR still carries generic `RuntimeDataBox.set(...)` / `substring(...)` calls
  - the landed `.hako` owner-side pilot is therefore still VM/reference-lane only today
  - active AOT already reaches the current concrete `store.array.str` lowering without that pilot
  - the exact-front owner is still publication/source-capture around the string births before/after `nyash.array.set_his`

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
- no generic slot API widening
- no public ABI changes
