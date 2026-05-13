---
Status: SSOT
Date: 2026-05-14
Scope: current lane / blocker / next pointer only.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# Self Current Task - Now (main)

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active lane: `phase-293x packed ArrayBox auto-use pilot`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- phase status: read `phase_status` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- taskboard: read `taskboard` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- task breakdown:
  `docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md`
- record / packed ArrayBox SSOT:
  `docs/development/current/main/design/record-and-packed-array-lowering-ssot.md`
- mimalloc port purpose:
  `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`
- current blocker token: `M210 decommit/recommit/reuse EXE hardening`
- update policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Next

- continue phase-293x after M209; next blocker is M210
  decommit/recommit/reuse EXE hardening
- BoxTorrent mini, binary-trees, mimalloc-lite, the `hako_alloc` VM-only
  page/free-list port, allocator-stress, BoxTorrent allocator-backed store, and
  JSON stream aggregator are landed
- phase-294x exact `usize` substrate work needed by the mimalloc port is no
  longer the active default lane. C207 emits metadata-only
  `array_record_autouse_eligibility_plans`, and C208 emits metadata-only
  `array_record_materialization_boundary_plans`; C209 emits
  `array_record_packed_autouse_pilot_plans` and crate-private i64 column seams.
  C210 emits `hako_alloc_aligned_small_packed_store_pilot_plans` without
  leaking compiler internals into `hako_alloc` source. C211 emits
  `hako_alloc_huge_page_packed_store_pilot_plans` while preserving
  live/sentinel contracts. C212 adds the shared MIR backend capability gate and
  packed record fail-fast checker. C194 moves C210/C211 hako_alloc metadata
  invariants into MIR verification. M191 adds allocator-owned stats snapshots
  without mutable options or behavior changes, M192 adds a read-only
  purge/decommit policy inventory with OSVM execution inactive, M193 connects
  that policy to OSVM-backed heap page/backing observation as a dry-run only,
  M194 adds an execution entry that still returns blocked reports, and M195
  adds bounded caller-provided decommit execution while keeping unreserve and
  OS release inactive, M196 connects that bounded policy to the page-source
  decommit adapter only, M197 composes dry-run observation, bounded policy, and
  page-source adapter for heap page/backing state, M198 records successful
  decommit report page ids in a separate state marker, M199 blocks repeated
  decommit attempts before page-source execution, and M200 classifies
  decommitted pages as unavailable until a future recommit path exists, M201
  adds a blocked/report-only recommit attempt entry with no source execution,
  M202 adds a bounded caller-provided recommit policy, M203 connects that
  policy to a recommit-only page-source adapter, M204 transitions marker state
  with decommit/recommit generation counts, M205 composes the recommit path
  into page-local reactivation while page sourcing, unreserve, and OS release
  remain closed, M206 proves the two-generation decommit/recommit/reuse loop
  without a new allocator owner, M207 freezes the active/retired/
  decommitted/recommitted-active lifecycle vocabulary as a read-only
  observer/proof, C194b moves the selected M207 lifecycle report/function
  invariants into MIR verification, M208 freezes heap reuse priority as active
  → recommitted-active → retired-reactivate → fresh fallback while decommitted
  pages remain blocked until recommit, and M209 exposes read-only lifecycle
  event stats over the M207 observer and M208 reuse policy counters. Visible
  record materialization and packed record backend lowering remain closed.
- typed-object EXE allocation plus slot `field_set` / `field_get` now covers
  declared i64 fields, init-only untyped fields, handle storage, and observed
  empty user boxes, nullable handle storage through same-module RuntimeDataBox
  receiver origins, and the BoxTorrent `firstChunkId` / `refCount`
  module-generic prepass seam, plus BoxTorrent user-box string field returns,
  recursive same-module user-box method bodies, and typed-object handle
  global-call returns, allocator handle param-origin inference, and explicit
  same-module PHI type preservation; BoxTorrent mini, binary-trees, JSON stream
  aggregator, mimalloc-lite, and allocator-stress direct EXE parity now pass.
  The next default implementation work should come from the `.hako`
  mimalloc / `hako_alloc` completeness lane on top of the capability substrate,
  not from host allocator replacement.
- if a real app exposes a compiler expressivity blocker, fix the compiler seam
  structurally instead of adding app-side workaround code
- current mirrors are thinned; update `CURRENT_STATE.toml` and the phase-293x
  card/taskboard first
- current allocator/provider task ladder is:
  `docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md`;
  the ladder is closed through M103, and proof-bundle consumption now has a
  fail-fast runtime entry, a caller-provided selected-provider precondition,
  and selected-provider proof validation under the activation owner while
  provider selection and actual consumption remain inactive. M104 is the next
  row only inside the optional future host-replacement ladder; it is not the
  default next task for the mimalloc port.
- post-M101 implementation order:
  `docs/development/current/main/design/allocator-provider-post-m101-implementation-ladder-ssot.md`
- latest docs/inventory baseline: `291x-691` remains the historical backlog
  inventory; current status is in `CURRENT_STATE.toml`
- do not reopen broad `plan/facts` or `lower::planner_compat` ownership work
  without focused BoxShape lanes and SSOT cards
- normalized-shadow / normalization cleanup burst is closed; larger findings
  move to a new lane
- no-growth checkpoint: `classifiers=0 rows=0`; no `.inc` method/box string
  classifiers are allowlisted
- task-order source:
  `docs/development/current/main/phases/phase-291x/291x-488-current-task-order-baseline-refresh-card.md`
- detailed landed history: phase card files and `CURRENT_STATE.toml`, not this
  mirror

## Rules

- keep BoxShape and BoxCount separate
- keep Stage-B adapter thinning separate from CoreMethodContract migration
- do not add hot inline lowering without proof/evidence gate
- do not update current mirrors for every landed card
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`
3. `docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md`
4. `docs/development/current/main/phases/phase-293x/README.md`
5. `docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md`
6. `docs/development/current/main/design/current-docs-update-policy-ssot.md`
7. `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
8. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
```
