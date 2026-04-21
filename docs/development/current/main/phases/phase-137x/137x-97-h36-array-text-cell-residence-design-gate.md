# 137x-H36 ArrayTextCell Residence Design Gate

Status: active design gate; H36.4 piece residence pilot rejected.

Current blocker token:
`137x-H39.4 combined edit-observer region executor`.

## Context

Post-H34 evidence moved the remaining whole-front owner to flat text residence
copy cost:

- `__memmove_avx512_unaligned_erms`: `48.59%`
- len-half edit closure:
  `array_string_insert_const_mid_lenhalf_by_index_store_same_slot_str::{closure}`:
  `26.13%`
- observer-store closure: `16.08%`

H29 already rejected another flat `String::insert_str` / byte-copy bypass. H36
therefore must not repeat flat string surgery. The next clean question is
whether `ArrayTextCell` can host a non-flat residence representation without
leaking representation semantics into MIR, `.inc`, or public Array behavior.

## Owner Boundary

- MIR owns legality, provenance, publication boundary, and route/session
  contracts.
- `.inc` consumes MIR metadata and emits calls only.
- Rust runtime owns `ArrayTextCell` residence mechanics.
- `ArrayTextCell` must remain an internal array text residence cell, not a
  public semantic value.

## Inventory

Current `ArrayTextCell` is flat-only:

- `ArrayTextCell::Flat(String)`
- `as_str(&self) -> &str`
- `as_mut_string(&mut self) -> &mut String`
- `into_string(self) -> String`
- operation leaf: `insert_const_mid_lenhalf`

The non-flat blocker is API shape, not enum syntax:

- `as_str(&self) -> &str` cannot return a borrowed contiguous view for a
  piece/gap representation without hidden materialization.
- `as_mut_string(&mut self) -> &mut String` forces every mutation back to flat
  string storage.
- visible Array paths use `as_str` for `get`, equality, formatting, membership,
  sorting, and boxing.
- hot runtime paths still use `as_mut_string` in generic session updates and
  region executors.

## Decision

Do not add `ArrayTextCell::Piece` or `ArrayTextCell::Gap` yet.

First land a behavior-preserving API split:

- replace hot-path direct `as_mut_string` use with operation methods on
  `ArrayTextCell`
- keep visible/materializing paths explicit via `to_string` / `into_string`
- keep temporary borrowed text reads behind a closure API that can later return
  either borrowed flat text or a transient materialized string
- keep the first split flat-only and test-backed

This is BoxShape work. It must not add a new accepted MIR shape.

## H36.1 Implementation Card

Name: `ArrayTextCell operation API split`.

Allowed:

- add flat-only methods such as:
  - `contains_literal(&self, needle: &str) -> bool`
  - `append_suffix(&mut self, suffix: &str)`
  - `len(&self) -> usize`
  - existing `insert_const_mid_lenhalf(&mut self, middle: &str) -> i64`
- replace `value.as_str()` / `value.as_mut_string()` in array text hot paths
  with those operation methods where the caller does not need public text
  materialization
- keep visible paths using explicit materialization helpers

Forbidden:

- adding `Piece` / `Gap` in H36.1
- changing MIR metadata or `.inc` emit shape
- source-content assumptions such as rows always containing `"line"`
- search-result cache
- changing public Array identity or equality semantics

Acceptance:

- `cargo fmt --check`
- `cargo test -q array::tests --lib`
- `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`
- `tools/checks/current_state_pointer_guard.sh`
- no perf keeper claim for H36.1; it is a structural precondition

## Later H36.2 Gate

Only after H36.1 is green, decide one of:

- open a narrow non-flat residence pilot behind `ArrayTextCell`
- reject non-flat residence for this lane and hand residual `memmove` to a
  later TextCell / allocator lane

The H36.2 keeper gate must include fresh whole stat/asm and rollback notes.

## H36.1 Result

Landed as a BoxShape-only split:

- `ArrayTextCell::{contains_literal, append_suffix}` own the hot text
  operations for flat text cells.
- fallback string updates use `ArrayTextCell` string leaf wrappers instead of
  duplicating short literal / short suffix helpers in `ops/text.rs`.
- `ops/text.rs` now calls the cell boundary; no MIR, `.inc`, public ABI, or
  representation variant changed.
- verification: `cargo fmt --check`, `git diff --check`,
  `cargo test -q array::tests --lib`,
  `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`, and
  `tools/checks/current_state_pointer_guard.sh`.

Next: H36.2 must refresh `kilo_kernel_small` whole stat/asm from a rebuilt
release artifact before any representation implementation.

