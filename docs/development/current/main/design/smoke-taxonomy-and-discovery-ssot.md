---
Status: SSOT
Scope: smoke profile taxonomy and discovery rules
Decision: accepted
Related:
- CURRENT_TASK.md
- docs/development/current/main/phases/phase-29cq/README.md
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
- suite manifests become the human-facing execution contract

## Current pressure snapshot

As of 2026-03-21, the smoke tree is heavily concentrated in a few leaves:

- `integration`: about `1200+` scripts
- `integration/apps`: about `271` active scripts at the leaf root
- `integration/rc_gc_alignment`: `4` scripts, split out of `integration/apps` as the first live semantic family
- `integration/apps/archive`: about `225` archived scripts
- `integration/joinir`: about `170` scripts
- `quick/core`: about `63` scripts

This is still too dense for casual human navigation, especially under `integration/apps`, but the first live split has already been carved out as `integration/rc_gc_alignment`.

## Suite-first contract

- `tools/smokes/v2/suites/<profile>/<suite>.txt` is the primary human-facing execution contract.
- `run.sh --profile <profile> --suite <suite>` is the preferred daily/presubmit entry for curated packs.
- `--profile` remains the compatibility floor and coarse lane selector.
- `--suite` is additive: it applies an allowlist intersection over the live profile set.
- recursive discovery remains as a compatibility mechanism for uncatalogued profile runs, not as the long-term organization model.

Current seeded suites:

- `integration/presubmit`
- `integration/collection-core`
- `integration/vm-hako-core`
- `integration/selfhost-core`
- `integration/joinir-bq`

## Discovery fallback contract

- `tools/smokes/v2/run.sh` auto-discovers `*.sh` under `profiles/$PROFILE`.
- Discovery is recursive, but it now prunes support buckets by directory name:
  - `archive`
  - `lib`
  - `tmp`
  - `fixtures`
- Scripts under those directories remain directly runnable by `bash ...`, but they are not live profile members in `run.sh`.
- suite manifests may only reference paths that survive this live discovery fallback.

## Manifest format and failure contract

- manifest format:
  - `#` comment allowed
  - one relative path per line
  - path is relative to `tools/smokes/v2/profiles/<profile>/`
- fail-fast cases:
  - missing manifest
  - duplicate manifest entry
  - manifest entry that is not part of live discovery for that profile

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
- runner-level suite entry is allowed for curated packs (`run.sh --profile ... --suite ...`), but `--profile` remains the compatibility floor.
- Single-purpose scripts are evidence pins or blocker probes.
- `1 blocker = 1 pin` remains valid, but pins should fold back into packs after the lane reaches stop line.
- Use `tools/checks/smoke_inventory_report.sh` for milestone inventory instead of manual ad-hoc pruning.
- Inventory reports are suite-aware and scoped to the target subtree; use a profile root for whole-profile coverage and a semantic subtree for domain coverage.

## First reorganization order

1. Fix discovery semantics so support buckets are not live.
2. Introduce suite manifests without changing `--profile` compatibility.
3. Prefer suite manifests for daily/presubmit entry before any semantic path split.
4. Keep inventory tooling aligned with the same prune contract.
5. Split `integration/apps` by semantic domain before any mass rename; the first live split is `integration/rc_gc_alignment/`, and the next active family should be `json`.
6. Move historical residue to `archive/` buckets only after docs and packs stop pointing at the old path.

## First safe target

The first overloaded bucket to split is:

- `tools/smokes/v2/profiles/integration/apps/`

First live split already landed:

- `tools/smokes/v2/profiles/integration/rc_gc_alignment/`

Recommended next semantic groups:

- `json`
- `ring1_providers`
- `mir_shape`
- `phase29ck_boundary`

Do not mass-move all archived content in the same slice. Archive separation and active semantic split should remain separate commits.
