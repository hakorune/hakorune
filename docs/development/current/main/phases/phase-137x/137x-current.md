# 137x Current Entry

This is the active entry for phase-137x. The long `README.md` keeps historical
ledger details; current implementation work should start here.

## Current Lane

- lane: `137x-H owner-first optimization return`
- front: `kilo_kernel_small`
- current blocker token: `137x-H40 MIR-owned byte-boundary proof for text-cell edits`
- current benchmark state:
  - `C 85 ms / Ny AOT 5 ms`
  - `ny_aot_instr=35428450`
  - `ny_aot_cycles=6679916`
- active owner:
  - H27 removed the outer edit path's `nyash.array.string_len_hi` call by
    lowering the MIR-owned len-half insert-mid edit contract to one
    runtime-private helper
  - H28.1 removed the fixed-literal `Pattern::is_contained_in` search owner
    inside the H26 observer-store region executor
  - H28.2 removed the short-literal prefix `bcmp` / `__memcmp_evex_movbe`
    compare owner introduced by H28.1's `starts_with` check
  - H28.3 removed the short-suffix append `memcpy` call from the
    observer-store runtime executor
  - H28.5 refreshed ownership: residual `memmove` is primarily the outer
    len-half edit closure, not append capacity
  - H29 rejected a runtime-private `String::insert_str` bypass; local
    byte-copy surgery did not become a keeper
  - H39.4 consumes the combined edit-observer MIR proof as one runtime-private
    executor call; the old per-iteration len-half helper is no longer emitted
  - H39.5.3 specializes 4-byte literal observer mechanics inside the combined
    executor without changing MIR, `.inc`, or public ABI
  - H39.5.4 refreshes the residual owner after the 4-byte literal observer
    cleanup; the next narrow seam is byte-boundary legality, not another
    runtime-only leaf
  - latest preserved-AOT top after H39.5.4:
    - combined region executor closure: `75.26%`
    - `__memmove_avx512_unaligned_erms`: `10.03%`
    - `_int_malloc`: `2.05%`
    - `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle`: `0.30%`
- non-owners:
  - fallback/promotion: H23a observed `update_text_resident_hit=179999`
  - helper-local resident/fallback compaction: H23b regressed to `ny_aot_instr=45910743`
  - per-iteration exported fused helper call: removed by H25c.2c-4
  - write-lock acquire/release in emitted AOT loop: moved inside one Rust call
  - inner `array.get(j).indexOf("line")` + suffix store: removed from emitted
    AOT loop by H26
  - outer len-half edit `string_len_hi`: removed by H27 from the emitted edit
    path; residual `string_len_hi` belongs to the final 64-row sum loop only
  - per-iteration outer edit helper:
    `nyash.array.string_insert_mid_lenhalf_store_hisi` is removed from
    current `ny_main` by H39.4

## Active Contract

- MIR owns:
  - residence-session eligibility
  - loop/session lifetime
  - edit split policy such as `source_len / 2`
  - byte-boundary / encoding-preservation proof for text-cell edit fast leaves
  - alias/publication boundary
  - covered route facts
- `.inc` owns:
  - metadata consumption
  - later begin/update/end emit shape only
  - no raw MIR shape rediscovery
- Rust runtime owns:
  - write guard mechanics
  - text storage/slot access
  - mutation execution
  - no legality/provenance inference

## H28 Active

Goal: split the remaining observer-store region executor owner without making
runtime or `.inc` semantic owners.

- target owner:
  - suffix mutation/copy inside the H26 observer-store runtime-private write
    frame
  - allocator/copy side effects that remain after the H28.1 literal-search
    keeper and H28.2 short-literal prefix compare cleanup
- allowed next work:
  - add MIR metadata only if the executor needs a generic fact such as
    `needle_literal`, `observer_kind`, or `mutation_kind` to choose a runtime
    executor variant
  - keep `.inc` as metadata-to-call emit only
  - keep runtime as execution mechanics only: search, copy, mutation, and guard
    residence inside one call
- reject seam:
  - no source-prefix assumption such as every row contains `"line"`
  - no search-result cache
  - no runtime-owned legality/provenance/publication
  - no benchmark-named whole-loop helper
  - no C-side raw shape rediscovery
- first step:
  - inspect the H26 observer-store runtime helper and decide whether the next
    keeper is a fixed-literal search executor, a copy/mutation split, or a
    no-code closeout requiring more MIR proof

### H28.4 text append capacity owner probe

- owner split:
  - H25 guard mechanics is closed; do not reopen it under this card
  - H28.4 is a separate owner-first slice for resident `String` capacity miss
    and old-content copy under the H26 observer-store suffix append executor
- worker/local evidence:
  - the short suffix byte-copy path no longer calls `memcpy`
  - residual `__memmove` maps best to growth/reallocation copy around the
    append leaf, with adjacent write-frame mechanics still visible
  - `Boxed -> Text` promotion is one-shot and not the steady-state owner
- decision:
  - first confirm the capacity-growth hypothesis from the append leaf
  - any keeper implementation must be Rust-only and runtime-private
  - MIR metadata, `.inc` lowering, and public ABI stay unchanged
- allowed:
  - a narrow text append headroom policy based only on storage facts such as
    suffix length, current length, and current capacity
  - unit tests that prove the append leaf still matches `String::push_str`
- forbidden:
  - source-prefix or benchmark-name branches
  - search-result cache
  - runtime-owned legality/provenance/publication
  - C-side shape planning or new MIR metadata only for capacity tuning
- keeper gate:
  - whole `kilo_kernel_small` improves in instruction/cycle count and the
    `__memmove` share drops
  - exact `kilo_micro_array_string_store` and middle
    `kilo_meso_substring_concat_array_set_loopcarry` stay no-regression
  - reject if the reduction only shifts cost into allocator / `_int_malloc`
    owners

Result:

- trial:
  - added a Rust-only short append headroom policy in `append_short_text_suffix`
  - no MIR metadata, `.inc` lowering, or public ABI changed
- verification:
  - `cargo test -q append_text_suffix --lib`
  - `cargo test -q text_contains_literal --lib`
  - `cargo fmt --check`
  - `git diff --check`
  - `bash tools/perf/build_perf_release.sh`
  - whole `kilo_kernel_small` first run: `C 82 ms / Ny AOT 7 ms`,
    `ny_aot_instr=61363741`, `ny_aot_cycles=17616053`
  - whole rerun: `C 82 ms / Ny AOT 8 ms`,
    `ny_aot_instr=61364376`, `ny_aot_cycles=17951505`
  - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9265802`, `ny_aot_cycles=2367573`
  - middle `kilo_meso_substring_concat_array_set_loopcarry`:
    `C 3 ms / Ny AOT 4 ms`, `ny_aot_instr=16570977`,
    `ny_aot_cycles=3472466`
  - asm after trial: `__memmove_avx512_unaligned_erms` dropped to `34.76%`,
    but `with_array_text_write_txn` rose to `31.09%` and the
    observer-store closure to `27.10%`
- verdict:
  - rejected; lower `memmove` share did not translate into instruction/cycle
    or wall-time keeper
  - code was reverted to the H28.3 append leaf
  - H28.5 must refresh residual `memmove` ownership with callsite/callgraph
    evidence before another code slice

### H28.5 residual memmove owner refresh

- goal:
  - distinguish append-capacity growth from outer len-half edit copy and
    write-frame mechanics before further runtime surgery
- allowed:
  - perf/asm/callgraph evidence collection
  - docs-only owner decision
- forbidden:
  - more helper-local copy/capacity changes without a target transition
  - MIR or `.inc` changes unless the refreshed owner proves a missing contract

Result:

- commands:
  - `bash tools/perf/build_perf_release.sh`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`
  - manual `perf record --call-graph dwarf` on the generated
    `kilo_kernel_small` AOT executable
- evidence:
  - whole `kilo_kernel_small`: `C 84 ms / Ny AOT 7 ms`,
    `ny_aot_instr=60616017`, `ny_aot_cycles=17782048`
  - asm top after returning to H28.3 code:
    - `__memmove_avx512_unaligned_erms`: `37.20%`
    - observer-store region closure: `28.98%`
    - `with_array_text_write_txn` closure: `26.22%`
    - `nyash.array.string_insert_mid_lenhalf_store_hisi`: `3.26%`
  - callgraph attributes the dominant `__memmove` child to
    `array_string_insert_const_mid_lenhalf_by_index_store_same_slot_str`
    closure (`27.91%`)
  - append / realloc growth under `alloc::raw_vec::finish_grow` accounts for
    only about `0.93%`
- verdict:
  - H28 observer-store search/copy split is closed
  - append capacity is not the next owner; do not reopen H28.4 headroom
    without new evidence
  - next active card is H29: len-half edit copy owner decision under the
    MIR-owned H27 edit contract

## H29 Result

Goal: decide whether the outer len-half edit copy owner can be reduced cleanly
without making runtime or `.inc` a semantic owner.

- target owner:
  - overlapping copy inside
    `array_string_insert_const_mid_lenhalf_by_index_store_same_slot_str`
  - this is H27 edit execution mechanics, not observer-store append capacity
- guard:
  - no source-shape shortcut beyond the existing MIR-owned H27 edit contract
  - no benchmark-named whole-loop helper
  - no runtime-owned legality/provenance/publication
  - no `.inc` raw shape rediscovery
- first step:
  - inspect whether the current in-place copy is already the minimal physical
    executor for the H27 contract
  - if yes, close as data-structure/gap-buffer successor work rather than
    local byte-copy surgery

Result:

- trial:
  - replaced the len-half helper's `String::insert_str` path with an explicit
    runtime-private reserve + suffix shift + middle copy leaf
  - no MIR metadata, `.inc` lowering, or public ABI changed
