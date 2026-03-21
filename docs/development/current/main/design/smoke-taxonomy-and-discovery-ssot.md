---
Status: SSOT
Scope: smoke profile taxonomy and discovery rules
Decision: accepted
Related:
- CURRENT_TASK.md
- tools/smokes/v2/run.sh
- tools/checks/smoke_inventory_report.sh
- docs/tools/check-scripts-index.md
---

# Smoke taxonomy and discovery SSOT

## Goal

Keep smoke navigation human-readable while preserving the existing runner contract.

The structural target is:

- daily entry stays small
- blocker pins stay traceable
- support buckets do not become live profile members by accident

## Current pressure snapshot

As of 2026-03-21, the smoke tree is heavily concentrated in a few leaves:

- `integration`: about `1200+` scripts
- `integration/apps`: about `275` active scripts at the leaf root
- `integration/apps/archive`: about `225` archived scripts
- `integration/joinir`: about `170` scripts
- `quick/core`: about `63` scripts

This is too dense for casual human navigation, especially under `integration/apps`.

## Discovery contract

- `tools/smokes/v2/run.sh` auto-discovers `*.sh` under `profiles/$PROFILE`.
- Discovery is recursive, but it now prunes support buckets by directory name:
  - `archive`
  - `lib`
  - `tmp`
  - `fixtures`
- Scripts under those directories remain directly runnable by `bash ...`, but they are not live profile members in `run.sh`.

## Taxonomy rules

### Rule 1: top level stays by run tier

- `profiles/quick/`
- `profiles/integration/`
- `profiles/full/`
- `profiles/plugins/`
- `profiles/archive/`

### Rule 2: second level is semantic domain

Prefer:

- `core/`
- `collections/`
- `array/`
- `map/`
- `string/`
- `parser/`
- `joinir/`
- `selfhost/`
- `runtime/`
- `vm/`
- `analyze/`

Use `apps/` only for app-level end-to-end cases. Do not keep every feature probe in `apps/`.

### Rule 3: third level is intent

Inside a domain, prefer intent buckets over phase buckets:

- `smoke/`
- `contract/`
- `gate/`
- `canary/`
- `probe/`
- `parity/`
- `regression/`
- `inventory/`

Phase IDs may remain in filenames, but they should not be the primary folder key for new structure.

### Rule 4: support buckets are never daily discovery

- `archive/` is for retired pins and manual replay
- `lib/` is for shared helpers
- `tmp/` is for scratch or generated artifacts
- `fixtures/` is for reusable inputs

These names are reserved and should not contain live profile entries that must run through `run.sh`.

## Operating rules

- Daily entry uses `tools/checks/dev_gate.sh` or lane gate packs.
- Single-purpose scripts are evidence pins or blocker probes.
- `1 blocker = 1 pin` remains valid, but pins should fold back into packs after the lane reaches stop line.
- Use `tools/checks/smoke_inventory_report.sh` for milestone inventory instead of manual ad-hoc pruning.

## First reorganization order

1. Fix discovery semantics so support buckets are not live.
2. Keep inventory tooling aligned with the same prune contract.
3. Split `integration/apps` by semantic domain before any mass rename.
4. Move historical residue to `archive/` buckets only after docs and packs stop pointing at the old path.

## First safe target

The first overloaded bucket to split is:

- `tools/smokes/v2/profiles/integration/apps/`

Recommended first semantic groups:

- `array`
- `map`
- `string`
- `selfhost`
- `stageb`
- `analyze`
- `core_direct`
- `parity`

Do not mass-move all archived content in the same slice. Archive separation and active semantic split should remain separate commits.
