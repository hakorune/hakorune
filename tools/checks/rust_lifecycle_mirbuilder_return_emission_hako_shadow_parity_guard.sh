#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-return-emission-hako-shadow-parity-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

GENERATOR="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_return_emission_artifacts.py"
PLAN_TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_return_emission.py"
PROJECTOR="$ROOT_DIR/lang/src/compiler/lib/return_emission_projector.hako"
SUPPORT="$ROOT_DIR/lang/src/compiler/lib/projector_support.hako"
README="$ROOT_DIR/lang/src/compiler/lib/README.md"
PLAN="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-plan-v0.json"
ORACLE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-derived-hako-oracle-v0.json"
SHADOW_RESULT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-hako-shadow-result-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$ROOT_DIR/tools/bin/hako" "$GENERATOR" "$PLAN_TOOL" "$PROJECTOR" "$SUPPORT" "$README" "$PLAN" "$ORACLE" "$SHADOW_RESULT"

python3 "$GENERATOR" --check
python3 "$PLAN_TOOL" --check-reference --drift-probes
bash "$ROOT_DIR/tools/bin/hako" --backend mir --verify "$PROJECTOR"

python3 - "$PROJECTOR" "$SUPPORT" "$README" "$PLAN" "$ORACLE" "$SHADOW_RESULT" <<'PY'
from __future__ import annotations

import json
from pathlib import Path
import sys

projector = Path(sys.argv[1]).read_text(encoding="utf-8")
support = Path(sys.argv[2]).read_text(encoding="utf-8")
readme = Path(sys.argv[3]).read_text(encoding="utf-8")
plan = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))
oracle = json.loads(Path(sys.argv[5]).read_text(encoding="utf-8"))
shadow_result = json.loads(Path(sys.argv[6]).read_text(encoding="utf-8"))

required_projector_text = [
    "ReturnEmissionHakoProjector",
    "project_shadow_record(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token)",
    "project_shadow_json(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token)",
    'CompilerProjectionValueBox.object_set(shadow_record, "kind", CompilerProjectionValueBox.create_string("ReturnEmissionHakoShadowProjectorV1"))',
    'CompilerProjectionValueBox.object_set(shadow_record, "family_id", CompilerProjectionValueBox.create_string("hakorune_mir_builder::return_emission"))',
    'CompilerProjectionValueBox.object_set(shadow_record, "stage_id", CompilerProjectionValueBox.create_string("return_emission"))',
    'CompilerProjectorSupportBox.ok(shadow_record)',
]
for needle in required_projector_text:
    if needle not in projector:
        raise SystemExit(f"missing return emission projector text: {needle}")

required_support_text = [
    'CanonicalJsonWriterBox.canonicalize(shadow_record)',
    'ok(shadow_record)',
]
for needle in required_support_text:
    if needle not in support:
        raise SystemExit(f"missing projector support text: {needle}")

if "return_emission_projector.hako" not in readme:
    raise SystemExit("README must keep the return emission projector landing zone visible")

if plan.get("kind") != "MirBuilderReturnEmissionPlanV1":
    raise SystemExit("plan kind drift")
if "ReturnEmission" not in (plan.get("available_capabilities") or []):
    raise SystemExit("ReturnEmission capability missing")

result = shadow_result.get("result") or {}
shadow_record = result.get("shadow_record") or {}
if shadow_result.get("kind") != "MirBuilderReturnEmissionDerivedHakoShadowResultV1":
    raise SystemExit("shadow result kind drift")
if shadow_result.get("subject") != plan.get("subject"):
    raise SystemExit("shadow result subject drift")
if result.get("err") != 0 or result.get("err_line") != "":
    raise SystemExit("shadow result must be green")
if shadow_record.get("kind") != "ReturnEmissionShadowCandidateV1":
    raise SystemExit("shadow record kind drift")
if shadow_record.get("family_id") != "hakorune_mir_builder::return_emission":
    raise SystemExit("shadow record family drift")
if shadow_record.get("stage_id") != "return_emission":
    raise SystemExit("shadow record stage drift")
if shadow_record.get("subject") != plan.get("subject"):
    raise SystemExit("shadow record subject drift")
if shadow_record.get("source_authority") != "src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module":
    raise SystemExit("shadow record source authority drift")
execution_profile = shadow_record.get("execution_profile") or {}
if execution_profile.get("result_value_transport") != "ValueIdAsI64":
    raise SystemExit("shadow record transport drift")
if execution_profile.get("target_block_terminated") is not False:
    raise SystemExit("shadow record target block state drift")
result_contract = shadow_record.get("result_contract") or {}
expected_contract = {
    "terminator": "MirInstruction::Return",
    "value": "Some(result_value)",
    "value_transport": "ValueIdAsI64",
    "successors": "Empty",
}
for key, expected in expected_contract.items():
    if result_contract.get(key) != expected:
        raise SystemExit(f"shadow record contract drift: {key}={result_contract.get(key)}")
shadow_json = result.get("shadow_json")
if shadow_json != json.dumps(shadow_record, indent=2, sort_keys=True) + "\n":
    raise SystemExit("shadow result canonical JSON drift")
non_claims = shadow_result.get("non_claims") or {}
for key in [
    "return_type_publication",
    "full_finalize_module",
    "already_terminated_block_behavior",
    "mainline_selected",
    "runtime_fallback",
]:
    if non_claims.get(key) != 0:
        raise SystemExit(f"shadow result non-claim must remain 0: {key}")

print("output_contract=rust-lifecycle-mirbuilder-return-emission-hako-shadow-parity-guard-v0")
print("return_emission_hako_shadow_parity=green")
print("projector=ReturnEmissionHakoProjector")
print("canonical_json_parity=green")
print("generated_hako_change=0")
print("runtime_fallback=0")
print("summary=ok")
PY
