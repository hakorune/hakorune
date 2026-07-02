#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/semantic-selector-no-lexical-tiebreak-guard-v0.json"
TOOL="$ROOT/tools/rust_lifecycle/semantic_selector_no_lexical_tiebreak_guard.py"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/2043-MIRBUILDER-SEMANTIC-SELECTOR-NO-LEXICAL-TIEBREAK-GUARD-001.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"

python3 "$TOOL" --check

python3 - "$FIXTURE" "$CARD" "$STATE" "$TASK_ORDER" <<'PY'
import json
import sys
import tomllib
from pathlib import Path

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
card = Path(sys.argv[2]).read_text(encoding="utf-8")
state = tomllib.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
task_order = Path(sys.argv[4]).read_text(encoding="utf-8")

def need(cond, msg):
    if not cond:
        raise SystemExit(msg)

token = "MIRBUILDER-SEMANTIC-SELECTOR-NO-LEXICAL-TIEBREAK-GUARD-001"
next_card = "MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-001"

need(fixture.get("kind") == "MirBuilderSemanticSelectorNoLexicalTiebreakGuardV1", "bad kind")
need(fixture.get("token") == token, "bad token")
need(token in card, "card missing token")

policy = fixture.get("guard_policy") or {}
for key in [
    "manual_owner_selection",
    "owner_name_as_proof",
    "lexical_order_as_proof",
    "fixture_name_as_proof",
    "manifest_order_as_proof",
]:
    need(policy.get(key) is False, f"guard policy drift: {key}")
need(policy.get("first_eligible_selection_requires_exactly_one_guard") is True, "exactly-one policy drift")
need(policy.get("historical_findings_are_not_new_authority") is True, "historical policy drift")

active = fixture.get("active_enforcement") or {}
need(active.get("active_file_count") == 3, "active file count drift")
need(active.get("forbidden_active_finding_count") == 0, "active forbidden findings drift")
need(active.get("exactly_one_guarded_selection_count") == 1, "exactly-one count drift")

historical = fixture.get("historical_findings") or {}
need(historical.get("finding_count") == 9, "historical finding count drift")
need(historical.get("next_cleanup_card") == "MIRBUILDER-HISTORICAL-SEED-SELECTOR-QUARANTINE-001", "cleanup card drift")

decision = fixture.get("decision") or {}
need(decision.get("kind") == "GuardDefined", "decision kind drift")
need(decision.get("selected_next_card") == next_card, "next card drift")

claims = fixture.get("claims") or {}
for key in [
    "manual_owner_selection",
    "owner_name_as_proof",
    "lexical_order_as_proof",
    "fixture_name_as_proof",
    "manifest_order_as_proof",
    "first_eligible_without_exactly_one_guard",
    "source_plan_materialization",
    "native_seed_materialization",
    "hako_generation",
    "hako_adopted_decision",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "runner_semantic_owner",
]:
    need(claims.get(key) == 0, f"forbidden claim drift: {key}")

need(state.get("latest_card") == token, "CURRENT_STATE latest card drift")
for needle in [
    token,
    "selected_next_card = MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-001",
    "source_selfhost_claim = 0",
]:
    need(needle in task_order, f"task-order missing {needle}")

print("output_contract=rust-lifecycle-semantic-selector-no-lexical-tiebreak-guard")
print("forbidden_active_finding_count=0")
print("historical_finding_count=9")
print("selected_next_card=MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-001")
print("source_selfhost_claim=0")
print("summary=ok")
PY
