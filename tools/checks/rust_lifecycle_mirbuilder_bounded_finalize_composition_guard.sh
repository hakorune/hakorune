#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_bounded_finalize_composition.py --check-reference --drift-probes

python3 - <<'PY'
import json
from pathlib import Path

plan = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bounded-finalize-composition-plan-v0.json").read_text())

if plan.get("kind") != "MirBuilderBoundedFinalizeCompositionPlanV1":
    raise SystemExit("unexpected bounded finalize plan kind")
if "FinalizeModuleComposition" not in (plan.get("available_capabilities") or []):
    raise SystemExit("FinalizeModuleComposition capability missing")
steps = [row.get("step") for row in plan.get("composition") or []]
for required in [
    "append_return_if_unterminated",
    "update_return_type_from_result",
    "type_propagation",
    "module_add_main_function",
    "inject_condition_fn_if_missing",
    "refresh_module_plans_subset",
    "return_module",
]:
    if required not in steps:
        raise SystemExit(f"missing finalize step: {required}")
condition = next(row for row in plan["composition"] if row.get("step") == "inject_condition_fn_if_missing")
if condition.get("required_by_source") is not True:
    raise SystemExit("condition_fn injection must be marked source-required")
non_claims = plan.get("non_claims") or {}
for key in [
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
output_contract=rust-lifecycle-mirbuilder-bounded-finalize-composition-guard-v0
bounded_finalize_composition_guard=green
capability=FinalizeModuleComposition
condition_fn_injection=source_required
full_finalize_module_claim=0
generated_hako_change=0
runtime_fallback=0
summary=ok
REPORT
