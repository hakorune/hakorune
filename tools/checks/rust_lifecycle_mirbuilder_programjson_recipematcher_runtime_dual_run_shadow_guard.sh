#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-programjson-recipematcher-runtime-dual-run-shadow-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-recipematcher-runtime-dual-run-shadow-guard-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/3231-MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-DUAL-RUN-SHADOW-GUARD-001.md"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
CURRENT_STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
SHADOW_GATE="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_shadow_parity_gate.sh"
DESIGN_STOP_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_runtime_route_shadow_switch_design_stop_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$SHADOW_GATE" "$DESIGN_STOP_GUARD"

SHADOW_OUT="$(guard_cached_run "$TAG" bash "$SHADOW_GATE")"
if ! grep -q '^recipe_matcher_shadow_parity=1$' <<<"$SHADOW_OUT"; then
  printf '%s\n' "$SHADOW_OUT" >&2
  guard_fail "$TAG" "RecipeMatcher shadow parity prerequisite is not green"
fi
if ! grep -q '^matcher_result_equal=1$' <<<"$SHADOW_OUT"; then
  printf '%s\n' "$SHADOW_OUT" >&2
  guard_fail "$TAG" "RecipeMatcher shadow parity did not prove equal matcher results"
fi

DESIGN_OUT="$(guard_cached_run "$TAG" bash "$DESIGN_STOP_GUARD")"
if ! grep -q '^design_stop=1$' <<<"$DESIGN_OUT"; then
  printf '%s\n' "$DESIGN_OUT" >&2
  guard_fail "$TAG" "runtime route shadow-switch design stop prerequisite is not green"
fi
if ! grep -q '^recommended_default=A_SHADOW_ONLY_DUAL_RUN_GUARD$' <<<"$DESIGN_OUT"; then
  printf '%s\n' "$DESIGN_OUT" >&2
  guard_fail "$TAG" "design stop does not recommend the shadow-only dual-run guard"
fi

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$CURRENT_STATE" "$SHADOW_OUT" <<'PY'
import json
import sys
from pathlib import Path

fixture_path, card_path, task_order_path, current_state_path, shadow_out = sys.argv[1:]
fixture = json.loads(Path(fixture_path).read_text(encoding="utf-8"))
card = Path(card_path).read_text(encoding="utf-8")
task_order = Path(task_order_path).read_text(encoding="utf-8")
current_state = Path(current_state_path).read_text(encoding="utf-8")

token = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-DUAL-RUN-SHADOW-GUARD-001"
if fixture.get("kind") != "MirBuilderProgramJsonRecipeMatcherRuntimeDualRunShadowGuardV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != token:
    raise SystemExit("bad fixture token")

contract = fixture.get("dual_run_contract") or {}
required_contract = {
    "mode": "shadow_only_dual_run",
    "runtime_authority": "Rust ASTNode route",
    "shadow_route": "ProgramJSON matcher result",
    "comparison": "canonical matcher result fields",
    "mismatch_policy": "fail_fast_gate_only",
    "runtime_route_switch": False,
    "programjson_runtime_route_authority": False,
    "recipe_matcher_input_authority": False,
    "route_selection": False,
    "mir_lowering": False,
}
for key, expected in required_contract.items():
    if contract.get(key) != expected:
        raise SystemExit(f"dual-run contract drift: {key}")

guard_contract = fixture.get("guard_contract") or {}
for key in [
    "aot_required",
    "shadow_parity_gate_required",
    "design_stop_guard_required",
    "rust_authority_route_required",
    "programjson_shadow_route_required",
    "canonical_field_compare_required",
    "mismatch_fails_gate",
    "no_runtime_route_switch",
    "no_programjson_runtime_authority",
    "no_hidden_route_selection",
    "no_runtime_fallback",
]:
    if guard_contract.get(key) is not True:
        raise SystemExit(f"guard contract missing true: {key}")
if guard_contract.get("vm_only_main_acceptance") is not False:
    raise SystemExit("VM-only main acceptance must stay false")

expected_rows = {
    "local_loop_body_if_branch_return",
    "local_loop_body_if_branch_return_alt_names",
}
rows = fixture.get("rows") or []
if {row.get("row_id") for row in rows} != expected_rows:
    raise SystemExit("unexpected dual-run rows")
canonical = {
    "matched": 1,
    "contract_kind": "LoopWithExit",
    "has_break": 0,
    "has_continue": 0,
    "has_return": 1,
}
for row in rows:
    if row.get("authority_expected") != canonical:
        raise SystemExit(f"authority expected drift: {row.get('row_id')}")
    if row.get("shadow_expected") != canonical:
        raise SystemExit(f"shadow expected drift: {row.get('row_id')}")
    if row.get("parity", {}).get("dual_run_match") != 1:
        raise SystemExit(f"missing dual-run parity marker: {row.get('row_id')}")

claims = fixture.get("claims") or {}
for key in [
    "dual_run_shadow_guard",
    "runtime_authority_remains_rust_astnode",
    "programjson_shadow_checked",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"missing positive claim: {key}")
for key, value in claims.items():
    if key in {
        "dual_run_shadow_guard",
        "runtime_authority_remains_rust_astnode",
        "programjson_shadow_checked",
    }:
        continue
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")

for key in [
    "runtime_route_switch",
    "route_selection",
    "mir_lowering",
    "mir_mutation",
    "id_allocation",
    "runtime_fallback",
    "source_selfhost_claim",
]:
    if f"{key}=0" not in shadow_out:
        raise SystemExit(f"shadow prerequisite missing forbidden zero: {key}")

for needle in [
    token,
    "runtime_authority=rust_astnode",
    "programjson_runtime_route_authority=0",
    "runtime_route_switch=0",
]:
    if needle not in card:
        raise SystemExit(f"card missing: {needle}")
for needle in [
    token,
    "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-EXPANDED-ROWS-001",
]:
    if needle not in task_order:
        raise SystemExit(f"task-order missing: {needle}")
allowed_latest = [
    f'latest_card = "{token}"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-EXPANDED-ROWS-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-CONSULTATION-002"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001"',
    'latest_card = "GUARD-CACHE-EMIT-EXE-AND-DIRTY-MEMO-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-COVERAGE-FLOOR-SELECTION-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ACCEPTED-FLOOR-MATRIX-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-CONTINUE-PRESENT-ROW-SHAPE-DESIGN-STOP-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-LOOP-BODY-IFCONTINUE-IFRETURN-ASSIGNMENT-BOXCOUNT-ACCEPTED-FLOOR-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-BREAK-PRESENT-VERIFIED-RECIPE-SUPPORT-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-BREAK-CONTINUE-PRESENT-VERIFIED-RECIPE-SUPPORT-001"',
    'latest_card = "MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-DECISION-ROW-001"',
]
if not any(needle in current_state for needle in allowed_latest):
    raise SystemExit("CURRENT_STATE latest card drift")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-programjson-recipematcher-runtime-dual-run-shadow-guard-v0
token=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-DUAL-RUN-SHADOW-GUARD-001
owner=ProgramJsonRecipeMatcherExecutionBoundaryBox
row_count=2
dual_run_shadow_guard=1
runtime_authority=rust_astnode
programjson_shadow_checked=1
dual_run_match=1
mismatch_count=0
mismatch_policy=fail_fast_gate_only
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
new_backend_route=0
new_abi=0
vm_only_proof_as_main_acceptance=0
summary=ok
REPORT
