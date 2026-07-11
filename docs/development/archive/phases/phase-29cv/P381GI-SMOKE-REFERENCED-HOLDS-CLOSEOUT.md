# P381GI Smoke Referenced Holds Closeout

Date: 2026-05-06
Scope: close T6 after zero-ref v2 and legacy root smoke delete waves.

## Decision

T6 broad smoke deletion is closed for this lane.

The remaining smoke/archive surface is held because it is referenced, suite
protected, manual-archive governed, or needs an owner-specific lifecycle card.
Do not continue deleting by directory.

## Final T6 Counts

| Surface | Scripts | Closeout reading |
| --- | ---: | --- |
| `tools/smokes/v2/profiles/archive` | 77 | 21 report-orphan rows remain, but they require owner/ref cleanup before deletion |
| `tools/smokes/v2/profiles/integration/archive` | 13 | suite protected |
| `tools/smokes/v2/profiles/integration/apps/archive` | 184 | 176 report-orphan rows remain, but they have refs or owner holds |
| legacy `tools/smokes` outside `v2` | 10 | referenced/owner-held |
| `tools/archive/manual-smokes` | 35 | manual archive policy governs deletion |
| `tools/smokes/v2/profiles` | 1419 | post T6 zero-ref deletion baseline |

## Report Snapshots

`tools/smokes/v2/profiles/archive`:

```text
Total: 77
Referenced: 56
Orphan candidates: 21
  - Wrapper-only orphan candidates: 0
```

`tools/smokes/v2/profiles/integration/apps/archive`:

```text
Total: 184
Referenced: 8
Orphan candidates: 176
  - Wrapper-only orphan candidates: 0
```

These remaining orphan rows are report-orphans, not delete approvals. P381GC's
first-wave rule still applies: no full-path refs, no basename refs outside
accepted historical/archive docs, no suite membership, no guard/wrapper manifest
execution, and an owning SSOT that classifies the script as deletable.

## What Landed In T6

- P381GC locked the five bucket inventory and blocked directory-level deletion.
- P381GD fixed inventory summary class-column reads.
- P381GE produced the first zero-ref v2 archive candidate list.
- P381GF deleted only those 45 v2 archive scripts.
- P381GG classified four legacy root-smoke zero-ref scripts.
- P381GH deleted only those four legacy root-smoke scripts.

## Validation

```bash
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/archive \
  SMOKE_INVENTORY_LABEL=t6_closeout_profiles_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh

SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/integration/apps/archive \
  SMOKE_INVENTORY_LABEL=t6_closeout_integration_apps_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh
```

```bash
find tools/smokes/v2/profiles/archive -type f -name '*.sh' | wc -l
find tools/smokes/v2/profiles/integration/archive -type f -name '*.sh' | wc -l
find tools/smokes/v2/profiles/integration/apps/archive -type f -name '*.sh' | wc -l
find tools/smokes -path tools/smokes/v2 -prune -o -type f -name '*.sh' -print | wc -l
find tools/archive/manual-smokes -type f -name '*.sh' | wc -l
find tools/smokes/v2/profiles -type f -name '*.sh' | wc -l
```

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

The remaining phase-29cv work is optional polish, not another must-fix smoke
deletion wave:

- doc compaction / mirror thinning
- targeted helper dedup only when a local owner seam is clear
