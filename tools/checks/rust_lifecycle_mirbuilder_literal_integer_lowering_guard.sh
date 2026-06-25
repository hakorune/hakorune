#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_literal_integer_lowering.py --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-literal-integer-lowering-plan-v0.json").read_text())

if plan.get("kind") != "MirBuilderLiteralIntegerLoweringPlanV1":
    raise SystemExit("unexpected literal integer plan kind")
if "LiteralIntegerLowering" not in (plan.get("available_capabilities") or []):
    raise SystemExit("LiteralIntegerLowering capability missing")
shape = plan.get("selected_source_shape") or {}
if shape.get("literal_payload_transport") != "ScalarI64":
    raise SystemExit("literal payload transport must be ScalarI64")
contract = plan.get("result_contract") or {}
if contract.get("emitted_instruction") != "ConstValue::Integer":
    raise SystemExit("literal lowering must emit ConstValue::Integer")
if contract.get("published_type") != "MirType::Integer":
    raise SystemExit("literal lowering must publish MirType::Integer")
non_claims = plan.get("non_claims") or {}
for key in [
    "return_emission",
    "generated_hako_artifact",
    "backend_route_changed",
    "abi_changed",
    "runtime_fallback",
]:
    if non_claims.get(key) != 0:
        raise SystemExit(f"non-claim must remain 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-literal-integer-lowering-guard-v0
literal_integer_lowering_guard=green
capability=LiteralIntegerLowering
value_transport=ScalarI64
published_type=MirType::Integer
generated_hako_change=0
runtime_fallback=0
summary=ok
REPORT
