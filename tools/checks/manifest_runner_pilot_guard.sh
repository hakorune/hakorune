#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="manifest-runner-pilot-guard"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/phase_card_paths.sh"

ROW_RUNNER="$ROOT_DIR/tools/checks/run_row_guard.sh"
PROOF_RUNNER="$ROOT_DIR/tools/checks/run_proof_app.sh"
SHARED_RUNNER="$ROOT_DIR/tools/checks/lib/manifest_runner.py"
ROW_MANIFEST="$ROOT_DIR/tools/checks/guard_rows.toml"
PROOF_MANIFEST="$ROOT_DIR/tools/checks/proof_apps.toml"
OWNER_PACK_SUITE="$ROOT_DIR/tools/smokes/v2/suites/integration/phase2050-owner-pack.txt"
AGGREGATE_NODE_MANIFEST="$ROOT_DIR/tools/smokes/v2/suites/integration/aggregate-nodes.txt"
PHASE2050_RUN_ALL="$ROOT_DIR/tools/smokes/v2/profiles/integration/core/phase2050/run_all.sh"
PHASE2050_DIR="$ROOT_DIR/tools/smokes/v2/profiles/integration/core/phase2050"
CARD="$(guard_require_phase293x_card "$TAG" "293x-243-D199-MANIFEST-RUNNER-LIBRARY-CLEANUP.md")"
CHECK_INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
DEV_GATE="$ROOT_DIR/tools/checks/dev_gate.sh"
ALLOCATOR_GATE="$ROOT_DIR/tools/checks/k2_wide_allocator_gate.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" python3
guard_require_files "$TAG" \
  "$ROW_RUNNER" \
  "$PROOF_RUNNER" \
  "$SHARED_RUNNER" \
  "$ROW_MANIFEST" \
  "$PROOF_MANIFEST" \
  "$OWNER_PACK_SUITE" \
  "$AGGREGATE_NODE_MANIFEST" \
  "$PHASE2050_RUN_ALL" \
  "$CARD" \
  "$CHECK_INDEX" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE"
guard_require_exec_files "$TAG" "$ROW_RUNNER" "$PROOF_RUNNER" "$SHARED_RUNNER" "$0"

guard_expect_in_file "$TAG" "Status: Complete" "$CARD" "D199 card must be complete"
guard_expect_in_file "$TAG" "manifest_runner.py" "$CARD" "D199 card must name the shared runner"
guard_expect_in_file "$TAG" "manifest_runner_pilot_guard.sh" "$CARD" "D199 card must name this guard"
guard_expect_in_file "$TAG" "manifest_runner_pilot_guard.sh" "$CHECK_INDEX" "check index must list this guard"
guard_expect_in_file "$TAG" "tools/checks/lib/manifest_runner.py" "$CHECK_INDEX" "check index must mention shared runner library"

for wrapper in "$ROW_RUNNER" "$PROOF_RUNNER"; do
  guard_expect_in_file "$TAG" "manifest_runner.py" "$wrapper" "$(basename "$wrapper") must delegate to manifest_runner.py"
  if rg -n "<<|tomllib|subprocess|def main|import argparse|shell=True|eval\\(" "$wrapper"; then
    guard_fail "$TAG" "$(basename "$wrapper") regrew embedded runner logic"
  fi
done

guard_expect_in_file "$TAG" "tomllib" "$SHARED_RUNNER" "shared runner must own TOML parsing"
guard_expect_in_file "$TAG" "subprocess.run" "$SHARED_RUNNER" "shared runner must own argv-array subprocess dispatch"
guard_expect_in_file "$TAG" "--validation-profile" "$SHARED_RUNNER" "shared runner must expose validation profile selection"
guard_expect_in_file "$TAG" "--level" "$SHARED_RUNNER" "shared runner must expose level-specific command selection"
python3 - "$ROOT_DIR" "$PROOF_MANIFEST" <<'PY'
import pathlib
import sys
import tomllib

root = pathlib.Path(sys.argv[1]).resolve()
manifest = root / sys.argv[2]

def load_entries(path: pathlib.Path, stack: tuple[pathlib.Path, ...] = ()) -> list[dict]:
    if path in stack:
        cycle = " -> ".join(str(item.relative_to(root)) for item in (*stack, path))
        raise SystemExit(f"manifest include cycle: {cycle}")

    data = tomllib.loads(path.read_text(encoding="utf-8"))
    includes = data.get("includes", [])
    if not isinstance(includes, list) or not all(isinstance(item, str) and item for item in includes):
        raise SystemExit(f"{path.relative_to(root)} includes must be a list of non-empty strings")

    rows: list[dict] = []
    for include in includes:
        include_path = root / include
        if not include_path.is_file():
            raise SystemExit(f"missing included manifest: {include}")
        rows.extend(load_entries(include_path, (*stack, path)))

    local_rows = data.get("proof_apps")
    if not isinstance(local_rows, list):
        raise SystemExit(f"{path.relative_to(root)} must contain [[proof_apps]] entries")
    for row in local_rows:
        if not isinstance(row, dict):
            raise SystemExit(f"{path.relative_to(root)} proof app entry is not a table")
        rows.append(row)
    return rows

entries = load_entries(manifest)
if not any("validation_profile" in row for row in entries):
    raise SystemExit("proof manifest must carry validation profile pilot fields")
if not any("cmd_l2" in row for row in entries):
    raise SystemExit("proof manifest must carry L2 split-command pilot fields")
PY
if rg -n "shell=True|eval\\(" "$SHARED_RUNNER"; then
  guard_fail "$TAG" "shared runner must not use shell=True or eval"
fi

