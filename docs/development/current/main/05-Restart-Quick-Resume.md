---
Status: Active
Date: 2026-05-13
Scope: 再起動直後に 2-5 分で current lane に戻るための最短手順。
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Heavy gates are not first-step restart work. Run them only when the next code
slice is ready:

```bash
tools/checks/dev_gate.sh quick
cargo check -q
```

## Current Lane

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: `phase-293x packed ArrayBox auto-use pilot`
- active phase: read `active_phase` from `CURRENT_STATE.toml`
- latest card: read `latest_card_path` from `CURRENT_STATE.toml`
- current blocker token: `M193 purge/decommit dry-run observer`
- record / packed ArrayBox SSOT:
  `docs/development/current/main/design/record-and-packed-array-lowering-ssot.md`
- mimalloc port purpose:
  `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Handoff Snapshot

- latest landed card: read `latest_card_path` in `CURRENT_STATE.toml`
- current blocker token: `M193 purge/decommit dry-run observer`
- latest known checkpoint: read `latest_card` / `latest_card_path` in
  `CURRENT_STATE.toml`; `291x-691` remains the historical warning-backlog
  inventory baseline
- no-growth checkpoint: `classifiers=0 rows=0`; no `.inc` method/box string
  classifiers are allowlisted
- worktree expectation: clean after the last commit unless an active slice is
  underway

## Immediate Next

- continue `phase-293x` from M193 purge/decommit dry-run observer.
  C207 emits `array_record_autouse_eligibility_plans`, C208 emits
  `array_record_materialization_boundary_plans`, and C209 emits
  `array_record_packed_autouse_pilot_plans` plus crate-private i64 column seams.
  C210 emits `hako_alloc_aligned_small_packed_store_pilot_plans` while keeping
  hako_alloc source compiler-internal-free. C211 emits
  `hako_alloc_huge_page_packed_store_pilot_plans` while preserving
  live/sentinel contracts. C212 adds the shared MIR backend capability gate and
  packed record fail-fast checker. C194 moves C210/C211 hako_alloc metadata
  invariants into MIR verification. M191 adds allocator-owned stats snapshots
  without mutable options or behavior changes, and M192 adds a read-only
  purge/decommit policy inventory with OSVM execution inactive. Visible record
  materialization and packed record backend lowering remain closed.
- BoxTorrent mini, binary-trees, mimalloc-lite, the `hako_alloc` VM-only
  page/free-list port, allocator-stress, BoxTorrent allocator-backed store, and
  JSON stream aggregator are landed with `real-apps` smoke coverage
- typed-object EXE allocation plus slot field get/set now covers declared i64
  fields, init-only untyped fields, handle storage, and observed empty user
  boxes, nullable handle storage through same-module RuntimeDataBox receiver
  origins, and the BoxTorrent `firstChunkId` / `refCount` module-generic
  prepass seam, plus recursive same-module user-box method bodies,
  typed-object handle global-call returns, allocator handle param-origin
  inference, and explicit same-module PHI type preservation; BoxTorrent mini,
  binary-trees, JSON stream aggregator, mimalloc-lite, and allocator-stress
  direct EXE parity now pass
- parent EXE boundary gate, only when checking the parked real-app lane:
  `tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight`
- phase-294x exact `usize` substrate work needed by the mimalloc port is no
  longer the active default lane. The active lane is packed record / ArrayBox
  compiler expressivity for mimalloc metadata completeness.
- do not hide compiler blockers in app code; if a real app exposes a Stage0 or
  VM/compiler seam, fix the compiler structurally first
- parent real-app gate, only when checking the parked real-app lane:
  `tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight`
- current mirrors are thinned; update `CURRENT_STATE.toml` and the phase-293x
  card/taskboard first
- do not reopen broad `plan/facts` or `lower::planner_compat` ownership work
  without focused BoxShape lanes and SSOT cards
- normalized-shadow / normalization cleanup burst is closed; larger findings
  move to a new lane
- use `docs/development/current/main/phases/phase-291x/291x-488-current-task-order-baseline-refresh-card.md`
  for the current task-order baseline
- use `docs/development/current/main/phases/phase-291x/291x-smoke-index.md`
  for smoke selection
- keep docs mirrors thin; update `CURRENT_STATE.toml` and the active card first
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- keep phase-137x observe-only unless app work reopens a real blocker

## Restart Notes

- do not paste landed-card history into restart/current mirrors
- do not run heavy perf ladders during restart unless explicitly requested
