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

- current optimization lane: `phase-137x runtime-executor corridor-local slot transport`
- background compiler lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- immediate next: `slot-kept until first true external boundary A/B on kilo_micro_substring_concat`
- immediate follow-on: `if the exact front wins and whole-kilo stays neutral, promote the corridor-local publish-last executor path and only then reopen llvm-export follow-on`
- top queued cut: `publication tail removal without generic widening`
- current reading:
  - active exact front is already 100% on the landed `piecewise_subrange_hsiii` fast path
  - dominant cost is `publish -> objectize -> fresh handle issue`, not `materialize_piecewise_all_three`
  - `with_text_read_session_ready` / TLS entry is still visible, but it is secondary
  - public ABI stays handle-based; unpublished slot transport stays runtime-private and corridor-local

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
