#!/bin/bash
# CallMethodizeBox keeps source-execution canonical call MIR unchanged when methodize is ON.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

BIN="$ROOT/target/release/hakorune"
if [ ! -x "$BIN" ]; then
  echo "[SKIP] mirbuilder_call_methodize_canonical_identity_canary_vm (hakorune not built)"
  exit 0
fi

tmp_exe="/tmp/mirbuilder_call_methodize_identity_$$.exe"
build_log="/tmp/mirbuilder_call_methodize_identity_build_$$.log"
trap 'rm -f "$tmp_exe" "$build_log"' EXIT

if ! timeout "${HAKO_BUILD_TIMEOUT:-120}" env -i PATH="$PATH" HOME="${HOME:-}" USER="${USER:-}" "$BIN" --emit-exe "$tmp_exe" "$ROOT/lang/src/runner/stage1_cli_env.hako" >"$build_log" 2>&1; then
  cat "$build_log" >&2
  echo "[FAIL] mirbuilder_call_methodize_canonical_identity_canary_vm (build failed)" >&2
  exit 1
fi

set +e
out="$(env -i PATH="$PATH" HOME="${HOME:-}" USER="${USER:-}" HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir STAGE1_SOURCE_TEXT='static box Main { main() { return "hello".length() } }' "$tmp_exe" 2>&1)"; rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  echo "$out" >&2
  echo "[FAIL] mirbuilder_call_methodize_canonical_identity_canary_vm (rc=$rc)" >&2
  exit 1
fi
if ! echo "$out" | grep -q '"op":"call"'; then
  echo "$out" >&2
  echo "[FAIL] mirbuilder_call_methodize_canonical_identity_canary_vm (call missing)" >&2
  exit 1
fi
if ! echo "$out" | grep -q '"callee":{"type":"Method"'; then
  echo "$out" >&2
  echo "[FAIL] mirbuilder_call_methodize_canonical_identity_canary_vm (method callee missing)" >&2
  exit 1
fi
if ! echo "$out" | grep -q '"method":"length"'; then
  echo "$out" >&2
  echo "[FAIL] mirbuilder_call_methodize_canonical_identity_canary_vm (length missing)" >&2
  exit 1
fi
if echo "$out" | grep -q '"op":"mir_call"\|"func":'; then
  echo "$out" >&2
  echo "[FAIL] mirbuilder_call_methodize_canonical_identity_canary_vm (canonical call was methodized)" >&2
  exit 1
fi

echo "[PASS] mirbuilder_call_methodize_canonical_identity_canary_vm"
exit 0
