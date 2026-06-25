#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_current_function_take.py --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-current-function-take-plan-v0.json").read_text())

if plan.get("kind") != "MirBuilderCurrentFunctionTakePlanV1":
    raise SystemExit("unexpected current function take plan kind")
if "CurrentFunctionTake" not in (plan.get("available_capabilities") or []):
    raise SystemExit("CurrentFunctionTake capability missing")
profile = plan.get("execution_profile") or {}
if profile.get("function_transport") != "MirFunctionPreparedMain":
    raise SystemExit("function transport must be MirFunctionPreparedMain")
contract = plan.get("result_contract") or {}
if contract.get("taken_value") != "MirFunctionPreparedMain":
    raise SystemExit("taken value must be MirFunctionPreparedMain")
if contract.get("post_take_state") != "None":
    raise SystemExit("post-take state must be None")
if contract.get("local_binding") != "function":
    raise SystemExit("local binding must be function")
non_claims = plan.get("non_claims") or {}
for key in [
    "type_propagation",
    "type_hint_provision",
    "metadata_value_type_publication",
    "phi_return_type_inference",
    "full_finalize_module",
    "generated_hako_artifact",
    "backend_route_changed",
    "abi_changed",
    "runtime_fallback",
    "mainline_selected",
]:
    if non_claims.get(key) != 0:
        raise SystemExit(f"non-claim must remain 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-current-function-take-guard-v0
current_function_take_guard=green
capability=CurrentFunctionTake
function_transport=MirFunctionPreparedMain
type_propagation_claim=0
generated_hako_change=0
runtime_fallback=0
summary=ok
REPORT