- verification:
  - `cargo test -q -p nyash_kernel insert_mid_lenhalf_store_by_index_returns_result_len`
  - `cargo test -q -p nyash_kernel insert_mid_store_by_index`
  - `cargo test -q detects_lenhalf_insert_mid_same_slot_edit_route --lib`
  - `cargo fmt --check`
  - `bash tools/perf/build_perf_release.sh`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`
- evidence:
  - whole `kilo_kernel_small`: `C 83 ms / Ny AOT 7 ms`,
    `ny_aot_instr=60494965`, `ny_aot_cycles=17790198`
  - asm top after trial:
    - `__memmove_avx512_unaligned_erms`: `40.84%`
    - `with_array_text_write_txn` closure: `30.00%`
    - observer-store region closure: `20.99%`
    - `nyash.array.string_insert_mid_lenhalf_store_hisi`: `3.21%`
- verdict:
  - rejected and reverted
  - the active H27 edit is a contiguous `String` mid-insert; the suffix move is
    structural for that representation
  - further keeper work must start from a representation decision, not another
    local byte-copy leaf

## H30 Active

Goal: decide whether a narrow array text edit residence representation can
reduce the H27 len-half mid-insert suffix-copy owner cleanly.

- target owner:
  - contiguous `String` mid-insert suffix movement in the outer edit path
  - this is storage representation mechanics under the MIR-owned H27 edit
    contract
- allowed:
  - docs-first inventory of representation options such as gap-buffer,
    segmented text cell, or piece-table-style residence
  - a runtime-private prototype only if publication/materialization boundaries
    stay explicit and current public Array/String ABI remains unchanged
  - MIR metadata changes only if a generic contract fact is missing; helper
    names must not become truth
- forbidden:
  - benchmark-named whole-loop helpers
  - source-prefix assumptions
  - semantic/search-result cache
  - runtime-owned legality, provenance, or publication decisions
  - `.inc` raw MIR shape rediscovery
- first step:
  - inventory current `ArrayStorage::Text`, observer-store, append, length, and
    publication consumers to see whether a non-contiguous residence can stay
    boxed inside runtime mechanics
  - if the representation would leak into MIR/public ABI, reject H30 and stop
    local kilo surgery

### H30.1 Inventory Result

- current storage shape:
  - `ArrayStorage::Text(Vec<String>)`
  - `storage.rs` exposes text values as plain `String` for promotion,
    boxing, formatting, capacity, and clone helpers
- hot runtime text APIs:
  - `slot_with_text_raw(idx, |&str| ...)`
  - `slot_text_len_raw(idx)`
  - `slot_update_text_raw(idx, |&mut String| ...)`
  - `slot_update_text_resident_first_raw(idx, |&mut String| ...)`
  - `slot_text_region_update_sum_raw(..., |&mut String| ...)`
  - `slot_text_indexof_suffix_store_region_raw(...)`
- public / visible array consumers:
  - `get_index_i64` materializes `Text` slots to `StringBox`
  - `Clone`, `fmt_box`, `to_string_box`, `equals`, and `Debug` match
    `ArrayStorage::Text` directly
  - `store`, `remove`, `capacity`, and sequence ops also pattern-match the
    text variant
- decision:
  - do not replace `Text(Vec<String>)` directly with a gap/piece structure in
    the next code slice; that would leak representation details across array
    ops and make rollback large
  - next clean code step is BoxShape-only: introduce an internal
    `ArrayTextCell` boundary while keeping the first implementation
    flat-string-only
  - only after the flat `ArrayTextCell` wrapper is green should H30 open a
    piece/gap representation variant behind the same runtime-private boundary
- acceptance for the next code slice:
  - no MIR metadata changes
  - no `.inc` changes
  - no public ABI changes
  - no behavior change; tests should prove text lane store/read/mutate,
    visible `get`, equality/formatting, and observer-store routes still see the
    same string contents
  - perf is observational only for the wrapper slice; keeper judgment belongs
    to the later non-flat representation slice

### H30.1 Code Result

- implementation:
  - added a flat-only `ArrayTextCell` wrapper
  - changed `ArrayStorage::Text` from `Vec<String>` to
    `Vec<ArrayTextCell>`
  - kept public Array/String behavior unchanged by materializing flat text at
    visible boundaries (`get`, formatting, equality, boxing, sequence ops)
- guard held:
  - no MIR metadata changes
  - no `.inc` changes
  - no public ABI changes
  - no non-flat representation yet
- verification:
  - `cargo fmt --check`
  - `git diff --check`
  - `cargo check -q`
  - `cargo test -q array::tests --lib`
  - `cargo test -q text_contains_literal --lib`
  - `cargo test -q slot_store_text_births_text_lane --lib`
  - `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`
  - `tools/checks/dev_gate.sh quick`

### H30.2 Code Result

Goal: close the H27 edit operation boundary before any non-flat text residence
prototype.

- result:
  - added `ArrayTextCell::insert_const_mid_lenhalf` as the runtime-private
    edit operation boundary for the MIR-owned H27 len-half contract
  - added `ArrayBox::slot_insert_const_mid_lenhalf_raw` so the kernel helper
    no longer exposes `&mut String` as the dominant hot operation surface for
    text-resident slots
  - kept the implementation flat-only in this slice
- problem closed:
  - `ArrayTextCell` is now the storage boundary, but H27 len-half edit still
    reaches the hot slot through an exported helper closure that exposes
    `&mut String`
  - adding a gap/piece variant while that API remains dominant would leak the
    flat representation back into plugin/runtime helper code
- decision kept:
  - first add a runtime-private `ArrayTextCell` edit operation for the
    MIR-owned len-half insert-mid contract
  - make the H27 len-half helper call that operation through `ArrayBox`
  - keep the operation flat-only in this slice; the non-flat representation
    decision remains blocked until this operation boundary is green
- guard held:
  - MIR metadata changes
  - `.inc` lowering changes
  - public ABI changes
  - benchmark-named helpers
  - runtime legality/provenance/publication decisions
- verification:
  - `cargo fmt --check`
  - `git diff --check`
  - `tools/checks/current_state_pointer_guard.sh`
  - `cargo check -q`
  - `cargo test -q slot_insert_const_mid_lenhalf_raw --lib`
  - `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`
  - `cargo test -q array::tests --lib`
  - `tools/checks/dev_gate.sh quick`
- acceptance result:
  - existing H27 len-half helper behavior is unchanged
  - `ArrayTextCell` becomes the owner of the flat edit mechanics for this hot
    operation
  - focused array/text and kernel insert-mid tests pass

### H30.3 Closed Without Keeper

Goal: decide whether to open a non-flat `ArrayTextCell` edit residence
prototype behind the H30.2 operation boundary.

- design question:
  - can the H27 len-half insert-mid edit avoid repeated contiguous suffix
    movement without leaking representation details into MIR, `.inc`, public
    ABI, or plugin facade code?
- candidate options:
  - gap buffer:
    - useful for local edits near the same cursor
    - risky for this front because the edit point is recomputed as `len / 2`
      each iteration, so gap movement can remain structural
  - piece-cell / deferred edit residence:
    - keeps logical text as pieces and materializes only at explicit visible
      boundaries
    - better aligned with the current owner because the repeated edit moves
      descriptors instead of copying the full suffix bytes
- decision rule:
  - prefer a narrow piece-cell prototype if it can live entirely inside
    `ArrayTextCell`
  - reject H30.3 if the representation requires MIR metadata changes, `.inc`
    route changes, public ABI widening, runtime legality/provenance decisions,
    or semantic/search-result cache
- acceptance for any prototype:
  - `ArrayTextCell` owns the non-flat variant and materialization boundary
  - existing text-lane read, length, equality/formatting, visible `get`, and
    H26/H27 hot helpers keep behavior
  - the whole-front target must reduce the residual `memmove` owner; no
    improvement means revert/reject, not broader runtime surgery
- prototype tried:
  - narrow `ArrayTextCell::Pieces(Vec<String>)` residence
  - H27 len-half insert promoted large flat text into pieces
  - H26 observer-store used cell-owned `contains` / suffix append operations
  - no MIR, `.inc`, public ABI, legality, provenance, or publication changes
- result:
  - code reverted; no keeper landed
  - measurement hygiene note: the first perf read for this prototype was taken
    before rebuilding release artifacts, so those numbers are stale and must
    not be used as keeper evidence
- verdict:
  - piece vector residence has a credible risk of moving the cost from
    contiguous suffix bytes to descriptor movement and cache/materialization
    mechanics
  - gap-buffer has the same structural risk for this front because the edit
    point is recomputed as `len / 2`, not a stable cursor
  - do not continue local gap/piece representation surgery without a fresh
    valid-release owner proof

### H31 Result

Goal: refresh the whole-front owner after H30 rejection before opening the next
implementation card.

- reason:
  - H28.4 append headroom and H29 byte-copy surgery failed to turn the current
    `memmove` owner into a keeper
  - H30.3 exposed a measurement hygiene issue: runtime perf must rebuild
    release artifacts before judgment
- first step:
  - rerun whole `kilo_kernel_small` stat / asm and attribute the active
    `memmove` call path
  - decide whether the next card belongs to observer-store transaction
    mechanics, text edit residence, publication/materialization, or another
    substrate seam
- guard:
  - no code changes until the new owner family is fixed in this doc
  - no MIR / `.inc` changes unless the owner proof shows a missing generic
    contract fact
  - no runtime helper-name or benchmark-name truth
- evidence:
  - source inspection selected the extra kernel-private
    `with_array_text_write_txn` closure surface as the first narrow cleanup
  - measurement rule fixed for the lane:
    run `tools/perf/build_perf_release.sh` before runtime perf judgment
- verdict:
  - H30 local gap/piece text residence is closed without keeper
  - next card should use valid-release perf only

### H32 Code Result

Goal: decide the next narrow observer-store implementation card.

- candidate seams:
  - suffix mutation path inside `slot_text_indexof_suffix_store_region_raw`
  - transaction/facade overhead around `with_array_text_write_txn`
  - missing owner attribution inside the observer-store region executor
- first step:
  - inspect the hot observer-store source only around the sampled functions
  - avoid broad runtime redesign; choose one seam and one keeper gate
- decision:
  - first try transaction facade thinning
  - `ArrayTextWriteTxn` is a kernel-private wrapper around
    `with_array_box -> slot_update_text_*`; it does not own legality or
    provenance
  - flatten `with_array_text_slot_update*` to call `with_array_box` directly
    and remove the extra `with_array_text_write_txn` closure surface
- acceptance:
  - no public ABI, MIR, or `.inc` change
  - existing same-slot update behavior and resident/fallback observation
    semantics stay unchanged
  - whole perf must improve or at least move the `with_array_text_write_txn`
    symbol out of the top owner list; otherwise revert/reject
- guard:
  - `.hako`, MIR metadata, and `.inc` stay unchanged unless the seam proves a
    missing generic contract fact
  - runtime remains executor-only; no legality/provenance/publication decisions
  - no benchmark-named whole-loop helper
- implementation:
  - removed `ArrayTextWriteTxn` and `with_array_text_write_txn`
  - `with_array_text_slot_update` now calls
    `with_array_box(handle, |arr| arr.slot_update_text_raw(idx, f)).flatten()`
  - `with_array_text_slot_update_resident_first` now calls
    `slot_update_text_resident_first_raw` directly and preserves the
    Resident/Fallback outcome mapping
- valid-release verification:
  - `tools/perf/build_perf_release.sh`
  - whole `kilo_kernel_small`: `C 84 ms / Ny AOT 7 ms`
  - `ny_aot_instr=60315390`
  - `ny_aot_cycles=17714067`
  - top asm:
    - `__memmove_avx512_unaligned_erms`: `40.82%`
    - len-half closure:
      `array_string_insert_const_mid_lenhalf_by_index_store_same_slot_str::{closure}`:
      `25.39%`
    - observer-store closure:
      `array_string_indexof_const_suffix_region_store::{closure...}`:
      `24.05%`
    - `nyash.array.string_insert_mid_lenhalf_store_hisi`: `3.23%`
    - `nyash.array.string_len_hi`: `1.08%`
- verdict:
  - keep as structural cleanup and owner-shift: the extra transaction facade
    symbol is gone
  - not a wall-time keeper: whole remains `Ny AOT 7 ms`
  - next owner proof should use the post-H32 valid-release asm, not stale
    pre-rebuild readings

### H33 Result

Goal: choose the next implementation card from valid post-H32 evidence.

- valid-release evidence:
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`
  - direct-runner top:
    - `__memmove_avx512_unaligned_erms`: `35.52%`
    - observer-store closure:
      `array_string_indexof_const_suffix_region_store::{closure...}`:
      `27.45%`
    - len-half closure:
      `array_string_insert_const_mid_lenhalf_by_index_store_same_slot_str::{closure}`:
      `31.17%`
    - `nyash.array.string_insert_mid_lenhalf_store_hisi`: `1.25%`
    - no hot `nyash.array.string_len_hi`
