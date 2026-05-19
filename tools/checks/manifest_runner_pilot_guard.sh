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
if ! printf '%s\n' "$row_list" | rg -Fq "current-state-pointer"; then
  guard_fail "$TAG" "row runner list must expose current-state-pointer"
fi
if ! printf '%s\n' "$row_list" | rg -Fq "proof-app-manifest-test-entry"; then
  guard_fail "$TAG" "row runner list must expose proof-app-manifest-test-entry"
fi
if ! printf '%s\n' "$row_list" | rg -Fq "k2-wide-manifest-wrapper"; then
  guard_fail "$TAG" "row runner list must expose k2-wide-manifest-wrapper"
fi
proof_list="$("$PROOF_RUNNER" --list)"
for proof_id in M200 M214 M215; do
  if ! printf '%s\n' "$proof_list" | rg -q "^${proof_id}\\b"; then
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

echo "[$TAG] ok"
