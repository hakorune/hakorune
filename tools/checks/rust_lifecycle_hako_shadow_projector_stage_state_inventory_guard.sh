#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-hako-shadow-projector-stage-state-inventory-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

LIB_DIR="$ROOT_DIR/lang/src/compiler/lib"
README="$LIB_DIR/README.md"

PROJECTORS=(
  "return_emission_projector.hako|hakorune_mir_builder::return_emission|return_emission"
  "function_region_stack_pop_projector.hako|hakorune_mir_builder::function_region_stack_pop|function_region_stack_pop"
  "slot_registry_release_projector.hako|hakorune_mir_builder::slot_registry_release|slot_registry_release"
  "module_metadata_publication_projector.hako|hakorune_mir_builder::module_metadata_publication|module_metadata_publication"
  "record_packed_layout_refresh_projector.hako|hakorune_mir_builder::record_packed_layout_refresh|record_packed_layout_refresh"
  "typed_object_plan_refresh_projector.hako|hakorune_mir_builder::typed_object_plan_refresh|typed_object_plan_refresh"
  "direct_state_plan_refresh_projector.hako|hakorune_mir_builder::direct_state_plan_refresh|direct_state_plan_refresh"
  "all_functions_phi_materialization_projector.hako|hakorune_mir_builder::all_functions_phi_materialization|all_functions_phi_materialization"
)

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$README" "$ROOT_DIR/tools/bin/hako"

for entry in "${PROJECTORS[@]}"; do
  file="${entry%%|*}"
  rest="${entry#*|}"
  family_id="${rest%%|*}"
  stage_id="${rest##*|}"
  guard_require_files "$TAG" "$LIB_DIR/$file"
  bash "$ROOT_DIR/tools/bin/hako" --backend mir --verify "$LIB_DIR/$file"
  python3 - "$LIB_DIR/$file" "$family_id" "$stage_id" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
family_id = sys.argv[2]
stage_id = sys.argv[3]
text = path.read_text(encoding="utf-8")

required = [
    "project_shadow_record(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token)",
    "project_shadow_json(plan, python_oracle, hako_shadow, parity_gate, promotion_token, retirement_token)",
    f'family_id", CompilerProjectionValueBox.create_string("{family_id}")',
    f'stage_id", CompilerProjectionValueBox.create_string("{stage_id}")',
    'python_oracle',
    'hako_shadow',
    'parity_gate',
    'promotion_token',
    'retirement_token',
]
for needle in required:
    if needle not in text:
        raise SystemExit(f"missing projector inventory text: {path} :: {needle}")
PY
done

python3 - <<'PY'
from pathlib import Path

root = Path("lang/src/compiler/lib")
readme = (root / "README.md").read_text(encoding="utf-8")

expected_lines = [
    "first shadow-projector support library:",
    "second shadow-projector support library:",
    "third shadow-projector support library:",
    "fourth shadow-projector support library:",
    "fifth shadow-projector support library:",
    "sixth shadow-projector support library:",
    "seventh shadow-projector support library:",
    "eighth shadow-projector support library:",
]
for needle in expected_lines:
    if needle not in readme:
        raise SystemExit(f"missing README shadow-projector inventory text: {needle}")

for needle in [
    "return_emission_projector.hako",
    "function_region_stack_pop_projector.hako",
    "slot_registry_release_projector.hako",
    "module_metadata_publication_projector.hako",
    "record_packed_layout_refresh_projector.hako",
    "typed_object_plan_refresh_projector.hako",
    "direct_state_plan_refresh_projector.hako",
    "all_functions_phi_materialization_projector.hako",
    "ordinary `.hako` library home",
]:
    if needle not in readme:
        raise SystemExit(f"missing README projector inventory text: {needle}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-hako-shadow-projector-stage-state-inventory-v0
library_home=lang/src/compiler/lib/
projector_inventory=green
projector_count=8
stage_state_fields=green
family_id=green
stage_id=green
python_oracle=green
hako_shadow=green
parity_gate=green
promotion_token=green
retirement_token=green
abi_surface=0
host_surface=0
syntax_surface=0
runtime_fallback=0
source_selfhost_claim=0
summary=ok
REPORT
