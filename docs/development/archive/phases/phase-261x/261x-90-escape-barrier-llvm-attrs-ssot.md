# Phase 261x — Escape / Barrier -> LLVM Attrs SSOT

Status: ACTIVE
Phase: 261x

## Decision

1. The first attrs feed is a builder-finalization seam, not per-call-site attribute threading.
2. The first cut is conservative:
   - pure read-only query helpers may get `readonly`
   - pointer arguments on known runtime bridges may get `nocapture`
3. MIR escape/barrier vocabulary stays the source of future widening decisions, but this cut does not require a new MIR JSON feed yet.

## Scope

- `readonly` targets:
  - `nyash.string.len_h`
  - `nyash.string.charCodeAt_h`
  - `nyash.string.eq_hh`
  - `nyash.string.indexOf_hh`
  - `nyash.string.lastIndexOf_hh`
  - `nyash.integer.get_h`
  - `nyash.bool.get_h`
  - `nyash.float.unbox_to_f64`
  - `nyash.any.length_h`
- `nocapture` pointer arguments:
  - `nyash.console.log`
  - `nyash.console.warn`
  - `nyash.console.error`
  - `nyash.string.concat_ss`
  - `nyash.string.concat_si`
  - `nyash.string.concat_is`
  - `nyash.string.substring_sii`
  - `nyash.string.indexOf_ss`
  - `nyash.string.lastIndexOf_ss`
  - `nyash.box.from_i8_string`
  - `nyrt_string_length`

## Out of Scope

- no `readnone` promotion yet
- no `noalias` widening yet
- no MIR-side escape fact export yet
- no per-instruction ad-hoc attrs in lowering helpers

