#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-slot-registry-release-hako-shadow-promotion-decision-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SCRIPT="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_slot_registry_release_hako_shadow_promotion_decision.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/slot-registry-release-hako-shadow-promotion-decision-v0.json"
DERIVED_ARTIFACT_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_mirbuilder_slot_registry_release_derived_artifact_guard.sh"
STAGE_INVENTORY_GUARD="$ROOT_DIR/tools/checks/rust_lifecycle_hako_shadow_projector_stage_state_inventory_guard.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$SCRIPT" "$FIXTURE" "$DERIVED_ARTIFACT_GUARD" "$STAGE_INVENTORY_GUARD"

bash "$DERIVED_ARTIFACT_GUARD"
bash "$STAGE_INVENTORY_GUARD"
python3 "$SCRIPT" --check

cat <<'REPORT'
output_contract=rust-lifecycle-slot-registry-release-hako-shadow-promotion-decision-v0
family_id=hakorune_mir_builder::slot_registry_release
stage_id=slot_registry_release
current_stage=HakoShadow
selected_stage=HakoMainline
decision=Promote
reason_token=SlotRegistryReleaseHakoShadowParityGreen
python_oracle_retained=1
hako_shadow_retained=1
promotion_token_explicit=1
retirement_token_explicit=1
runtime_fallback=0
new_backend_route=0
new_abi=0
host_env_lookup=0
source_selfhost_claim=0
summary=ok
REPORT
