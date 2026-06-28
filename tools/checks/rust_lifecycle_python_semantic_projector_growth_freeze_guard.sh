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
classified_files: dict[str, set[str]] = {}
for bucket in role_buckets:
    role = bucket.get("role")
    if role not in role_names:
        raise SystemExit(f"role bucket missing role name: {bucket}")
    patterns = bucket.get("patterns", [])
    if not isinstance(patterns, list) or not patterns:
        raise SystemExit(f"empty patterns for role bucket: {role}")
    for pattern in patterns:
        if not isinstance(pattern, str) or not pattern:
            raise SystemExit(f"bad pattern in role bucket: {role}")
        matches = [path for path in root.glob(pattern) if path.is_file()]
        if not matches:
            raise SystemExit(f"inventory pattern matches no files: {pattern}")
        for path in matches:
            classified_files.setdefault(path.relative_to(root).as_posix(), set()).add(role)
        matched_patterns += 1

if matched_patterns < 10:
    raise SystemExit("inventory too small to be useful")

scope = inventory.get("scope")
if scope != "tools/rust_lifecycle":
    raise SystemExit(f"unexpected inventory scope: {scope}")

active_python_files = sorted(path.relative_to(root).as_posix() for path in (root / scope).glob("*.py"))
expected_active_count = inventory.get("active_python_file_count")
if expected_active_count != len(active_python_files):
    raise SystemExit(
        f"active Python converter file count drift: "
        f"expected {expected_active_count}, got {len(active_python_files)}"
    )

missing = [path for path in active_python_files if path not in classified_files]
if missing:
    raise SystemExit("unclassified active Python converter files: " + ", ".join(missing))

role_file_counts = inventory.get("role_file_counts")
if not isinstance(role_file_counts, dict):
    raise SystemExit("role_file_counts must be a map")
for role in required_roles:
    expected = role_file_counts.get(role)
    actual = sum(1 for roles in classified_files.values() if role in roles)
    if expected != actual:
        raise SystemExit(f"role file count drift for {role}: expected {expected}, got {actual}")

allowed_overlaps = inventory.get("allowed_role_overlaps", {})
if not isinstance(allowed_overlaps, dict):
    raise SystemExit("allowed_role_overlaps must be a map")

for path, roles in sorted(classified_files.items()):
    if len(roles) <= 1:
        continue
    allowed = allowed_overlaps.get(path)
    if not isinstance(allowed, list) or set(allowed) != roles:
        raise SystemExit(f"unexpected role overlap for {path}: {sorted(roles)}")

for path, roles in sorted(allowed_overlaps.items()):
    if path not in classified_files:
        raise SystemExit(f"allowed role overlap points to missing file: {path}")
    if set(roles) != classified_files[path]:
        raise SystemExit(
            f"allowed role overlap drift for {path}: "
            f"expected {sorted(roles)}, got {sorted(classified_files[path])}"
        )

print("freeze_inventory=green")
print(f"role_buckets={len(role_buckets)}")
print(f"matched_patterns={matched_patterns}")
print(f"classified_python_files={len(active_python_files)}")
print(f"allowed_role_overlaps={len(allowed_overlaps)}")
print("summary=ok")
PY
