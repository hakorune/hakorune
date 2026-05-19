#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="proof-app-manifest-test-entry-guard"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PROOF_MANIFEST="tools/checks/proof_apps.toml"
PROOF_RUNNER="tools/checks/run_proof_app.sh"
SHARED_RUNNER="tools/checks/lib/manifest_runner.py"
DESIGN="docs/development/current/main/design/guard-manifest-migration-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-575-GUARD-MANIFEST-001-PROOF-APP-TEST-WRAPPER-CLEANUP.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" \
  "$PROOF_MANIFEST" \
  "$PROOF_RUNNER" \
  "$SHARED_RUNNER" \
  "$DESIGN" \
  "$CARD"
guard_require_exec_files "$TAG" "$PROOF_RUNNER" "$SHARED_RUNNER" "$0"

guard_expect_in_file "$TAG" "Guard Manifest Migration SSOT" "$DESIGN" "migration SSOT must exist"
guard_expect_in_file "$TAG" "proof_app_manifest_test_entry_guard.sh" "$CARD" "phase card must name this guard"

python3 - "$ROOT_DIR" "$PROOF_MANIFEST" <<'PY'
import os
import pathlib
import re
import sys
import tomllib

root = pathlib.Path(sys.argv[1]).resolve()
manifest_path = root / sys.argv[2]

def load_entries(path: pathlib.Path, stack: tuple[pathlib.Path, ...] = ()) -> list[dict]:
    if path in stack:
        cycle = " -> ".join(str(item.relative_to(root)) for item in (*stack, path))
        raise SystemExit(f"proof_apps.toml include cycle: {cycle}")
    data = tomllib.loads(path.read_text(encoding="utf-8"))
    if data.get("schema_version") != 0:
        raise SystemExit(f"{path.relative_to(root)} schema_version must be 0")
    includes = data.get("includes", [])
    if not isinstance(includes, list) or not all(isinstance(item, str) and item for item in includes):
        raise SystemExit(f"{path.relative_to(root)} includes must be a list of non-empty strings")

    entries: list[dict] = []
    for include in includes:
        entries.extend(load_entries(root / include, (*stack, path)))

    local_entries = data.get("proof_apps", [])
    if not isinstance(local_entries, list):
        raise SystemExit(f"{path.relative_to(root)} proof_apps must be a list")
    for entry in local_entries:
        if not isinstance(entry, dict):
            raise SystemExit(f"{path.relative_to(root)} proof_apps entry is not a table")
        entries.append(entry)
    return entries

entries = load_entries(manifest_path)
if not isinstance(entries, list) or not entries:
    raise SystemExit("proof_apps.toml must contain [[proof_apps]] entries")

direct_guard = re.compile(r"k2_wide_[A-Za-z0-9_]+_guard\.sh")
checked = 0
errors: list[str] = []

for entry in entries:
    if not isinstance(entry, dict):
        errors.append("proof_apps entry is not a table")
        continue
    proof_id = entry.get("id")
    app = entry.get("app")
    if not isinstance(proof_id, str) or not proof_id:
        errors.append(f"bad proof id: {proof_id!r}")
        continue
    if not isinstance(app, str) or not app:
        errors.append(f"{proof_id}: bad app path: {app!r}")
        continue

    test_path = root / app / "test.sh"
    if not test_path.is_file():
        errors.append(f"{proof_id}: missing app test entry: {test_path.relative_to(root)}")
        continue
    if not os.access(test_path, os.X_OK):
        errors.append(f"{proof_id}: app test entry is not executable: {test_path.relative_to(root)}")
        continue

    text = test_path.read_text(encoding="utf-8")
    if "tools/checks/run_proof_app.sh" not in text:
        errors.append(f"{proof_id}: test.sh must call tools/checks/run_proof_app.sh")
    if f"--only {proof_id}" not in text:
        errors.append(f"{proof_id}: test.sh must select --only {proof_id}")
    if direct_guard.search(text):
        errors.append(f"{proof_id}: test.sh must not call k2_wide_* guard directly")
    checked += 1

if errors:
    for error in errors:
        print(f"[proof-app-manifest-test-entry-guard] ERROR: {error}", file=sys.stderr)
    raise SystemExit(1)

print(f"[proof-app-manifest-test-entry-guard] checked={checked}")
PY

echo "[$TAG] ok"
