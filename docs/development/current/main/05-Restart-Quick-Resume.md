---
Status: Active
Date: 2026-04-18
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md
  - docs/development/current/main/phases/phase-137x/README.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
```

## Current

- lane:
  - `phase-137x kernel observability vocabulary + array-store two-stage pilot`
- blocker:
  - `none`
- worktree:
  - dirty is expected; do not reset unrelated compiler-lane diffs just to make the tree clean
- current snapshot:
  - `kilo_micro_substring_concat = C 2 ms / Ny AOT 3 ms`
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 150 ms`
  - `kilo_kernel_small_hk = C 80 ms / Ny AOT 782 ms`
- immediate next:
  - `freeze kernel-common observability vocabulary and land carrier_kind / publish_reason`
- immediate follow-on:
  - `Stage A: same protocol .hako pilot on store.array.str`

## Current Handoff

- current broad owner family is `array/string-store`
- duplicated producer is already fixed in trusted direct MIR; runtime publication/source-capture stayed hot
- `indexOf` stays a side diagnosis, not the active keeper card
- keep public ABI / legality ownership unchanged
- compare `.hako` only under:
  - `Stage A: same protocol`
  - `Stage B: same public ABI / different seam`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
4. `docs/development/current/main/phases/phase-137x/README.md`
5. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
6. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
7. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
8. `docs/development/current/main/design/string-birth-sink-ssot.md`
9. `docs/development/current/main/15-Workstream-Map.md`
10. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md` (`phase-29bq` に戻るときだけ)

## Current Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```
