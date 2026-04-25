#!/bin/bash
# stageb_bundle_duplicate_fail_vm.sh — Stage‑B: duplicate named bundles → Fail‑Fast

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

main='static box Main { method main(args) { return 0 } }'
util_a='static box Util { method id(a) { return a } }'
util_b='static box Util { method id(a) { return a } }'

set +e
out=$(NYASH_CLI_VERBOSE=0 \
  NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_VARMAP_GUARD_STRICT=0 NYASH_BLOCK_SCHEDULE_VERIFY=0 \
  NYASH_JOINIR_DEV=0 HAKO_JOINIR_STRICT=0 \
  NYASH_ALLOW_USING_FILE=0 HAKO_ALLOW_USING_FILE=0 NYASH_USING_AST=1 \
  "$NYASH_BIN" --backend vm \
  "$ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- \
  --bundle-mod "Util:$util_a" --bundle-mod "Util:$util_b" --source "$main" 2>&1)
rc=$?
set -e

echo "$out" | grep -q "\\[bundle/duplicate\\] Util" || {
  echo "[FAIL] stageb_bundle_duplicate_fail_vm (missing duplicate tag)" >&2
  echo "$out" | tail -n 60 >&2 || true
  exit 1
}
echo "[PASS] stageb_bundle_duplicate_fail_vm"
exit 0
