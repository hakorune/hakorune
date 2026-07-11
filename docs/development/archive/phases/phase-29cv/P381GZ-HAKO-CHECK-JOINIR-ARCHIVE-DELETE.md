# P381GZ hako_check JoinIR Archive Delete

Date: 2026-05-06
Scope: delete the zero-ref archived Phase 124 `hako_check` JoinIR smoke.

## Context

After P381GY, `tools/smokes/v2/profiles/archive/core/phase124/hako_check_joinir.sh`
remained a standalone orphan candidate in the refreshed archive inventory:

- `fullpath_ref_count = 0`
- `basename_ref_count = 0`
- `suite_hit_count = 0`
- `class = orphan_candidate`

Its remaining textual references are archive-phase history only. No active
suite, guard, or live owner still points at this script.

## Deleted Path

- `tools/smokes/v2/profiles/archive/core/phase124/hako_check_joinir.sh`

## Result

This is a single-script delete-last cleanup:

- no compiler behavior changed
- no active `hako_check` owner changed
- no suite-protected or manual-archive bucket was touched

## Validation

```bash
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/archive \
  SMOKE_INVENTORY_LABEL=post_p381gz_profiles_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh

test ! -e tools/smokes/v2/profiles/archive/core/phase124/hako_check_joinir.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
