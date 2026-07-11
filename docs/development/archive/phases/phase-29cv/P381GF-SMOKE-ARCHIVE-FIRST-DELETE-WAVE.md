# P381GF Smoke Archive First Delete Wave

Date: 2026-05-06
Scope: delete only the P381GE zero-ref v2 archive candidates.

## Decision

Deleted exactly the 45 scripts listed by
`P381GE-SMOKE-ARCHIVE-FIRST-CANDIDATE-LIST.md`.

No legacy `tools/smokes` root script was deleted. No
`tools/archive/manual-smokes` script was deleted. No suite-protected
`tools/smokes/v2/profiles/integration/archive` script was deleted.

## Count Delta

| Surface | Before | After | Delta |
| --- | ---: | ---: | ---: |
| `tools/smokes/v2/profiles/archive` | 81 | 77 | -4 |
| `tools/smokes/v2/profiles/integration/archive` | 13 | 13 | 0 |
| `tools/smokes/v2/profiles/integration/apps/archive` | 225 | 184 | -41 |
| `tools/smokes/v2/profiles` | 1464 | 1419 | -45 |

## Post-Delete Inventory

`tools/smokes/v2/profiles/archive`:

```text
Total: 77
Referenced: 56
Orphan candidates: 21
  - Wrapper-only orphan candidates: 0
```

`tools/smokes/v2/profiles/integration/archive`:

```text
Total: 13
Referenced: 13
Orphan candidates: 0
  - Wrapper-only orphan candidates: 0
phase29ck-boundary-legacy 12/12 (missing 0)
phase29x-derust-archive 1/1 (missing 0)
```

`tools/smokes/v2/profiles/integration/apps/archive`:

```text
Total: 184
Referenced: 8
Orphan candidates: 176
  - Wrapper-only orphan candidates: 0
```

Remaining report-orphan rows are not automatically delete candidates. They
still have full-path or basename references, or they need owner policy cleanup
before deletion.

## Validation

```bash
while read -r path; do
  test ! -e "$path" || exit 1
done < target/smoke_inventory/t6_zero_ref_candidate_paths.txt
```

```bash
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/archive \
  SMOKE_INVENTORY_LABEL=t6_after_delete_profiles_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh

SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/integration/archive \
  SMOKE_INVENTORY_LABEL=t6_after_delete_integration_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh

SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/integration/apps/archive \
  SMOKE_INVENTORY_LABEL=t6_after_delete_integration_apps_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh
```

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

The next T6 cleanup is not another broad archive sweep. The remaining clear
front is the held legacy root-smoke group from P381GE:

- `tools/smokes/archive/smoke_async_spawn.sh`
- `tools/smokes/curated_phi_invariants.sh`
- `tools/smokes/parity_quick.sh`
- `tools/smokes/unified_members.sh`

Those need root-tool lifecycle classification before deletion.
