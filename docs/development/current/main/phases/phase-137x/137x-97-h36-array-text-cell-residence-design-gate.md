# 137x-H36 ArrayTextCell Residence Design Gate

Status: active design gate; H36.1 landed, H36.2 decision active.

Current blocker token:
`137x-H36.2 ArrayTextCell residence decision`.

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