- callgraph probe:
  - `target/perf_state/h33_kilo_kernel_small.callgraph.perf.data`
  - top shifted between observer-store and len-half closure, but
    `string_len_hi` did not reappear as an active owner
  - `memmove` remains a broad copy symptom; previous H29 byte-copy surgery
    already failed, so do not reopen local insert-copy surgery without a new
    representation proof
- verdict:
  - close H33 as an owner-decision card
  - next implementation card is a narrow runtime-private observer-store byte
    leaf thinning: short literal prefix check and short suffix byte write
  - MIR remains legality/provenance/publication owner
  - `.inc` remains metadata-to-call emit only

### H34 Result

Goal: reduce observer-store closure cost with a runtime-private short-byte leaf
only.

- scope:
  - `src/boxes/array/ops/text.rs`
  - `text_contains_literal` prefix path for short const needles
  - `append_short_text_suffix` for short const suffixes
- allowed:
  - byte-level mechanics such as unaligned fixed-width prefix compare and
    fixed-width suffix write
  - no new public ABI
  - no MIR or `.inc` metadata change
- forbidden:
  - source-prefix semantic assumptions such as rows always containing `"line"`
  - search-result cache
  - runtime legality/provenance inference
  - reopening len-half representation or byte-copy surgery from `memmove`
    percentage alone
- keeper gate:
  - `cargo test -q array::tests --lib`: pass
  - `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`: pass
  - `tools/perf/build_perf_release.sh`: pass
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 3`:
    `C 83 ms / Ny AOT 7 ms`,
    `ny_aot_instr=50229601`, `ny_aot_cycles=16375916`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_kernel_small '' 20`:
    - `__memmove_avx512_unaligned_erms`: `53.25%`
    - len-half closure: `26.76%`
    - observer-store closure: `14.03%`
  - no-regression checks:
    - `kilo_meso_substring_concat_array_set_loopcarry`:
      `C 3 ms / Ny AOT 4 ms`, `ny_aot_instr=16570930`
    - `kilo_micro_array_string_store`:
      `C 10 ms / Ny AOT 4 ms`, `ny_aot_instr=9265993`
- verdict:
  - keep H34 as a runtime-private mechanics keeper
  - whole wall remains `Ny AOT 7 ms`, but primary direct-only instruction count
    dropped from post-H32 `60315390` to `50229601`
  - observer-store closure shrank from `27.45%` to `14.03%`
  - next owner is now the len-half edit copy / residual `memmove` family, not
    observer-store search/suffix mechanics

### H35 Result

Goal: decide the next valid post-H34 card for the remaining len-half copy owner.

- evidence:
  - post-H34 callgraph bundle:
    `target/perf_state/h35_kilo_kernel_small.callgraph.perf.data`
  - no-children top:
    - `__memmove_avx512_unaligned_erms`: `48.59%`
    - len-half closure:
      `array_string_insert_const_mid_lenhalf_by_index_store_same_slot_str::{closure}`:
      `26.13%`
    - observer-store closure:
      `array_string_indexof_const_suffix_region_store::{closure...}`:
      `16.08%`
    - `nyash.array.string_insert_mid_lenhalf_store_hisi`: `2.45%`
- verdict:
  - close H35 as owner decision
  - observer-store is no longer the primary code card after H34
  - remaining owner is flat text residence suffix movement under the H27
    len-half edit
  - do not repeat H29 local byte-copy surgery; it already failed without a
    representation proof
  - next step is a design gate for non-flat / gap / piece residence under
    `ArrayTextCell`

### H36 Result

Goal: decide whether `ArrayTextCell` should open a non-flat residence
representation for repeated len-half inserts.

- design questions:
  - can a non-flat cell preserve visible `ArrayBox` behavior while keeping
    internal len/indexOf/append/insert operations local?
  - where is materialization allowed, given existing `as_str` and
    `as_mut_string` callers?
  - should the first pilot be gap-like, piece-like, or rejected until a broader
    text residence interface exists?
- scope:
  - docs/design first
  - source inventory around `ArrayTextCell`, `ArrayStorage::Text`, visible
    get/format/equality paths, and observer-store operations
- forbidden:
  - benchmark-named representation
  - source content assumptions
  - MIR or `.inc` route changes before the runtime residence contract is clear
  - another flat `String::insert_str` bypass without a new structural proof
- exit:
  - first implementation card is H36.1, a behavior-preserving operation API
    split for `ArrayTextCell`
  - do not add a non-flat variant before H36.1 is green

SSOT:

- [137x-97 H36 ArrayTextCell residence design gate](./137x-97-h36-array-text-cell-residence-design-gate.md)

### H36.1 Result

Goal: split `ArrayTextCell` operation APIs before any non-flat residence
variant.

- scope:
  - `src/boxes/array/text_cell.rs`
  - `src/boxes/array/ops/text.rs`
- allowed:
  - flat-only `ArrayTextCell::{contains_literal, append_suffix}` helpers
  - replace hot-path `as_str` / `as_mut_string` calls where the caller wants an
    operation, not public materialization
- forbidden:
  - `Piece` / `Gap` variants
  - MIR or `.inc` edits
  - perf keeper claim
- acceptance:
  - `cargo fmt --check`
  - `git diff --check`
  - `cargo test -q array::tests --lib`
  - `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`
  - `tools/checks/current_state_pointer_guard.sh`

Result:

- landed as BoxShape-only runtime cleanup
- hot-path contains/append operations now go through `ArrayTextCell`
  methods / string leaf wrappers
- no `Piece` / `Gap`, no MIR or `.inc` edits, no public ABI change, and no perf
  keeper claim

### H36.2 Result

Goal: decide whether to open a narrow non-flat `ArrayTextCell` residence pilot
or reject it to a later TextCell / allocator lane.

- first step:
  - rebuild release artifacts
  - refresh whole `kilo_kernel_small` stat and asm after H36.1
  - only open representation code if fresh evidence still points at a
    structural flat-residence copy owner
- forbidden:
  - implementation from stale perf artifacts
  - benchmark-named representation
  - MIR or `.inc` route changes for a runtime residence experiment
  - another flat byte-copy bypass without a new representation proof

Evidence:

- rebuilt release artifacts with `tools/perf/build_perf_release.sh`
- whole `kilo_kernel_small = C 82 ms / Ny AOT 7 ms`
- `ny_aot_instr=50229407`, `ny_aot_cycles=16401030`
- asm top:
  - `__memmove_avx512_unaligned_erms`: `38.15%`
  - len-half edit closure: `33.22%`
  - observer-store closure: `20.21%`

Verdict:

- non-flat text residence remains justified by fresh owner evidence
- do not add `Piece` / `Gap` yet
- next card is H36.3: make visible text materialization/comparison explicit so
  a later non-flat representation does not leak through `as_str()` / derived
  equality / derived order

### H36.3 Result

Goal: split visible text materialization/comparison APIs before adding any
non-flat `ArrayTextCell` representation.

- allowed:
  - add flat-only `ArrayTextCell` helpers such as `to_visible_string`,
    `equals_text`, `cmp_text`, and `with_text`
  - replace visible Array get/format/equality/membership/sort paths that use
    raw `as_str()` or derived cell ordering
- forbidden:
  - adding `Piece` / `Gap`
  - changing MIR metadata, `.inc`, or public ABI
  - perf keeper claim
- acceptance:
  - `cargo fmt --check`
  - `git diff --check`
  - `cargo test -q array::tests --lib`
  - `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`
  - `tools/checks/current_state_pointer_guard.sh`

Result:

- landed BoxShape-only visible materialization split
- Array visible get/boxing/format/equality/membership/sort now use
  `ArrayTextCell` helpers instead of raw `as_str()` / derived cell ordering
- no `Piece` / `Gap`, no MIR or `.inc` edits, no public ABI change, and no perf
  keeper claim

### H36.4 Result

Goal: test a narrow runtime-private piece residence representation for repeated
len-half inserts.

- allowed:
  - add `ArrayTextCell::Pieces` or equivalent internal residence variant
  - keep len/contains/append/insert/materialize behavior owned by
    `ArrayTextCell`
  - materialize only at visible/public boundaries
- forbidden:
  - MIR metadata or `.inc` changes
  - public ABI changes
  - source-content assumptions such as rows containing `"line"`
  - benchmark-named representation or helper
  - semantic/search-result cache
- keeper gate:
  - behavior tests stay green
  - rebuild release artifacts before judging perf
  - whole `kilo_kernel_small` improves and `memmove` / len-half closure owner
    shrinks without moving dominant cost to allocator

Result:

- trial:
  - added a runtime-private `ArrayTextCell::Pieces { pieces, len }` variant
  - kept MIR, `.inc`, and public ABI unchanged
  - routed len/contains/append/insert/materialize through `ArrayTextCell`
- behavior verification:
  - `cargo fmt --check`
  - `git diff --check`
  - `tools/checks/current_state_pointer_guard.sh`
  - `cargo test -q array::tests --lib`
  - `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`
- perf:
  - release artifacts rebuilt before measuring
  - whole `kilo_kernel_small = C 85 ms / Ny AOT 114 ms`
  - `ny_aot_instr=2084599541`, `ny_aot_cycles=521801542`
- verdict:
  - rejected; naive piece vectors create work explosion
  - code was reverted
  - do not reopen non-flat residence without a bounded piece/gap proof and a
    plan for observer-store contains over non-flat text

### H37 Result

Goal: refresh the whole-front owner after the H36.4 rejection from reverted
code.

- first step:
  - rebuild release artifacts
  - rerun whole `kilo_kernel_small` stat and asm
  - choose the next owner family from fresh evidence
