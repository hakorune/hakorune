---
Status: SSOT
Date: 2026-05-16
Scope: current docs archive and slimming policy.
Related:
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-293x/README.md
---

# Current Docs Archive Policy

## Decision

Current docs are restart/navigation surfaces, not landed-history storage.

Use this split:

```text
current entry:
  CURRENT_STATE.toml
  CURRENT_TASK.md
  05-Restart-Quick-Resume.md
  10-Now.md
  active phase README

active execution:
  active card
  latest-card pointer
  taskboard only when its stable contract changes

durable design:
  design/*-ssot.md

historical execution:
  phase archive cards
  landed ledger
  old cards with optional forwarding stubs
```

## What Stays Live

Keep these in their current paths:

- `CURRENT_STATE.toml` and thin restart mirrors.
- `CURRENT_STATE.phase_status`.
- `CURRENT_STATE.latest_card_path`.
- active phase README.
- active taskboards named by `CURRENT_STATE.taskboard`.
- design SSOTs that remain current policy owners.
- check scripts and fixtures used by active or recent guards.

## What Moves To Archive

Landed phase cards can move when all are true:

- Status is landed / historical / superseded.
- The card is not `CURRENT_STATE.phase_status`.
- The card is not `CURRENT_STATE.latest_card_path`.
- The card is not the active row for a current taskboard.
- The card has no current guard that requires the old path.

Archive buckets for phase card directories:

```text
docs/development/current/main/phases/phase-293x/archive/cards/293x-000-099/
docs/development/current/main/phases/phase-293x/archive/cards/293x-100-199/
docs/development/current/main/phases/phase-293x/archive/cards/293x-200-299/
docs/development/current/main/phases/phase-293x/archive/cards/293x-300-399/
```

Keep a forwarding stub at the old path only when a current doc, guard, or script
still references the old path. If no tracked current reference exists, the
archive ledger is enough.

## Repository Artifact Lifecycle Consolidation (2026-07-11)

The file-count problem is historical artifact retention, not source-code
module granularity. Measured baseline:

```text
docs files = 16,534
tools files = 8,620
src files = 3,314
phase-296x direct files = 2,123
phase-296x archive files = 1,425
design direct files = 848
main direct files = 169
tools/checks files = 3,265
docs/private files = 4,251

phase-296x direct card sample:
  cards excluding README/STATUS/hygiene = 2,120
  status classified closed = 1,330
  externally referenced by tracked files = 788
  unreferenced archive candidates = 1,332
```

Counts are inventory evidence, not permission to move or delete files. In
particular, the 788 referenced cards disprove a review-free mass move.

Do not add more numbered `DOCS-SLIM-*` cards. The existing 001-027 history is
consolidated into one artifact-lifecycle refactor series with these ordered
steps:

### H0 - Baseline and warning-only guard

- extend an existing docs-slim/current-state guard; do not add a new `.sh`;
- report direct-card, direct-design, current-phase, and check-script counts;
- derive active paths from `CURRENT_STATE.toml`, active workstreams, and active
  card references;
- warning only at the measured baseline; hard limits are chosen after shrink;
- store machine-readable candidate/referenced sets in one manifest, not one
  file per row.

### H1 - phase-296x card archive batches

- generalize `tools/checks/lib/phase_card_paths.sh` from phase-293x to a
  phase-scoped resolver before moving referenced cards;
- archive unreferenced, closed cards first;
- keep the active card, active references, README, STATUS, and hygiene rule;
- use batches of at most 200 physical moves per commit;
- after every batch run pointer, link/reference, docs-slim, and `dev_gate quick`;
- no full duplicate at the old path; create a forwarding stub only when a live
  tracked reference cannot yet use the resolver.

### H2 - inactive phase directory archive

- derive the active phase closure from current pointers and tracked live docs;
- move only phases outside that closure;
- archive to `docs/development/archive/phases/` in bounded batches;
- preserve a compact phase index/stub where live navigation still requires the
  old path;
- do not claim that `git log --follow` prevents link or guard breakage.

### H3 - design authority registry

- use the existing `design/README.md` as the registry; do not create a second
  `INDEX.md` truth;
- classify each direct design file as authority, active navigation, candidate,
  superseded, or archive;
- move unregistered/superseded files only after inbound-reference inventory;
- introduce warning mode first, then require every direct design file to have
  one registry classification;
- target 50-100 authority/navigation entries is a review estimate, not a guard
  constant.

### H4 - check-script convergence and retirement

- build one caller/owner inventory from dev-gate groups, manifests, CI,
  check-script index, docs, and direct shell callers;
- card archival alone never proves a guard is obsolete;
- keep reusable behavior guards, but move repeated row assertions into
  `manifest_runner.py` manifests;
