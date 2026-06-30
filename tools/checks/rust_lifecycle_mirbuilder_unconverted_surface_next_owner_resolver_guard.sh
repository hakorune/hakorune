#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-unconverted-surface-next-owner-resolver-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

TOOL="$ROOT_DIR/tools/rust_lifecycle/mirbuilder_unconverted_surface_next_owner_resolver.py"
FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-unconverted-surface-next-owner-resolution-v0.json"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1874-MIRBUILDER-UNCONVERTED-SURFACE-NEXT-OWNER-RESOLVER-001.md"
REPORT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json"
CONTRACT="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-unconverted-surface-next-owner-resolver-task-contract-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TOOL" "$FIXTURE" "$CARD" "$REPORT" "$CONTRACT"

python3 "$TOOL" --check

python3 - <<'PY'
import json
from pathlib import Path

fixture = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-unconverted-surface-next-owner-resolution-v0.json").read_text())
report = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json").read_text())
contract = json.loads(Path("docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-unconverted-surface-next-owner-resolver-task-contract-v0.json").read_text())
card = Path("docs/development/current/main/phases/phase-296x/1874-MIRBUILDER-UNCONVERTED-SURFACE-NEXT-OWNER-RESOLVER-001.md").read_text()

token = "MIRBUILDER-UNCONVERTED-SURFACE-NEXT-OWNER-RESOLVER-001"
if fixture.get("kind") != "MirBuilderUnconvertedSurfaceNextOwnerResolutionV1":
    raise SystemExit("fixture kind mismatch")
if fixture.get("token") != token:
    raise SystemExit("fixture token mismatch")
if f"# 1874 - {token}" not in card:
    raise SystemExit("card token mismatch")

if fixture["input_authority"]["unconverted_surface_report"] != "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-crate-wide-unconverted-surface-report-v0.json":
    raise SystemExit("report authority drift")
if fixture["input_authority"]["resolver_task_contract"] != "docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-unconverted-surface-next-owner-resolver-task-contract-v0.json":
    raise SystemExit("contract authority drift")

rules = fixture["resolver_rules"]
if rules["exclusion_rules"] != contract["exclusion_rules"]:
    raise SystemExit("exclusion rules drift")
if rules["priority_rules"] != contract["priority_rules"]:
    raise SystemExit("priority rules drift")
if rules["manual_selection_allowed"] != 0:
    raise SystemExit("manual selection must be forbidden")

pool = fixture["candidate_pool"]
counts = pool["candidate_counts_by_priority"]
summary = report["summary"]
if counts["MissingProjectionPolicy"] != summary["missing_projection_policy_count"]:
    raise SystemExit("MissingProjectionPolicy count drift")
if counts["BorrowSurfaceNeedsPolicy"] != summary["borrow_policy_needed_count"]:
    raise SystemExit("BorrowSurfaceNeedsPolicy count drift")
if counts["CompositeSuspected"] != summary["composite_suspected_count"]:
    raise SystemExit("CompositeSuspected count drift")
if pool["selected_priority"] != "MissingProjectionPolicy":
    raise SystemExit("selected priority drift")
if pool["selected_priority_candidate_count"] != 1396:
    raise SystemExit("selected priority candidate count drift")
if not pool["selected_priority_cluster_summary"]:
    raise SystemExit("cluster summary missing")

decision = fixture["decision"]
if decision["kind"] != "KeepStopped":
    raise SystemExit("decision must KeepStopped while ambiguous")
if decision["reason_token"] != "AmbiguousNextOwnerCandidates":
    raise SystemExit("reason token drift")
if decision["selected_next_card"] != "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001":
    raise SystemExit("selected next card drift")
if decision["selected_source_id"] is not None:
    raise SystemExit("selected source id must be null")

claims = fixture["claims"]
for key in [
    "report_consumed",
    "resolver_implemented",
    "exactly_one_next_owner_selected_if_unambiguous",
    "multiple_candidates_keep_stopped",
    "zero_candidates_keep_stopped",
]:
    if claims.get(key) != 1:
        raise SystemExit(f"positive claim must be 1: {key}")
for key in [
    "manual_family_selection",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
    "hako_generation",
    "hako_adopted_decision",
    "native_seed_materialization",
]:
    if claims.get(key) != 0:
        raise SystemExit(f"non-claim must be 0: {key}")
PY

cat <<'REPORT'
output_contract=rust-lifecycle-mirbuilder-unconverted-surface-next-owner-resolution-v0
decision=KeepStopped
reason_token=AmbiguousNextOwnerCandidates
selected_priority=MissingProjectionPolicy
manual_family_selection=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
REPORT
