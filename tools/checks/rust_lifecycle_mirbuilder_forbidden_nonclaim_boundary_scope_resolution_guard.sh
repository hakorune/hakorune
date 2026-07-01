#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-forbidden-nonclaim-boundary-scope-resolution-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_forbidden_nonclaim_boundary_scope_resolution.py"

python3 "$TOOL" --check

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())

def die(message: str) -> None:
    print(f"[forbidden-nonclaim-boundary-scope-resolution-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

if data.get("kind") != "MirBuilderForbiddenNonclaimBoundaryScopeResolutionV1":
    die("fixture kind mismatch")
if data.get("token") != "MIRBUILDER-FORBIDDEN-NONCLAIM-BOUNDARY-SCOPE-RESOLUTION-001":
    die("fixture token mismatch")

policy = data.get("scope_resolution_policy") or {}
for key in [
    "forbidden_nonclaim_never_proves_seed_eligibility",
    "required_by_selected_narrow_seed_surface_blocks_seed",
    "wider_denied_boundary_mention_only_is_not_seed_evidence",
    "wider_denied_boundary_mention_only_may_be_excluded_from_seed_blockers",
    "unclassified_forbidden_nonclaim_blocks_seed",
]:
    if policy.get(key) is not True:
        die(f"policy flag must be true: {key}")
if policy.get("manual_boundary_reclassification") is not False:
    die("manual boundary reclassification must be false")

pool = data.get("candidate_pool") or {}
expected = {
    "input_owner_edge_count": 3,
    "required_by_selected_narrow_seed_surface_count": 0,
    "wider_denied_boundary_mention_only_count": 12,
    "scoped_forbidden_nonclaim_exclusion_count": 0,
    "permanent_forbidden_nonclaim_count": 0,
    "unclassified_forbidden_nonclaim_count": 0,
    "bridge_policy_v2_candidate_count": 3,
    "permanent_derived_candidate_count": 0,
    "diagnostic_lane_candidate_count": 0,
}
for key, value in expected.items():
    if pool.get(key) != value:
        die(f"candidate pool drift: {key}")

rows = data.get("owner_edge_rows") or []
if len(rows) != 3:
    die("owner edge rows must be 3")
for row in rows:
    if row.get("input_bridge_state") != "BridgeBlocked":
        die("input bridge state must be BridgeBlocked")
    if row.get("input_blocked_by") != ["ForbiddenNonClaimBoundaryStillDenied"]:
        die("input blocked reason mismatch")
    if row.get("resolved_bridge_state") != "BridgePolicyV2Candidate":
        die("row must become BridgePolicyV2Candidate")
    if row.get("selected_next_card") != "MIRBUILDER-STRICT-EMISSION-TO-NATIVE-SEED-BRIDGE-POLICY-V2-001":
        die("row selected next card mismatch")
    summary = row.get("summary") or {}
    if summary.get("wider_denied_boundary_mention_only_count") != 4:
        die("each row must have 4 mention-only forbidden nonclaims")
    for occurrence in row.get("boundary_occurrences") or []:
        if occurrence.get("input_class") != "ForbiddenNonClaimBoundary":
            die("occurrence input class mismatch")
        if occurrence.get("scope_class") != "WiderDeniedBoundaryMentionOnly":
            die("occurrence scope class mismatch")
        if occurrence.get("seed_eligibility_evidence") is not False:
            die("forbidden nonclaim must not be seed evidence")
        if occurrence.get("seed_eligibility_blocker") is not False:
            die("mention-only occurrence must not block selected narrow seed")

decision = data.get("decision") or {}
if decision.get("kind") != "SelectBridgePolicyV2":
    die("decision kind mismatch")
if decision.get("reason_token") != "ForbiddenNonclaimMentionOnlyCanBeScopedOut":
    die("decision reason mismatch")
if decision.get("selected_next_card") != "MIRBUILDER-STRICT-EMISSION-TO-NATIVE-SEED-BRIDGE-POLICY-V2-001":
    die("selected next card mismatch")

claims = data.get("claims") or {}
for key in [
    "manual_boundary_reclassification",
    "seed_eligibility_from_forbidden_nonclaim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_canonical_mir_instruction",
    "new_python_semantic_projector",
    "generated_artifact_as_native_edit_authority",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runner_semantic_owner",
]:
    if claims.get(key) != 0:
        die(f"claim must remain 0: {key}")
if claims.get("normalized_rerun_consumed") != 1:
    die("normalized rerun must be consumed")
if claims.get("denied_boundary_vocabulary_normalization_consumed") != 1:
    die("normalization fixture must be consumed")

print("[forbidden-nonclaim-boundary-scope-resolution-guard] OK")
PY
