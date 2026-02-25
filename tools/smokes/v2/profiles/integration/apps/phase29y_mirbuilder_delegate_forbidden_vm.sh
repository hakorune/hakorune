#!/bin/bash
# Phase 29y contract pin: `env.mirbuilder.emit` must fail-fast on mainline
# when HAKO_SELFHOST_NO_DELEGATE=1.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29y_mirbuilder_delegate_forbidden_min_v1.mir.json"
if [ ! -f "$FIXTURE" ]; then
  test_fail "phase29y_mirbuilder_delegate_forbidden_vm: fixture missing: $FIXTURE"
  exit 2
fi

stderr_forbidden="$(mktemp /tmp/phase29y_delegate_forbidden_stderr.XXXXXX.log)"
stderr_allowed="$(mktemp /tmp/phase29y_delegate_allowed_stderr.XXXXXX.log)"
cleanup() {
  rm -f "$stderr_forbidden" "$stderr_allowed"
}
trap cleanup EXIT

set +e
HAKO_SELFHOST_NO_DELEGATE=1 \
HAKO_V1_EXTERN_PROVIDER=0 \
"$NYASH_BIN" --mir-json-file "$FIXTURE" >/dev/null 2>"$stderr_forbidden"
forbidden_rc=$?
set -e

if [ "$forbidden_rc" -eq 0 ]; then
  echo "[INFO] STDERR_LOG(forbidden): $stderr_forbidden"
  test_fail "phase29y_mirbuilder_delegate_forbidden_vm: expected non-zero rc when delegate is forbidden"
  exit 1
fi

if ! rg -q '\[freeze:contract\]\[mirbuilder/delegate-forbidden\] env\.mirbuilder\.emit blocked \(HAKO_SELFHOST_NO_DELEGATE=1\)' "$stderr_forbidden"; then
  echo "[INFO] STDERR_LOG(forbidden): $stderr_forbidden"
  test_fail "phase29y_mirbuilder_delegate_forbidden_vm: missing freeze tag for delegate-forbidden contract"
  exit 1
fi

set +e
HAKO_SELFHOST_NO_DELEGATE=0 \
HAKO_V1_EXTERN_PROVIDER=0 \
"$NYASH_BIN" --mir-json-file "$FIXTURE" >/dev/null 2>"$stderr_allowed"
allowed_rc=$?
set -e

if [ "$allowed_rc" -ne 0 ]; then
  echo "[INFO] STDERR_LOG(allowed): $stderr_allowed"
  test_fail "phase29y_mirbuilder_delegate_forbidden_vm: expected rc=0 when delegate lock is disabled"
  exit 1
fi

test_pass "phase29y_mirbuilder_delegate_forbidden_vm: PASS (delegate-forbidden fail-fast pinned)"
