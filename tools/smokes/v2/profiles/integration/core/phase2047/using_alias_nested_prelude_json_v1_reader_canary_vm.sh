#!/bin/bash
# Nested prelude resolution via JsonV1ReaderBox (which uses other aliases internally)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

code=$(cat <<'HCODE'
using selfhost.vm.hakorune-vm.json_v1_reader as JsonV1ReaderBox
static box Main { method main(args) {
  // 内部でさらに using を持つ Box を呼び出して nested prelude を検証
  local seg = JsonV1ReaderBox.get_block0_instructions('{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[]}]}]}')
  // 空配列の内側は空文字列になる想定
  if seg != "" { return 1 }
  return 0
} }
HCODE
)

set +e
out=$(NYASH_USING_AST=1 run_nyash_vm -c "$code" 2>&1)
rc=$?
set -e

if [ "$rc" -eq 0 ]; then
  echo "[PASS] using_alias_nested_prelude_json_v1_reader_canary_vm"
  exit 0
fi
echo "[FAIL] using_alias_nested_prelude_json_v1_reader_canary_vm (rc=$rc)" >&2
printf '%s\n' "$out" | sed -n '1,160p' >&2
exit 1

