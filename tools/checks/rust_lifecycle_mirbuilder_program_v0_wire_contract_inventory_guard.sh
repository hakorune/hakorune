#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-program-v0-wire-contract-inventory"
source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/program-v0-wire-contract-inventory-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/program_v0_wire_contract_inventory.py"
CARD="$ROOT/docs/development/current/main/investigations/mirbuilder-hako-bounded-body-analysis-snapshot-v0-2026-07-12.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if data.get("token") != "MIRBUILDER-PROGRAMV0-WIRE-CONTRACT-INVENTORY-V0-001":
    raise SystemExit("token drift")
rows = data.get("rows") or []
if not rows or any(row.get("classification") not in {"Accepted", "KnownUnsupported", "SchemaMismatchStop"} for row in rows):
    raise SystemExit("unclassified wire row")
keys = {(row["domain"], row["tag"]) for row in rows}
if len(keys) != len(rows):
    raise SystemExit("duplicate wire row")
required_stops = {
    ("stmt", "FastMemRegion"),
    ("expr", "Float"),
    ("expr", "BrandConstruct"),
    ("expr", "BrandUnwrap"),
    ("expr", "RecordField"),
    ("expr", "RecordLiteral"),
    ("expr", "RecordUpdate"),
}
actual_stops = {
    (row["domain"], row["tag"])
    for row in rows
    if row["classification"] == "SchemaMismatchStop"
}
if actual_stops != required_stops:
    raise SystemExit(f"schema mismatch stop drift: {sorted(actual_stops)}")
claims = data.get("claims") or {}
for key in (
    "snapshot_implementation_started",
    "program_json_schema_changed",
    "source_kind_recovery",
    "mir_or_id_allocation",
    "planner_or_route_authority",
):
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim drift: {key}")
parser = data.get("parser_seams") or {}
if parser.get("extra_fields") != "tolerated_and_discarded":
    raise SystemExit("extra-field seam drift")
if parser.get("unknown_duplicate_fields") != "unproven":
    raise SystemExit("duplicate-key seam drift")
print("output_contract=ProgramV0WireContractInventoryV0")
print("all_variants_classified=1")
print("schema_mismatch_stops=7")
print("snapshot_implementation_started=0")
print("summary=ok")
PY
