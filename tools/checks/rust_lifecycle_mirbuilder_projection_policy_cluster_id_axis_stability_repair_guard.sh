#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-projection-policy-cluster-id-axis-stability-repair-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CLUSTER_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json"
PRIORITY_FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-projection-policy-cluster-priority-resolution-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1921-MIRBUILDER-PROJECTION-POLICY-CLUSTER-ID-AXIS-STABILITY-REPAIR-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CLUSTER_FIXTURE" "$PRIORITY_FIXTURE" "$CARD"

python3 - <<'PY'
import json
from collections import Counter
from pathlib import Path

token = "MIRBUILDER-PROJECTION-POLICY-CLUSTER-ID-AXIS-STABILITY-REPAIR-001"
cluster_fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-missing-projection-policy-cluster-resolution-v0.json").read_text())
priority_fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-projection-policy-cluster-priority-resolution-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1921-MIRBUILDER-PROJECTION-POLICY-CLUSTER-ID-AXIS-STABILITY-REPAIR-001.md").read_text()

if token not in card:
    raise SystemExit("repair card token missing")

cluster_ids = [cluster["cluster_id"] for cluster in cluster_fixture["clusters"]]
duplicate_ids = [cluster_id for cluster_id, count in Counter(cluster_ids).items() if count > 1]
if duplicate_ids:
    raise SystemExit(f"duplicate cluster ids remain: {duplicate_ids[:3]}")

summary = cluster_fixture["summary"]
if summary["duplicate_cluster_id_count"] != 0:
    raise SystemExit("duplicate_cluster_id_count must be 0")
if summary["legacy_cluster_id_collision_count"] < 1:
    raise SystemExit("legacy_cluster_id_collision_count must preserve compatibility evidence")

for cluster in cluster_fixture["clusters"]:
    legacy = cluster.get("legacy_cluster_id")
    if not legacy:
        raise SystemExit("cluster missing legacy_cluster_id")
    if cluster["cluster_id"] == legacy:
        raise SystemExit("cluster_id must include axis qualifiers")
    for axis in ["borrow=", "control=", "type=", "call=", "verifier="]:
        if axis not in cluster["cluster_id"]:
            raise SystemExit(f"cluster_id missing axis qualifier: {axis}")

selected_cluster = priority_fixture["decision"]["selected_cluster_id"]
if selected_cluster and "borrow=" not in selected_cluster:
    raise SystemExit("priority selected cluster must use axis-qualified cluster_id")

claims = cluster_fixture["claims"]
if claims.get("cluster_id_is_unique") != 1:
    raise SystemExit("cluster_id_is_unique claim missing")
if claims.get("legacy_cluster_id_preserved") != 1:
    raise SystemExit("legacy_cluster_id_preserved claim missing")
for key in [
    "manual_family_selection",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "hako_emission",
    "hako_adopted_decision",
    "native_source_seed_materialization",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-projection-policy-cluster-id-axis-stability-repair
cluster_id_unique=1
legacy_cluster_id_preserved=1
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
