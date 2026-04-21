# 137x-H36 ArrayTextCell Residence Design Gate

Status: active design gate; H36.2 closed, H36.3 visible materialization split active.

Current blocker token:
`137x-H36.3 ArrayTextCell visible materialization split`.

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
