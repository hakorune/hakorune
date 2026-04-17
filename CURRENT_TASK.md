# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-18
Scope: current lane / next lane / restart order only.

## Purpose

- root から active lane / next lane に最短で戻る
- landed history と rejected history は phase docs / investigations を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
4. `docs/development/current/main/phases/phase-137x/README.md`
5. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
6. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
7. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
8. `docs/development/current/main/design/string-birth-sink-ssot.md`
9. `docs/development/current/main/15-Workstream-Map.md`
10. `git status -sb`
11. `tools/checks/dev_gate.sh quick`
12. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md` (`phase-29bq` に戻るときだけ)

## Current Lane

- expected worktree:
  - dirty is expected right now; do not reset unrelated changes just to make the tree look clean
  - unrelated dirty file is currently `crates/nyash_kernel/src/observe/sink/stderr.rs`
- active lane:
  - `phase-137x Stage A same-protocol array-store pilot + exact reread`
- background lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- current blocker:
  - `none`

## Current Snapshot

- keeper front is still closed:
  - `kilo_micro_substring_concat`
    - `C: 2 ms`
    - `Ny AOT: 3 ms`
  - `kilo_micro_substring_only`
    - `C: 3 ms`
    - `Ny AOT: 3 ms`
- current broad gap is no longer substring:
  - `kilo_micro_array_string_store`
    - `C: 10 ms`
    - `Ny AOT: 150 ms`
  - `kilo_kernel_small_hk`
    - `C: 80 ms`
    - `Ny AOT: 782 ms`
- current reading:
  - current main owner family is `array/string-store`, not `substring`
  - trusted direct MIR no longer duplicates the `text + "xy"` producer across `set(...)` and trailing `substring(...)`
  - runtime gap stayed open after the compiler-side placement fix, so duplicated birth is no longer the live owner
  - `Stage A` narrow owner slice is landed on the VM/reference lane:
    - `.hako` `ArrayCoreBox` now routes proven string-handle `set(...)` through `nyash.array.set_his`
    - cold tail stays in Rust
  - current exact `perf-observe` on `kilo_micro_array_string_store` still ranks publication/capture first:
    - `freeze_owned_bytes`
    - `issue_fresh_handle`
    - `capture_store_array_str_source`
    - `StringBox::perf_observe_from_owned`
    - `execute_store_array_str_slot_boundary`
  - next comparison must split:
    - implementation language cost
    - protocol / seam cost
  - `indexOf` stays a side diagnostic lane and is not the current keeper card

## Next

1. run `Stage A` exact reread on `kilo_micro_array_string_store`
2. compare the new `carrier_kind` / `publish_reason` counters against the Rust lane
3. keep `Stage B: delayed publication seam` separate until Stage A exact numbers exist

## Guardrails

- MIR/lowering still owns legality, `proof_region`, and `publication_boundary`
- keep carrier/publication split physically narrow
- do not widen this card into a generic slot API or helper substrate
- keep public ABI stable
- compare `Rust vs .hako` only under:
  - same protocol
  - same public ABI with different internal seam

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```

## Detail Pointers

- current evidence snapshot:
  - `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
- history / rejects / longer ledger:
  - `docs/development/current/main/phases/phase-137x/README.md`
- design anchors:
  - `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
  - `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
  - `docs/development/current/main/design/string-birth-sink-ssot.md`
