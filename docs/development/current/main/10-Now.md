---
Status: SSOT
Date: 2026-04-18
Scope: current lane / blocker / next pointer only.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md
  - docs/development/current/main/phases/phase-137x/README.md
---

# Self Current Task — Now (main)

## Current

- current optimization lane:
  - `phase-137x publication/source-capture reopen after compiler-known-length keeper`
- background compiler lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- blocker:
  - `none`

## Snapshot

- keeper front stays closed:
  - `kilo_micro_substring_concat = C 2 ms / Ny AOT 3 ms`
  - `kilo_micro_substring_only = C 3 ms / Ny AOT 3 ms`
- current broad gap:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 131 ms`
  - `kilo_kernel_small = C 80 ms / Ny AOT 741 ms`
- current bridge front:
  - `kilo_meso_substring_concat_array_set_loopcarry`
  - shape: `substring + concat + array.set + loopcarry`
  - role: adopted middle between exact micro and whole kilo
  - rule: use it to validate store/publication cuts without the whole-front `indexOf("line")` row-scan noise
- `indexOf` separation:
  - keep as side diagnosis; reread only when the main card reopens it
- completed audit lock (confirmed evidence):
  - exact audit: top samples are `substring_concat_hhii_export_impl 22.38%`, `string_concat_hh_export_impl 21.70%`, array string-store closure `17.34%`, `from_i8_string_const 13.07%`, `LocalKey::with 6.07%`, `memmove 3.51%`, `_int_malloc 1.75%`; wrapper names are not the live owner, current evidence points to inner publication / object-world entry
  - whole audit: top user symbols are `nyash.string.concat_hs 11.19%`, `execute_store_array_str_contract` closure `7.01%`, `insert_const_mid_fallback` closure `3.89%`, `array_get_index_encoded_i64` closure `3.62%`, `from_i8_string_const 3.52%`, libc `memmove 14.92%`, `_int_malloc 4.65%`; `concat_hs` hot instructions are TLS/helper-entry, not copy body
  - observability audit: the generic-fallback split is now locked by site-specific noinline symbols in `crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs`; tests passed with and without `perf-observe`
  - choice rule: perf/asm is now sufficient to choose the next keeper without another broad observability round
- current owner reading:
  - current main owner family is `array/string-store`
  - duplicated `text + "xy"` producer is already removed in trusted direct MIR
  - compiler-side known string-length propagation is now landed for const / substring-window / same-length string `phi`
  - active AOT entry IR on this front no longer emits `nyash.string.len_h` in `ny_main`
  - current exact owner is still publication/source-capture
  - current exact/meso/whole split is now explicit:
    - `kilo_micro_array_string_store` is dominated by the shared generic publish/objectize corridor behind `string_concat_hh` + `string_substring_concat_hhii`
    - `kilo_meso_substring_concat_array_set_loopcarry` is the adopted bridge front for the same store/publication corridor without whole-front `indexOf("line")` noise
    - `kilo_kernel_small` is dominated by `const_suffix` fallback plus `freeze_text_plan(Pieces3)` publication
  - current code pick stays whole-first:
    - next narrow seam is the borrowed-slot retarget/publication tail under `execute_store_array_str_contract`
    - first implementation target:
      - `try_retarget_borrowed_string_slot_take_verified_text_source`
      - `keep_borrowed_string_slot_source_keep`
    - middle remains the contradiction guard; if this seam does not lift meso, reopen `substring_hii -> borrowed_substring_plan_from_handle`
  - exact hot instructions carry host-handle atomics, TLS publish stores, alloc shim calls, and array-store handle/publication branches
  - exact loop still pays extra per-iter helper calls vs C: `from_i8_string_const` x2, `concat_hh` x1, `set_his` x1, `substring_concat_hhii` x1
  - whole hot closures still pay registry fetch, `lock cmpxchg`, vtable probes, and handle/cache publication before store completion
  - `Stage A` narrow owner slice is now landed on the VM/reference lane:
    - `.hako` `ArrayCoreBox` routes proven string-handle `set(...)` through `nyash.array.set_his`
    - same protocol, same cold Rust tail
  - `Stage A` exact reread is now closed on the active AOT front:
    - `store.array.str total=800000`
    - `plan.action_retarget_alias=800000`
    - `plan.action_store_from_source=0`
    - `carrier_kind.source_keep=0`
    - `publish_reason.generic_fallback=1600000`
  - trusted direct MIR still carries generic `RuntimeDataBox.set(...)` / `substring(...)`
  - active AOT lowering is separately locked:
    - direct MIR stays generic
    - entry LLVM IR still calls `nyash.array.set_his`
    - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh`
  - active AOT exact is therefore not the `.hako` owner pilot itself; the live owner stays publication/source-capture
  - slot-store boundary probes are now a rejected card:
    - v1 exact/whole: `252 ms / 765 ms`
    - v2 exact/whole: `211 ms / 1807 ms`
    - they cut at the wrong seam and broke the existing `set_his` fast path
  - a producer-side unpublished-outcome active probe is also rejected:
    - exact/whole: `236 ms / 2173 ms`
    - it regressed both fronts while changing the active boundary shape
  - helper-side keepers from these rejected cards are:
    - `b35382cf9`
    - runtime-side alias-retarget repair for kernel-slot store into existing string slots
  - latest `perf-observe` reread no longer ranks `string_len_export_slow_path`; the live top stays on `issue_fresh_handle` / `freeze_owned_bytes` / `capture_store_array_str_source` / `StringBox::perf_observe_from_owned`
  - latest observability split landed and is now pinned:
    - `lookup.registry_slot_read`
    - `lookup.caller_latest_fresh_tag`
    - `site.string_concat_hh.{materialize_owned_total/materialize_owned_bytes/objectize_box_total/publish_handle_total}`
    - `site.string_substring_concat_hhii.{materialize_owned_total/materialize_owned_bytes/objectize_box_total/publish_handle_total}`
  - latest raw exact observe reread on `kilo_micro_array_string_store` shows:
    - `lookup.registry_slot_read=800000`
    - `lookup.caller_latest_fresh_tag=800000`
    - `site.string_concat_hh.materialize_owned_total=800000`
    - `site.string_substring_concat_hhii.materialize_owned_total=800000`
  - latest raw whole observe reread on `kilo_kernel_small_hk` shows:
    - `const_suffix freeze_fallback=479728`
    - `freeze_text_plan_pieces3=60000`
    - `publish_reason.generic_fallback=539728`
    - `site.string_concat_hh.*=0`
    - `site.string_substring_concat_hhii.*=0`
  - latest runtime-fix-only reread stays on the same owner family:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 127 ms`
    - `kilo_kernel_small_hk = C 81 ms / Ny AOT 755 ms`
  - current whole-owner reread is now pinned:
    - first owner = `const_suffix` / `nyash.string.concat_hs`
    - secondary guard = `freeze_text_plan(Pieces3)` / `insert_hsi`
  - latest asm read:
    - `ny_main` loop shape is already close to C
    - the remaining gap is helper-entry branch / TLS / generic publication tail inside helper bodies
  - next first slice is no longer `len_h` removal; it is publication/source-capture reopen with the compiler-known-length lane fixed
  - evidence points to publication/object-world entry as the live owner on both fronts; this does not yet prove any representation / ABI change
  - latest design consult is accepted in narrowed form:
    - no syntax expansion
    - no public raw string / mutable bytes
    - keep the next widening inside runtime-private `const_suffix` / `TextPlan(Pieces3)` publication only
    - if publication timing wins, reuse existing runtime-private `TextPlan` / `OwnedBytes` seams first

## Next

1. keep `Stage A` parked as VM/reference-only
2. keep the compiler-known-length lane fixed and guarded on this front
3. keep exact micro, adopted middle, and whole kilo separate when choosing the next keeper
4. preserve the existing `set_his` fast path; do not reopen slot-store boundary probes
5. keep `const_suffix` as the first whole-front keeper owner and `Pieces3` as a guard lane
6. use `kilo_meso_substring_concat_array_set_loopcarry` as the first confirmation front between exact and whole when judging the next keeper
7. first narrow cut candidate stays in the store/publication corridor:
   - `execute_store_array_str_contract`
   - `array_get_index_encoded_i64`
   - `insert_const_mid_fallback`
8. treat allocator / GC as secondary diagnosis until that corridor loses on exact + meso + whole
9. implement whole-first at the borrowed-slot retarget/publication tail before reopening upstream substring planning
10. keep `Stage B` narrow and data-driven through runtime-private publication counters

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
3. `docs/development/current/main/phases/phase-137x/README.md`
4. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
5. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
6. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
7. `docs/development/current/main/design/string-birth-sink-ssot.md`
8. `docs/development/current/main/15-Workstream-Map.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```
