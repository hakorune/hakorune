#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_type_propagation_pipeline.py --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-propagation-pipeline-plan-v0.json").read_text())

if plan.get("kind") != "MirBuilderTypePropagationPipelinePlanV1":
    raise SystemExit("unexpected type propagation pipeline plan kind")
if "TypePropagationPipelineExecution" not in (plan.get("available_capabilities") or []):
    raise SystemExit("TypePropagationPipelineExecution capability missing")
profile = plan.get("execution_profile") or {}
if profile.get("function_transport") != "MirFunctionPreparedMain":
    raise SystemExit("function transport must be MirFunctionPreparedMain")
if profile.get("value_types") != "self.type_ctx.value_types":
    raise SystemExit("value_types must be self.type_ctx.value_types")
if plan.get("pipeline_steps") != [
    "seed_declared_field_types",
    "copy_propagation_initial",
    "binop_repropagation",
    "copy_propagation_after_binop",
    "phi_type_inference",
]:
    raise SystemExit("pipeline step order drift")
result = plan.get("result_contract") or {}
if result.get("entrypoint") != "TypePropagationPipeline::run":
    raise SystemExit("entrypoint drift")
if result.get("minimal_path_expected_result") != "Ok":
    raise SystemExit("minimal path expected result drift")
non_claims = plan.get("non_claims") or {}
for key in [
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
output_contract=rust-lifecycle-mirbuilder-type-propagation-pipeline-guard-v0
type_propagation_pipeline_guard=green
capability=TypePropagationPipelineExecution
entrypoint=TypePropagationPipeline::run
type_hint_provision_claim=0
generated_hako_change=0
runtime_fallback=0
summary=ok
REPORT
