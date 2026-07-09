#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="rust-lifecycle-mirbuilder-scalar-known-fastpath-string-hako-authority-pilot-basis"

source "$ROOT/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-scalar-known-fastpath-string-hako-authority-pilot-basis-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_string_hako_authority_pilot_basis.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/3392-MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-HAKO-AUTHORITY-PILOT-BASIS-001.md"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$TOOL" "$CARD" "$TASK_ORDER" "$MANIFEST"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$TASK_ORDER" "$MANIFEST" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
task_order = Path(sys.argv[3]).read_text(encoding="utf-8")
manifest = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))


def need(cond, msg):
    if not cond:
        raise SystemExit(msg)


token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-HAKO-AUTHORITY-PILOT-BASIS-001"
next_token = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001"

need(fixture.get("kind") == "MirBuilderScalarKnownFastpathStringHakoAuthorityPilotBasisV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")
need(next_token in card, "card missing next token")

basis = fixture.get("basis") or {}
need(basis.get("basis_only") is True, "basis must be basis-only")
need(basis.get("surface") == "StringScalarI64Routes", "wrong surface")
need("HomogeneousScalarI64NoPublicationReadSurface" in (basis.get("proof_axis") or []), "proof axis drift")
need(basis.get("selected_next_card") == next_token, "basis next drift")

summary = fixture.get("summary") or {}
for key in [
    "string_hako_authority_pilot_basis",
    "prior_scoped_read_authority_continuation",
    "homogeneous_scalar_i64_no_publication_read_surface",
    "rust_oracle_compat_checker_retained",
    "mismatch_fail_fast_required",
    "basis_only",
]:
    need(summary.get(key) == 1, f"summary positive drift: {key}")
for key in [
    "string_hako_route_decision_authority_pilot",
    "scalar_known_hako_runtime_route_authority",
    "source_selfhost_claim",
]:
    need(summary.get(key) == 0, f"summary forbidden drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "string_hako_route_decision_authority_pilot",
    "scalar_known_hako_runtime_route_authority",
    "rust_fastpath_rewired",
    "route_selection_authority_switch",
    "backend_lowering_authority",
    "source_selfhost_claim",
    "route_count_as_proof",
    "owner_name_as_proof",
    "source_path_as_authority",
    "route_membership_alone_as_proof",
    "manual_surface_selection",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

rows_by_token = {row.get("token"): row for row in manifest.get("rows") or []}
need(token in rows_by_token, "manifest missing token")
need(token in task_order and f"selected_next_card={next_token}" in task_order, "task order drift")

print("output_contract=rust-lifecycle-mirbuilder-scalar-known-fastpath-string-hako-authority-pilot-basis")
print("string_hako_authority_pilot_basis=1")
print("homogeneous_scalar_i64_no_publication_read_surface=1")
print("string_hako_route_decision_authority_pilot=0")
print("source_selfhost_claim=0")
print("selected_next_card=" + next_token)
print("summary=ok")
PY
