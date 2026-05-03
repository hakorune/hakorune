#!/bin/bash
# Return(Method recv Str/String, length|size, args=[]) belongs to the string-length owner, not return.string.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

BIN="$ROOT/target/release/hakorune"
if [ ! -x "$BIN" ]; then
  echo "[SKIP] mirbuilder_return_string_owner_boundary_canary_vm (hakorune not built)"
  exit 0
fi

if grep -q 'lower_return_method_string_length\|return.method.string.length' "$ROOT/lang/src/mir/builder/MirBuilderMinBox.hako"; then
  echo "[FAIL] mirbuilder_return_string_owner_boundary_canary_vm (MirBuilderMinBox widened)" >&2
  exit 1
fi

tmp_exe="/tmp/mirbuilder_return_string_owner_boundary_$$.exe"
build_log="/tmp/mirbuilder_return_string_owner_boundary_build_$$.log"
trap 'rm -f "$tmp_exe" "$build_log"' EXIT

if ! timeout "${HAKO_BUILD_TIMEOUT:-120}" env -i PATH="$PATH" HOME="${HOME:-}" USER="${USER:-}" "$BIN" --emit-exe "$tmp_exe" "$ROOT/lang/src/runner/stage1_cli_env.hako" >"$build_log" 2>&1; then
  cat "$build_log" >&2
  echo "[FAIL] mirbuilder_return_string_owner_boundary_canary_vm (build failed)" >&2
  exit 1
fi

set +e
str_out="$(env -i PATH="$PATH" HOME="${HOME:-}" USER="${USER:-}" HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir STAGE1_SOURCE_TEXT='static box Main { main() { return "hello" } }' "$tmp_exe" 2>&1)"; str_rc=$?
len_out="$(env -i PATH="$PATH" HOME="${HOME:-}" USER="${USER:-}" HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir STAGE1_SOURCE_TEXT='static box Main { main() { return "hello".length() } }' "$tmp_exe" 2>&1)"; len_rc=$?
size_out="$(env -i PATH="$PATH" HOME="${HOME:-}" USER="${USER:-}" HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir STAGE1_SOURCE_TEXT='static box Main { main() { return "hello".size() } }' "$tmp_exe" 2>&1)"; size_rc=$?
arg_out="$(env -i PATH="$PATH" HOME="${HOME:-}" USER="${USER:-}" HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir STAGE1_SOURCE_TEXT='static box Main { main() { return "hello".length(1) } }' "$tmp_exe" 2>&1)"; arg_rc=$?
set -e

if [ "$str_rc" -ne 0 ] || ! echo "$str_out" | grep -q '"type":"string"' || echo "$str_out" | grep -q '"op":"call"'; then
  echo "$str_out" >&2
  echo "[FAIL] mirbuilder_return_string_owner_boundary_canary_vm (direct string failed)" >&2
  exit 1
fi

if [ "$len_rc" -ne 0 ] || ! echo "$len_out" | grep -q '"op":"call"' || ! echo "$len_out" | grep -q '"method":"length"'; then
  echo "$len_out" >&2
  echo "[FAIL] mirbuilder_return_string_owner_boundary_canary_vm (length owner failed)" >&2
  exit 1
fi

if [ "$size_rc" -ne 0 ] || ! echo "$size_out" | grep -q '"op":"call"' || ! echo "$size_out" | grep -q '"method":"size"'; then
  echo "$size_out" >&2
  echo "[FAIL] mirbuilder_return_string_owner_boundary_canary_vm (size owner failed)" >&2
  exit 1
fi

if [ "$arg_rc" -eq 0 ] && echo "$arg_out" | grep -q '"op":"call"' && echo "$arg_out" | grep -q '"method":"length"'; then
  echo "$arg_out" >&2
  echo "[FAIL] mirbuilder_return_string_owner_boundary_canary_vm (non-empty args accepted)" >&2
  exit 1
fi

echo "[PASS] mirbuilder_return_string_owner_boundary_canary_vm"
exit 0
