---
Status: SSOT
Date: 2026-04-04
Scope: canonical compat-codegen bucket archive sweep. This SSOT keeps the archive move and doc / alias rewrite in one place.
Related:
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-50x/README.md
  - docs/development/current/main/phases/phase-50x/50x-90-rust-vm-source-archive-cleanup-ssot.md
  - docs/development/current/main/phases/phase-50x/50x-91-task-board.md
  - tools/compat/README.md
  - tools/selfhost/README.md
  - tools/selfhost/compat/README.md
  - tools/archive/legacy-selfhost/README.md
---

# 51x-90 Compat-Codegen Archival Sweep

## Intent

- compat-codegen is historical proof/example material, not a day-to-day owner.
- the archived home is `tools/archive/legacy-selfhost/compat-codegen/`.
- the old live home `tools/compat/legacy-codegen/` is retired once the archive move lands.

## Keep / Archive Reading

- keep:
  - proof-only gate semantics
  - compat-only wrapper semantics
  - historical PyVM direct-only references
- archive:
  - canonical compat-codegen payload bucket
  - transport wrapper
  - pack orchestrator

## Success Criteria

- callers and docs point at the archive path, not the live canonical bucket
- the archived payload keeps its historical contract but is not a default route
- top-level selfhost docs no longer imply compat-codegen is a current owner

## Failure Criteria

- live docs keep pointing at the canonical bucket after archive
- proof-only payloads become day-to-day examples again
- compat wrapper promotion sneaks back in through alias docs
