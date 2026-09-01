#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="coreplan-varmap-boundary"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

TASKBOARD="docs/development/current/main/workstreams/compiler-foundation-current.md"
LOCAL_PATCH_SSOT="docs/development/current/main/design/local-patch-prevention-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1022-COREPLAN-VARMAP-BOUNDARY-001.md"
CURRENT_CARD="docs/development/current/main/investigations/mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE_STEPS="tools/checks/lib/dev_gate_quick_steps.sh"
SELF_SCRIPT="tools/checks/coreplan_varmap_boundary_inventory_guard.sh"

echo "[$TAG] checking CorePlan variable_map write boundary inventory"

guard_require_files \
  "$TAG" \
  "$TASKBOARD" \
  "$LOCAL_PATCH_SSOT" \
  "$CARD" \
  "$CURRENT_CARD" \
  "$INDEX" \
  "$DEV_GATE_STEPS" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" \
  "COREPLAN-VARMAP-BOUNDARY-001" \
  "$TASKBOARD" \
  "taskboard must record variable_map boundary inventory sidecar"
guard_expect_fixed_in_file "$TAG" \
  "COREPLAN-VARMAP-BOUNDARY-001" \
  "$CARD" \
  "phase card must record variable_map boundary inventory sidecar"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$INDEX" \
  "check index must list this guard"
guard_expect_fixed_in_file "$TAG" \
  "$SELF_SCRIPT" \
  "$DEV_GATE_STEPS" \
  "dev_gate quick must include this guard"
guard_expect_fixed_in_file "$TAG" \
  "variable_map_role=defined_value_emission_cache" \
  "$TASKBOARD" \
  "taskboard must restate variable_map role"
guard_expect_fixed_in_file "$TAG" \
  "DEV-GATE-COREPLAN-VARMAP-ROLE-CENSUS-PRUNE-R0" \
  "$CURRENT_CARD" \
  "active card must record the role-aware variable_map census row"

python3 - <<'PY'
import re
from pathlib import Path

roots = [
    Path("src/mir/builder/control_flow/plan"),
    Path("src/mir/builder/ssa"),
]
pattern = re.compile(r"variable_map\s*\.\s*(insert|remove|clear)\s*\(")
sites = []
occurrences = {}
for root in roots:
    for path in sorted(root.rglob("*.rs")):
        text = path.read_text()
        for match in pattern.finditer(text):
            line_no = text[:match.start()].count("\n") + 1
            path_key = str(path)
            occurrences[path_key] = occurrences.get(path_key, 0) + 1
            sites.append((path_key, occurrences[path_key], line_no, match.group(1)))