- delete a script only when active caller count is zero, durable proof ownership
  has moved, and parity tests cover the manifest replacement;
- retire the corresponding check-index row in the same commit.

### H5 - lifecycle enforcement

- update card closeout to require archive/retain-with-retire_when plus guard
  retirement review;
- switch warning counts to hard no-growth limits only after H1-H4 establish a
  clean baseline;
- hard guards use active-reference closure, not an arbitrary "latest 20" rule;
- keep one-card-one-file and one-owner-per-guard; do not merge history into a
  giant ledger.

`docs/private` retention is a separate final sweep. It must not preempt current
navigation cleanup and requires an explicit private-data retention decision.

Series rollback rule:

```text
any unresolved tracked reference, guard failure, or pointer drift:
  stop the series
  revert only the current bounded move batch
  update the resolver/manifest before continuing
```

## Ledger Rule

Long landed history belongs in a ledger, not in current mirrors.

Recommended shape:

```text
Card | Status | Summary | Guard | Commit
```

`CURRENT_STATE.toml` keeps only a short `landed_tail`.

```text
target maximum:
  12 rows
```

## Line Budget Rule

Non-archive current docs have a soft limit of 2000 lines.

When a current doc crosses that limit, split it before adding more landed
history:

```text
active entry / SSOT:
  current decision, stop lines, active row, compact anchors

archive ledger:
  old session logs, exact evidence, full historical per-row prose
```

Archive ledgers may exceed 2000 lines when preserving traceability, but their
old active paths should become compact stubs or dashboards. Guard-compatible
anchors may stay in the active path when existing row guards still assert those
tokens.

## Guard Reference Rule

Implementation guards should not force taskboards to become landed-history
ledgers.

Prefer guard inputs in this order:

1. active card
2. durable SSOT
3. check-scripts index
4. code/test fixture
5. taskboard only when the taskboard's own contract changed

Do not add a taskboard assertion just to prove a card landed.

## First Slimming Phase

`DOCS-SLIM-001` owns policy and inventory only:

- add this SSOT
- trim `CURRENT_STATE.landed_tail`
- add guardrails to prevent regrowth
- produce archive bucket counts
- do not physically move old cards yet

Physical archive moves are `DOCS-SLIM-002+`.

## Second Slimming Phase

`DOCS-SLIM-002` owns archive manifest prep only:

- add `phase-293x/archive/` entry docs
- add card bucket protocol
- add a root-card count / direct-reference risk manifest
- guard that no cards moved yet
- keep physical moves for a later row after guard references are decoupled or
  forwarding stubs are planned

## Third Slimming Phase

`DOCS-SLIM-003` owns guard-reference decoupling:

- remove stale `CURRENT_STATE.latest_card` / `current_blocker_token` pins from
  old row guards, including `landed_tail` history pins
- make `DOCS-SLIM-002` guard check archive artifacts instead of live root-card
  counts
- add a phase-293x card resolver helper for future archive-bucket moves
- do not mass-convert existing direct card-reference guards yet
- do not physically move old cards yet

## Fourth Slimming Phase

`DOCS-SLIM-004` owns the first resolver adoption cluster:

- convert allocator-provider activation closeout guards to use
  `guard_require_phase293x_card`
- remove direct phase-293x card paths from the converted activation closeout
  scripts
- keep production allocator port closeout and other direct-reference guards for
  later rows
- do not physically move old cards yet

## Fifth Slimming Phase

`DOCS-SLIM-005` owns the production allocator port closeout resolver adoption:

- convert `k2_wide_production_allocator_port_closeout_guard.sh` to use
  `guard_require_phase293x_card`
- remove direct M46-M51 phase-card paths from that script
- keep phase README / taskboard proof assertions unchanged
- do not physically move old cards yet

## Sixth Slimming Phase

`DOCS-SLIM-006` owns the M10c runtime-decl resolver adoption:

- convert `k2_wide_runtime_decl_return_proof_row_guard.sh` to use
  `guard_require_phase293x_card`
- convert `k2_wide_native_ptr_decl_type_guard.sh` to use
  `guard_require_phase293x_card`
- convert `k2_wide_hako_mem_runtime_decl_guard.sh` to resolve hako_mem
  realloc / call-arg / free cards through the helper and pass them into the
  embedded Python checker via environment variables
- do not change runtime-decl manifests or return-proof semantics
- do not physically move old cards yet

## Seventh Slimming Phase

`DOCS-SLIM-007` owns the lifecycle ladder resolver adoption:

- convert `k2_wide_lifecycle_birth_new_only_guard.sh` to use
  `guard_require_phase293x_card`
- convert `k2_wide_parser_birth_direct_call_guard.sh` to use
  `guard_require_phase293x_card`
- convert `k2_wide_parser_birth_diagnostic_hint_guard.sh` to use
  `guard_require_phase293x_card`
