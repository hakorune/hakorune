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

- lane: `phase-137x trusted direct emit alignment keeper`
- background lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- immediate next: `re-read adjacent exact fronts on the trusted direct emit lane and decide whether runtime-executor slot transport stays active or parks as background proof`
- immediate follow-on: `promote the trusted direct emit keeper only if neighboring exact fronts stay green and whole-kilo remains neutral`

## Current Handoff

- blocker: `none`
- worktree: dirty is expected; do not reset unrelated compiler-lane diffs just to make the tree clean
- live front:
  - `kilo_micro_substring_concat`
  - accept gate=`kilo_micro_substring_only`
  - whole guard=`kilo_kernel_small_hk`
- live reading:
  - the active perf front was blocked by a direct-emit route mismatch, not by the runtime-publication tail on the keeper lane
  - perf AOT direct emit now matches the trusted phase direct route and emits the proof-bearing `substring_concat3_hhhii` payload
  - `kilo_micro_substring_concat` is now back to `Ny AOT: 1,665,250 instr / 983,016 cycles / 3 ms`
  - accept gate stays green: `kilo_micro_substring_only = 1,669,421 instr / 960,357 cycles / 3 ms`
  - whole guard stays neutral: `kilo_kernel_small_hk = 703 ms`
- next exact handoff:
  - keep perf direct emit aligned with the trusted stage1 route
  - re-run neighboring exact fronts before reopening runtime-executor work
  - keep public ABI / `proof_region` / `publication_boundary` ownership unchanged
  - keep the landed slot seam parked as background structure; do not widen it into a generic slot API, registry carrier, or remembered chain path unless a new exact front reopens the gap

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
