---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: sync the Program(JSON v0) delete-last blocker guidance across route-map and env-reference docs after P33.
Related:
  - docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - docs/reference/environment-variables.md
---

# P34 Sync Delete-Last Blocker Guidance

## Goal

Keep the public and internal docs aligned now that thin shell/test seam cleanup
is exhausted through P32.

After P33, the remaining `Program(JSON v0)` work is explicitly blocked on
keeper replacement. The route-map and env-reference docs should say that
clearly so future cleanup does not skip straight to Rust/public deletion.

## Decision

- sync the route-map SSOT with the current delete-last blocker posture
- sync the environment-variable reference so compat-only examples point to the
  explicit probe/bridge keepers
- keep the route names and compat examples unchanged

## Non-goals

- do not change CLI behavior
- do not change any keeper ownership
- do not claim a new deletion step is now safe

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
