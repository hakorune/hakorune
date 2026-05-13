# 293x-230 C194 Verifier-Owned Allocation Invariants

Status: Complete

## Purpose

C194 moves the allocator metadata invariants introduced by the C210/C211 packed
store pilots from proof-only expectations into MIR verifier-owned contracts.
This keeps future allocator rows from consuming malformed packed metadata rows
or silently drifting away from the record layouts that produced them.

## Decision

Decision: accepted.

Add a narrow verifier owner:

```text
mir::verification::hako_alloc_metadata
```

The owner checks only MIR metadata contracts for the current hako_alloc
metadata rows. It does not inspect live `.hako` algorithms, run allocation
behavior, or enable packed record backend lowering.

## Row Contract

C194 verifies:

```text
aligned-small metadata:
  source C209 packed pilot exists
  record owner is HakoAllocAlignedSmallMeta / HakoAllocAlignedSmallMetaStore
  column order is ptr=0, alignment=1, padded_size=2
  backing record layout exists and uses integer lanes
  private runtime storage remains enabled
  hako_alloc source compiler vocabulary remains absent
  public record materialization remains disabled

huge-page metadata:
  source C209 packed pilot exists
  record owner is HakoAllocHugePageMeta / HakoAllocHugePageMetaStore
  column order is page_id=0, ptr=1, requested_size=2, committed_size=3, live=4
  backing record layout exists and uses integer lanes
  released sentinels stay page_id=-1 and size=0
  private runtime storage remains enabled
  hako_alloc source compiler vocabulary remains absent
  public record materialization remains disabled
```

## Stop Lines

- Do not rewrite hako_alloc metadata stores.
- Do not add packed backend lowering.
- Do not add public record materialization.
- Do not broaden the MIR verifier into allocator algorithm proof.
- Do not touch provider activation, hooks, process allocator replacement, or
  `.inc` allocator/provider/mimalloc matching.

## Acceptance

- `MirVerifier::verify_module(...)` runs the C194 metadata invariant checker.
- Valid C210/C211 metadata rows pass.
- Missing source pilot, malformed column order, bad released sentinel, and
  visible materialization rows produce stable verifier errors.
- C194 guard stays local-run / index-listed and is not added to quick/dev gates.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_metadata_verifier_invariants_guard.sh
cargo test -q hako_alloc_metadata
```
