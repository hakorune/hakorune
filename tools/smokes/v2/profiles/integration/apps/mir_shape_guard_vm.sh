#!/bin/bash
# APP-3: mir_shape_guard (VM)
#
# Contract pin:
# - selfhost-first emitted MIR (port07 fixture) is not collapsed
# - collapsed fixture is fail-fast in strict mode (rc!=0)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

APP="$NYASH_ROOT/apps/tools/mir_shape_guard/main.hako"
FIXTURE_HAKO="$NYASH_ROOT/apps/tests/phase29bq_joinir_port07_expr_unary_compare_logic_seed_min.hako"
COLLAPSED_FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/collapsed_min.mir.json"
EMIT_HELPER="$NYASH_ROOT/tools/hakorune_emit_mir.sh"

if [ ! -f "$APP" ]; then
  test_fail "mir_shape_guard_vm: app not found: $APP"
  exit 2
fi
if [ ! -f "$FIXTURE_HAKO" ]; then
  test_fail "mir_shape_guard_vm: hako fixture not found: $FIXTURE_HAKO"
  exit 2
fi
if [ ! -f "$COLLAPSED_FIXTURE" ]; then
  test_fail "mir_shape_guard_vm: collapsed fixture not found: $COLLAPSED_FIXTURE"
  exit 2
fi
if [ ! -f "$EMIT_HELPER" ]; then
  test_fail "mir_shape_guard_vm: emit helper missing: $EMIT_HELPER"
  exit 2
fi

TMP_MIR="$(mktemp /tmp/mir_shape_guard_port07.XXXXXX.json)"
TMP_EMIT_LOG="$(mktemp /tmp/mir_shape_guard_emit.XXXXXX.log)"

set +e
env NYASH_DISABLE_PLUGINS=1 \
    HAKO_SELFHOST_BUILDER_FIRST=1 \
    HAKO_SELFHOST_NO_DELEGATE=1 \
    HAKO_JOINIR_STRICT=1 \
    HAKO_JOINIR_PLANNER_REQUIRED=1 \
    NYASH_JOINIR_STRICT=1 \
    NYASH_JOINIR_DEV=1 \
    NYASH_NY_COMPILER_TIMEOUT_MS=30000 \
    bash "$EMIT_HELPER" "$FIXTURE_HAKO" "$TMP_MIR" >"$TMP_EMIT_LOG" 2>&1
rc_emit=$?
set -e

if [ "$rc_emit" -ne 0 ]; then
  test_fail "mir_shape_guard_vm: emit helper failed (rc=$rc_emit)"
  tail -n 40 "$TMP_EMIT_LOG" || true
  exit 1
fi

if ! grep -Fq "[OK] MIR JSON written (selfhost-first):" "$TMP_EMIT_LOG"; then
  test_fail "mir_shape_guard_vm: emit route was not selfhost-first"
  exit 1
fi
if grep -Fq "[OK] MIR JSON written (direct-emit" "$TMP_EMIT_LOG"; then
  test_fail "mir_shape_guard_vm: direct-emit fallback detected"
  exit 1
fi
if grep -Fq "[OK] MIR JSON written (delegate:" "$TMP_EMIT_LOG"; then
  test_fail "mir_shape_guard_vm: delegate fallback detected"
  exit 1
fi

set +e
ok_output=$(NYASH_VM_HAKO_PREFER_STRICT_DEV=0 NYASH_VM_USE_FALLBACK=0 \
            NYASH_JOINIR_DEV=0 NYASH_JOINIR_STRICT=0 \
            MIR_SHAPE_INPUT="$TMP_MIR" MIR_SHAPE_STRICT=1 \
            run_nyash_vm "$APP" 2>&1)
ok_rc=$?
set -e

if [ "$ok_rc" -ne 0 ]; then
  test_fail "mir_shape_guard_vm: expected rc=0 on emitted MIR, got $ok_rc"
  echo "$ok_output" | tail -n 20
  exit 1
fi

if ! echo "$ok_output" | grep -q '^SUMMARY '; then
  test_fail "mir_shape_guard_vm: summary output missing"
  exit 1
fi
if ! echo "$ok_output" | grep -q 'collapsed=0'; then
  test_fail "mir_shape_guard_vm: emitted MIR unexpectedly collapsed"
  exit 1
fi

blocks=$(echo "$ok_output" | sed -n 's/.*blocks=\([0-9][0-9]*\).*/\1/p')
branch=$(echo "$ok_output" | sed -n 's/.*branch=\([0-9][0-9]*\).*/\1/p')
ret=$(echo "$ok_output" | sed -n 's/.*ret=\([0-9][0-9]*\).*/\1/p')

if [ -z "$blocks" ] || [ -z "$branch" ] || [ -z "$ret" ]; then
  test_fail "mir_shape_guard_vm: failed to parse summary counters"
  exit 1
fi
if [ "$blocks" -lt 3 ]; then
  test_fail "mir_shape_guard_vm: expected blocks>=3, got $blocks"
  exit 1
fi
if [ "$branch" -lt 1 ]; then
  test_fail "mir_shape_guard_vm: expected branch>=1, got $branch"
  exit 1
fi
if [ "$ret" -lt 2 ]; then
  test_fail "mir_shape_guard_vm: expected ret>=2, got $ret"
  exit 1
fi

set +e
bad_output=$(NYASH_VM_HAKO_PREFER_STRICT_DEV=0 NYASH_VM_USE_FALLBACK=0 \
             NYASH_JOINIR_DEV=0 NYASH_JOINIR_STRICT=0 \
             MIR_SHAPE_INPUT="$COLLAPSED_FIXTURE" MIR_SHAPE_STRICT=1 \
             run_nyash_vm "$APP" 2>&1)
bad_rc=$?
set -e

if [ "$bad_rc" -eq 0 ]; then
  test_fail "mir_shape_guard_vm: collapsed fixture should fail in strict mode"
  exit 1
fi
if ! echo "$bad_output" | grep -q 'collapsed=1'; then
  test_fail "mir_shape_guard_vm: collapsed marker missing for strict failure"
  exit 1
fi

test_pass "mir_shape_guard_vm"
