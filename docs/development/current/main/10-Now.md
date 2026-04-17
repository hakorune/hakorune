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

- current optimization lane: `phase-137x trusted direct emit alignment keeper`
- background compiler lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- immediate next: `re-read adjacent exact fronts on the trusted direct emit lane and decide whether runtime-executor slot transport stays active or parks as background proof`
- immediate follow-on: `if neighboring exact fronts stay green and whole-kilo stays neutral, promote the trusted direct emit keeper and only then reopen follow-on cuts`
- top queued cut: `direct-route confirmation before any new runtime widening`
- current reading:
  - active perf front was blocked by direct-emit route mismatch, not by runtime publication on the keeper lane
  - perf AOT direct emit now matches the trusted phase direct route and restores `kilo_micro_substring_concat` to near-parity (`3 ms`)
  - accept gate and whole guard stay green (`kilo_micro_substring_only = 3 ms`, `kilo_kernel_small_hk = 703 ms`)
  - public ABI / legality ownership stays unchanged; the landed slot seam remains background structure, not the current blocker

## Landing Snapshot

- latest landed:
  - `phase-137x`: publication-boundary keeper plus `perf-observe` measurement seam for the hot `piecewise` corridor
- active:
  - `phase-137x`: runtime-executor follow-on under `value-first / box-on-demand / publish-last`
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
