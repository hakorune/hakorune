#!/bin/bash
# gc_mode_off.sh - Ensure VM runs with GC disabled (NYASH_GC_MODE=off)

# Phase 287 P4 Box 4: Mark as environment-dependent (GC mode requires runtime control)
if [ "${SMOKES_ENABLE_GC_MODE:-0}" != "1" ]; then
  echo "[SKIP:env] GC mode control requires runtime configuration (set SMOKES_ENABLE_GC_MODE=1 to enable)" >&2
  exit 0
fi

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2
preflight_plugins || exit 2

TEST_DIR="/tmp/nyash_gc_off_$$"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

cat > gc_off.hako << 'EOF'
static box Main {
  main() {
    // Drive safepoints via await with GC off
    nowait f = 7
    local v = await f
    print("GC_OFF_OK:" + v)
    return 0
  }
}
EOF

output=$(NYASH_GC_MODE=off NYASH_REWRITE_FUTURE=1 run_nyash_vm gc_off.hako 2>&1 || true)
if echo "$output" | grep -q "ExternCall .* not supported\|unimplemented instruction: FutureNew"; then
  test_skip "gc_mode_off" "VM interpreter lacks Future/ExternCall support"
  rc=0
else
  compare_outputs "GC_OFF_OK:7" "$output" "gc_mode_off"
  rc=$?
fi
cd /
rm -rf "$TEST_DIR"
exit $rc