- forbidden:
  - using the rejected H36.4 release artifact as current baseline
  - reopening non-flat residence without bounded piece/gap proof
  - local byte-copy surgery already rejected by H29/H36

Evidence:

- rebuilt release artifacts from reverted H36.3 code
- whole `kilo_kernel_small = C 82 ms / Ny AOT 7 ms`
- `ny_aot_instr=50229360`, `ny_aot_cycles=16404095`
- asm top:
  - `__memmove_avx512_unaligned_erms`: `49.02%`
  - len-half edit closure: `22.74%`
  - observer-store closure: `18.88%`
  - `_int_realloc`: `0.90%`

Verdict:

- top owner is still flat len-half text movement.
- allocator is not dominant.
- naive pieces are rejected; do not reopen unbounded piece vectors.
- next card is H38 bounded gap / edit-buffer design, docs-first.

### H38 Result

Goal: decide whether a bounded gap/edit-buffer cell can reduce the len-half
movement owner without repeating H36.4 work explosion.

- design requirements:
  - keep representation private to `ArrayTextCell`
  - bounded move/update rule for repeated len-half insertion
  - explicit materialization policy for visible Array boundaries
  - `contains_literal` and `append_suffix` behavior that does not materialize
    every observer-store iteration
  - compaction/cap rules that cannot grow unbounded hidden work
- forbidden:
  - code before the above contract is written
  - MIR or `.inc` changes
  - benchmark-named representation
  - semantic/search-result cache

Decision:

- open a private bounded mid-gap variant inside `ArrayTextCell`.
- logical text is `left + right[right_start..]`.
- len-half insert moves the right boundary by offset, not by draining the
  active right tail.
- suffix append writes to the right tail.
- `contains_literal` checks left/right/boundary without full materialization.
- visible boundaries still materialize explicitly.
- cap/compaction rules prevent unbounded consumed right prefix and left-side
  overshoot.

### H38.1 Result

Goal: implement the runtime-private bounded mid-gap pilot inside
`ArrayTextCell`.

- allowed:
  - add the private cell variant and route cell methods through it.
  - keep generic `&mut String` fallback APIs materializing explicitly.
  - add focused unit tests for materialization, contains across the boundary,
    append, repeated len-half insert, and fallback compatibility.
- forbidden:
  - MIR or `.inc` changes.
  - public ABI changes.
  - benchmark-name/source-content branches.
  - semantic/search-result cache.
- keeper gate:
  - behavior tests stay green.
  - release artifacts are rebuilt before measuring.
  - whole `kilo_kernel_small` improves or the card records a reject with owner
    movement evidence.

Implementation:

- added private `ArrayTextCell::MidGap { left, right, right_start }`.
- logical text is `left + right[right_start..]`.
- len-half insert moves the right-side boundary by offset and caps left
  overshoot.
- `contains_literal`, `append_suffix`, and visible materialization stay inside
  the cell boundary.

Verification:

- `cargo fmt --check`
- `git diff --check`
- `tools/checks/current_state_pointer_guard.sh`
- `cargo test -q array::text_cell --lib`
- `cargo test -q array::tests --lib`
- `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`
- release artifacts rebuilt with `tools/perf/build_perf_release.sh`

Perf:

- whole `kilo_kernel_small = C 83 ms / Ny AOT 6 ms`
- `ny_aot_instr=60923714`, `ny_aot_cycles=12531473`
- asm top:
  - len-half edit closure: `49.27%`
  - observer-store closure: `41.58%`
  - `nyash.array.string_insert_mid_lenhalf_store_hisi`: `1.49%`
  - `nyash.array.string_indexof_suffix_store_region_hisisi`: `1.46%`
  - `__memmove_avx512_unaligned_erms`: `0.23%`
- guards:
  - exact `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`,
    `ny_aot_instr=9266540`, `ny_aot_cycles=2423297`
  - middle `kilo_meso_substring_concat_array_set_loopcarry =
    C 3 ms / Ny AOT 4 ms`, `ny_aot_instr=17650827`,
    `ny_aot_cycles=4300117`

Verdict:

- owner-moving keeper: `memmove` is no longer the dominant owner and
  wall/cycles improve.
- whole instruction count increased, so H39 must refresh the closure-internal
  owner before further representation work.

### H39 Result

Goal: pin the new post-mid-gap hot block.

- current owner candidates:
  - len-half edit closure internals.
  - observer-store closure internals after non-flat cells.
- first step:
  - rerun focused annotate / owner bundle for the two closures.
  - determine whether the next seam is materialization, boundary search,
    right-tail append, branch layout, or generic closure overhead.
- forbidden:
  - reopening unbounded pieces.
  - MIR / `.inc` changes before H39 proves a metadata seam.
  - semantic/search-result cache.

Evidence:

- len-half edit closure focused annotate:
  - local `62.33%` at `lock cmpxchg` in the write-lock acquire path.
  - conclusion: this owner is lock-boundary / call-region shape, not text
    representation.
- observer-store closure focused annotate:
  - local write-lock acquire only `2.60%`.
  - samples are in text-cell iteration, short-literal dispatch, and MidGap
    segment checks.
  - conclusion: this owner is a cell-loop/search mechanics seam, not the
    outer edit lock seam.

Verdict:

- H39 closed as owner refresh.
- do not reopen representation work immediately.
- next card must choose one of two owners before code:
  - outer edit lock-boundary, likely requiring a MIR-proven region boundary.
  - observer-store cell-loop mechanics, runtime-only if it stays generic.

### H39.1 Result

Goal: choose the next seam after H39.

- candidate A:
  - outer edit lock-boundary.
  - clean route would be MIR-owned region proof; `.inc` remains emit-only;
    runtime owns one-call executor mechanics.
- candidate B:
  - observer-store cell-loop mechanics.
  - clean route must not use source-content assumptions or search-result
    cache; it may only simplify generic literal/segment mechanics.
- first step:
  - compare expected win and rollback size.
  - write the selected implementation card before code.

Selected first probe:

- choose candidate B first because it is runtime-only and has small rollback.
- H39.1 implementation card: MidGap generic prefix fast path.
- rationale:
  - observer-store hot path repeatedly calls `contains_literal` on MidGap text.
  - a prefix hit is a valid generic literal fact, not a source-content branch
    and not a search-result cache.
  - if this is non-win, reject it and return to candidate A's MIR region
    design.
- acceptance:
  - behavior tests stay green.
  - whole `kilo_kernel_small` stat/asm is rerun from rebuilt release.
  - reject if observer-store closure does not shrink or if exact/middle
    guards regress.

Implementation:

- added a runtime-only generic prefix literal hit before the full MidGap
  segmented search.
- no MIR, `.inc`, public ABI, source-content branch, or search-result cache.

Verification:

- `cargo fmt --check`
- `git diff --check`
- `cargo test -q array::text_cell --lib`
- release artifacts rebuilt with `tools/perf/build_perf_release.sh`

Perf:

- whole `kilo_kernel_small = C 83 ms / Ny AOT 6 ms`
- `ny_aot_instr=60443810`, `ny_aot_cycles=11322220`
- asm top:
  - observer-store closure: `51.21%`
  - len-half edit closure: `30.53%`
  - `__memmove_avx512_unaligned_erms`: `4.62%`
- guards:
  - exact `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9266628`, `ny_aot_cycles=2432139`
  - middle `kilo_meso_substring_concat_array_set_loopcarry =
    C 3 ms / Ny AOT 3 ms`, `ny_aot_instr=17651373`,
    `ny_aot_cycles=4229069`

Verdict:

- small keeper: whole cycles improve from H38.1's `12531473` to `11322220`.
- next seam is the outer edit lock-boundary.

### H39.2 Result

Goal: design the outer edit lock-boundary reduction.

- current proof:
  - len-half edit closure focused annotate showed local `62.33%` at
    write-lock `lock cmpxchg`.
- allowed:
  - MIR-proven region boundary design.
  - one-call runtime executor if MIR owns legality/lifetime.
- forbidden:
  - hidden runtime session handle table.
  - `.inc` rediscovery of loop shape.
  - benchmark-named whole-loop helper.

Inventory:

- existing MIR metadata:
  - `array_text_edit_routes` proves the outer same-slot len-half
    insert-mid edit in block `23`
  - `array_text_observer_routes.executor_contract` proves the inner
    conditional `indexOf("line") >= 0` + same-slot suffix store region
  - `array_text_residence_sessions` is empty for `kilo_kernel_small` because
    the outer body is not a single loopcarry len/store body
- existing backend/runtime executor:
  - the H25 single-region executor is valid only for one covered
    loopcarry len/store route plus pure bookkeeping
  - the H26 observer-store executor is valid only for the inner row scan
  - the outer loop order is:
    `edit(row) -> undo++ -> if i % 8 == 0 { observer-store rows } -> i++`

Verdict:

- H39.2 is closed as design / stop-line.
- do not add an edit-only runtime session:
  - it would still acquire the write lock once per outer iteration
  - extending it across the observer-store call would hide lifetime/ordering
    legality in runtime
- the next clean keeper candidate is a MIR-owned combined region contract:
  - one proof covers the outer loop header/body/latch and the nested
    observer-store region
  - `.inc` consumes only that metadata and emits one begin-site call
  - Rust executes the edit + periodic observer-store in one RAII call; the
    write guard never crosses the C ABI

### H39.3 Result

Goal: implement the first bounded combined edit-observer region proof.

- contract shape:
  - `proof_region=outer_loop_with_periodic_observer_store`
  - `publication_boundary=none`
  - `carrier=array_lane_text_cell`
  - effects:
    - `store.cell(lenhalf_insert_mid_const)`
    - `observe.indexof`
    - `store.cell(const_suffix_append)`
    - `scalar_accumulator(+1)`
  - consumer capabilities:
    - `sink_store`
    - `compare_only`
    - `length_only_result_carry`
- required MIR facts:
  - outer loop bound and row modulus are constant
  - edit route and observer route use the same array root
  - the edit row index is `outer_i % row_modulus`
  - the observer trigger is `outer_i % period == 0`
  - the nested observer executor has `execution_mode=single_region_executor`
  - no publish/objectize/generic escape appears between the covered effects
  - the final scalar value used after the loop is the MIR-proven accumulator
    result, not a runtime-inferred side effect
- implementation order:
  1. metadata-only route/proof + MIR JSON emission
  2. `.inc` reader that validates the contract and still does no raw shape
     rediscovery
  3. one-call runtime-private RAII executor
  4. perf keeper gate
- reject seams:
  - no hidden session table
  - no helper-name truth in MIR
  - no benchmark-name branch in runtime
  - no source-content assumption beyond const needle/suffix metadata
  - no broad loop executor framework until this single pattern wins

Implementation:

