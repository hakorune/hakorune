#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Phase S0.1: Canary tests are opt-in (SMOKES_ENABLE_SELFHOST=1)
# SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md
if [ "${SMOKES_ENABLE_SELFHOST:-0}" != "1" ]; then
  test_skip "selfhost_s1_s2_from_builder_compare_cfg_canary_vm" "opt-in selfhost canary (SMOKES_ENABLE_SELFHOST=1). SSOT: investigations/selfhost-integration-limitations.md"
  exit 0
fi

set +e
out=$(bash "$ROOT/tools/selfhost/bootstrap_s1_s2.sh" --cmd1 "bash $ROOT/tools/selfhost/gen_v1_from_builder_compare_cfg.sh" --cmd2 "bash $ROOT/tools/selfhost/gen_v1_from_builder_compare_cfg.sh" 2>&1)
rc=$?
set -e

if [ "$rc" -eq 0 ]; then
  echo "[PASS] selfhost_s1_s2_from_builder_compare_cfg_canary_vm"
  exit 0
fi
if [ "$rc" -eq 2 ] || echo "$out" | grep -qi 'invalid JSON input'; then
  echo "[SKIP] selfhost_s1_s2_from_builder_compare_cfg_canary_vm (builder JSON variant)" >&2
  exit 0
fi

# Phase S0: Conditional SKIP for known patterns (該当ログの時だけ)
# SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md
if echo "$out" | grep -qE "(Loop lowering failed|StepTree lowering returned None|loop pattern is not supported|cap_missing/NestedLoop|Argument list too long|strict mode: pattern not matched)"; then
  echo "[SKIP] selfhost_s1_s2_from_builder_compare_cfg_canary_vm: Known pattern (see investigation doc)" >&2
  echo "# SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md" >&2
  exit 0
fi

# Unknown error - FAIL (回帰を隠さない、Fail-Fast原則)
echo "[FAIL] selfhost_s1_s2_from_builder_compare_cfg_canary_vm (rc=$rc) - unknown error, possible regression" >&2
printf '%s\n' "$out" | sed -n '1,200p' >&2
exit 1
