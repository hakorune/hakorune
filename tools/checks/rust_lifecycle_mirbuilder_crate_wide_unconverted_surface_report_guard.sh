#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-crate-wide-unconverted-surface-report-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_crate_wide_unconverted_surface_report.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1826-MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-001.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1826-MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-001.md").read_text()

token = "MIRBUILDER-CRATE-WIDE-UNCONVERTED-SURFACE-REPORT-001"
if fixture.get("kind") != "MirBuilderCrateWideUnconvertedSurfaceReportV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if token not in card:
    raise SystemExit("card missing token")

input_info = fixture.get("input") or {}
if input_info.get("scan_unit") != "rust_function_or_method":
    raise SystemExit("scan unit drift")
if input_info.get("join_unit") != "semantic_owner_edge":
    raise SystemExit("join unit drift")
if input_info.get("scan_method") != "regex_source_text_v0":
    raise SystemExit("scan method drift")

items = fixture.get("items") or []
if not items:
    raise SystemExit("items missing")
provenance = fixture.get("provenance") or {}
for key in [
    "tool_version",
    "source_root_hash",
    "native_owner_seed_capability_survey_hash",
    "source_selfhost_family_guard_manifest_hash",
    "variable_context_reference_projection_contract_hash",
]:
    if not provenance.get(key):
        raise SystemExit(f"provenance missing {key}")
if provenance.get("tool_version") != "regex_source_text_v0":
    raise SystemExit("provenance tool version drift")

reason_table = fixture.get("reason_token_table") or {}
if not reason_table:
    raise SystemExit("reason token table missing")

seen = set()
for item in items:
    source_id = item.get("source_id")
    if not source_id:
        raise SystemExit("item missing source_id")
    if source_id in seen:
        raise SystemExit(f"duplicate source_id: {source_id}")
    seen.add(source_id)
    if not item.get("classification"):
        raise SystemExit(f"item missing classification: {source_id}")
    reason_token = item.get("reason_token")
    if not reason_token:
        raise SystemExit(f"item missing reason_token: {source_id}")
    if reason_token not in reason_table:
        raise SystemExit(f"item reason token is not in table: {reason_token}")
    if not item.get("owner_edge_confidence"):
        raise SystemExit(f"item missing owner edge confidence: {source_id}")
    if item.get("owner_edge_confidence") == "Heuristic" and item.get("next_owner_kind") != "None":
        raise SystemExit(f"heuristic owner edge selected an owner: {source_id}")
    if item.get("is_public_surface") and item.get("classification") == "IgnoredNonSemanticHelper":
        raise SystemExit(f"public ignored helper is not allowed: {source_id}")

reverse_checks = fixture.get("reverse_evidence_checks") or {}
for key in ["known_owner_edge_count", "orphan_source_surface_count", "orphan_evidence_row_count", "orphan_evidence_rows"]:
    if key not in reverse_checks:
        raise SystemExit(f"reverse evidence checks missing {key}")
if not fixture.get("owner_cluster_rules"):
    raise SystemExit("owner cluster rules missing")
if not fixture.get("joinir_plan_subcluster_rules"):
    raise SystemExit("joinir plan subcluster rules missing")
if not fixture.get("plan_feature_subcluster_rules"):
    raise SystemExit("plan feature subcluster rules missing")
if not fixture.get("loop_cond_feature_subcluster_rules"):
    raise SystemExit("loop-cond feature subcluster rules missing")
if not fixture.get("loop_cond_bc_subcluster_rules"):
    raise SystemExit("loop-cond break/continue subcluster rules missing")
cluster_summary = fixture.get("missing_projection_cluster_summary") or []
if not cluster_summary:
    raise SystemExit("missing projection cluster summary missing")
joinir_plan_summary = fixture.get("joinir_plan_subcluster_summary") or []
if not joinir_plan_summary:
    raise SystemExit("joinir plan subcluster summary missing")
plan_feature_summary = fixture.get("plan_feature_subcluster_summary") or []
if not plan_feature_summary:
    raise SystemExit("plan feature subcluster summary missing")
loop_cond_summary = fixture.get("loop_cond_feature_subcluster_summary") or []
if not loop_cond_summary:
    raise SystemExit("loop-cond feature subcluster summary missing")
loop_cond_bc_summary = fixture.get("loop_cond_bc_subcluster_summary") or []
if not loop_cond_bc_summary:
    raise SystemExit("loop-cond break/continue subcluster summary missing")