- convert `k2_wide_reuse_lifecycle_explicit_methods_guard.sh` to use
  `guard_require_phase293x_card`
- keep lifecycle / parser / hako_alloc behavior unchanged
- do not physically move old cards yet

## Eighth Slimming Phase

`DOCS-SLIM-008` owns the recent cleanup guard resolver adoption:

- convert `k2_wide_looprange_ast_rename_guard.sh` to use
  `guard_require_phase293x_card`
- convert `k2_wide_loopclean_while_parser_facade_guard.sh` to use
  `guard_require_phase293x_card`
- convert `k2_wide_clean_stage1_lowering_stmt_split_guard.sh` to use
  `guard_require_phase293x_card`
- keep parser / Stage1 lowering behavior unchanged
- do not physically move old cards yet

## Ninth Slimming Phase

`DOCS-SLIM-009` owns the C197-C200 proof surface resolver adoption:

- convert `k2_wide_logical_condition_surface_guard.sh` to use
  `guard_require_phase293x_card`
- convert `k2_wide_check_block_surface_guard.sh` to use
  `guard_require_phase293x_card`
- convert `k2_wide_compound_assignment_surface_guard.sh` to use
  `guard_require_phase293x_card`
- convert `k2_wide_guard_else_surface_guard.sh` to use
  `guard_require_phase293x_card`
- keep parser / syntax / proof app behavior unchanged
- do not physically move old cards yet

## Tenth Slimming Phase

`DOCS-SLIM-010` owns the manifest runner pilot guard decoupling:

- remove landed-history phase README and old real-app taskboard pins from
  `manifest_runner_pilot_guard.sh`
- keep D199 card, check index, wrapper, manifest, shared runner, and gate-leak
  assertions
- resolve the D199 card through `guard_require_phase293x_card`
- keep manifest runner behavior unchanged
- do not physically move old cards yet

## Eleventh Slimming Phase

`DOCS-SLIM-011` owns the record metadata README decoupling:

- remove landed-history phase README pins from the record / metadata guard
  cluster
- keep card status, implementation, record SSOT, and check-index assertions
- keep record, metadata, and packed-array behavior unchanged
- do not physically move old cards yet

## Twelfth Slimming Phase

`DOCS-SLIM-012` owns the inline record probe resolver adoption:

- convert `k2_wide_arraybox_inline_record_probe_guard.sh` to use
  `guard_require_phase293x_card`
- convert `k2_wide_arraybox_inline_record_plan_probe_guard.sh` to use
  `guard_require_phase293x_card`
- convert `k2_wide_metadata_store_indexed_read_guard.sh` to use
  `guard_require_phase293x_card`
- remove landed-history phase README pins from the trio
- keep probe, plan, metadata-store, and check-index assertions unchanged
- do not physically move old cards yet

## Thirteenth Slimming Phase

`DOCS-SLIM-013` owns the packed record guard cluster README pin decoupling:

- remove landed-history phase README pins from the packed record guard cluster
- keep card status, implementation, record SSOT, taskboard rows, tests, and
  check-index assertions
- keep probe, pilot, packed-store, and backend behavior unchanged
- do not physically move old cards yet

## Fourteenth Slimming Phase

`DOCS-SLIM-014` owns the packed record guard cluster taskboard pin decoupling:

- remove landed-history taskboard pins from the packed record guard cluster
- keep card status, implementation, record SSOT, tests, and check-index
  assertions
- keep probe, pilot, packed-store, and backend behavior unchanged
- keep the README-only decoupling from `DOCS-SLIM-013` intact
- do not physically move old cards yet

## Fifteenth Slimming Phase

`DOCS-SLIM-015` owns the allocator hook guard band README pin decoupling:

- remove landed-history phase README pins from the allocator hook guard band
- keep card status, taskboard rows, implementation checks, and check-index
  assertions
- keep hook, dry-run, and activation behavior unchanged
- keep the taskboard-pin decoupling for a follow-up row
- do not physically move old cards yet

## Sixteenth Slimming Phase

`DOCS-SLIM-016` owns the allocator hook guard band real-app taskboard pin
decoupling:

- remove landed-history real-app taskboard pins from the allocator hook guard
  band
- keep card status, design taskboard, implementation checks, and check-index
  assertions
- keep hook, dry-run, and activation behavior unchanged
- keep the README-only decoupling from `DOCS-SLIM-015` intact
- do not physically move old cards yet

## Seventeenth Slimming Phase

`DOCS-SLIM-017` owns the allocator provider guard band README pin decoupling:

- remove landed-history phase README pins from the allocator provider guard
  band
- keep card status, design taskboard, implementation checks, and check-index
  assertions
- keep provider boundary, manifest, and task-breakdown behavior unchanged
- keep the taskboard-pin decoupling for a follow-up row
- do not physically move old cards yet

