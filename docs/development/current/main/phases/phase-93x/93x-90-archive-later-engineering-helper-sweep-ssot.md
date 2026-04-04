# Phase 93x SSOT — archive-later engineering helper sweep

Status: Draft
Date: 2026-04-05
Scope: archive-later helper relocation / doc repoint の最終整理。

## Goal

`tools/selfhost/` 直下に残っていた legacy engineering helpers を archive bucket へ退避し、current/docs の live surface を薄く保つ。

## Inventory

### Moved to archive

- `tools/archive/legacy-selfhost/engineering/legacy_main_readiness.sh`
- `tools/archive/legacy-selfhost/engineering/pre_promote_legacy_main_removal.sh`
- `tools/archive/legacy-selfhost/engineering/promote_tier2_case.sh`
- `tools/archive/legacy-selfhost/engineering/program_analyze.sh`
- `tools/archive/legacy-selfhost/engineering/program_analyze.hako`
- `tools/archive/legacy-selfhost/engineering/gen_v1_min.sh`

### Repointed docs

- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/15-Workstream-Map.md`
- `docs/development/current/main/phases/README.md`
- `docs/development/current/main/design/*.md` touched by the archive path repoint
- `docs/development/current/main/phases/phase-29bq/*.md` touched by the archive path repoint
- `docs/development/current/main/phases/phase-45x/45x-90-vm-residual-cleanup-ssot.md`
- `docs/development/current/main/phases/phase-48x/48x-90-smoke-source-cleanup-ssot.md`
- `docs/development/current/main/phases/phase-49x/49x-90-legacy-wording-compat-route-cleanup-ssot.md`
- `docs/development/current/main/phases/phase-67x/67x-90-selfhost-folder-split-ssot.md`
- `tools/selfhost/README.md`
- `tools/archive/legacy-selfhost/README.md`

## Acceptance

- no live `tools/selfhost/(legacy_main_readiness|pre_promote_legacy_main_removal|promote_tier2_case|program_analyze|gen_v1_min).sh` references remain outside archive paths
- `git diff --check` passes
- moved shell helpers pass `bash -n`

