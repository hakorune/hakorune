---
Status: SSOT
Date: 2026-04-18
Scope: current lane / blocker / next pointer only.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md
  - docs/development/current/main/phases/phase-137x/README.md
---

# Self Current Task — Now (main)

## Current

- current optimization lane:
  - `phase-137x kernel observability vocabulary + array-store two-stage pilot`
- background compiler lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- blocker:
  - `none`

## Snapshot

- keeper front stays closed:
  - `kilo_micro_substring_concat = C 2 ms / Ny AOT 3 ms`
  - `kilo_micro_substring_only = C 3 ms / Ny AOT 3 ms`
- current broad gap:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 150 ms`
  - `kilo_kernel_small_hk = C 80 ms / Ny AOT 782 ms`
- `indexOf` separation:
  - keep as side diagnosis; reread only when the main card reopens it
- current owner reading:
  - current main owner family is `array/string-store`
  - duplicated `text + "xy"` producer is already removed in trusted direct MIR
  - current exact owner is still publication/source-capture
  - next cut is 2-stage comparison:
    - `Stage A: same protocol .hako pilot`
    - `Stage B: same public ABI, delayed publication seam`

## Next

1. freeze kernel-common observability vocabulary
2. land `carrier_kind` / `publish_reason` in `perf-observe`
3. run `Stage A: same protocol .hako pilot` on narrow `store.array.str`
4. keep `Stage B` separate until Stage A exact numbers exist

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
3. `docs/development/current/main/phases/phase-137x/README.md`
4. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
5. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
6. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
7. `docs/development/current/main/design/string-birth-sink-ssot.md`
8. `docs/development/current/main/15-Workstream-Map.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```
