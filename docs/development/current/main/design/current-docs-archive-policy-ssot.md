---
Status: SSOT
Date: 2026-05-15
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
