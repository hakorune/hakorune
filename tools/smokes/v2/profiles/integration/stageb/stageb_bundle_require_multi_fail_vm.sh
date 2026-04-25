#!/bin/bash
# stageb_bundle_require_multi_fail_vm.sh — Stage‑B: require‑mod 複数の一部不足 → Fail（タグ）

set -euo pipefail
if [ "${SMOKES_ENABLE_STAGEB:-0}" != "1" ]; then
  echo "[SKIP] stageb_bundle_require_multi_fail_vm (SMOKES_ENABLE_STAGEB=1 to enable)"
  exit 0
fi
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT/tools/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

main='static box Main { method main(args) { return 0 } }'
u1='static box U1 { method id(a){ return a } }'

set +e
out=$(
  stageb_export_vm_compile_env
  bash -lc "cd '$ROOT' && '$NYASH_BIN' --backend vm '$ROOT/lang/src/compiler/entry/compiler_stageb.hako' -- --bundle-mod 'U1:$u1' --require-mod U1 --require-mod U2 --source '$main'" 2>&1)
rc=$?
set -e

echo "$out" | grep -q "\[bundle/missing\] U2" || {
  echo "[FAIL] stageb_bundle_require_multi_fail_vm (missing tag)" >&2
  echo "$out" | tail -n 60 >&2 || true
  exit 1
}
echo "[PASS] stageb_bundle_require_multi_fail_vm"
exit 0
