#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="tools-dev-surface-inventory-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

README="$ROOT_DIR/tools/dev/README.md"
guard_require_files "$TAG" "$README"

EXPECTED_FILES=(
  "README.md"
  "at_local_preexpand.sh"
  "cargo_check_safe.sh"
  "dev_sugar_preexpand.sh"
  "direct_loop_progression_sweep.sh"
  "exdev_rename_copy_fallback.c"
  "hako_debug_run.sh"
  "hako_preinclude.sh"
  "phase2160_mirbuilder_module_load_probe.sh"
  "phase29cg_stage2_bootstrap_phi_verify.sh"
  "phase29ch_program_json_compat_route_probe.sh"
  "phase29ck_boundary_explicit_compat_probe.sh"
  "phase29ck_boundary_historical_alias_probe.sh"
  "phase29ck_stage1_mir_dialect_probe.sh"
)

ACTUAL_FILE="${TMPDIR:-/tmp}/${TAG}.actual.$$"
EXPECTED_FILE="${TMPDIR:-/tmp}/${TAG}.expected.$$"
trap 'rm -f "$ACTUAL_FILE" "$EXPECTED_FILE"' EXIT

find "$ROOT_DIR/tools/dev" -maxdepth 1 -type f -printf '%f\n' | sort >"$ACTUAL_FILE"
printf '%s\n' "${EXPECTED_FILES[@]}" | sort >"$EXPECTED_FILE"

if ! diff -u "$EXPECTED_FILE" "$ACTUAL_FILE"; then
  guard_fail "$TAG" "tools/dev active file set drifted; update tools/dev/README.md and this guard together"
fi

for file in "${EXPECTED_FILES[@]}"; do
  if ! rg -q "\`$file\`" "$README"; then
    guard_fail "$TAG" "tools/dev/README.md missing inventory row for $file"
  fi
done

echo "[$TAG] ok files=${#EXPECTED_FILES[@]}"
