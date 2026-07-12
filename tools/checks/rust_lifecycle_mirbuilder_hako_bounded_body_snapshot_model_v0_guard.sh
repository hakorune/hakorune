#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-hako-bounded-body-snapshot-model-v0"
source "$ROOT/tools/checks/lib/guard_common.sh"

DIR="$ROOT/lang/src/compiler/analysis/bounded_body_snapshot"
FIXTURE="$ROOT/tools/checks/fixtures/bounded_body_snapshot_model_v0.hako"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_files "$TAG" "$DIR/outcome_v0.hako" "$DIR/path_v0.hako" \
  "$DIR/budget_v0.hako" "$DIR/snapshot_model_v0.hako" "$FIXTURE"

NYASH_DISABLE_PLUGINS=1 cargo run -q --features vm-reference --bin hakorune -- \
  --backend vm "$FIXTURE" >/tmp/hakorune-bounded-body-snapshot-model-v0.log 2>&1
grep -q 'RC: 0' /tmp/hakorune-bounded-body-snapshot-model-v0.log || {
  tail -n 80 /tmp/hakorune-bounded-body-snapshot-model-v0.log
  guard_fail "$TAG" "Hako snapshot model VM fixture failed"
}

python3 - "$DIR" <<'PY'
import sys
from pathlib import Path

root = Path(sys.argv[1])
sources = list(root.glob("*.hako"))
joined = "\n".join(path.read_text(encoding="utf-8") for path in sources)
for needle in ("Ready", "Unsupported", "InvalidInput", "$.body", "max_node_count", "max_total_text_bytes"):
    if needle not in joined:
        raise SystemExit(f"missing model contract: {needle}")
for forbidden in ("indexOf", "substring(", "MIRBuilder", "planner", "route"):
    if forbidden in joined:
        raise SystemExit(f"forbidden model dependency: {forbidden}")
for path in sources:
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")
print("output_contract=HakoBoundedBodySnapshotModelV0")
print("three_way_outcome=1")
print("structural_path=1")
print("schema_budget=1")
print("immutable_publication_model=1")
print("raw_json_reader=0")
print("summary=ok")
PY
