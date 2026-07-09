#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-active-v1.json"
HISTORY="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-history-v1.jsonl"
LEGACY_MANIFEST="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json"
SPLIT_PROVENANCE="$ROOT/docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-manifest-split-v1.json"
SPLIT_TOOL="$ROOT/tools/rust_lifecycle/source_selfhost_family_manifest_split.py"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
POLICY="$ROOT/docs/development/current/main/design/current-docs-update-policy-ssot.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"

python3 "$SPLIT_TOOL" --check
python3 - "$ROOT" "$MANIFEST" "$HISTORY" "$LEGACY_MANIFEST" "$SPLIT_PROVENANCE" "$STATE" "$POLICY" "$INDEX" <<'PY'
import json
import sys
from pathlib import Path
import tomllib
import hashlib

root, manifest_path, history_path, legacy_path, provenance_path, state_path, policy_path, index_path = map(Path, sys.argv[1:])

def die(message: str) -> None:
    print(f"[source-selfhost-family-guard] {message}", file=sys.stderr)
    raise SystemExit(1)

def read(path: Path) -> str:
    if not path.exists():
        die(f"missing path: {path.relative_to(root)}")
    return path.read_text()

manifest = json.loads(read(manifest_path))
legacy = json.loads(read(legacy_path))
provenance = json.loads(read(provenance_path))
history_rows = [json.loads(line) for line in read(history_path).splitlines() if line.strip()]
state = tomllib.loads(read(state_path))
policy = read(policy_path)
index = read(index_path)

if manifest.get("kind") != "SourceSelfhostFamilyGuardActiveManifestV1":
    die("manifest kind mismatch")
if manifest.get("family_scope") != "SourceSelfhost":
    die("manifest family scope mismatch")

blocker = manifest.get("current_blocker_token")
if state.get("current_blocker_token") != blocker:
    die("CURRENT_STATE blocker drift")
if not state.get("latest_card") or not state.get("latest_card_path"):
    die("CURRENT_STATE latest pointer missing")
if state["latest_card"] not in state["latest_card_path"]:
    die("CURRENT_STATE latest path drift")

guard_policy = manifest.get("guard_policy") or {}
for key in [
    "historical_row_guards_remain_callable",
    "new_rows_prefer_family_guard",
    "current_state_pointer_guard_owns_live_latest_card",
]:
    if guard_policy.get(key) != 1:
        die(f"guard policy flag must be 1: {key}")
if guard_policy.get("row_guard_latest_card_allowlists") != 0:
    die("row guard latest-card allowlists must be disabled")

rows = manifest.get("rows") or []
if not rows:
    die("manifest rows missing")
if set(manifest.get("active_roles") or []) != {"current_semantic", "current_maintenance"}:
    die("active role set mismatch")
if provenance.get("kind") != "SourceSelfhostFamilyManifestSplitV1":
    die("split provenance kind mismatch")
if provenance.get("source_manifest_sha256") != hashlib.sha256(legacy_path.read_bytes()).hexdigest():
    die("legacy manifest provenance drift")
legacy_by_token = {row.get("token"): row for row in legacy.get("rows") or []}
active_by_token = {row.get("token"): row for row in rows}
history_by_token = {row.get("token"): row for row in history_rows}
if len(active_by_token) != len(rows) or len(history_by_token) != len(history_rows):
    die("duplicate token in active/history split")
if set(active_by_token) & set(history_by_token):
    die("active/history token overlap")
if not set(legacy_by_token) <= (set(active_by_token) | set(history_by_token)):
    die("active/history token partition lost a frozen row")
for token, row in history_by_token.items():
    if row != legacy_by_token.get(token):
        die(f"history row drift: {token}")
for token, row in active_by_token.items():
    if token in legacy_by_token and row != legacy_by_token[token]:
        die(f"baseline active row drift: {token}")
for row in rows:
    if row.get("role") not in {"current_semantic", "current_maintenance"}:
        die(f"non-active role in active manifest: {row.get('token')}")
for row in history_rows:
    if row.get("role") not in {"current_queue", "historical_traceability"}:
        die(f"non-history role in history ledger: {row.get('token')}")
if manifest.get("current_blocker_token") not in active_by_token:
    die("CURRENT_STATE blocker missing from active manifest")
seen = set()
for row in [*rows, *history_rows]:
    token = row.get("token")
    if not token:
        die("row token missing")
    if token in seen:
        die(f"duplicate row token: {token}")
    seen.add(token)
    card = row.get("card") or ""
    if not card:
        die(f"row card missing: {token}")
    card_path = root / card
    card_text = read(card_path)
    if token not in card_text:
        die(f"card missing row token: {token}")
    fixture = row.get("fixture") or ""
    if fixture:
        fixture_path = root / fixture
        fixture_data = json.loads(read(fixture_path))
        if fixture_data.get("kind") is None:
            die(f"fixture missing kind: {fixture}")
    legacy_guard = row.get("legacy_guard") or ""
    if legacy_guard and not (root / legacy_guard).exists():
        die(f"legacy guard missing: {legacy_guard}")

for key, expected in (manifest.get("forbidden_claims") or {}).items():
    if expected != 0:
        die(f"forbidden claim must be zero in manifest: {key}")

for needle in [
    "do not create a dedicated `.sh` guard for every row",
    "prefer one reusable lane guard per bucket",
    "Past row guards must not pin `CURRENT_STATE.latest_card`",
]:
    if needle not in policy:
        die(f"policy missing guard consolidation rule: {needle}")

if "rust_lifecycle_source_selfhost_family_guard.sh" not in index:
    die("check-scripts index missing family guard")

print("[source-selfhost-family-guard] OK")
PY
