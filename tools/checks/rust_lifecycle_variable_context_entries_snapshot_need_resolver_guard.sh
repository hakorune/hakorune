#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="rust-lifecycle-variable-context-entries-snapshot-need-resolver"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-296x/1797-MIRBUILDER-VARIABLE-CONTEXT-ENTRIES-SNAPSHOT-NEED-RESOLVER-001.md"
FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-entries-snapshot-need-resolution-v0.json"
BOUNDED_READINESS_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-bounded-native-surface-readiness-resolution-v0.json"
REFERENCE_CONTRACT_FIXTURE="docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-reference-projection-contract-v0.json"
NATIVE_SOURCE="apps/lib/hakorune_mir_builder/variable_context.hako"
STATE="docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

guard_require_files "$TAG" \
  "$CARD" \
  "$FIXTURE" \
  "$BOUNDED_READINESS_FIXTURE" \
  "$REFERENCE_CONTRACT_FIXTURE" \
  "$NATIVE_SOURCE" \
  "$STATE" \
  "$TASK_ORDER" \
  "$INDEX"

python3 - "$CARD" "$FIXTURE" "$BOUNDED_READINESS_FIXTURE" "$REFERENCE_CONTRACT_FIXTURE" "$NATIVE_SOURCE" "$STATE" "$TASK_ORDER" "$INDEX" <<'PY'
from __future__ import annotations

import json
import sys
import tomllib
from pathlib import Path

card_path = Path(sys.argv[1])
fixture_path = Path(sys.argv[2])
bounded_readiness_path = Path(sys.argv[3])
reference_contract_path = Path(sys.argv[4])
native_source_path = Path(sys.argv[5])
state_path = Path(sys.argv[6])
task_order_path = Path(sys.argv[7])
index_path = Path(sys.argv[8])

card = card_path.read_text(encoding="utf-8")
fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
bounded_readiness = json.loads(bounded_readiness_path.read_text(encoding="utf-8"))
reference_contract = json.loads(reference_contract_path.read_text(encoding="utf-8"))
native_source = native_source_path.read_text(encoding="utf-8")
state = tomllib.loads(state_path.read_text(encoding="utf-8"))
task_order = task_order_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

token = "MIRBUILDER-VARIABLE-CONTEXT-ENTRIES-SNAPSHOT-NEED-RESOLVER-001"
output_contract = "rust-lifecycle-variable-context-entries-snapshot-need-resolution-v0"
reason_token = "NoCurrentConsumerRequiresEntriesSnapshot"
next_action = "NextRouteFamilySelectionPolicy"

require(f"# {token}" in card, "card token drift")
require(f"output_contract={output_contract}" in card, "card output contract drift")
require("EntriesSnapshotNotNeededForBoundedNativeSurface" in card, "card decision drift")
require(f"reason_token:\n  {reason_token}" in card, "card reason drift")
require(f"next_action:\n  {next_action}" in card, "card next action drift")

require(fixture.get("kind") == "VariableContextEntriesSnapshotNeedResolutionV1", "fixture kind drift")
require(fixture.get("output_contract") == output_contract, "fixture output contract drift")

fixture_state = fixture.get("current_state") or {}
require(fixture_state.get("latest_card") == token, "fixture latest card drift")
require(fixture_state.get("current_blocker_token") == token, "fixture blocker drift")

input_state = fixture.get("input_state") or {}
require(input_state.get("bounded_readiness_card") == "MIRBUILDER-VARIABLE-CONTEXT-BOUNDED-NATIVE-SURFACE-READINESS-RESOLVER-001", "fixture bounded readiness card drift")
require(input_state.get("entries_snapshot_state") == "FutureConsumerNeedOnly", "fixture entries state drift")
require(input_state.get("snapshot_owned_state") == "NamingCompatibilityCleanupOnly", "fixture snapshot_owned state drift")
require(input_state.get("restore_owned_state") == "NamingCompatibilityCleanupOnly", "fixture restore_owned state drift")
require(input_state.get("mut_lease_state") == "DeferredUntilLiveNeed", "fixture mut_lease state drift")

consumer_hits = fixture.get("consumed_evidence", {}).get("consumer_hits") or {}
for key in ["entries_snapshot", "snapshot_owned", "restore_owned"]:
    require(consumer_hits.get(key) == 0, f"fixture consumer hit drift: {key}")