missing_items = [item for item in items if item.get("classification") == "MissingProjectionPolicy"]
for item in missing_items:
    if not item.get("likely_owner_cluster") or item.get("likely_owner_cluster") == "NotMissingProjectionPolicy":
        raise SystemExit(f"missing projection item lacks owner cluster: {item.get('source_id')}")
cluster_count_sum = sum(item.get("count", 0) for item in cluster_summary)
if cluster_count_sum != len(missing_items):
    raise SystemExit("missing projection cluster summary count drift")
joinir_plan_items = [item for item in missing_items if item.get("likely_owner_cluster") == "JoinIRPlanCluster"]
for item in joinir_plan_items:
    if not item.get("joinir_plan_subcluster"):
        raise SystemExit(f"JoinIRPlanCluster item lacks subcluster: {item.get('source_id')}")
joinir_plan_sum = sum(item.get("count", 0) for item in joinir_plan_summary)
if joinir_plan_sum != len(joinir_plan_items):
    raise SystemExit("JoinIR plan subcluster summary count drift")
plan_feature_items = [item for item in joinir_plan_items if item.get("joinir_plan_subcluster") == "PlanFeatureMaterializerCluster"]
for item in plan_feature_items:
    if not item.get("plan_feature_subcluster"):
        raise SystemExit(f"PlanFeatureMaterializerCluster item lacks subcluster: {item.get('source_id')}")
plan_feature_sum = sum(item.get("count", 0) for item in plan_feature_summary)
if plan_feature_sum != len(plan_feature_items):
    raise SystemExit("plan feature subcluster summary count drift")
loop_cond_items = [item for item in plan_feature_items if item.get("plan_feature_subcluster") == "LoopCondFeatureCluster"]
for item in loop_cond_items:
    if not item.get("loop_cond_feature_subcluster"):
        raise SystemExit(f"LoopCondFeatureCluster item lacks subcluster: {item.get('source_id')}")
loop_cond_sum = sum(item.get("count", 0) for item in loop_cond_summary)
if loop_cond_sum != len(loop_cond_items):
    raise SystemExit("loop-cond feature subcluster summary count drift")
loop_cond_bc_items = [item for item in loop_cond_items if item.get("loop_cond_feature_subcluster") == "LoopCondBreakContinueCluster"]
for item in loop_cond_bc_items:
    if not item.get("loop_cond_bc_subcluster"):
        raise SystemExit(f"LoopCondBreakContinueCluster item lacks subcluster: {item.get('source_id')}")
loop_cond_bc_sum = sum(item.get("count", 0) for item in loop_cond_bc_summary)
if loop_cond_bc_sum != len(loop_cond_bc_items):
    raise SystemExit("loop-cond break/continue subcluster summary count drift")

summary = fixture.get("summary") or {}
if summary.get("scanned_surface_count") != len(items):
    raise SystemExit("summary scanned surface count drift")
if summary.get("classified_once_count") != len(items):
    raise SystemExit("classified count drift")

decision = fixture.get("decision") or {}
if decision.get("kind") != "KeepStopped":
    raise SystemExit("current report should keep Source Selfhost stopped")
if decision.get("selected_next_card") != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("selected next card drift")

claims = fixture.get("claims") or {}
for key in [
    "tool_output_matches_checked_in_fixture",
    "scan_unit_rust_function_or_method",
    "join_unit_semantic_owner_edge",
    "scan_method_regex_source_text_v0",
    "every_scanned_public_method_classified_exactly_once",
    "every_unconverted_item_has_reason_token",
    "every_reason_token_is_stable",
    "owner_edge_confidence_recorded",
    "likely_owner_cluster_recorded",
    "missing_projection_items_clustered",
    "joinir_plan_items_subclustered",
    "plan_feature_items_subclustered",
    "loop_cond_feature_items_subclustered",
    "loop_cond_break_continue_items_subclustered",
    "heuristic_owner_edge_not_selectable",
    "public_ignored_requires_reason",
    "multiple_candidates_keep_stopped",
    "borrow_policy_known_does_not_select_owner",
    "composite_suspected_is_not_decomposition_proof",
    "generated_artifact_only_is_not_native_edit_authority",
    "support_lane_only_is_not_hako_adoption_candidate",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"positive claim must be 1: {key}")
for key in [
    "rust_ast_parser_required",
    "rustc_adapter_required",
    "semantic_inference_beyond_existing_ssot",
    "manual_family_selection",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-crate-wide-unconverted-surface-report-v0
scan_unit=rust_function_or_method
join_unit=semantic_owner_edge
decision=KeepStopped
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
summary=ok
REPORT
