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
if rg -n "shell=True|eval\\(" "$SHARED_RUNNER"; then
  guard_fail "$TAG" "shared runner must not use shell=True or eval"
fi

if rg -n "run_row_guard|run_proof_app|manifest_runner_pilot_guard" "$DEV_GATE" "$ALLOCATOR_GATE"; then
  guard_fail "$TAG" "manifest runner pilots must not be wired into dev_gate or allocator-wide gate yet"
fi

if ! "$ROW_RUNNER" --list | rg -Fq "current-state-pointer"; then
  guard_fail "$TAG" "row runner list must expose current-state-pointer"
fi
if ! "$PROOF_RUNNER" --list | rg -Fq "M200"; then
  guard_fail "$TAG" "proof app runner list must expose M200"
fi
"$ROW_RUNNER" --profile pilot --dry-run >/dev/null
"$PROOF_RUNNER" --profile pilot --dry-run >/dev/null
"$ROW_RUNNER" --only current-state-pointer >/dev/null

echo "[$TAG] ok"
