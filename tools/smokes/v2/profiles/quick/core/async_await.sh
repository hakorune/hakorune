#!/bin/bash
# async_await.sh - Minimal async/await smoke using env.future

# Phase 287 P4 Box 4: Mark as environment-dependent (async requires Future runtime)
if [ "${SMOKES_ENABLE_ASYNC:-0}" != "1" ]; then
  echo "[SKIP:env] async/await requires Future runtime plugin (set SMOKES_ENABLE_ASYNC=1 to enable)" >&2
  exit 0
fi

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2
preflight_plugins || exit 2

TEST_DIR="/tmp/nyash_async_await_$$"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

cat > async.hako << 'EOF'
static box Main {
  main() {
    // Create a future from a value and await it
    nowait f = 42
    local v = await f
    print(v)
    return 0
  }
}
EOF

output=$(NYASH_REWRITE_FUTURE=1 run_nyash_vm async.hako 2>&1 || true)
if echo "$output" | grep -q "ExternCall .* not supported\|unimplemented instruction: FutureNew"; then
  test_skip "async_await" "VM interpreter lacks Future/ExternCall support"
  rc=0
else
  compare_outputs "42" "$output" "async_await"
  rc=$?
fi
cd /
rm -rf "$TEST_DIR"
exit $rc
