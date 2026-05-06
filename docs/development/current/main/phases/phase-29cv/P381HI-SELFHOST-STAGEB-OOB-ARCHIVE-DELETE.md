# P381HI Selfhost Stage-B OOB Archive Delete

Date: 2026-05-07
Scope: retire the last remaining archive-only selfhost OOB smoke after its
manual-canary keep status was no longer needed.

## Context

`tools/smokes/v2/profiles/archive/selfhost/selfhost_stageb_oob_vm.sh` had been
held as a manual OOB diagnostic canary:

- it was already outside `integration/selfhost`
- it only ran when `SMOKES_ENABLE_STAGEB_OOB=1`
- no active suite or gate depended on it

At this point, the archive selfhost Stage-B OOB script was the last remaining
archive orphan candidate and the only blocked cleanup todo in this lane. The
remaining keep rationale lived only in docs:

- `docs/development/current/main/design/selfhost-smoke-retirement-inventory-ssot.md`
- `lang/src/vm/README.md`

## Change

- Deleted `tools/smokes/v2/profiles/archive/selfhost/selfhost_stageb_oob_vm.sh`
- Removed the archived-manual-canary row from
  `selfhost-smoke-retirement-inventory-ssot.md`
- Removed the `SMOKES_ENABLE_STAGEB_OOB` opt-in note from `lang/src/vm/README.md`

## Result

The selfhost archive-only OOB manual canary is fully retired:

- no active integration or daily gate changed
- no current smoke suite depends on the archive script
- the last blocked cleanup item in this lane is gone

## Validation

```bash
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/archive \
  SMOKE_INVENTORY_INCLUDE_PRUNED=1 \
  bash tools/checks/smoke_inventory_report.sh

test ! -e tools/smokes/v2/profiles/archive/selfhost/selfhost_stageb_oob_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
