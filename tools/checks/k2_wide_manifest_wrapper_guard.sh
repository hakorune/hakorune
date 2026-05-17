#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-manifest-wrapper-guard"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

MANIFEST="tools/checks/guard_rows.toml"
DESIGN="docs/development/current/main/design/guard-manifest-migration-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-577-GUARD-MANIFEST-003-SEGMENT-CLOSEOUT-THIN-WRAPPERS.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$MANIFEST" "$DESIGN" "$CARD"
guard_require_exec_files "$TAG" "$0"

guard_expect_in_file "$TAG" "tools/checks/impl" "$DESIGN" "migration SSOT must name implementation command location"
guard_expect_in_file "$TAG" "k2_wide_manifest_wrapper_guard.sh" "$CARD" "GM003 card must name this guard"

python3 - "$ROOT_DIR" "$MANIFEST" <<'PY'
import os
import pathlib
import sys
import tomllib

root = pathlib.Path(sys.argv[1]).resolve()
manifest_path = root / sys.argv[2]
data = tomllib.loads(manifest_path.read_text(encoding="utf-8"))

expected = {
    "hako-alloc-segment-arena-bitmap-closeout": {
        "wrapper": "tools/checks/k2_wide_hako_alloc_segment_arena_bitmap_inventory_closeout_guard.sh",
        "impl": "tools/checks/impl/k2_wide_hako_alloc_segment_arena_bitmap_inventory_closeout_guard.sh",
    },
    "hako-alloc-segment-lifecycle-closeout": {
        "wrapper": "tools/checks/k2_wide_hako_alloc_segment_lifecycle_scalar_state_closeout_guard.sh",
        "impl": "tools/checks/impl/k2_wide_hako_alloc_segment_lifecycle_scalar_state_closeout_guard.sh",
    },
    "hako-alloc-segment-page-membership-closeout": {
        "wrapper": "tools/checks/k2_wide_hako_alloc_segment_page_membership_scalar_closeout_guard.sh",
        "impl": "tools/checks/impl/k2_wide_hako_alloc_segment_page_membership_scalar_closeout_guard.sh",
    },
    "hako-alloc-reclaim-scheduler-marker-closeout": {
        "wrapper": "tools/checks/k2_wide_hako_alloc_reclaim_scheduler_marker_closeout_guard.sh",
        "impl": "tools/checks/impl/k2_wide_hako_alloc_reclaim_scheduler_marker_closeout_guard.sh",
    },
    "hako-alloc-reclaim-scheduler-ledger-closeout": {
        "wrapper": "tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_closeout_guard.sh",
        "impl": "tools/checks/impl/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_closeout_guard.sh",
    },
    "hako-alloc-reclaim-scheduler-ledger-consume-closeout": {
        "wrapper": "tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_closeout_guard.sh",
        "impl": "tools/checks/impl/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_closeout_guard.sh",
    },
    "hako-alloc-reclaim-scheduler-ledger-roundtrip-closeout": {
        "wrapper": "tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_closeout_guard.sh",
        "impl": "tools/checks/impl/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_closeout_guard.sh",
    },
    "hako-alloc-reclaim-scheduler-scalar-lane-closeout": {
        "wrapper": "tools/checks/k2_wide_hako_alloc_reclaim_scheduler_scalar_lane_closeout_guard.sh",
        "impl": "tools/checks/impl/k2_wide_hako_alloc_reclaim_scheduler_scalar_lane_closeout_guard.sh",
    },
}

rows = data.get("rows")
if not isinstance(rows, list):
    raise SystemExit("guard_rows.toml must contain [[rows]] entries")

by_id = {}
for row in rows:
    if isinstance(row, dict) and isinstance(row.get("id"), str):
        by_id[row["id"]] = row

errors: list[str] = []
for row_id, spec in expected.items():
    row = by_id.get(row_id)
    if row is None:
        errors.append(f"missing manifest row: {row_id}")
        continue
    expected_cmd = ["bash", spec["impl"]]
    if row.get("cmd") != expected_cmd:
        errors.append(f"{row_id}: cmd must be {expected_cmd!r}, got {row.get('cmd')!r}")

    wrapper = root / spec["wrapper"]
    impl = root / spec["impl"]
    if not wrapper.is_file():
        errors.append(f"{row_id}: wrapper missing: {spec['wrapper']}")
        continue
    if not impl.is_file():
        errors.append(f"{row_id}: implementation command missing: {spec['impl']}")
        continue
    if not os.access(wrapper, os.X_OK):
        errors.append(f"{row_id}: wrapper is not executable: {spec['wrapper']}")
    if not os.access(impl, os.X_OK):
        errors.append(f"{row_id}: implementation command is not executable: {spec['impl']}")

    text = wrapper.read_text(encoding="utf-8")
    if f"run_row_guard.sh\" --only {row_id}" not in text:
        errors.append(f"{row_id}: wrapper must delegate to run_row_guard.sh --only {row_id}")
    forbidden = [
        "guard_common.sh",
        "pure_first_exe_guard.sh",
        "guard_expect_in_file",
        "guard_require_files",
        "mktemp",
        "rg -n",
        "python3 -",
    ]
    for marker in forbidden:
        if marker in text:
            errors.append(f"{row_id}: wrapper regrew embedded guard body marker: {marker}")

if errors:
    for error in errors:
        print(f"[k2-wide-manifest-wrapper-guard] ERROR: {error}", file=sys.stderr)
    raise SystemExit(1)

print(f"[k2-wide-manifest-wrapper-guard] checked={len(expected)}")
PY

echo "[$TAG] ok"
