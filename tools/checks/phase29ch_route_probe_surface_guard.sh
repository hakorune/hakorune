#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="phase29ch-route-probe-surface-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

guard_require_files "$TAG" \
  "$ROOT_DIR/tools/dev/phase29ch_program_json_compat_route_probe.sh"

while IFS= read -r path; do
  rel="${path#$ROOT_DIR/}"
  case "$rel" in
    tools/dev/phase29ch_program_json_compat_route_probe.sh)
      ;;
    *)
      guard_fail "$TAG" "unexpected active phase29ch diagnostics probe: $rel"
      ;;
  esac
done < <(find "$ROOT_DIR/tools/dev" -maxdepth 1 -type f -name 'phase29ch_*.sh' | sort)

echo "[$TAG] ok"
