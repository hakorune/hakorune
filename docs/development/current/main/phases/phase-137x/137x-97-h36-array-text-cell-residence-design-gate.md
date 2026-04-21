# 137x-H36 ArrayTextCell Residence Design Gate

Status: active design gate; H36.4 piece residence pilot rejected.

Current blocker token:
`137x-H38.1 bounded mid-gap residence pilot`.

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