## Eighteenth Slimming Phase

`DOCS-SLIM-018` owns the allocator provider guard band real-app taskboard pin
decoupling:

- remove landed-history real-app taskboard pins from the allocator provider
  guard band
- keep card status, phase README, implementation checks, and check-index
  assertions
- keep provider boundary, manifest, and task-breakdown behavior unchanged
- keep the README-only decoupling from `DOCS-SLIM-017` intact
- do not physically move old cards yet

## Nineteenth Slimming Phase

`DOCS-SLIM-019` owns the allocator provider proof/registry guard band real-app
taskboard pin decoupling:

- remove landed-history real-app taskboard pins from the allocator provider
  proof/registry guard band
- keep card status, phase README, implementation checks, and check-index
  assertions
- keep provider proof, registry, activation-entry, selection, and proof-bundle
  behavior unchanged
- keep the README-only decoupling from `DOCS-SLIM-017` and the 018 taskboard
  decoupling intact
- do not physically move old cards yet

## Twentieth Slimming Phase

`DOCS-SLIM-020` owns the allocator provider manifest/readiness/registry guard
band real-app taskboard pin decoupling:

- remove landed-history real-app taskboard pins from the allocator provider
  manifest/readiness/registry guard band
- keep card status, phase README, implementation checks, and check-index
  assertions
- keep provider manifest parser, manifest CLI, readiness preflight, registry
  boundary, and combined dry-run behavior unchanged
- keep the README-only decoupling from `DOCS-SLIM-017` and the 018/019
  taskboard decouplings intact
- do not physically move old cards yet

## Twenty-first Slimming Phase

`DOCS-SLIM-021` owns the allocator provider boundary/manifest/task breakdown
guard band real-app taskboard pin decoupling:

- remove landed-history real-app taskboard pins from the allocator provider
  boundary/manifest/task breakdown guard band
- keep card status, design taskboard, implementation checks, and check-index
  assertions
- keep provider boundary vocabulary, manifest vocabulary, and task breakdown
  behavior unchanged
- keep the README-only decoupling from `DOCS-SLIM-017` and the 018-020
  taskboard decouplings intact
- do not physically move old cards yet

## Twenty-second Slimming Phase

`DOCS-SLIM-022` owns the allocator provider manifest/readiness/registry guard
band phase README pin decoupling:

- remove landed-history phase README pins from the allocator provider
  manifest/readiness/registry guard band
- keep card status, design taskboard, implementation checks, and check-index
  assertions
- keep provider manifest parser, manifest CLI, readiness preflight, registry
  boundary, and combined dry-run behavior unchanged
- keep the README-only decoupling from `DOCS-SLIM-017` and the 018-021
  taskboard decouplings intact
- do not physically move old cards yet

## Twenty-third Slimming Phase

`DOCS-SLIM-023` owns the allocator provider proof/rollback/activation safety
guard band phase README pin decoupling:

- remove landed-history phase README pins from the allocator provider
  proof/rollback/activation safety guard band
- keep card status, design taskboard, implementation checks, and check-index
  assertions
- keep provider proof, registry snapshot, selection decision, proof bundle,
  rollback preflight, activation safety, and activation decision behavior
  unchanged
- keep the README-only decoupling from `DOCS-SLIM-017` and the 018-022
  taskboard decouplings intact
- do not physically move old cards yet

## Twenty-fourth Slimming Phase

`DOCS-SLIM-024` owns the production allocator port and mimalloc closeout guard
band phase README pin decoupling:

- remove landed-history phase README pins from the production allocator port
  entry/closeout and mimalloc allocator closeout guard band
- keep card status, design taskboard, real-app taskboard, implementation
  checks, and check-index assertions
- keep production allocator port entry/closeout and mimalloc allocator
  closeout behavior unchanged
- keep the README-only decoupling from `DOCS-SLIM-017` and the 018-023
  taskboard decouplings intact
- do not physically move old cards yet

## Twenty-fifth Slimming Phase

`DOCS-SLIM-025` owns the docs-slim card metadata helper extraction:

- add a shared helper for repeated docs-slim card metadata assertions
- convert the DOCS-SLIM-022/023/024 guards to use the helper for card
  metadata assertions
- keep the per-script landed-history pin assertions and gate-leak checks in
  place
- do not physically move old cards yet

## Twenty-sixth Slimming Phase

`DOCS-SLIM-026` owns the phase-card resolver leak helper extraction:

- add a shared helper for repeated phase-card resolver leak checks
- convert the DOCS-SLIM-004/005/006/007/008/009/010/013/014/015/016/017/
  018/019/020/021/022/023/024 guards to use the helper
- keep the per-row landed-history pin assertions and card metadata checks in
  place
- do not physically move old cards yet
