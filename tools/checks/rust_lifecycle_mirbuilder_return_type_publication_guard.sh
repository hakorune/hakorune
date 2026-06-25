#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_return_type_publication.py --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-type-publication-plan-v0.json").read_text())

if plan.get("kind") != "MirBuilderReturnTypePublicationPlanV1":
    raise SystemExit("unexpected return type publication plan kind")
if "ReturnTypePublication" not in (plan.get("available_capabilities") or []):
    raise SystemExit("ReturnTypePublication capability missing")
profile = plan.get("execution_profile") or {}
if profile.get("result_value_transport") != "ValueIdAsI64":
    raise SystemExit("result value transport must be ValueIdAsI64")
if profile.get("result_value_type") != "MirType::Integer":
    raise SystemExit("result value type must be MirType::Integer")
contract = plan.get("result_contract") or {}
if contract.get("signature_return_type") != "MirType::Integer":
    raise SystemExit("signature return type must be MirType::Integer")
if contract.get("source_value_type_owner") != "LiteralIntegerLowering":
    raise SystemExit("source value type owner must remain LiteralIntegerLowering")
non_claims = plan.get("non_claims") or {}
for key in [
    "module_take",
    "verify_typed_values",
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
output_contract=rust-lifecycle-mirbuilder-return-type-publication-guard-v0
return_type_publication_guard=green
capability=ReturnTypePublication
signature_return_type=MirType::Integer
module_take_claim=0
generated_hako_change=0
runtime_fallback=0
summary=ok
REPORT
