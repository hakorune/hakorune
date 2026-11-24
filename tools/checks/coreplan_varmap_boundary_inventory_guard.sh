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
checked_loop_sites = {
    "src/mir/builder/control_flow/plan/lowerer/source_loop_completion.rs#1",
    "src/mir/builder/control_flow/plan/lowerer/source_loop_driver.rs#1",
}

if site_ids != checked_loop_sites:
    print("[coreplan-varmap-boundary] ERROR: checked Loop cache writer inventory drifted")
    print("unexpected:", sorted(site_ids - checked_loop_sites))
    print("missing:", sorted(checked_loop_sites - site_ids))
    raise SystemExit(1)

if remove_or_clear:
    print("[coreplan-varmap-boundary] ERROR: variable_map remove/clear under CorePlan/SSA is forbidden")
    print("\n".join(f"{path}:{line}: variable_map.{op}" for path, _ordinal, line, op in remove_or_clear))
    raise SystemExit(1)

print(f"[coreplan-varmap-boundary] checked_loop_cache_writers={len(site_ids)}")
print("[coreplan-varmap-boundary] variable_map_remove_clear_sites=0")
PY

echo "[$TAG] variable_map_no_growth_guard=1"
echo "[$TAG] ok"