## H36.2 Result

Fresh owner proof after H36.1:

- release artifacts rebuilt with `tools/perf/build_perf_release.sh`
- `kilo_kernel_small = C 82 ms / Ny AOT 7 ms`
- `ny_aot_instr=50229407`, `ny_aot_cycles=16401030`
- asm top:
  - `__memmove_avx512_unaligned_erms`: `38.15%`
  - len-half edit closure: `33.22%`
  - observer-store closure: `20.21%`

Verdict:

- non-flat `ArrayTextCell` residence remains justified.
- do not add `Piece` / `Gap` directly after H36.2.
- first land H36.3: visible materialization/comparison API split, because
  current visible `as_str()` and derived ordering would leak flat storage
  assumptions into a non-flat cell.

## H36.3 Implementation Card

Name: `ArrayTextCell visible materialization split`.

Allowed:

- add flat-only helpers such as:
  - `to_visible_string(&self) -> String`
  - `equals_text(&self, needle: &str) -> bool`
  - `cmp_text(&self, other: &Self) -> Ordering`
  - `with_text(&self, f: impl FnOnce(&str) -> R) -> R`
- route visible Array get/boxing/format/equality/membership/sort paths through
  those helpers.

Forbidden:

- `Piece` / `Gap` variants
- MIR or `.inc` edits
- public ABI changes
- perf keeper claim

Acceptance:

- `cargo fmt --check`
- `git diff --check`
- `cargo test -q array::tests --lib`
- `tools/checks/current_state_pointer_guard.sh`

## H36.3 Result

Landed as BoxShape-only cleanup:

- visible Array get/boxing/format/equality/membership/sort paths route through
  `ArrayTextCell` materialization/comparison helpers.
- raw borrowed `as_str()` and derived cell ordering no longer define visible
  Array behavior outside the cell boundary.
- no `Piece` / `Gap`, MIR, `.inc`, public ABI, or perf keeper claim.
- verification: `cargo fmt --check`, `git diff --check`,
  `cargo test -q array::tests --lib`,
  `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`, and
  `tools/checks/current_state_pointer_guard.sh`.

## H36.4 Implementation Card

Name: `ArrayTextCell piece residence pilot`.

Allowed:

- add a narrow runtime-private non-flat cell variant for repeated len-half
  insert mechanics.
- keep `len`, `contains_literal`, `append_suffix`, `insert_const_mid_lenhalf`,
  and visible materialization inside `ArrayTextCell`.
- use storage facts only; do not depend on benchmark names or source content.

Forbidden:

- MIR or `.inc` edits.
- public ABI changes.
- semantic/search-result cache.
- source-content assumptions such as every row containing `"line"`.

Keeper gate:

- behavior tests stay green.
- release artifacts are rebuilt before measuring.
- whole `kilo_kernel_small` improves and `memmove` / len-half closure share
  shrinks without simply moving the owner to allocator.

## H36.4 Result

Rejected.

- behavior gates were green.
- release artifacts were rebuilt before measuring.
- whole `kilo_kernel_small = C 85 ms / Ny AOT 114 ms`.
- `ny_aot_instr=2084599541`, `ny_aot_cycles=521801542`.
- code was reverted.

Verdict:

- naive piece vectors are not a keeper; they cause work explosion.
- do not reopen non-flat residence without a bounded piece/gap proof.
- H37 must refresh whole owner evidence from the reverted H36.3 code state.

## H37 Result

Reverted-code owner refresh:

- release artifacts rebuilt from the reverted H36.3 code state.
- `kilo_kernel_small = C 82 ms / Ny AOT 7 ms`.
- `ny_aot_instr=50229360`, `ny_aot_cycles=16404095`.
- asm top:
  - `__memmove_avx512_unaligned_erms`: `49.02%`
  - len-half edit closure: `22.74%`
  - observer-store closure: `18.88%`
  - `_int_realloc`: `0.90%`

Verdict:

- flat len-half movement remains the top owner.
- allocator is not dominant.
- next work is H38 bounded gap / edit-buffer design, docs-first. No code until
  rollback, materialization, contains, append, and cap/compaction rules are
  fixed.

## H38 Design

Name: `ArrayTextCell bounded mid-gap`.

Representation:

- private variant only: logical text is `left + right[right_start..]`.
- `left` owns the prefix up to the edit boundary.
- `right_start` marks the consumed prefix of `right`; moving the edit boundary
  right increments an offset instead of draining or memmoving the active right
  tail.
- suffix append writes to `right`'s end.

Allowed operations:

- `len` is `left.len() + right[right_start..].len()`.
- len-half insert:
  - computes the MIR-owned split policy result locally from current byte len.
  - if the split is inside `right`, moves only the small prefix bytes into
    `left` and advances `right_start`.
  - if the split is inside `left`, inserts there and caps left-side overshoot
    with an explicit rebalance.
- `contains_literal` checks `left`, the active right tail, and the single
  boundary crossing without materializing the full text.
- `append_suffix` appends to `right`.
- visible Array boundaries materialize explicitly through
  `to_visible_string` / `into_string` / `with_text`.

Rollback / fallback:

- invalid UTF-8 byte-boundary splits fall back to the existing flat
  materialization behavior.
- generic `&mut String` update APIs may materialize the cell explicitly; hot
  H38.1 paths must stay on cell operations.

Cap / compaction:

- the consumed right prefix is compacted only when it is both large and larger
  than the active right tail.
- left-side overshoot is capped by rebalancing from explicit materialization,
  preventing a no-append workload from growing hidden per-edit movement
  without bound.

Forbidden:

- MIR or `.inc` changes.
- public ABI changes.
- benchmark-name or source-content branches.
- semantic/search-result cache.
- unbounded piece vectors.

H38 verdict:

- design is sufficient to open H38.1 code.
- keeper still requires fresh whole stat/asm after a rebuilt release artifact.

## H38.1 Result

Implementation:

- added the private `ArrayTextCell::MidGap { left, right, right_start }`
  variant.
- kept MIR, `.inc`, public ABI, benchmark names, and semantic/search-result
  cache untouched.
- `contains_literal`, `append_suffix`, len-half insert, and visible
  materialization stay inside `ArrayTextCell`.

Verification:

- `cargo fmt --check`
- `git diff --check`
- `tools/checks/current_state_pointer_guard.sh`
- `cargo test -q array::text_cell --lib`
- `cargo test -q array::tests --lib`
- `cargo test -q -p nyash_kernel insert_mid_store_by_index --lib`
- release artifacts rebuilt with `tools/perf/build_perf_release.sh`

Perf:

- whole `kilo_kernel_small = C 83 ms / Ny AOT 6 ms`.
- `ny_aot_instr=60923714`, `ny_aot_cycles=12531473`.
- asm top:
  - len-half edit closure: `49.27%`
  - observer-store closure: `41.58%`
  - `nyash.array.string_insert_mid_lenhalf_store_hisi`: `1.49%`
  - `nyash.array.string_indexof_suffix_store_region_hisisi`: `1.46%`
  - `__memmove_avx512_unaligned_erms`: `0.23%`
- contradiction guards:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 3 ms`,
    `ny_aot_instr=9266540`, `ny_aot_cycles=2423297`
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 4 ms`,
    `ny_aot_instr=17650827`, `ny_aot_cycles=4300117`

Verdict:

- H38.1 is an owner-moving keeper: the dominant `memmove` seam is removed and
  whole wall/cycles improve.
- keep an instruction-count watch because whole instruction count increased
  versus H37.
- next card is H39 post-mid-gap closure owner refresh.

## H39 Result

Focused annotation after H38.1:

- len-half edit closure:
  - top local sample is the write-lock acquire path:
    `lock cmpxchg` at local `62.33%`.
  - this is not a text-copy owner.
- observer-store closure:
  - write-lock acquire is local `2.60%`.
  - samples are in the text-cell loop / short-literal / MidGap segment checks.

Verdict:

- do not reopen representation work immediately.
- next design must split two seams:
  - outer edit lock-boundary: needs a MIR-proven region boundary if pursued.
  - observer-store cell-loop: may be runtime-only only if it preserves
    generic literal semantics and does not become a search-result cache.

## H39.1 Implementation Card

Name: `MidGap generic prefix fast path`.

Decision:

- run the observer-store cell-loop probe before the heavier outer edit
  lock-boundary design.
- keep the probe runtime-only inside `ArrayTextCell`.

Allowed:

- check a generic prefix literal hit before the full MidGap segmented search.
- keep behavior identical to `str::contains`.

Forbidden:

- source-content assumptions such as the literal being `"line"`.
- search-result cache.
- MIR, `.inc`, or public ABI changes.

Reject if:

- observer-store closure does not shrink.
- exact or middle guards regress.

## H39.1 Result

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

- whole `kilo_kernel_small = C 83 ms / Ny AOT 6 ms`.
- `ny_aot_instr=60443810`, `ny_aot_cycles=11322220`.
- asm top:
  - observer-store closure: `51.21%`
  - len-half edit closure: `30.53%`
  - `__memmove_avx512_unaligned_erms`: `4.62%`
