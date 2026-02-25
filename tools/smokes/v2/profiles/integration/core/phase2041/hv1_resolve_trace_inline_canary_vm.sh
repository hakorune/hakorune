#!/usr/bin/env bash
# Resolver trace (dev): inline source with using alias (Nyash package) should resolve via text-merge
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

TMP="/tmp/resolve_trace_inline_$$.hako"
cat > "$TMP" <<'NY'
using json_native as JSON
static box Main { method main(args) { return 0 } }
NY

set +e
# Call binary directly to capture resolver traces (filter in run_nyash_vm would strip them)
OUT=$(NYASH_ENABLE_USING=1 NYASH_USING_AST=1 NYASH_RESOLVE_TRACE=1 "$NYASH_BIN" --backend vm "$TMP" 2>&1)
RC=$?
set -e
rm -f "$TMP" || true

echo "$OUT" | grep -q "\[using/resolve\]" || echo "$OUT" | grep -q "\[using/text-merge\]" || {
  echo "[FAIL] expected resolver trace logs" >&2
  echo "$OUT" >&2
  exit 1
}

echo "[PASS] hv1_resolve_trace_inline_canary_vm"
