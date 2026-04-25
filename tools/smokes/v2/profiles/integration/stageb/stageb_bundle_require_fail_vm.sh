#!/bin/bash
# stageb_bundle_require_fail_vm.sh — Stage‑B: require-mod 不満足 → 非0終了

set -euo pipefail
if [ "${SMOKES_ENABLE_STAGEB:-0}" != "1" ]; then
  echo "[SKIP] stageb_bundle_require_fail_vm (SMOKES_ENABLE_STAGEB=1 to enable)"
  exit 0
fi
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

main='static box Main { method main(args) { return 0 } }'

set +e
out=$(NYASH_CLI_VERBOSE=0 \
  NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_VARMAP_GUARD_STRICT=0 NYASH_BLOCK_SCHEDULE_VERIFY=0 \
  NYASH_JOINIR_DEV=0 HAKO_JOINIR_STRICT=0 \
  NYASH_ALLOW_USING_FILE=0 HAKO_ALLOW_USING_FILE=0 NYASH_USING_AST=1 \
  bash -lc "cd '$ROOT' && '$NYASH_BIN' --backend vm '$ROOT/lang/src/compiler/entry/compiler_stageb.hako' -- --require-mod Util --source '$main'" 2>&1)
rc=$?
set -e

echo "$out" | grep -q "\[bundle/missing\] Util" || {
  echo "[FAIL] stageb_bundle_require_fail_vm (missing error tag)" >&2
  echo "$out" | tail -n 60 >&2 || true
  exit 1
}
echo "[PASS] stageb_bundle_require_fail_vm"
exit 0
