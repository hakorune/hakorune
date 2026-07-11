# 170x-90: direct-kernel substring plan proof SSOT

Status: SSOT
Date: 2026-04-12
Scope: make boundary `pure-first` lower helper-result `substring()` from MIR `direct_kernel_entry.plan.proof` instead of remembered concat-chain side state.

## Goal

- keep canonical MIR as the only semantic owner
- keep `StringCorridorCandidate.plan.proof` as the proof surface
- let boundary `substring()` consume the same proof-bearing lane that `length()` already uses
- avoid another remembered/helper-name bridge on the current direct-helper route

## Diagnosis

Current string bridge state was asymmetric:

- `length()` on helper-result receivers already read:
  - `direct_kernel_entry.plan.start/end`
- `substring()` on the same receivers still read:
  - `resolve_string_concat_chain_terms(...)`
  - `remember_string_concat_*`

That meant the same concat-triplet truth existed twice:

1. canonical MIR candidate proof
2. boundary-local remembered concat-chain state

The second source is bridge-only and should shrink when the first already exists.

## Fix

### 1. Widen the proof payload, not the public IR

`StringCorridorCandidateProof::ConcatTriplet` now also carries:

- `left_value`
- `right_value`

These are carrier value ids for the existing left/right triplet pieces.
The existing source/window/shared-source fields stay intact because sink/relation code still uses them.

### 2. Emit the same proof in MIR JSON

`src/runner/mir_json_emit/mod.rs` must export:

- `left_value`
- `middle`
- `right_value`

under `plan.proof.kind = concat_triplet`.

### 3. Add a boundary reader

Boundary lowering adds a narrow reader for:

- `direct_kernel_entry`
- `plan.proof.kind = concat_triplet`
- piece carrier values

That reader is allowed to feed only the existing direct substring helper route:

- `nyash.string.substring_concat3_hhhii`

### 4. Keep old remembered concat-chain as compatibility fallback

If the plan-proof payload is absent, boundary lowering still falls back to:

- `resolve_string_concat_chain_terms(...)`

This cut shrinks the bridge on the proven lane first; it does not delete compatibility memory all at once.

## Acceptance

- helper-result `substring()` on a `direct_kernel_entry` fixture lowers through:
  - `direct_kernel_plan_proof`
  - `substring_concat3_hhhii`
- the same fixture does not lower consumer `%r14` through:
  - `nyash.string.substring_hii`
  - `nyash.string.insert_hsi`
- existing direct-kernel `length()` proof stays green
- live direct emit + exact asm/perf + `quick` gate stay green

## Non-Goals

- no sink rewrite change
- no `phi_merge` carry widening
- no host-boundary publication wave
- no runtime helper retuning
