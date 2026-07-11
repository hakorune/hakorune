# P381GD Smoke Inventory Report Class Column Fix

Date: 2026-05-06
Scope: T6 smoke/archive inventory tooling correctness before deletion.

## Problem

P381GC found that `tools/checks/smoke_inventory_report.sh` writes this TSV
schema:

```text
path family suffix fullpath_ref_count basename_ref_count wrapper_only suite_hit_count suite_names class
```

The summary section counted orphan classes from column 7. Column 7 is
`suite_hit_count`; `class` is column 9. That made summary orphan counts depend
on suite membership instead of the TSV class field.

## Change

- Added explicit `FAMILY_COLUMN=2` and `CLASS_COLUMN=9` constants.
- Updated summary orphan counts to read `CLASS_COLUMN`.
- Updated top orphan family summaries to read `CLASS_COLUMN` and
  `FAMILY_COLUMN`.

The TSV schema and per-row class production are unchanged. This is tooling
correctness only; it does not delete or move smoke scripts.

## Validation

```bash
bash -n tools/checks/smoke_inventory_report.sh
```

Generated fixture proving the summary reads the `class` column:

```bash
tmp_dir=$(mktemp -d target/smoke_inventory_column_fixture.XXXXXX)
trap 'rm -rf "$tmp_dir"' EXIT
printf '#!/usr/bin/env bash\ntrue\n' > "$tmp_dir/phase999_plain_vm.sh"
printf '#!/usr/bin/env bash\nexec "./phase999_plain_vm.sh"\n' > "$tmp_dir/phase999_wrapper_vm.sh"
SMOKE_INVENTORY_DIR="$tmp_dir" \
  SMOKE_INVENTORY_LABEL=column_fixture \
  bash tools/checks/smoke_inventory_report.sh
grep -q '^Total: 2$' target/smoke_inventory/column_fixture_summary.txt
grep -q '^Orphan candidates: 2$' target/smoke_inventory/column_fixture_summary.txt
grep -q '^  - Wrapper-only orphan candidates: 1$' \
  target/smoke_inventory/column_fixture_summary.txt
```

Archive bucket sanity check:

```bash
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/integration/archive \
  SMOKE_INVENTORY_LABEL=post_fix_integration_archive_incl \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh
```

Observed summary:

```text
Total: 13
Referenced: 13
Orphan candidates: 0
  - Wrapper-only orphan candidates: 0
phase29ck-boundary-legacy 12/12 (missing 0)
phase29x-derust-archive 1/1 (missing 0)
```

## Result

The inventory report summary now agrees with the TSV `class` column. T6 can move
from tooling correctness to per-script deletion-candidate classification.

Next concrete T6 slice:

```text
run the five bucket inventories and produce a per-script delete-candidate list
```

Deletion is still blocked until candidates also satisfy the P381GC first-wave
rule: no full-path refs, no basename refs outside accepted historical/archive
docs, no suite membership, no guard/wrapper manifest execution, and an owning
SSOT that classifies the script as deletable.
