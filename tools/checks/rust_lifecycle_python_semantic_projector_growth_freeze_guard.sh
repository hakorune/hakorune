#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
from __future__ import annotations

import json
import re
from pathlib import Path

root = Path.cwd()
doc = root / "docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md"
text = doc.read_text()
match = re.search(r"```json\n(.*?)\n```", text, re.S)
if not match:
    raise SystemExit("missing JSON inventory block in role SSOT")
inventory = json.loads(match.group(1))

if inventory.get("kind") != "PythonConverterRoleInventoryV1":
    raise SystemExit("wrong inventory kind")
if inventory.get("checkpoint") != "PYTHON-SEMANTIC-PROJECTOR-GROWTH-FREEZE-001":
    raise SystemExit("freeze checkpoint token drift")

role_buckets = inventory.get("role_buckets")
if not isinstance(role_buckets, list) or not role_buckets:
    raise SystemExit("role buckets missing")

role_names = {bucket.get("role") for bucket in role_buckets}
required_roles = {"FactsAdapter", "SemanticProjector", "DeterministicEmitter", "GuardOrchestrator"}
if not required_roles.issubset(role_names):
    raise SystemExit(f"missing role buckets: {sorted(required_roles - role_names)}")

projector_follow_ons = inventory.get("semantic_projector_follow_on_tokens", {})
if not isinstance(projector_follow_ons, dict) or not projector_follow_ons:
    raise SystemExit("semantic projector follow-on token map missing")
for key, value in projector_follow_ons.items():
    if not isinstance(key, str) or not key:
        raise SystemExit("semantic projector follow-on token key drift")
    if not isinstance(value, str) or not value:
        raise SystemExit(f"semantic projector follow-on token missing for {key}")

matched_patterns = 0
for bucket in role_buckets:
    patterns = bucket.get("patterns", [])
    if not isinstance(patterns, list) or not patterns:
        raise SystemExit(f"empty patterns for role bucket: {bucket.get('role')}")
    for pattern in patterns:
        if not isinstance(pattern, str) or not pattern:
            raise SystemExit(f"bad pattern in role bucket: {bucket.get('role')}")
        if not list(root.glob(pattern)):
            raise SystemExit(f"inventory pattern matches no files: {pattern}")
        matched_patterns += 1

if matched_patterns < 10:
    raise SystemExit("inventory too small to be useful")

print("freeze_inventory=green")
print(f"role_buckets={len(role_buckets)}")
print(f"matched_patterns={matched_patterns}")
print("summary=ok")
PY
