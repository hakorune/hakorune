---
Status: Active
Date: 2026-04-18
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/10-Now.md
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

- lane: `phase-137x runtime-executor corridor-local slot transport`
- background lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- immediate next: `slot-kept until first true external boundary A/B on kilo_micro_substring_concat`
- immediate follow-on: `promote the corridor-local publish-last executor path only if exact front wins and whole-kilo stays neutral`

## Current Handoff

- blocker: `none`
- worktree: dirty is expected; do not reset unrelated compiler-lane diffs just to make the tree clean
- live front:
  - `kilo_micro_substring_concat`
  - accept gate=`kilo_micro_substring_only`
  - whole guard=`kilo_kernel_small_hk`
- live reading:
  - route selection is no longer the blocker; the active front is already on the landed single-session `piecewise_subrange_hsiii` path
  - dominant cost is publication tail: `publish_kernel_text_slot_boundary` / `StringBox` / fresh `handle_issue`
  - `materialize_piecewise_all_three` is measured as secondary on this front
  - `with_text_read_session_ready` / TLS entry is still visible, but it is the second hotspot
- next exact handoff:
  - keep public ABI handle-based
  - keep `proof_region` / `publication_boundary` legality MIR-owned
  - prove `slot-kept until first true external boundary` only on the active corridor
  - keep slot transport corridor-local; the cold publish adapter alone owns `StringBox` / `Arc` / handle issue
  - do not widen this card into a generic slot API, registry carrier, or remembered chain path

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-137x/README.md`
3. `docs/development/current/main/10-Now.md`
4. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
5. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
6. `docs/development/current/main/15-Workstream-Map.md`
7. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md` (`phase-29bq` に戻るときだけ)

## Current Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```
