---
Status: Active
Date: 2026-04-18
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md
  - docs/development/current/main/phases/phase-137x/README.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
```

## Current

- lane:
  - `phase-137x publication/source-capture reopen after compiler-known-length keeper`
- blocker:
  - `none`
- worktree:
  - clean is expected; do not resurrect `stash@{0}` unless you are explicitly reopening the rejected slot-store boundary probe
- current snapshot:
  - `kilo_micro_substring_concat = C 2 ms / Ny AOT 3 ms`
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 132 ms`
  - `kilo_meso_substring_concat_array_set_loopcarry` is the adopted middle bridge:
    - `substring + concat + array.set + loopcarry`
    - use it to confirm store/publication cuts without the whole-front `indexOf("line")` row-scan noise
  - `kilo_kernel_small_hk = C 80 ms / Ny AOT 731 ms`
- immediate next:
  - `judge the next keeper on exact -> meso -> whole, with the adopted middle bridge between array-store exact and whole kilo`
- immediate follow-on:
  - `keep the first narrow cut inside the store/publication corridor before allocator / GC follow-ups`
- immediate code seam:
  - `execute_store_array_str_contract` whole-first
  - specifically `try_retarget_borrowed_string_slot_take_verified_text_source -> keep_borrowed_string_slot_source_keep`
  - middle (`substring_hii -> borrowed_substring_plan_from_handle`) stays the contradiction guard, not the first code cut
- latest non-keeper:
  - `producer-side unpublished-outcome active probe regressed to 236 ms exact / 2173 ms whole and is reverted`
- latest observability split:
  - `lookup_array_store_str_source_obj` is now visible as:
    - `lookup.registry_slot_read`
    - `lookup.caller_latest_fresh_tag`
  - publish site counters now exist for the exact micro front:
    - `site.string_concat_hh.*`
    - `site.string_substring_concat_hhii.*`
  - latest raw whole observe reread proves those exact-micro sites are not the whole-kilo owner:
    - `const_suffix freeze_fallback=479728`
    - `freeze_text_plan_pieces3=60000`
    - `site.string_concat_hh.*=0`
    - `site.string_substring_concat_hhii.*=0`

## Current Handoff

- current broad owner family is `array/string-store`
- duplicated producer is already fixed in trusted direct MIR; runtime publication/source-capture stayed hot
- compiler-side known string-length propagation is now landed for const / substring-window / same-length string `phi`
- active AOT entry IR on this front no longer emits `nyash.string.len_h` in `ny_main`
- `.hako` owner-side Stage A pilot is landed on the VM/reference lane; `ArrayCoreBox` now routes proven string-handle `set(...)` through `nyash.array.set_his`
- active AOT already reaches `nyash.array.set_his` without that VM/reference pilot
- slot-store boundary delayed-publication probes are rejected:
  - `v1 = 252 ms / 765 ms`
  - `v2 = 211 ms / 1807 ms`
  - wrong cut; do not reopen this before a new design decision
- helper-only keeper from that rejected card is committed as `b35382cf9`
- latest `perf-observe` reread no longer ranks `string_len_export_slow_path`; the live top stays publication/source-capture
- exact micro vs adopted middle vs whole kilo must now be read separately:
  - exact micro owner = shared generic publish/objectize behind `string_concat_hh` + `string_substring_concat_hhii`
  - adopted middle = `kilo_meso_substring_concat_array_set_loopcarry`, used to confirm the same corridor without `indexOf("line")` row-scan noise
  - whole kilo owner = `const_suffix` fallback + `freeze_text_plan(Pieces3)` publication
- next narrow cut candidate is the store/publication corridor around:
  - `execute_store_array_str_contract`
  - `array_get_index_encoded_i64`
  - `insert_const_mid_fallback`
- allocator / GC (`memmove` / `gc_alloc` / `_int_malloc`) stays secondary diagnosis until that corridor is disproved
- `indexOf` stays a side diagnosis, not the active keeper card
- keep public ABI / legality ownership unchanged
- next first slice is no longer `len_h` removal; it is publication/source-capture reopen with the compiler-known-length lane fixed
- current plain-release reread after reverting the failed active probe:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 132 ms`
  - `kilo_kernel_small_hk = C 80 ms / Ny AOT 731 ms`
- latest design consult is accepted in narrowed form:
  - no syntax expansion
  - no public raw string / mutable bytes
  - the next widening stays inside runtime-private `const_suffix` / `Pieces3` publication, not helper-site specialization
  - reuse existing `TextPlan` / `OwnedBytes` seams before inventing a new carrier
- compare `.hako` only under:
  - `Stage A: same protocol`
  - `Stage B: same public ABI / different seam`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
4. `docs/development/current/main/phases/phase-137x/README.md`
5. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
6. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
7. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
8. `docs/development/current/main/design/string-birth-sink-ssot.md`
9. `docs/development/current/main/15-Workstream-Map.md`
10. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md` (`phase-29bq` に戻るときだけ)

## Current Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```
