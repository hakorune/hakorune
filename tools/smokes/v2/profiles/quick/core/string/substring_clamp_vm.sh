#!/bin/bash
# substring_clamp_vm.sh — String.substring clamps to [0,size] and start<=end

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

code='static box Main { main() {
  local s="abcd";
  print(s.substring(-5, 2));
  print(s.substring(2, 99));
  print(s.substring(3, 1));
  return 0
} }'
out=$(run_nyash_vm -c "$code")
expected=$(cat <<EOT
ab
cd

EOT
)
if [ "$out" = "$expected" ]; then
  echo "[PASS] substring_clamp_vm"
else
  echo "[FAIL] substring_clamp_vm" >&2
  echo '--- expected ---' >&2
  printf '%s\n' "$expected" >&2
  echo '--- got ---' >&2
  printf '%s\n' "$out" >&2
  exit 1
fi
