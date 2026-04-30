---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: sync post-P20 raw emit inventory wording so docs no longer imply archive scripts own direct raw flag spelling.
Related:
  - docs/development/current/main/phases/phase-29ci/P20-ARCHIVE-PROGRAM-JSON-V0-EMIT-HELPER-SYNC.md
  - docs/development/current/main/phases/phase-29ci/P7-RAW-COMPAT-CALLER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P12-REMAINING-RAW-COMPAT-CALLERS.md
---

# P21 Program JSON v0 Inventory Wording Sync

## Goal

After P20, archive joinir pins no longer spell `--emit-program-json-v0`
directly. Update inventory wording so the current state is unambiguous:

- runtime deprecation text keeps the public warning.
- `tools/lib/program_json_v0_compat.sh` is the shell emit spelling SSOT.
- current and archive smoke callers use helpers.

No code behavior changes.

## Acceptance

```bash
rg -n -g '!tools/historical/**' -g '!target/**' -- '--emit-program-json-v0' src tools
bash tools/checks/current_state_pointer_guard.sh
```
