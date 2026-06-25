#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_typed_value_verification.py --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-value-verification-plan-v0.json").read_text())

if plan.get("kind") != "MirBuilderTypedValueVerificationPlanV1":
    raise SystemExit("unexpected typed-value verification plan kind")
if "TypedValueDefinitionVerification" not in (plan.get("available_capabilities") or []):
    raise SystemExit("TypedValueDefinitionVerification capability missing")
profile = plan.get("execution_profile") or {}
if profile.get("current_function") != "Present":
    raise SystemExit("current_function must be Present")
contract = plan.get("verification_contract") or {}
if contract.get("typed_values") != "builder.type_ctx.value_types":
    raise SystemExit("typed_values source drift")
if contract.get("definition_sources") != ["compute_def_blocks(func)", "func.params"]:
    raise SystemExit("definition sources drift")
if contract.get("excluded_value") != "ValueId::INVALID":
    raise SystemExit("ValueId::INVALID exclusion drift")
if contract.get("fail_fast_tag") != "[freeze:contract][value_lifecycle/typed_without_def]":
    raise SystemExit("fail-fast tag drift")
result = plan.get("result_contract") or {}
if result.get("minimal_path_expected_result") != "Ok":
    raise SystemExit("minimal path expected result drift")
non_claims = plan.get("non_claims") or {}
for key in [
    "current_function_take",
    "type_propagation",
    "type_hint_provision",
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
output_contract=rust-lifecycle-mirbuilder-typed-value-verification-guard-v0
typed_value_verification_guard=green
capability=TypedValueDefinitionVerification
typed_values=builder.type_ctx.value_types
definition_sources=compute_def_blocks(func),func.params
current_function_take_claim=0
generated_hako_change=0
runtime_fallback=0
summary=ok
REPORT
