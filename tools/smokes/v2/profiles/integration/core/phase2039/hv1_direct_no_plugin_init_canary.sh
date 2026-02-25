#!/bin/bash
# Test: HV1 direct route bypasses plugin initialization completely
# Expected: No UnifiedBoxRegistry logs, only rc output
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Create minimal MIR JSON v0: main() { return 42; }
tmp_json="/tmp/hv1_direct_test_$$.json"
cat > "$tmp_json" <<'JSONEND'
{
  "functions": [
    {
      "name": "main",
      "blocks": [
        {
          "id": 0,
          "instructions": [
            {"op": "const", "dst": 1, "value": {"type": "int", "value": 42}},
            {"op": "ret", "value": 1}
          ]
        }
      ]
    }
  ]
}
JSONEND

# Create dummy input file (filename required by CLI, but not used in hv1 direct route)
tmp_nyash="/tmp/hv1_test_$$.hako"
echo "# Dummy file for HV1 direct route" > "$tmp_nyash"

set +e
# Run with HV1 direct route, suppress nyash.toml noise
# Explicitly unset NYASH_CLI_VERBOSE to prevent MIR dumps
# Capture stdout and stderr separately
stdout_file="/tmp/hv1_stdout_$$.txt"
stderr_file="/tmp/hv1_stderr_$$.txt"
env -u NYASH_CLI_VERBOSE HAKO_VERIFY_PRIMARY=hakovm NYASH_SKIP_TOML_ENV=1 NYASH_VERIFY_JSON="$(cat "$tmp_json")" "$NYASH_BIN" --backend vm "$tmp_nyash" >"$stdout_file" 2>"$stderr_file"
rc=$?
output_stdout=$(cat "$stdout_file")
output_stderr=$(cat "$stderr_file")
rm -f "$stdout_file" "$stderr_file"
set -e

rm -f "$tmp_json" "$tmp_nyash"

# Check 1: Exit code should be 42 (from MIR return value)
if [ "$rc" -ne 42 ]; then
  echo "[FAIL] hv1_direct_no_plugin_init_canary: expected rc=42, got rc=$rc" >&2
  exit 1
fi

# Check 2: No plugin initialization logs should appear in stderr
if echo "$output_stderr" | grep -q "UnifiedBoxRegistry"; then
  echo "[FAIL] hv1_direct_no_plugin_init_canary: UnifiedBoxRegistry log found (plugin init not bypassed)" >&2
  echo "Stderr:" >&2
  echo "$output_stderr" >&2
  exit 1
fi

# Check 3: stdout should be exactly "42" (strip trailing newline for comparison)
stdout_clean=$(echo "$output_stdout" | tr -d '\n')
if [ "$stdout_clean" != "42" ]; then
  echo "[FAIL] hv1_direct_no_plugin_init_canary: expected stdout '42', got '$stdout_clean'" >&2
  echo "Full stdout: '$output_stdout'" >&2
  echo "Full stderr: '$output_stderr'" >&2
  exit 1
fi

echo "[PASS] hv1_direct_no_plugin_init_canary"
exit 0
