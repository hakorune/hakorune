---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: classify the selfhost identity check root helper as a protected keep.
Related:
  - tools/ROOT_SURFACE.md
  - tools/selfhost_identity_check.sh
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md
---

# P60 Selfhost Identity Check Owner Classification

## Goal

Resolve the remaining root manifest hold for `tools/selfhost_identity_check.sh`
without moving or weakening the identity gate.

This is a BoxShape classification slice. It changes ownership documentation
only.

## Owner Reading

- `selfhost-bootstrap-route-ssot.md` names `tools/selfhost_identity_check.sh`
  as the G1 identity contract.
- `coreplan-migration-roadmap-ssot.md` also points at the full identity gate as
  accepted G1 evidence.
- The helper is referenced by current selfhost and Stage1 contract docs.
- Heavy/full mode may need prebuilt Stage1/Stage2 artifacts, but that makes the
  helper protected rather than archiveable.

## Decision

- keep `tools/selfhost_identity_check.sh` at tools root
- classify it as a protected current identity gate in `tools/ROOT_SURFACE.md`
- do not move it under archive or compat-only debug paths

## Non-goals

- do not run the heavy Stage1/Stage2 full identity build
- do not change identity compare behavior
- do not change Stage1 contract or Program(JSON v0) delete-last policy

## Acceptance

```bash
bash -n tools/selfhost_identity_check.sh
rg -n 'tools/selfhost_identity_check.sh' \
  docs/development/current/main/design/selfhost-bootstrap-route-ssot.md \
  docs/development/current/main/design/coreplan-migration-roadmap-ssot.md \
  tools/ROOT_SURFACE.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
