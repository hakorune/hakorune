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
  - `phase-137x Stage A exact reread closed + active AOT route diagnosis`
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
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 153 ms`
  - `kilo_kernel_small_hk = C 85 ms / Ny AOT 786 ms`
- `indexOf` separation:
  - keep as side diagnosis; reread only when the main card reopens it
- current owner reading:
  - current main owner family is `array/string-store`
  - duplicated `text + "xy"` producer is already removed in trusted direct MIR
  - current exact owner is still publication/source-capture
  - `Stage A` narrow owner slice is now landed on the VM/reference lane:
    - `.hako` `ArrayCoreBox` routes proven string-handle `set(...)` through `nyash.array.set_his`
    - same protocol, same cold Rust tail
  - `Stage A` exact reread is now closed on the active AOT front:
    - `store.array.str total=800000`
    - `plan.action_retarget_alias=800000`
    - `plan.action_store_from_source=0`
    - `carrier_kind.source_keep=0`
    - `publish_reason.generic_fallback=1600000`
  - trusted direct MIR still carries generic `RuntimeDataBox.set(...)` / `substring(...)`
  - active AOT exact is therefore not yet the `.hako` owner pilot itself

## Next

1. close whether active AOT can legally select the Stage A owner seam for `store.array.str`
2. keep exact rereads pinned on publication/source-capture while that route question is open
3. keep `Stage B` separate until the active AOT route question is closed

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
