#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-next-hako-adoption-candidate-selection-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SCRIPT="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_next_hako_adoption_candidate_selection.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/next-hako-adoption-candidate-selection-v0.json"
ROUTE_MANIFEST="$ROOT_DIR/lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
CLOSEOUT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-route-matrix-closeout-v0.json"
ROADMAP="$ROOT_DIR/docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$SCRIPT" "$FIXTURE" "$ROUTE_MANIFEST" "$CLOSEOUT" "$ROADMAP"

python3 "$SCRIPT" --check

cat <<'REPORT'
output_contract=rust-lifecycle-next-hako-adoption-candidate-selection-v0
candidate_pool_state=Blocked
eligible_candidate_count=0
manual_next_owner_selection=0
support_lane_projection_as_candidate=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
reason_token=NoEligibleDerivedMainlineRouteCandidate
summary=ok
REPORT
