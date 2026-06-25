#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py --check

python3 - <<'PY'
import json
from pathlib import Path

plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-plan-v0.json").read_text())
result = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-first-red-edge-result-v0.json").read_text())

if plan.get("kind") != "MinimalMirBuilderExecutionPathPlanV1":
    raise SystemExit("unexpected plan kind")
if result.get("kind") != "MinimalMirBuilderFirstRedEdgeResultV1":
    raise SystemExit("unexpected result kind")
if "first_unsupported_edge" in plan:
    raise SystemExit("plan must not handwrite first_unsupported_edge")
profile = plan.get("execution_profile") or {}
if profile.get("kind") != "PreparedMirBuilderStateV1":
    raise SystemExit("entry is not prepared-state")
if profile.get("runtime_fallback") is not False:
    raise SystemExit("runtime fallback must be false")
if plan.get("explicit_non_claims", {}).get("bundle_size_as_proof") != 0:
    raise SystemExit("bundle size cannot be a proof")

first = result.get("first_unsupported_edge") or {}
expected = {
    "callsite": "MirBuilder::finalize_module -> take current_module",
    "deny_reason": "UnsupportedTypeTransport",
    "deny_detail": "CurrentModuleTakeRequired",
    "semantic_owner": "MirBuilder::finalize_module current_module take",
    "next_slice_token": "MIRBUILDER-CURRENT-MODULE-TAKE-001",
}
for key, value in expected.items():
    if first.get(key) != value:
        raise SystemExit(f"first unsupported edge expected {key}={value}, got {first.get(key)}")

reached = result.get("reached_prefix") or []
statuses = [row.get("status") for row in reached]
if statuses != ["Available", "Available", "Available", "ProfileExcluded", "Available", "Available", "Available", "Available", "Available", "Available", "Available", "Available", "Available", "Unsupported"]:
    raise SystemExit(f"unexpected reached statuses: {statuses}")
for row in reached:
    if row.get("status") == "Available" and "contract_reference" in row:
        contract = row["contract_reference"]
        if not contract.get("manifest_path") or not contract.get("family_id"):
            raise SystemExit("available contract edge lacks manifest/family reference")
for row in result.get("not_reached_edges") or []:
    if row.get("status") != "NotReached":
        raise SystemExit("post-frontier edge is not NotReached")

claims = result.get("claims") or {}
if claims.get("first_edge_result_is_derived") != 1:
    raise SystemExit("first edge result is not marked derived")
if claims.get("generated_hako_change") != 0:
    raise SystemExit("selection must not change generated Hako")
if claims.get("new_backend_route") != 0:
    raise SystemExit("selection must not add backend routes")

# Report is derived from the result fixture (no hardcoded echo) so it never
# goes stale when the frontier edge changes.
print("output_contract=rust-lifecycle-mirbuilder-minimal-execution-path-selection-guard-v0")
print("selection_guard=green")
print("entry_is_prepared_state=1")
print(f"first_unsupported_edge={first['callsite']}")
print(f"deny_detail={first['deny_detail']}")
print(f"next_slice_token={first['next_slice_token']}")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
