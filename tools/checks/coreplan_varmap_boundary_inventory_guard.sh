#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="coreplan-varmap-boundary"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

TASKBOARD="docs/development/current/main/workstreams/compiler-foundation-current.md"
LOCAL_PATCH_SSOT="docs/development/current/main/design/local-patch-prevention-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-1022-COREPLAN-VARMAP-BOUNDARY-001.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE_STEPS="tools/checks/lib/dev_gate_quick_steps.sh"
SELF_SCRIPT="tools/checks/coreplan_varmap_boundary_inventory_guard.sh"

echo "[$TAG] checking CorePlan variable_map write boundary inventory"

guard_require_files \
  "$TAG" \
  "$TASKBOARD" \
  "$LOCAL_PATCH_SSOT" \
  "$CARD" \
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
  "variable_map_direct_insert_sites=54" \
  "$CARD" \
  "phase card must record robust variable_map insert baseline"

python3 - <<'PY'
import re
from pathlib import Path

roots = [
    Path("src/mir/builder/control_flow/plan"),
    Path("src/mir/builder/ssa"),
]
pattern = re.compile(r"variable_map\s*\.\s*(insert|remove|clear)\s*\(")
sites = []
for root in roots:
    for path in sorted(root.rglob("*.rs")):
        text = path.read_text()
        for match in pattern.finditer(text):
            line_no = text[:match.start()].count("\n") + 1
            sites.append((str(path), line_no, match.group(1)))

insert_sites = [site for site in sites if site[2] == "insert"]
remove_or_clear = [site for site in sites if site[2] in ("remove", "clear")]
stmt_insert_sites = [
    site
    for site in insert_sites
    if site[0] == "src/mir/builder/control_flow/plan/parts/stmt.rs"
]
parts_helper_bypass_sites = [
    site
    for site in insert_sites
    if site[0]
    in (
        "src/mir/builder/control_flow/plan/parts/stmt.rs",
        "src/mir/builder/control_flow/plan/parts/conditional_update.rs",
        "src/mir/builder/control_flow/plan/parts/loop_/final_values.rs",
    )
]

max_insert_count = 54
if len(insert_sites) > max_insert_count:
    print(
        f"[coreplan-varmap-boundary] ERROR: direct variable_map insert sites grew "
        f"from <= {max_insert_count} to {len(insert_sites)}"
    )
    print("\n".join(f"{path}:{line}: variable_map.{op}" for path, line, op in insert_sites))
    raise SystemExit(1)

if remove_or_clear:
    print("[coreplan-varmap-boundary] ERROR: variable_map remove/clear under CorePlan/SSA is forbidden")
    print("\n".join(f"{path}:{line}: variable_map.{op}" for path, line, op in remove_or_clear))
    raise SystemExit(1)

if parts_helper_bypass_sites:
    print("[coreplan-varmap-boundary] ERROR: selected parts files must publish through var_map_scope helpers")
    print("\n".join(f"{path}:{line}: variable_map.{op}" for path, line, op in parts_helper_bypass_sites))
    raise SystemExit(1)

print(f"[coreplan-varmap-boundary] variable_map_direct_insert_sites={len(insert_sites)}")
print("[coreplan-varmap-boundary] variable_map_remove_clear_sites=0")
print("[coreplan-varmap-boundary] parts_stmt_direct_variable_map_insert_sites=0")
print("[coreplan-varmap-boundary] selected_parts_direct_variable_map_insert_sites=0")
PY

echo "[$TAG] variable_map_no_growth_guard=1"
echo "[$TAG] ok"