decision = fixture.get("decision") or {}
require(decision.get("kind") == "EntriesSnapshotNotNeededForBoundedNativeSurface", "fixture decision kind drift")
require(decision.get("need_state") == "NotNeeded", "fixture need state drift")
require(decision.get("reason_token") == reason_token, "fixture reason token drift")
require(decision.get("next_action") == next_action, "fixture next action drift")

non_requirements = fixture.get("non_requirements") or {}
for key in ["entries_snapshot_implemented", "snapshot_owned_implemented", "restore_owned_implemented", "mut_lease_selected"]:
    require(non_requirements.get(key) == 0, f"fixture non-requirement drift: {key}")

claims = fixture.get("claims") or {}
for key in [
    "entries_snapshot_needed",
    "entries_snapshot_implemented",
    "snapshot_owned_implemented",
    "restore_owned_implemented",
    "full_variable_context_claim",
    "source_selfhost_claim",
    "runtime_fallback",
    "new_backend_route",
    "new_abi",
    "new_python_semantic_projector",
    "manual_family_selection",
]:
    require(claims.get(key) == 0, f"fixture claim drift: {key}")

bounded_need = bounded_readiness.get("not_required_for_bounded_readiness") or {}
require(bounded_need.get("entries_snapshot") == "FutureConsumerNeedOnly", "bounded readiness entries drift")
require(bounded_need.get("snapshot_owned") == "NamingCompatibilityCleanupOnly", "bounded readiness snapshot_owned drift")
require(bounded_need.get("restore_owned") == "NamingCompatibilityCleanupOnly", "bounded readiness restore_owned drift")
require(bounded_need.get("mut_lease") == "DeferredUntilLiveNeed", "bounded readiness mut_lease drift")

current_api = set((bounded_readiness.get("bounded_surface") or {}).get("current_native_api", []))
for name in ["lookup", "contains", "len", "is_empty", "snapshot", "restore", "replace_owned_map", "insert", "remove"]:
    require(name in current_api, f"bounded readiness native API missing {name}")

reference_candidates = set(reference_contract.get("future_native_api_candidates", []))
for name in ["entries_snapshot", "snapshot_owned", "restore_owned"]:
    require(name in reference_candidates, f"reference contract future candidate missing {name}")

require("variable_map_mut" not in native_source, "native source must not expose variable_map_mut")
for term in ["entries_snapshot", "snapshot_owned", "restore_owned"]:
    require(term not in native_source, f"native source must not expose {term}")

roots = [Path("src"), Path("apps"), Path("tests")]
consumer_scan_hits = {term: 0 for term in ["entries_snapshot", "snapshot_owned", "restore_owned"]}
for root in roots:
    if not root.exists():
        continue
    for path in root.rglob("*"):
        if not path.is_file():
            continue
        if path.suffix not in {".rs", ".hako", ".md", ".json", ".sh", ".toml", ".txt"}:
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for term in consumer_scan_hits:
            consumer_scan_hits[term] += text.count(term)

for term, count in consumer_scan_hits.items():
    require(count == 0, f"consumer scan found {term}: {count}")

latest_card_path = state.get("latest_card_path")
require(isinstance(latest_card_path, str) and Path(latest_card_path).exists(), "current-state latest card path missing")
landed_tail = state.get("landed_tail") or []
require(any("1797 resolves `entries_snapshot`" in row for row in landed_tail), "current-state missing 1797 provenance")

for needle in [
    token,
    next_action,
    "entries_snapshot_state = NotNeededForBoundedNativeSurface",
    "NextRouteFamilySelectionPolicy",
]:
    require(needle in task_order, f"task-order missing {needle}")

require("tools/checks/rust_lifecycle_variable_context_entries_snapshot_need_resolver_guard.sh" in index, "check index missing guard")

print(f"output_contract={output_contract}")
print("decision=EntriesSnapshotNotNeededForBoundedNativeSurface")
print("need_state=NotNeeded")
print(f"reason_token={reason_token}")
print(f"next_action={next_action}")
print("entries_snapshot_needed=0")
print("entries_snapshot_implemented=0")
print("no_current_source_consumer=1")
print("no_current_test_consumer=1")
print("current_native_api_unmodified=1")
print("future_api_candidates_retained=1")
print("full_variable_context_claim=0")
print("source_selfhost_claim=0")
print("runtime_fallback=0")
print("new_backend_route=0")
print("new_abi=0")
print("new_python_semantic_projector=0")
print("summary=ok")
PY
