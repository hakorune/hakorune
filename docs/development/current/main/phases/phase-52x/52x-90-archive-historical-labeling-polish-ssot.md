---
Status: SSOT
Date: 2026-04-04
Scope: archive historical labeling polish after source cleanup. This SSOT keeps archive wording, proof/historical evidence, and no-widen boundaries in one place.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-51x/README.md
  - tools/archive/legacy-selfhost/README.md
  - tools/archive/legacy-selfhost/compat-codegen/README.md
  - tools/archive/legacy-selfhost/compat-codegen/run_compat_pure_pack.sh
---

# 52x-90 Archive-Historical Labeling Polish

## Intent

- archive history stays available, but the wording stays historical-only.
- canonical compat-codegen / rust-vm archive evidence stays minimal and explicit.
- no archive surface should read like a daily owner lane.

## Keep / Archive Reading

- keep:
  - canonical historical evidence files
  - proof-only / compat-only archive notes
  - archive README references that point at historical evidence
- archive:
  - live-owner phrasing
  - duplicate active-route wording
  - any comment that makes the archive bucket look like day-to-day ownership

## Success Criteria

- archive README / wrapper text reads historical-only
- current docs no longer imply archive surfaces are live owners
- active source remains free of `rust-vm` wording

## Failure Criteria

- archive wording drifts back toward current-owner language
- historical references are duplicated instead of collapsed to canonical evidence
- proof-only / compat keep semantics are widened in archive docs