- added MIR-owned `array_text_combined_regions` metadata
- the route is derived from:
  - the H27 `array_text_edit_routes` len-half same-slot edit proof
  - the H26 `array_text_observer_routes.executor_contract` nested
    observer-store proof
  - outer loop PHI/bound/modulus/period facts
- no `.inc` lowering, runtime helper, public ABI, or behavior changed

Verification:

- `cargo check -q`
- `cargo fmt --check`
- `cargo test -q benchmark_kilo_kernel_small_has_combined_edit_observer_region -- --nocapture`
- `cargo run -q --bin hakorune -- --emit-mir-json target/perf_state/h39_combined_region.mir.json benchmarks/bench_kilo_kernel_small.hako`
- MIR JSON proof:
  - one `array_text_combined_regions` entry
  - `proof=outer_lenhalf_edit_with_periodic_observer_store`
  - `loop_bound_const=60000`
  - `row_modulus_const=64`
  - `observer_period_const=8`
  - `observer_bound_const=64`
  - accumulator PHI is distinct from the loop-index PHI

Verdict:

- metadata keeper: MIR now owns the combined region legality/proof.
- next card may lower this metadata to one begin-site runtime call.

### H39.4 Result

Goal: consume `array_text_combined_regions` as a one-call executor.

- `.inc` contract:
  - read only `array_text_combined_regions`
  - validate `execution_mode=single_region_executor`
  - emit one begin-site call
  - mark only the MIR-covered blocks unreachable
  - do not rescan raw block/window shape
- Rust runtime contract:
  - one RAII call owns the write guard internally
  - execute `lenhalf insert-mid` then periodic observer-store in the same
    order as `.hako`
  - return the MIR-proven scalar accumulator result
  - no guard/session handle crosses C ABI
- keeper gate:
  - generated `ny_main` no longer calls per-iteration
    `nyash.array.string_insert_mid_lenhalf_store_hisi`
  - whole `kilo_kernel_small` wall/cycles/instructions improve or the card is
    rejected
  - exact and middle guards stay no-regression

Implementation:

- added a `.inc` metadata reader for `array_text_combined_regions`
- generic lowering emits one begin-site call to a runtime-private combined
  region executor and marks only MIR-covered blocks unreachable
- Rust executes the outer len-half edit and periodic observer-store in one
  RAII write-guard frame
- tightened MIR proof so this executor only accepts zero-initial accumulator
  shapes, matching the returned scalar contract

Verification:

- `cargo fmt --check`
- `cargo check -q`
- `cargo test -q benchmark_kilo_kernel_small_has_combined_edit_observer_region -- --nocapture`
- `bash tools/perf/build_perf_release.sh`
- current object check: `ny_main` calls
  `nyash.array.string_lenhalf_insert_mid_periodic_indexof_suffix_region_hiisiiisisi`
  and no longer calls `nyash.array.string_insert_mid_lenhalf_store_hisi`

Perf:

- whole `kilo_kernel_small = C 82 ms / Ny AOT 5 ms`
- `ny_aot_instr=49691801`, `ny_aot_cycles=9882715`
- asm top:
  - combined region executor closure: `82.23%`
  - `__memmove_avx512_unaligned_erms`: `7.01%`
  - `alloc::sync::Arc<T,A>::drop_slow`: `1.70%`
  - `str` range get: `1.45%`
  - `_int_malloc`: `1.10%`
  - `realloc`: `1.00%`
- guards:
  - exact `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9265823`, `ny_aot_cycles=2405196`
  - middle `kilo_meso_substring_concat_array_set_loopcarry =
    C 4 ms / Ny AOT 3 ms`, `ny_aot_instr=17651027`,
    `ny_aot_cycles=4262736`

Verdict:

- keeper: whole wall/cycles/instructions improve and the emitted hot loop no
  longer calls the per-iteration outer edit helper.
- next owner is inside the combined executor closure, not `.inc` route
  selection or runtime lock-boundary frequency.

### H39.5 Result

Goal: refresh the owner inside the H39.4 combined executor before another code
slice.

- first step:
  - annotate / inspect the combined executor closure from the current
    `kilo_kernel_small` direct AOT executable
  - split the owner between len-half edit mutation, periodic observer-store
    cell loop, visible text/range access, and residual allocation/copy
- allowed:
  - perf/asm/source confirmation only until the owner block is pinned
  - runtime-only cleanup if the hot block is a mechanical executor leaf
  - MIR metadata only if the hot block proves a missing generic contract fact
- forbidden:
  - `.inc` shape rediscovery
  - hidden runtime legality/session state
  - search-result cache or source-content assumptions
  - broad allocator/arena work before allocator/copy is again dominant

Evidence:

- direct AOT top still names the combined executor closure as the dominant
  owner (`80-82%` local top in the direct runner).
- focused annotate shows samples in runtime mechanics, not in `.inc`:
  - loop index / observer-period arithmetic still lowers to division on the
    current constants (`row_modulus=64`, `observer_period=8`)
  - MidGap insert/contains/append branches and UTF-8 boundary checks are the
    visible source-side hot block
  - residual libc `memmove` remains secondary (`5-7%`)

Verdict:

- H39.5 closes as owner refresh.
- first narrow code slice is runtime-only: replace power-of-two modulo in the
  combined executor with bitmask arithmetic.

### H39.5.1 Result

Goal: reduce combined executor loop arithmetic without touching MIR or `.inc`.

- implementation:
  - if `row_modulus` or `observer_period` is a power of two, use `step & mask`
    instead of `%`
  - keep generic modulo fallback for non-power-of-two metadata
- keeper gate:
  - whole `kilo_kernel_small` improves or is at least no-regression
  - exact/middle guards stay no-regression
  - no MIR metadata, `.inc`, or public ABI changes

Implementation:

- runtime-only change in `ArrayBox::slot_text_lenhalf_insert_mid_periodic_indexof_suffix_region_raw`
- use `step & mask` for power-of-two `row_modulus` / `observer_period`
- keep `%` fallback for non-power-of-two metadata

Evidence:

- whole guard:
  - `kilo_kernel_small = C 83 ms / Ny AOT 6 ms`
  - `ny_aot_instr=49271666`
  - `ny_aot_cycles=9282981`
- direct AOT asm top:
  - combined executor closure: `88.57%`
  - `__memmove_avx512_unaligned_erms`: `5.06%`
  - `_int_malloc`: `0.55%`
  - `_int_realloc`: `0.40%`
- exact guard:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
  - `ny_aot_instr=9265976`
  - `ny_aot_cycles=2404527`
- middle guard:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 4 ms`
  - `ny_aot_instr=17651126`
  - `ny_aot_cycles=4237981`

Verdict:

- H39.5.1 is accepted as a narrow runtime-internal cleanup.
- It is not a wall-time keeper: whole/middle integer `ms` did not improve.
- Next work must keep the owner-first discipline and inspect the combined
  executor closure again before touching code.

### H39.5.2 Result

Goal: split the post-pow2 combined executor hot block before the next code
slice.

- first step:
  - annotate the H39.5.1 direct AOT executor closure
  - separate MidGap text access, UTF-8/range checks, append/contains mechanics,
    and residual copy/allocation
- allowed:
  - runtime-only cleanup if the sampled block is a mechanical text-cell leaf
  - MIR metadata only if the sampled block proves a missing generic contract
    fact
- forbidden:
  - `.inc` shape rediscovery
  - hidden runtime legality/session state
  - search-result cache or source-content assumptions
  - public ABI widening for a local cleanup

Implementation:

- runtime-only change in `ArrayTextCell`
- replace hot MidGap right slices with debug-asserted unchecked helpers:
  - `active_mid_gap_right(right, right_start)`
  - `mid_gap_right_range(right, start, end)`
- no MIR metadata, `.inc`, or public ABI changes

Evidence:

- whole guard:
  - `kilo_kernel_small = C 84 ms / Ny AOT 5 ms`
  - `ny_aot_instr=42303268`
  - `ny_aot_cycles=8732285`
- direct AOT asm top:
  - combined executor closure: `89.17%`
  - `__memmove_avx512_unaligned_erms`: `5.52%`
  - `_int_realloc`: `1.04%`
  - `core::str::Range::get` is no longer in the top report
- exact guard:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
  - `ny_aot_instr=9265804`
  - `ny_aot_cycles=2352051`
- middle guard:
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 4 ms`
  - `ny_aot_instr=17651020`
  - `ny_aot_cycles=4233835`

Verdict:

- H39.5.2 is a keeper.
- The hot closure remains dominant, so the next slice must re-annotate before
  code rather than guessing at a source helper.

### H39.5.3 Active

Goal: refresh the residual owner after the MidGap range cleanup.

- first step:
  - re-run direct AOT asm/annotate on `kilo_kernel_small`
  - split the remaining combined executor closure between insert-mid copy,
    short literal search, append suffix, allocation, and branch mechanics
- allowed:
  - runtime-only leaf cleanup if a sampled source helper is mechanically
    redundant
  - reject local cleanup and stop if the remaining owner is broad copy/alloc
- forbidden:
  - `.inc` shape rediscovery
  - MIR metadata changes without a missing contract fact
  - search-result cache or source-content assumptions

Result:

- implementation:
  - runtime-only 4-byte literal observer leaf in `ArrayTextCell`
  - combined executor precomputes the 4-byte literal word once and uses it for
    text-cell observer checks
  - no MIR metadata, `.inc`, or public ABI changes
  - no search-result cache and no assumption that the source starts with
    `"line"`
