#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="program-json-dev-surface-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ARCHIVED_ACTIVE_PATHS=(
  "tools/dev/program_json_v0"
  "tools/dev/phase29ch_program_json_cold_compat_probe.sh"
  "tools/dev/phase29ch_program_json_explicit_mode_gate_probe.sh"
  "tools/dev/phase29ch_program_json_helper_exec_probe.sh"
  "tools/dev/phase29ch_program_json_text_only_probe.sh"
  "tools/dev/phase29ch_raw_direct_stage1_cli_probe.sh"
  "tools/dev/phase29ch_selfhost_program_json_helper_probe.sh"
)

for rel in "${ARCHIVED_ACTIVE_PATHS[@]}"; do
  if [[ -e "$ROOT_DIR/$rel" ]]; then
    guard_fail "$TAG" "archived Program(JSON) dev surface returned: $rel"
  fi
done

guard_require_files "$TAG" \
  "$ROOT_DIR/tools/dev/phase29ch_program_json_compat_route_probe.sh" \
  "$ROOT_DIR/tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh"

echo "[$TAG] ok"
