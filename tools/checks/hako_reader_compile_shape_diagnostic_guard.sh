#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-reader-compile-shape-diagnostic"
RUNNER="$ROOT/tools/perf/hako_reader_compile_shape.py"
REPORT="$(mktemp /tmp/hako-reader-compile-shape.XXXXXX.json)"
trap 'rm -f "$REPORT"' EXIT

cd "$ROOT"
cargo build -q --release --features vm-reference --bin hakorune
python3 "$RUNNER" \
  --bin "$ROOT/target/release/hakorune" \
  --timeout-sec 10 \
  --json-out "$REPORT" >/dev/null

python3 - "$REPORT" <<'PY'
import json
import sys
from pathlib import Path

report = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if report.get("contract") != "HakoReaderCompileShapeDiagnosticV0":
    raise SystemExit("unexpected diagnostic contract")
rows = {row["case"]: row for row in report["results"]}
required = {
    "baseline",
    "tracked_model",
    "tracked_root_reader",
    "tracked_leaf_reader",
    "tracked_child_reader",
    "tracked_call_reader",
    "tracked_stmt_reader",
    "tracked_direct_reader",
    "branch_1",
    "branch_4",
    "branch_8",
    "branch_12",
    "recursion_none",
    "recursion_direct",
    "recursion_helper",
    "recursion_loop",
    "import_0",
    "import_1",
    "import_3",
    "import_5",
    "combined_plain",
    "combined_extern",
}
missing = sorted(required - rows.keys())
if missing:
    raise SystemExit(f"missing diagnostic rows: {missing}")
for name in sorted(required):
    row = rows[name]
    if row["parse"]["status"] != "ok":
        raise SystemExit(f"parse boundary failed: {name}")
    if row["compile"]["status"] != "ok":
        raise SystemExit(f"compile boundary failed: {name}")
    if "build_module" not in row["compile"]["stages"]:
        raise SystemExit(f"missing MIR phase evidence: {name}")
print("fast_shape_matrix=green")
print("reader_execution_reached=0")
print("original_30s_reproduced=0")
print("summary=ok")
PY

echo "[$TAG] ok"