- evidence:
  - whole `kilo_kernel_small = C 85 ms / Ny AOT 5 ms`
  - `ny_aot_instr=35428450`
  - `ny_aot_cycles=6679916`
  - direct AOT asm top:
    - combined executor closure: `81.80%`
    - `__memmove_avx512_unaligned_erms`: `8.44%`
    - `_int_malloc`: `2.80%`
  - exact `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
  - exact `ny_aot_instr=9266200`
  - exact `ny_aot_cycles=2437087`
  - middle `kilo_meso_substring_concat_array_set_loopcarry =
    C 3 ms / Ny AOT 4 ms`
  - middle `ny_aot_instr=17650994`
  - middle `ny_aot_cycles=4214918`
- verdict:
  - H39.5.3 is a keeper.
  - The next owner is no longer generic 4-byte observer dispatch; residual
    samples are combined executor work plus `memmove` / allocator mechanics.

### H39.5.4 Active

Goal: refresh the residual copy/allocation owner after the 4-byte literal
observer cleanup.

- first step:
  - re-run direct AOT asm/annotate on `kilo_kernel_small`
  - split the remaining combined executor closure between MidGap insert copy,
    suffix append copy, observer scan branch mechanics, `memmove`, and
    allocation
- allowed:
  - runtime-only leaf cleanup if a sampled block is mechanically redundant
  - stop and record a reject/defer if the remaining owner is broad copy/alloc
    rather than a narrow leaf
- forbidden:
  - `.inc` shape rediscovery
  - MIR metadata changes without a missing contract fact
  - search-result cache or source-content assumptions

Result:

- evidence:
  - preserved AOT perf data:
    `target/perf_state/h39_5_4_kilo_kernel_small.perf.data`
  - preserved annotate:
    `target/perf_state/h39_5_4_kilo_kernel_small.annotate.txt`
  - direct AOT top:
    - combined region executor closure: `75.26%`
    - `__memmove_avx512_unaligned_erms`: `10.03%`
    - `_int_malloc`: `2.05%`
    - `_int_realloc`: `0.36%`
    - `alloc::raw_vec::RawVecInner<A>::reserve::do_reserve_and_handle`: `0.30%`
    - `nyash.array.string_len_hi`: `0.05%` and belongs only to the final
      64-row sum loop
  - annotate still samples the MidGap insert path around the byte legality
    check / branch block; this is a correctness boundary, not a runtime
    mechanics duplicate
- verdict:
  - close H39.5.4 as no-code owner refresh
  - further runtime-only unchecked slicing would move legality into Rust
  - open H40 so MIR can own the byte-boundary / ASCII-preservation proof and
    runtime can consume it as executor-only metadata

### H40 Active

Goal: move the remaining MidGap text-cell edit byte-boundary proof to MIR
metadata before considering a runtime fast leaf that skips boundary checks.

- target front:
  - `kilo_kernel_small`
- owner hypothesis:
  - residual executor samples include UTF-8 byte-boundary legality checks in
    the len-half text-cell edit path
  - the legal fact is derivable from MIR/lowering facts such as ASCII literals,
    byte-index split policy, and covered text-cell edit region
  - Rust must not infer that fact from current storage contents or benchmark
    source shape
- MIR owns:
  - `byte_boundary_safe` / `ascii_preserved` proof for the covered edit region
  - split boundary policy for byte-indexed edits
  - consumer/publication boundary remains `publication_boundary=none`
- `.inc` owns:
  - metadata consumption only
  - passing the proof bit to the existing runtime executor or a narrow variant
  - no raw MIR rediscovery of literals, loop shape, or source provenance
- Rust runtime owns:
  - executing the fast leaf only when the MIR proof is explicit
  - preserving the safe checked path when the proof is absent
- forbidden:
  - source-content assumptions such as rows starting with `"line"`
  - benchmark-named helpers
  - search-result cache
  - runtime-owned encoding/provenance inference
  - `.inc` planner fallback or shape rediscovery
- first implementation step:
  - inspect the existing `array_text_combined_regions` metadata writer,
    `.inc` reader, and runtime executor signature
  - add the smallest generic proof field and a MIR JSON fixture/test before
    changing runtime boundary checks
- accept gate:
  - MIR JSON carries the proof only for the proven `kilo_kernel_small` covered
    edit region
  - `.inc` consumes that metadata without scanning raw MIR instructions
  - runtime behavior is unchanged when the proof is absent
  - exact/middle/whole gates remain green before any keeper claim

H40.1 metadata proof slice:

- implementation:
  - `array_text_combined_regions` now carries optional
    `byte_boundary_proof=ascii_preserved_text_cell`
  - JSON also emits `text_encoding=ascii_preserved` and
    `split_boundary_policy=byte_index_safe`
  - the proof is only produced when MIR sees an ASCII seed loop for the same
    array plus ASCII edit / observer / suffix literals in the covered region
  - `.inc` accepts and mirrors the optional proof bit; it still does not
    rediscover source shape
  - runtime behavior is unchanged in this slice
- verification:
  - `cargo fmt --check`
  - `git diff --check`
  - `cargo check -q`
  - `cargo test -q benchmark_kilo_kernel_small_has_combined_edit_observer_region -- --nocapture`
  - `cargo run -q --bin hakorune -- --emit-mir-json target/perf_state/h40_byte_boundary_proof.mir.json benchmarks/bench_kilo_kernel_small.hako`
  - `bash tools/perf/build_perf_release.sh`
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_kernel_small 1 1`
  - JSON proof check:
    `byte_boundary_proof=ascii_preserved_text_cell`,
    `text_encoding=ascii_preserved`,
    `split_boundary_policy=byte_index_safe`
  - smoke perf/compile check:
    `kilo_kernel_small = C 82 ms / Ny AOT 5 ms`,
    `ny_aot_instr=35428267`, `ny_aot_cycles=6731377`
- next:
  - compile the AOT path with the `.inc` reader active
  - add a proof-consuming runtime fast leaf or helper variant only after the
    checked/no-proof path remains unchanged

### H28.1 runtime-private literal search executor

- decision:
  - current MIR metadata already carries the needed generic facts:
    `observer_kind=indexof`, `observer_arg0_repr=const_utf8`, literal byte len,
    `effects=[observe.indexof, store.cell]`, and `publication_boundary=none`
  - do not add a sibling plan or new `.inc` planner for this slice
  - keep the emitted helper unchanged and replace only the runtime executor's
    `str::contains` Pattern path with a small literal byte-search leaf
- owner boundary:
  - MIR remains legality/provenance/publication owner
  - `.inc` remains metadata-to-call emit only
  - Rust owns only search mechanics for the MIR-proven literal observer
- keeper gate:
  - `kilo_kernel_small` must improve or prove the owner moved from
    `<&str as Pattern>::is_contained_in`
  - exact/middle guards must stay non-regressing
- reject gate:
  - no source-prefix assumption such as rows always containing `"line"`
  - no search-result cache
  - no runtime legality inference

Result:

- code:
  - replaced the runtime executor's `str::contains` call with a private
    `text_contains_literal` leaf for short UTF-8 literals
  - no MIR metadata shape changed
  - no `.inc` emit shape changed
- verification:
  - `cargo test -q text_contains_literal --lib`
  - `bash tools/perf/build_perf_release.sh`
  - whole `kilo_kernel_small`: `C 84 ms / Ny AOT 9 ms`,
    `ny_aot_instr=60662079`, `ny_aot_cycles=20100504`
  - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9265703`, `ny_aot_cycles=2442083`
  - middle `kilo_meso_substring_concat_array_set_loopcarry`:
    `C 3 ms / Ny AOT 3 ms`, `ny_aot_instr=16570264`,
    `ny_aot_cycles=3533303`
- asm owner after H28.1:
  - `__memmove_avx512_unaligned_erms`: `43.99%`
  - `with_array_text_write_txn` closure: `23.17%`
  - `__memcmp_evex_movbe`: `15.35%`
  - observer-store region closure: `8.07%`
  - `Pattern::is_contained_in` is no longer a top owner
- verdict:
  - keeper: H28.1 removed the fixed-literal search owner without changing MIR
    authority or `.inc` responsibility
  - next seam is H28.2 short-literal prefix compare cleanup before returning
    to suffix mutation/copy / allocation split

### H28.2 runtime-private short-literal prefix compare cleanup

- owner correction:
  - annotate of `nyash.array.string_indexof_suffix_store_region_hisisi` shows
    the remaining `__memcmp_evex_movbe` samples come from the H28.1
    `starts_with` prefix check lowering to libc `bcmp`
  - this is still runtime search mechanics, not MIR legality and not suffix
    mutation/copy
- decision:
  - replace the short-literal prefix check with the same local byte compare used
    by the short-literal search leaf
  - keep the generic long-needle fallback on `str::contains`
  - do not change MIR metadata or `.inc`
- keeper gate:
  - `__memcmp_evex_movbe` should drop from the top owner list or clearly move
    below mutation/copy
  - exact/middle guards must remain no-regression
- next after this slice:
  - only after the compare owner is gone, return to suffix mutation/copy /
    allocation split

Result:

- code:
  - replaced the short-literal prefix `starts_with` check with a private byte
    loop so the runtime leaf no longer lowers the prefix probe to libc `bcmp`
  - no MIR metadata shape changed
  - no `.inc` emit shape changed
- verification:
  - `cargo test -q text_contains_literal --lib`
  - `cargo fmt --check`
  - `bash tools/perf/build_perf_release.sh`
  - whole `kilo_kernel_small`: `C 83 ms / Ny AOT 7 ms`,
    `ny_aot_instr=64501392`, `ny_aot_cycles=18956185`
  - exact `kilo_micro_array_string_store`: `C 11 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9266032`, `ny_aot_cycles=2341864`
  - middle `kilo_meso_substring_concat_array_set_loopcarry`:
    `C 3 ms / Ny AOT 4 ms`, `ny_aot_instr=16571251`,
    `ny_aot_cycles=3446763`
- asm owner after H28.2:
  - `__memmove_avx512_unaligned_erms`: `39.78%`
  - `with_array_text_write_txn` closure: `29.06%`
  - observer-store region closure: `23.51%`
  - `__memcmp_evex_movbe` is no longer a top owner
- verdict:
  - keeper: H28.2 removes the accidental libc compare owner without changing
    MIR authority or `.inc` responsibility
  - next seam is H28.3 suffix mutation/copy / write-frame owner split

### H28.3 runtime-private short suffix append cleanup

- owner correction:
  - annotate of the observer-store closure shows the remaining `memmove` owner
    is the short `value.push_str(suffix)` append after a MIR-proven hit
  - this is suffix copy mechanics inside the existing runtime executor, not new
    route legality
- decision:
  - add a runtime-private short-suffix byte append leaf for small UTF-8 suffixes
  - keep long suffixes on `String::push_str`
  - do not add MIR metadata, `.inc` shape logic, source-prefix assumptions, or
    search-result cache
- keeper gate:
  - whole `kilo_kernel_small` must improve or show `__memmove` moved below the
    write-frame closure
  - exact/middle guards must stay non-regressing

Result:

- code:
  - added a runtime-private `append_text_suffix` leaf
  - suffixes of `1..=8` bytes append through checked pointer writes instead of
    `String::push_str`
  - long suffixes stay on `String::push_str`
  - no MIR metadata shape changed
  - no `.inc` emit shape changed
- verification:
  - `cargo test -q append_text_suffix --lib`
  - `cargo test -q text_contains_literal --lib`
  - `cargo fmt --check`
  - `bash tools/perf/build_perf_release.sh`
  - whole `kilo_kernel_small`: `C 82 ms / Ny AOT 7 ms`,
    `ny_aot_instr=60615291`, `ny_aot_cycles=17586950`
  - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9266365`, `ny_aot_cycles=2326918`
  - middle `kilo_meso_substring_concat_array_set_loopcarry`:
    `C 3 ms / Ny AOT 4 ms`, `ny_aot_instr=16571079`,
    `ny_aot_cycles=3398840`
