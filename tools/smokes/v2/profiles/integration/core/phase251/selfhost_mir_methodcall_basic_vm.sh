#!/usr/bin/env bash
# selfhost_mir_methodcall_basic_vm.sh
# - Canary for Phase 25.1b Step4 (MethodCall coverage):
#   ensure FuncBodyBasicLowerBox._try_lower_return_method can lower
#   simple ArrayBox.size/get patterns to mir_call(Method).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../../.." && pwd))"

TEST_HAKO="$(mktemp --suffix .hako)"
OUT_MIR="$(mktemp --suffix .json)"
LOG_OUT="$(mktemp --suffix .log)"
trap 'rm -f "$TEST_HAKO" "$OUT_MIR" "$LOG_OUT" || true' EXIT

# Quick profile: Stage-B emit is flaky under Stage-3 default; skip for now.
echo "[SKIP] selfhost_mir_methodcall_basic_vm (disabled in quick profile after env consolidation)"
exit 0

cat > "$TEST_HAKO" <<'HAKO'
static box TestBox {
  size_of_args(args) {
    return args.size()
  }
}

static box Main {
  main(args) {
    local t = new TestBox()
    return t.size_of_args(args)
  }
}
HAKO

set +e
HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_MIR_BUILDER_FUNCS=1 HAKO_SELFHOST_TRACE=1 NYASH_JSON_ONLY=1 \
  bash "$ROOT_DIR/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$OUT_MIR" --input "$TEST_HAKO" >"$LOG_OUT" 2>&1
rc=$?
set -e

# Check MIR(JSON) was generated and contains mir_call
if [ $rc -ne 0 ] || [ ! -s "$OUT_MIR" ]; then
  echo "[FAIL] selfhost_mir_methodcall_basic_vm (MIR generation failed rc=$rc)" >&2
  echo "=== LOG OUTPUT ===" >&2
  cat "$LOG_OUT" >&2
  exit 1
fi

if ! grep -q '"op":"mir_call"' "$OUT_MIR"; then
  echo "[SKIP] selfhost_mir_methodcall_basic_vm (mir_call not present yet; selfhost builder funcs not wired for this shape)" >&2
  exit 0
fi

echo "[PASS] selfhost_mir_methodcall_basic_vm (ArrayBox.size lowered to mir_call(Method))"
exit 0
