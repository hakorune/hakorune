#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-exact-exe-minimal-config-evidence"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-48-MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-EVIDENCE.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-47-MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_exact_exe_minimal_config_evidence_guard.sh"
TOOL="tools/allocator/hako_minimal_config_evidence.py"

echo "[$TAG] checking phase-295x exact-EXE minimal config evidence"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$TOOL"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$TOOL"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-EVIDENCE-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-EVIDENCE-295X-001' "$PREV_CARD" "previous row must select this evidence"
guard_expect_in_file "$TAG" 'hako-exact-exe-minimal-config-evidence-v0' "$CARD" "card must define evidence contract"
guard_expect_in_file "$TAG" 'hako-exact-exe-minimal-config-evidence-v0' "$TOOL" "tool must emit evidence contract"
guard_expect_in_file "$TAG" 'runtime_config_profile' "$TOOL" "tool must verify runtime config profile"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-CLOSEOUT-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_min_config_evidence.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.json"

python3 "$TOOL" --out "$report" \
  --workload representative-empty-v0 \
  --workload representative-small-block-v0 \
  --workload representative-realloc-aligned-v0

python3 - "$report" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))
if data.get("output_contract") != "hako-exact-exe-minimal-config-evidence-v0":
    raise SystemExit("bad output contract")
if data.get("winner_claim") != 0:
    raise SystemExit("winner claim must remain closed")
rows = data.get("rows", [])
if len(rows) != 3:
    raise SystemExit(f"expected 3 rows, got {len(rows)}")
for row in rows:
    workload = row["workload"]
    root = int(row["root_external_peak_rss_bytes"])
    empty = int(row["empty_external_peak_rss_bytes"])
    reduction = int(row["rss_reduction_bytes"])
    if root <= 0 or empty <= 0:
        raise SystemExit(f"{workload}: RSS values must be positive")
    if reduction != root - empty:
        raise SystemExit(f"{workload}: reduction arithmetic mismatch")
    if reduction <= 0:
        raise SystemExit(f"{workload}: minimal config must reduce RSS")
    print(
        "[phase295x-min-config-evidence] "
        f"workload={workload} "
        f"root_external_peak_rss_bytes={root} "
        f"empty_external_peak_rss_bytes={empty} "
        f"rss_reduction_bytes={reduction}"
    )
PY

echo "[$TAG] ok"