- asm owner after H28.3:
  - `__memmove_avx512_unaligned_erms`: `38.17%`
  - `with_array_text_write_txn` closure: `26.80%`
  - observer-store region closure: `26.43%`
  - annotate shows the short suffix path no longer calls `memcpy`; residual
    `memmove` is capacity growth / old-content copy or adjacent write-frame
    mechanics
- verdict:
  - small keeper: H28.3 reduces whole-front instruction/cycle count without
    changing MIR authority or `.inc` responsibility
  - next seam is H28.4 capacity growth / write-frame owner decision

## H27 Landed

Goal: move the outer edit path's len-half split decision above the backend.

- target shape:
  - `array.get(row)` source
  - `source.length()`
  - `split = length / 2`
  - `source.substring(0, split) + const + source.substring(split, length)`
  - same-array, same-index `set`
- MIR contract:
  - `edit_kind=insert_mid_const`
  - `split_policy=source_len_div_const(2)`
  - `publication_boundary=none`
  - `carrier=array_lane_text_cell`
  - `effects=[load.ref, store.cell]`
  - `consumer_capabilities=[sink_store]`
  - `materialization_policy=text_resident_or_stringlike_slot`
- backend rule:
  - consume metadata by `get_block/get_instruction_index`
  - emit one helper that computes the current slot length and split inside the
    runtime-private mutation frame
  - skip only MIR-covered len/split/substring/concat/set instructions
  - do not rediscover this legality from raw JSON
- runtime rule:
  - execute one same-slot insert-mid edit for the selected cell
  - compute `split = current_text.len() / 2` as the MIR-selected policy
  - do not decide legality, provenance, publication, or route fallback
- acceptance:
  - PASS: emitted `kilo_kernel_small` outer edit path no longer calls
    `nyash.array.string_len_hi`
  - PASS: exact and middle guards remain no-regression
  - result:
    - whole `kilo_kernel_small`: `C 83 ms / Ny AOT 10 ms`,
      `ny_aot_instr=144977171`, `ny_aot_cycles=30931233`
    - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 4 ms`
    - middle `kilo_meso_substring_concat_array_set_loopcarry`:
      `C 4 ms / Ny AOT 4 ms`
  - route proof:
    - MIR JSON emits one `array_text_edit_routes` entry with
      `edit_kind=insert_mid_const`,
      `split_policy=source_len_div_const(2)`,
      `proof=array_get_lenhalf_insert_mid_same_slot`
    - backend route trace hits `stage=array_text_edit_lenhalf`
      with `reason=mir_route_metadata`
    - lowered outer edit block emits one
      `nyash.array.string_insert_mid_lenhalf_store_hisi` call
  - verdict:
    - small keeper / contract cleanup
    - instruction count improved by about `3.1%`, cycles by about `2.6%`;
      wall time stayed in the same `10 ms` band
    - next code card must start from the H28 observer-store search/copy owner,
      not from more len-half edit surgery

## H25a Landed

- Added metadata-only `array_text_residence_sessions`.
- The loopcarry benchmark exposes exactly one session route:
  - `scope=loop_backedge_single_body`
  - `proof=loopcarry_len_store_only`
  - `consumer_capability=slot_text_len_store_session`
  - `publication_boundary=none`
- Behavior is unchanged:
  - no lowering change
  - no runtime/session helper change
  - no `.inc` route change

## H25b Landed

- Worker design check rejected a direct long-lived runtime begin/end ABI as the
  next step:
  - `ArrayStorage` and write guards are private runtime mechanics.
  - exporting/storing guards across C ABI calls would require unsafe lifetime
    or self-referential state.
  - `.inc` cannot infer preheader/exit placement from CFG without becoming a
    planner again.
- Extended `array_text_residence_sessions` as MIR-owned placement metadata:
  - `begin_block` + `begin_placement=before_preheader_jump`
  - `update_block` + `update_instruction_index` +
    `update_placement=route_instruction`
  - `end_block` + `end_placement=exit_block_entry`
  - `skip_instruction_indices`
- H25b remains behavior-preserving. The backend can now lower begin/update/end
  later without rediscovering loop shape from raw MIR.

## H25c.1 Landed

- Renamed active `.inc` array/text reader seams from `*_route_plan` to
  `*_route_metadata` so `plan` stays MIR-internal.
- Added `array_text_residence_sessions` metadata consumption in
  `hako_llvmc_ffi_generic_method_get_window.inc`.
- The current lowering consumes the residence-session metadata first, then maps
  it to the existing loopcarry update helper. This is still behavior-preserving:
  no runtime session helper and no begin/end calls yet.

## H25c.2 Task Split

H25c.2 is split so the clean substrate and the perf keeper decision do not
collapse into one risky change.

- H25c.2a `runtime-private session substrate`
  - status: landed
  - intent: add an `ArrayBox`-local closure-scoped `ArrayTextSlotSession`
    substrate, plus an optional kernel-private `ArrayTextWriteTxn` wrapper.
  - landed:
    - `ArrayTextSlotSession` now owns text-slot update mechanics inside one
      `ArrayBox` write-lock frame.
    - `slot_update_text_resident_raw(...)` and `slot_update_text_raw(...)`
      are adapters over that substrate.
    - `slot_update_text_resident_first_raw(...)` reports whether the update
      hit an already text-resident lane without exposing a guard or slot borrow.
    - kernel-private `array_text_write_txn.rs` wraps handle lookup and
      resident-first/fallback outcome mapping.
    - same-slot string write helpers call the transaction wrapper without
      adding public ABI names.
  - files:
    - `src/boxes/array/ops/text.rs`, or `src/boxes/array/ops/text_session.rs`
    - `src/boxes/array/ops.rs` if a new module is split out
    - `src/boxes/array/tests.rs`
    - `crates/nyash_kernel/src/plugin/array_text_write_txn.rs`
    - `crates/nyash_kernel/src/plugin/mod.rs`
    - optional thin consumer in `crates/nyash_kernel/src/plugin/array_string_slot_write.rs`
    - optional private forwarding in `crates/nyash_kernel/src/plugin/array_runtime_substrate.rs`
  - allowed:
    - hold `RwLockWriteGuard` only inside one Rust stack frame
    - expose only closure-scoped methods such as `with_text_slot_session(...)`
      or `update_text_slot_session(...)`
    - keep existing `slot_update_text_resident_raw(...)` and
      `slot_update_text_raw(...)` as compatibility adapters
    - keep resident and fallback paths distinct
    - add unit tests for text-lane mutation and mixed-array fallback behavior
  - forbidden:
    - public begin/end ABI
    - session handle table
    - storing guard/slot references outside the call stack
    - runtime legality/provenance inference
    - new `nyash.array.*` exported ABI symbols
    - new environment variables
  - keeper expectation: none. This is substrate-only unless later perf evidence
    shows a safe executor boundary can use it.
  - task granularity:
    - H25c.2a-1: add ArrayBox-local `ArrayTextSlotSession` and outcome/kind
      enums.
    - H25c.2a-2: rebuild existing raw update methods as adapters over that
      session substrate.
    - H25c.2a-3: add array unit tests for resident hit, boxed string hit,
      boxed non-string miss, negative index miss, and resident-only non-promotion.
    - H25c.2a-4: add kernel-private `ArrayTextWriteTxn` wrapper for handle
      lookup and resident-first/fallback outcome mapping; observe accounting
      stays at existing helper call sites.
    - H25c.2a-5: refactor existing slot write helpers to call the transaction
      wrapper without changing exported ABI names.
- H25c.2b `single-call executor decision`
  - status: closed as clean non-keeper
  - decide whether `slot_text_len_store_session` can be executed as one
    capability-generic runtime call whose entire proven region stays inside one
    Rust call stack.
  - verdict:
    - accepted as a contract-cleaning boundary only.
    - existing metadata can select the update instruction without `.inc`
      re-scanning CFG or raw neighboring instructions.
    - current one-call executor shape is still one Rust call per iteration, so
      it cannot remove the measured per-iteration write-lock acquire/release
      owner.
    - keep this as clean closeout, not as a perf keeper.
  - rejected for keeper:
    - begin/update/end ABI with session handle table.
    - guard or slot borrow crossing C ABI.
    - runtime-owned legality from residence state.
    - benchmark-named whole-loop helper.
- H25c.2c `single-region executor contract`
  - status: landed
  - intent: open the next keeper path as a MIR-proven region replacement nested
    under `array_text_residence_sessions`, not as a new sibling plan family.
  - H25c.2c-1 landed:
    - `ArrayTextResidenceSessionRoute.executor_contract` is now emitted as
      nested metadata.
    - The current loopcarry benchmark exports:
      - `execution_mode=single_region_executor`
      - `proof_region=loop_backedge_single_body`
      - `publication_boundary=none`
      - `carrier=array_lane_text_cell`
      - `effects=[store.cell, length_only_result_carry]`
      - `consumer_capabilities=[sink_store, length_only]`
      - `materialization_policy=text_resident_or_stringlike_slot`
    - Route tests assert the contract and MIR JSON emission preserves it.
    - Behavior is unchanged; no backend execution replacement yet.
  - H25c.2c-2 landed:
    - `hako_llvmc_ffi_generic_method_get_window.inc` validates the nested
      `executor_contract` before accepting `array_text_residence_sessions`.
    - Missing or mismatched executor contract fields reject the route instead
      of letting `.inc` infer the contract from CFG/raw shape.
    - Active lowering still maps the validated session to the existing
      per-iteration loopcarry update helper; no region replacement yet.
    - Probe trace hit:
      `stage=array_text_residence_session result=hit reason=mir_route_metadata`.
  - H25c.2c-3 landed:
    - `executor_contract.region_mapping` now carries the loop index PHI,
      loop bound, accumulator PHI, accumulator exit use, row index, and row
      modulus needed by a later region replacement.
    - `.inc` validates that `region_mapping` exists, that numeric fields are
      present, that `row_index_value` matches the top-level `index_value`, and
      that the exit accumulator aliases the accumulator PHI.
    - Active backend trace still hits `array_text_residence_session` through
      `mir_route_metadata`; lowering behavior remains unchanged.
  - H25c.2c-4 landed:
    - MIR `region_mapping` now also proves the loop index and accumulator
      initial constants are `0`; runtime does not infer that fact.
    - `.inc` matches the MIR-selected begin block and emits one
      `nyash.array.string_insert_mid_subrange_len_store_region_hiisi` call,
      then skips the covered header/body region without redefining PHI values.
    - Runtime executes the proven loop inside
      `ArrayBox::slot_text_region_update_sum_raw(...)`; the write guard stays
      inside one Rust call stack and no session table or begin/end ABI is used.
    - Probe trace hit:
      `stage=array_text_residence_region_begin result=hit reason=mir_region_mapping`.
  - contract shape:
    - `executor_contract.execution_mode = single_region_executor`
    - `proof_region = loop_backedge_single_body`
    - `publication_boundary = none`
    - `effects = store.cell + LengthOnly result carry`
    - `carrier = ArrayLane(Text) / Cell`
    - `consumer_capability = { SinkStore, LengthOnly }`
    - `materialization_policy` must be MIR-owned if fallback behavior is needed.
  - backend rule:
    - `.inc` emits one call from the MIR-selected begin site and skips the
      covered region.
    - `.inc` must not infer loop legality, preheader, exit, PHI mapping, or
      fallback policy.
  - runtime rule:
    - one-call RAII executor may acquire the array write guard once, resolve the
      target slot once, run the proven region internally, return final length,
      and drop the guard before returning.
    - no guard/session table, TLS continuity, hidden legality, or public
      begin/end ABI.
- H25c.3 `keeper probe`
  - status: passed as partial keeper
  - timing: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 5 ms`
  - instruction/cycle transition:
    - before H25c.2c: `ny_aot_instr=40330160`, `ny_aot_cycles=12366672`
    - after H25c.2c: `ny_aot_instr=28630426`, `ny_aot_cycles=7033574`
  - target transition: hot path no longer emits the per-iteration
    `string_insert_mid_subrange_len_store_hisi` helper; owner moved into the
    runtime-private region executor and its text mutation/copy body.