if rg -n "run_row_guard|run_proof_app|manifest_runner_pilot_guard" "$DEV_GATE" "$ALLOCATOR_GATE"; then
  guard_fail "$TAG" "manifest runner pilots must not be wired into dev_gate or allocator-wide gate yet"
fi

row_list="$("$ROW_RUNNER" --list)"
if ! rg -Fq "current-state-pointer" <<<"$row_list"; then
  guard_fail "$TAG" "row runner list must expose current-state-pointer"
fi
if ! rg -Fq "proof-app-manifest-test-entry" <<<"$row_list"; then
  guard_fail "$TAG" "row runner list must expose proof-app-manifest-test-entry"
fi
if ! rg -Fq "k2-wide-manifest-wrapper" <<<"$row_list"; then
  guard_fail "$TAG" "row runner list must expose k2-wide-manifest-wrapper"
fi
proof_list="$("$PROOF_RUNNER" --list)"
for proof_id in M200 M214 M215; do
  if ! rg -q "^${proof_id}\\b" <<<"$proof_list"; then
    guard_fail "$TAG" "proof app runner list must expose ${proof_id}"
  fi
done
"$ROW_RUNNER" --profile pilot --dry-run >/dev/null
"$PROOF_RUNNER" --profile pilot --dry-run >/dev/null
"$PROOF_RUNNER" --validation-profile scalar-mir --dry-run | rg -F "MIMAP-153A" >/dev/null
"$PROOF_RUNNER" --row-kind inventory --dry-run | rg -F "MIMAP-151A" >/dev/null
"$PROOF_RUNNER" --closeout-pack segment-map-readiness --dry-run | rg -F "validation_profile=scalar-mir" >/dev/null
"$PROOF_RUNNER" --validation-profile scalar-mir --level L2 --dry-run | rg -F -- "--level L2" >/dev/null
"$ROW_RUNNER" --only current-state-pointer >/dev/null

python3 - "$ROW_MANIFEST" "$OWNER_PACK_SUITE" "$AGGREGATE_NODE_MANIFEST" "$PHASE2050_RUN_ALL" "$PHASE2050_DIR" <<'PY'
import pathlib
import sys
import tomllib

row_manifest, suite_path, aggregate_manifest, run_all_path, phase_dir = map(pathlib.Path, sys.argv[1:])
if not phase_dir.is_dir():
    raise SystemExit("phase2050 owner-pack directory is missing")
rows = tomllib.loads(row_manifest.read_text(encoding="utf-8"))["rows"]
owner_rows = [row for row in rows if row.get("id") == "smoke-owner-pack-phase2050"]
if len(owner_rows) != 1:
    raise SystemExit("phase2050 owner-pack registry row must be unique")
expected_cmd = [
    "bash", "tools/smokes/v2/run.sh", "--profile", "quick",
    "--owner-profile", "integration", "--suite", "phase2050-owner-pack",
    "--dry-run", "--skip-preflight",
]
row = owner_rows[0]
if row.get("row_kind") != "smoke-owner-pack" or row.get("cmd") != expected_cmd:
    raise SystemExit("phase2050 owner-pack registry row drifted")

suite_entries = [
    line.strip()
    for line in suite_path.read_text(encoding="utf-8").splitlines()
    if line.strip() and not line.lstrip().startswith("#")
]
if len(suite_entries) != len(set(suite_entries)) or not suite_entries:
    raise SystemExit("phase2050 owner-pack suite must be non-empty and duplicate-free")
profile_dir = phase_dir.parents[1]
live_entries = sorted(
    str(path.relative_to(profile_dir))
    for path in phase_dir.glob("*.sh")
    if path.name != "run_all.sh"
)
if sorted(suite_entries) != live_entries:
    raise SystemExit("phase2050 owner-pack suite must cover exactly the live leaf scripts")

aggregate_rows = [
    line.strip()
    for line in aggregate_manifest.read_text(encoding="utf-8").splitlines()
    if line.strip() and not line.lstrip().startswith("#")
]
if aggregate_rows != ["core/phase2050/run_all.sh|ExplicitOnlyAggregate|phase2050-owner-pack"]:
    raise SystemExit("phase2050 aggregate-node manifest drifted")
if suite_path.name != "phase2050-owner-pack.txt" or suite_path.parent.name != "integration":
    raise SystemExit("phase2050 owner-pack must remain integration-owned")
if "core/phase2050/run_all.sh" in suite_entries:
    raise SystemExit("aggregate wrapper must not be a child leaf")

run_all = run_all_path.read_text(encoding="utf-8")
owner_invocation = "--profile quick --owner-profile integration --suite phase2050-owner-pack"
if run_all.count(owner_invocation) != 1 or "--filter" in run_all:
    raise SystemExit("phase2050 run_all must use one exact owner-pack invocation and no filter")
print("[manifest-runner-pilot-guard] phase2050 owner-pack linkage=exact")
PY

integration_dry_run="$("$ROOT_DIR/tools/smokes/v2/run.sh" --profile integration --dry-run --skip-preflight 2>&1)"
if ! rg -q "Found 962 test files" <<<"$integration_dry_run"; then
  guard_fail "$TAG" "integration discovery must find exactly 962 leaves after phase2050 exclusion"
fi
if rg -q "profiles/integration/core/phase2050/run_all\.sh" <<<"$integration_dry_run"; then
  guard_fail "$TAG" "phase2050 aggregate wrapper leaked into normal integration discovery"
fi
phase2050_count="$(rg -c 'profiles/integration/core/phase2050/' <<<"$integration_dry_run" || true)"
if [ "$phase2050_count" -ne 5 ]; then
  guard_fail "$TAG" "normal phase2050 discovery must contain exactly five leaves"
fi

echo "[$TAG] ok"
