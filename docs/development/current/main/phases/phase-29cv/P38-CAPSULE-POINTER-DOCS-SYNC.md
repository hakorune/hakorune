---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: sync phase-29cv pointers and delete-last docs after the P37 compat capsule split.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-29cv/P24-KEEPER-DELETE-LAST-ORDER.md
  - docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md
  - docs/development/current/main/phases/phase-29cv/P37-PROGRAM-JSON-V0-COMPAT-CAPSULE-SSOT.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - tools/selfhost/README.md
---

# P38 Capsule Pointer Docs Sync

## Goal

Remove stale phase pointers and older delete-last wording now that P37 named the
remaining Program(JSON v0) surfaces as compat capsules.

## Decision

- point the compact current-state phase status at the P37 capsule SSOT
- point the taskboard at P33 delete-last blockers
- refresh P24 so it no longer claims the run-stage1 wrapper thin slice is next
- keep the route-map explicit that Rust/public compat is final delete-last
- clarify that `stageb-delegate` remains because bridge/archive replacement is
  incomplete, not because MIR-first proof is missing

This is docs-only and does not change routes or keeper behavior.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
