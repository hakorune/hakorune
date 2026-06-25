#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_return_emission.py --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-plan-v0.json").read_text())

if plan.get("kind") != "MirBuilderReturnEmissionPlanV1":
    raise SystemExit("unexpected return emission plan kind")
if "ReturnEmission" not in (plan.get("available_capabilities") or []):
    raise SystemExit("ReturnEmission capability missing")
profile = plan.get("execution_profile") or {}
if profile.get("result_value_transport") != "ValueIdAsI64":
    raise SystemExit("result value transport must be ValueIdAsI64")
contract = plan.get("result_contract") or {}
if contract.get("terminator") != "MirInstruction::Return":
    raise SystemExit("return emission must publish Return terminator")
if contract.get("value") != "Some(result_value)":
    raise SystemExit("return emission must connect result_value")
if contract.get("successors") != "Empty":
    raise SystemExit("Return terminator must have no CFG successors")
non_claims = plan.get("non_claims") or {}
for key in [
    "return_type_publication",
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
output_contract=rust-lifecycle-mirbuilder-return-emission-guard-v0
return_emission_guard=green
capability=ReturnEmission
terminator=MirInstruction::Return
value_transport=ValueIdAsI64
return_type_publication_claim=0
generated_hako_change=0
runtime_fallback=0
summary=ok
REPORT
