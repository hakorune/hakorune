---
Status: SSOT
Date: 2026-04-18
Scope: current lane / blocker / next pointer だけを置く薄い mirror。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Self Current Task — Now (main)

## Current

- current optimization lane: `phase-137x trusted direct emit keeper + array-store placement-window reread`
- background compiler lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- immediate next: `prove the duplicated const-suffix producer window on kilo_micro_array_string_store before reopening runtime work`
- immediate follow-on: `keep runtime slot transport parked and move the next cut to compiler-local placement facts unless the placement proof fails`
- top queued cut: `compiler-local producer -> store -> substring window, not new runtime widening`
- current reading:
  - active perf front was blocked by direct-emit route mismatch, not by runtime publication on the keeper lane
  - perf AOT direct emit now matches the trusted phase direct route and restores `kilo_micro_substring_concat` to near-parity (`3 ms`)
  - adjacent exact rereads move the next owner to `kilo_micro_array_string_store` (`174 ms`) and `kilo_leaf_array_string_indexof_const` (`61 ms`)
  - latest landed follow-on is `93e390455 refactor: split array string-store perf seams`
  - `perf-observe` on `kilo_micro_array_string_store` still reads publication/capture first:
    - `freeze_owned_bytes`
    - `issue_fresh_handle`
    - `StringBox::perf_observe_from_owned`
    - `capture_store_array_str_source`
  - trusted direct MIR still duplicates `text + "xy"` once for `set(...)` and once for trailing `substring(...)`
  - widening `const_suffix` beyond `direct_set` did not move exact or whole and was rejected
  - public ABI / legality ownership stays unchanged; the landed slot seam remains background structure, not the current blocker

## Landing Snapshot

- latest landed:
  - `phase-137x`: publication-boundary keeper plus `perf-observe` measurement seams for the hot `piecewise` corridor and the next `store.array.str` reread
- active:
  - `phase-137x`: compiler-local placement proof under `value-first / box-on-demand / publish-last`
  - blocker=`none`
- detail owner:
  - current truth and restart order live in `CURRENT_TASK.md`
  - detailed evidence and rejects stay in `docs/development/current/main/phases/phase-137x/README.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-137x/README.md`
3. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
4. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
5. `docs/development/current/main/15-Workstream-Map.md`
6. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md` (`phase-29bq` に戻るときだけ)

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```