insert_sites = [site for site in sites if site[3] == "insert"]
remove_or_clear = [site for site in sites if site[3] in ("remove", "clear")]
site_ids = {f"{path}#{ordinal}" for path, ordinal, _line, _op in insert_sites}
test_only_sites = {
    *(f"src/mir/builder/control_flow/plan/composer/coreloop_v2_nested_minimal.rs#{n}" for n in range(1, 5)),
    "src/mir/builder/control_flow/plan/features/generic_loop_body/nested_depth_observer_tests.rs#1",
    "src/mir/builder/control_flow/plan/features/generic_loop_located_composer_tests.rs#1",
    "src/mir/builder/control_flow/plan/features/generic_loop_whole_parity_tests.rs#1",
    *(f"src/mir/builder/control_flow/plan/normalizer/helpers_pure_value.rs#{n}" for n in range(1, 5)),
    *(f"src/mir/builder/control_flow/plan/normalizer/tests.rs#{n}" for n in range(1, 4)),
    "src/mir/builder/control_flow/plan/parts/associated_source/located_hook_tests.rs#1",
    "src/mir/builder/control_flow/plan/parts/associated_source/located_parity_tests.rs#1",
}
disconnected_sites = {
    "src/mir/builder/control_flow/plan/features/generic_loop_located_composer.rs#1",
}
canonical_sites = {
    "src/mir/builder/control_flow/plan/parts/var_map_scope.rs#1",
}
# This is a finite shrink-only baseline.  Reseal rows may remove a known
# live site, but a new path or ordinal must first be classified in a bounded
# row; changing this guard is not a substitute for that inventory decision.
known_live_sites = {
    *(f"src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs#{n}" for n in range(1, 6)),
    *(f"src/mir/builder/control_flow/plan/features/carrier_merge.rs#{n}" for n in range(1, 4)),
    "src/mir/builder/control_flow/plan/features/generic_loop_pipeline.rs#1",
    "src/mir/builder/control_flow/plan/features/loop_cond_bc.rs#1",
    "src/mir/builder/control_flow/plan/features/loop_cond_bc_item.rs#1",
    "src/mir/builder/control_flow/plan/features/loop_cond_bc_item_stmt.rs#1",
    *(f"src/mir/builder/control_flow/plan/features/loop_cond_bc_util.rs#{n}" for n in range(1, 4)),
    "src/mir/builder/control_flow/plan/features/loop_cond_co_pipeline.rs#1",
    "src/mir/builder/control_flow/plan/features/loop_cond_continue_with_return_phi_materializer.rs#1",
    "src/mir/builder/control_flow/plan/features/loop_cond_return_in_body_phi_materializer.rs#1",
    *(f"src/mir/builder/control_flow/plan/features/loop_true_break_continue_pipeline.rs#{n}" for n in range(1, 3)),
    "src/mir/builder/control_flow/plan/lowerer/loop_completion.rs#1",
    *(f"src/mir/builder/control_flow/plan/normalizer/cond_lowering_prelude.rs#{n}" for n in range(1, 6)),
    "src/mir/builder/control_flow/plan/parts/associated_source/raw_lowering.rs#1",
    *(f"src/mir/builder/control_flow/plan/parts/dispatch/if_join.rs#{n}" for n in range(1, 3)),
    "src/mir/builder/control_flow/plan/parts/if_general.rs#1",
    *(f"src/mir/builder/control_flow/plan/parts/loop_/loop_v0.rs#{n}" for n in range(1, 4)),
}
known_sites = test_only_sites | disconnected_sites | canonical_sites | known_live_sites

if not site_ids <= known_sites:
    print("[coreplan-varmap-boundary] ERROR: unknown variable_map site entered the finite role inventory")
    print("\n".join(sorted(site_ids - known_sites)))
    raise SystemExit(1)

if remove_or_clear:
    print("[coreplan-varmap-boundary] ERROR: variable_map remove/clear under CorePlan/SSA is forbidden")
    print("\n".join(f"{path}:{line}: variable_map.{op}" for path, _ordinal, line, op in remove_or_clear))
    raise SystemExit(1)

if site_ids & test_only_sites != test_only_sites:
    print("[coreplan-varmap-boundary] ERROR: test-only role inventory drifted")
    raise SystemExit(1)
if site_ids & disconnected_sites != disconnected_sites:
    print("[coreplan-varmap-boundary] ERROR: disconnected role inventory drifted")
    raise SystemExit(1)
if site_ids & canonical_sites != canonical_sites:
    print("[coreplan-varmap-boundary] ERROR: canonical owner inventory drifted")
    raise SystemExit(1)

live_sites = site_ids - test_only_sites - disconnected_sites
reseal_sites = live_sites - canonical_sites

canonical_path = Path("src/mir/builder/control_flow/plan/parts/var_map_scope.rs")
if "publish_emission_cache" not in canonical_path.read_text():
    print("[coreplan-varmap-boundary] ERROR: canonical cache owner is missing")
    raise SystemExit(1)

print(
    "[coreplan-varmap-boundary] role-aware inventory "
    f"raw={len(site_ids)} test_only={len(site_ids & test_only_sites)} "
    f"disconnected={len(site_ids & disconnected_sites)} live={len(live_sites)} "
    f"canonical={len(live_sites & canonical_sites)} reseal={len(reseal_sites)} "
    "(baseline=51/16/1/34/1/33)"
)
print("[coreplan-varmap-boundary] variable_map_remove_clear_sites=0")
PY

echo "[$TAG] variable_map_no_growth_guard=1"
echo "[$TAG] ok"
