#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

SCRIPT="tools/rust_lifecycle/mirbuilder_dev_birth_verification.py"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-dev-birth-verification-plan-v0.json"

python3 "$SCRIPT" --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

path = Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-dev-birth-verification-plan-v0.json")
plan = json.loads(path.read_text())

assert plan["kind"] == "MirBuilderDevBirthVerificationPlanV1"
assert "DevBirthVerification" in plan["available_capabilities"]
assert plan["execution_profile"]["function_transport"] == "MirFunctionPreparedMain"
assert plan["execution_profile"]["context"] == "finalize_module"
assert plan["guard_conditions"] == [
    "using_is_dev",
    "stageb_dev_verify_enabled",
    "cli_verbose_enabled",
]
assert plan["verification_steps"] == [
    "IterateFunctionBlocks",
    "ScanNewBoxInstructions",
    "SkipStageBDriverBox",
    "SkipStringBox",
    "ExpectBirthTailByBoxTypeAndArity",
    "LookAheadThreeInstructions",
    "AcceptMethodBirthOnSameReceiver",
    "AcceptConstStringGlobalCompatibilityPath",
    "WarnOnMissingBirth",
    "WarnSummaryWhenAnyMissing",
]
assert plan["result_contract"]["mutates"] == []
assert plan["result_contract"]["side_effect"] == "dev_warning_only"
assert plan["result_contract"]["entrypoint"] == "MirBuilder::finalize_module dev birth verification block"
assert plan["result_contract"]["minimal_path_expected_result"] == "NoErrorReturn"
for key in [
    "module_function_insertion",
    "condition_fn_injection",
    "all_functions_phi_materialization",
    "region_stack_pop",
    "slot_registry_release",
    "metadata_publication",
    "semantic_refresh",
    "full_finalize_module",
    "generated_hako_artifact",
    "runtime_fallback",
]:
    assert plan["non_claims"][key] == 0, key

print("output_contract=rust-lifecycle-mirbuilder-dev-birth-verification-guard-v0")
print("dev_birth_verification_guard=green")
print("capability=DevBirthVerification")
print(f"entrypoint={plan['result_contract']['entrypoint']}")
print("module_function_insertion_claim=0")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