## Next Slice

H25c.2c/H25c.3 are closed. The next owner-first slice is H25d:
`region executor inner mutation owner`.

Required order:
1. Re-run `bench_micro_aot_asm.sh` and annotate
   `slot_text_region_update_sum_raw` before editing runtime internals.
2. Fix only the sampled block owner.
3. Keep MIR contract unchanged unless a new legality/materialization fact is
   genuinely required.
4. Reject helper-name or benchmark-name dispatch.

H25d.1 implementation target:

- owner evidence:
  - `slot_text_region_update_sum_raw` is the top sampled symbol after H25c.
  - the runtime region executor still calls the generic
    `ArrayTextSlotSession::update` path for every iteration, even when storage
    is already `ArrayStorage::Text`.
- change scope:
  - make the text-resident region executor loop directly over
    `Text(Vec<String>)` after taking the single write guard
  - keep the existing compatible fallback for boxed/stringlike arrays
  - do not change MIR metadata, `.inc` lowering, public ABI, or helper names
- keeper gate:
  - behavior tests remain green
  - `kilo_meso_substring_concat_array_set_loopcarry` must not regress from the
    H25c partial keeper baseline (`C 3 ms / Ny AOT 5 ms`)

H25d.1 probe:

- result: `ny_aot_instr=24851120`, `ny_aot_cycles=6700078`, `Ny AOT 5 ms`
- verdict: keeper as instruction/cycles reduction, not a new ms keeper
- next owner:
  - `array_string_insert_const_mid_subrange_len_region_store_len` inlined
    mutation closure (`66.75%`)
  - `__memmove_avx512_unaligned_erms` (`18.52%`)

H25d.2 implementation target:

- split `update_insert_const_mid_subrange_len_value` into:
  - hot in-place fixed len-store path
  - cold semantic fallback for generic UTF-8/materialization cases
- keep UTF-8 boundary checks in the hot path; do not assume ASCII without MIR
  proof
- do not change MIR metadata, `.inc` lowering, public ABI, or helper names

H25d.2 probe:

- result: `ny_aot_instr=16570239`, `ny_aot_cycles=3459091`, `Ny AOT 4 ms`
- verdict: keeper; the cold fallback split removed the generic materialization
  body from the active hot path
- next owner:
  - hot mutation closure (`63.01%`)
  - `__memmove_avx512_unaligned_erms` (`23.20%`)

H25d.3 implementation target:

- replace the fixed in-place path's small overlapping shifts with manual byte
  moves for short text cells
- keep `ptr::copy` fallback for larger cells
- preserve UTF-8 boundary checks and keep MIR/ABI unchanged

H25d.3 probe:

- result: `ny_aot_instr=22511003`, `ny_aot_cycles=4765539`, `Ny AOT 4 ms`
- verdict: rejected; manual byte moves increase instruction/cycle count versus
  H25d.2, so keep the existing `ptr::copy` path

H25d.4 implementation target:

- hoist `observe::enabled()` out of the per-iteration region mutation closure
- keep all semantics and MIR/ABI unchanged

H25d.4 probe:

- result: `ny_aot_instr=22510404`, `ny_aot_cycles=4773551`, `Ny AOT 4 ms`
- verdict: rejected; instruction/cycle regression versus H25d.2, reverted

H25d final state:

- accepted code: H25d.1 + H25d.2 only
- final repeated stat: `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 3 ms`,
  `ny_aot_instr=16570267`, `ny_aot_cycles=3471656`
- final asm top:
  - region store mutation closure: `52.65%`
  - `__memmove_avx512_unaligned_erms`: `35.67%`
- H25d.5 closeout:
  - residual `memmove` / mutation surgery is not reopened from the current
    percentage alone
  - H25d.3 manual byte moves and H25d.4 observe hoist both regressed, so
    H25d accepted code remains H25d.1 + H25d.2
- next slice:
  - H26.2 `.inc` metadata validation and one-call emit
  - add `begin_block` / `begin_to_header_block` to MIR-owned
    `executor_contract.region_mapping`; backend placement must not rediscover
    loop entry from raw CFG
  - H26.3 runtime one-call observer-store executor after metadata consume lands
  - keep the region proof under MIR-owned observer metadata
  - do not add source-prefix, source-length, or ASCII assumptions unless MIR
    provides an explicit generic proof

H26.2/H26.3/H26.4 observer-store region executor keeper:

- landed:
  - `executor_contract.region_mapping` now includes `begin_block` and
    `begin_to_header_block`
  - `.inc` preloads the MIR-owned observer-store region before block emission,
    then emits one `nyash.array.string_indexof_suffix_store_region_hisisi`
    call and marks covered blocks unreachable
  - runtime executor holds the array write guard inside one call and performs
    compare-only `indexOf` plus same-slot const suffix store
- verified:
  - whole `kilo_kernel_small`: `C 82 ms / Ny AOT 10 ms`,
    `ny_aot_instr=149657283`, `ny_aot_cycles=31829608`
  - exact `kilo_micro_array_string_store`: `C 10 ms / Ny AOT 3 ms`,
    `ny_aot_instr=9266329`, `ny_aot_cycles=2400782`
  - middle `kilo_meso_substring_concat_array_set_loopcarry`:
    `C 3 ms / Ny AOT 4 ms`,
    `ny_aot_instr=16570773`, `ny_aot_cycles=3435120`
  - whole asm owner refresh:
    `<&str as core::str::pattern::Pattern>::is_contained_in` `35.05%`,
    `__memmove_avx512_unaligned_erms` `23.82%`,
    `nyash.array.string_len_hi` `20.97%`
- next owner:
  - H26 should not reopen source-prefix/source-length/ASCII assumptions without
    MIR proof
  - next probe should decide whether residual search / length observer can be
    represented by generic MIR consumer capability, or whether H26 closes here
    and a new owner-refresh card opens

H25e post-parity owner refresh:

- exact `kilo_micro_array_string_store`:
  - stat: `C 10 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9265624`, `ny_aot_cycles=2385663`
  - asm: `ny_main` `76.27%`; residual libc `memmove` is below 1%
- middle `kilo_meso_substring_concat_array_set_loopcarry`:
  - stat: `C 3 ms / Ny AOT 4 ms`,
    `ny_aot_instr=16570861`, `ny_aot_cycles=3387096`
  - asm: region mutation closure `61.25%`,
    `__memmove_avx512_unaligned_erms` `25.60%`
  - verdict: do not reopen H25d from this percentage alone; H25d.3/H25d.4
    already rejected local copy/observe surgery
- whole `kilo_kernel_small`:
  - stat: `C 81 ms / Ny AOT 20 ms`,
    `ny_aot_instr=232160997`, `ny_aot_cycles=83942461`
  - asm: `memchr::...find_avx2` `34.56%`,
    `with_array_text_write_txn` closure `29.63%`,
    `LocalKey::with` `12.69%`,
    `__memmove_avx512_unaligned_erms` `6.75%`,
    `nyash.array.string_len_hi` `6.33%`
  - MIR already emits one `array_text_observer_routes` entry for
    `array_get_receiver_indexof` with `consumer_shape=found_predicate`,
    `publication_boundary=none`, and const needle `"line"`
  - next code owner: extend that observer route into a MIR-proven
    observer + conditional suffix-store region executor

H26.1 MIR nested observer-store executor contract:

- landed:
  - `array_text_observer_routes` now carries an optional nested
    `executor_contract`
  - whole-front `bench_kilo_kernel_small.hako` emits one contract for `main`
    with `execution_mode=single_region_executor`,
    `effects=[observe.indexof, store.cell]`, const needle `"line"`, and suffix
    `"ln"`
  - route metadata stays under the existing observer route; no benchmark-named
    sibling plan family was added
- structure:
  - observer route detection remains in `src/mir/array_text_observer_plan.rs`
  - nested region proof/detection is isolated in
    `src/mir/array_text_observer_region_contract.rs`
- verified:
  - `cargo test -q array_text_observer_plan::tests::attaches_executor_contract_for_observer_conditional_suffix_store_region -- --nocapture`
  - `cargo run -q --bin hakorune -- --emit-mir-json target/perf_state/h26_kilo_kernel_small_observer_store.mir.json benchmarks/bench_kilo_kernel_small.hako`
  - `cargo check -q`

Reject immediately if the implementation requires:
- runtime deciding session legality from residence state
- `.inc` scanning raw MIR JSON for session shape
- benchmark-specific whole-loop helper
- session across publish/objectize/generic fallback/unknown side-effect boundary
- any session handle table that stores `RwLockWriteGuard` or borrowed slot data
- moving loop semantics into runtime without a MIR-owned region contract

## Validation Anchor

- `cargo check -q`
- `cargo test -q benchmark_meso_substring_concat_array_set_loopcarry_has_len_store_route -- --nocapture`
- `cargo run -q --bin hakorune -- --emit-mir-json target/perf_state/h25_loopcarry.mir.json benchmarks/bench_kilo_meso_substring_concat_array_set_loopcarry.hako`
- H25c.2a substrate:
  - `cargo test -q slot_update_text --lib`
  - `cargo check -q -p nyash_kernel`
- after behavior change:
  - `bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3`
  - `bash tools/perf/bench_micro_aot_asm.sh kilo_meso_substring_concat_array_set_loopcarry '' 20`

## Temporary Counters

H23 split counters are evidence-only and may stay while H25 session work is
active:
- `update_text_resident_hit`
- `update_text_resident_miss`
- `update_text_fallback_hit`
- `update_text_fallback_miss`

Retire them after H25 either lands a session keeper or rejects the session
hypothesis. Do not add a new env var for them.
