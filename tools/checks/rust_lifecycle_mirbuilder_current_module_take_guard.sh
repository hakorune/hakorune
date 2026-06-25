#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_current_module_take.py --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-current-module-take-plan-v0.json").read_text())

if plan.get("kind") != "MirBuilderCurrentModuleTakePlanV1":
    raise SystemExit("unexpected current module take plan kind")
if "CurrentModuleTake" not in (plan.get("available_capabilities") or []):
    raise SystemExit("CurrentModuleTake capability missing")
profile = plan.get("execution_profile") or {}
if profile.get("module_transport") != "MirModuleMinimalShell":
    raise SystemExit("module transport must be MirModuleMinimalShell")
contract = plan.get("result_contract") or {}
if contract.get("taken_value") != "MirModuleMinimalShell":
    raise SystemExit("taken value must be MirModuleMinimalShell")
if contract.get("post_take_state") != "None":
    raise SystemExit("post-take state must be None")
non_claims = plan.get("non_claims") or {}
for key in [
    "verify_typed_values",
    "current_function_take",
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
output_contract=rust-lifecycle-mirbuilder-current-module-take-guard-v0
current_module_take_guard=green
capability=CurrentModuleTake
module_transport=MirModuleMinimalShell
verify_typed_values_claim=0
generated_hako_change=0
runtime_fallback=0
summary=ok
REPORT