- guards:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`,
    `ny_aot_instr=9266628`, `ny_aot_cycles=2432139`
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 3 ms`,
    `ny_aot_instr=17651373`, `ny_aot_cycles=4229069`

Verdict:

- small keeper: whole cycles improve from H38.1's `12531473` to `11322220`.
- outer edit lock-boundary remains the next structural seam.

## H46 design card - bounded bridge/spill on top of MidGap

Goal:

- keep the next slice inside `ArrayTextCell` BoxShape only, so the combined
  executor spends longer on text-resident edits instead of bouncing through
  visible flat-string materialization and overlap shifts.

Shape:

```rust
enum ArrayTextCell {
    Flat(String),
    MidGap {
        left: String,
        bridge: String,
        right: String,
        right_start: usize,
    },
}
```

- logical visible text is `left + bridge + right[right_start..]`
- `bridge` is a bounded spill segment near the edit center, not an unbounded
  piece vector
- the H38/H38.1 bounded-residence principle stays intact: no new public shape,
  no new MIR contract, no planner logic in `.inc`

Why this seam:

- H45 pinned the residual `__memmove` owner to a broad `ArrayTextCell`
  mid-insert / flat fallback / materialization family, not to a fresh narrow
  suffix or left-copy leaf
- current `MidGap` keeps right-oriented append cheap, but when the split walks
  back into the left side it tends to rebuild or flatten more aggressively
- a bounded `bridge` gives the cell one extra resident spill slot, so left-side
  returns can stay inside the cell before crossing the explicit visible
  materialization boundary

Invariants:

- `bridge.len() <= ARRAY_TEXT_CELL_BRIDGE_LIMIT` at all times; the initial H46
  slice uses a small fixed bound and rejects unbounded segmentation
- visible/public text still materializes only through
  `to_visible_string()`, `with_text()`, and `into_string()`
- runtime remains mechanics only; legality/provenance still belongs to MIR
  metadata owners and `.inc` remains emit only
- append stays right-oriented; `bridge` exists to absorb left-return spill, not
  to create a generic piece framework
- if a byte-boundary-safe edit cannot keep the bound or preserve internal
  invariants cheaply, the cell falls back to `Flat(String)` explicitly

Boundary behavior:

- `contains_literal` must check across `left|bridge`, `bridge|right_visible`,
  and the fully resident segments, without changing public semantics
- repeated byte-boundary-safe len-half inserts may spill resident bytes into
  `bridge` instead of forcing an immediate full materialization
- `append_suffix` should continue to prefer the active right side; it must not
  reopen generic session materialization as the primary hot path

Reject seams:

- no MIR metadata widening
- no `.inc` planner regression
- no runtime legality/provenance inference
- no benchmark-name or source-content assumptions
- no unbounded piece/piece-vector expansion
- no reopening suffix / left-copy micro leaves

## H46.1 first implementation slice

Scope:

- implement the bounded bridge/spill shape only for
  `insert_const_mid_lenhalf_byte_boundary_safe`
- keep the generic non-byte-safe path and `as_mut_string()` compatibility path
  unchanged for now
- keep code changes local to `ArrayTextCell` plus focused tests

Keeper gate:

- structure:
  - `src/mir/*`, `.inc` emit logic, public ABI aliases, and visible Array
    behavior stay untouched
- behavior:
  - repeated byte-boundary-safe len-half insert stays result-equivalent to flat
    string reference behavior
  - contains works across `left|bridge|right`
  - append after repeated inserts preserves current visible output
  - explicit flatten/materialization entry points still produce identical
    strings
- perf:
  - compare against the H45 baseline on `kilo_kernel_small`
  - accept only if `__memmove` share and/or the broad combined owner family
    shrink without exact/middle regressions

Reject gate:

- if cost merely migrates from `__memmove` into another child of the same broad
  owner family, reject
- if the bounded bridge triggers H36.4-style work explosion, reject

### H46.1 probe result - rejected

- implementation attempt: bounded `MidGap + bridge` on
  `insert_const_mid_lenhalf_byte_boundary_safe`
- result on `kilo_kernel_small`:
  - `ny_aot_instr=142651499`
  - `ny_aot_cycles=90126830`
  - `Ny AOT 22 ms`
- perf top moved deeper into libc instead of narrowing the owner:
  - `__memmove_avx512_unaligned_erms 54.59%`
  - `_int_malloc 21.74%`
- decision:
  - reject and revert the code slice
  - keep H46 open, but do not reopen this bounded bridge card without a fresh
    source-pinned reason that avoids bridge-local shift/allocation churn
