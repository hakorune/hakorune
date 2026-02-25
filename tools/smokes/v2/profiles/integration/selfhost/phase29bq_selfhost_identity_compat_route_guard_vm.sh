#!/bin/bash
# Phase 29bq: identity compat-route explicit opt-in guard smoke
#
# Contract pin:
# 1) `--cli-mode auto|stage0` requires explicit `--allow-compat-route`.
# 2) With opt-in present, validation passes through to normal setup checks.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

IDENTITY_SCRIPT="$NYASH_ROOT/tools/selfhost_identity_check.sh"
if [ ! -x "$IDENTITY_SCRIPT" ]; then
  log_error "identity script not found/executable: $IDENTITY_SCRIPT"
  exit 2
fi

stderr_no_optin="$(mktemp /tmp/phase29bq_identity_compat_guard_no_optin_stderr.XXXXXX.log)"
stderr_with_optin="$(mktemp /tmp/phase29bq_identity_compat_guard_with_optin_stderr.XXXXXX.log)"
cleanup() {
  rm -f "$stderr_no_optin" "$stderr_with_optin"
}
trap cleanup EXIT

set +e
"$IDENTITY_SCRIPT" \
  --mode smoke \
  --skip-build \
  --cli-mode auto \
  --bin-stage1 /tmp/__missing_stage1__.bin \
  --bin-stage2 /tmp/__missing_stage2__.bin \
  >/dev/null 2>"$stderr_no_optin"
rc_no_optin=$?
set -e

if [ "$rc_no_optin" -ne 2 ]; then
  log_error "expected rc=2 without --allow-compat-route, got rc=$rc_no_optin"
  echo "STDERR_LOG(no-optin): $stderr_no_optin"
  exit 1
fi

if ! rg -q '^\[G1\] compat route requires explicit opt-in: --allow-compat-route \(cli-mode=auto\)$' "$stderr_no_optin"; then
  log_error "missing explicit opt-in guard message for compat route"
  echo "STDERR_LOG(no-optin): $stderr_no_optin"
  exit 1
fi

set +e
"$IDENTITY_SCRIPT" \
  --mode smoke \
  --skip-build \
  --cli-mode auto \
  --allow-compat-route \
  --bin-stage1 /tmp/__missing_stage1__.bin \
  --bin-stage2 /tmp/__missing_stage2__.bin \
  >/dev/null 2>"$stderr_with_optin"
rc_with_optin=$?
set -e

if [ "$rc_with_optin" -ne 2 ]; then
  log_error "expected rc=2 with missing binaries after opt-in, got rc=$rc_with_optin"
  echo "STDERR_LOG(with-optin): $stderr_with_optin"
  exit 1
fi

if rg -q 'compat route requires explicit opt-in' "$stderr_with_optin"; then
  log_error "opt-in guard fired unexpectedly when --allow-compat-route was set"
  echo "STDERR_LOG(with-optin): $stderr_with_optin"
  exit 1
fi

if ! rg -q '^\[G1:FAIL\] Stage1 binary not found: /tmp/__missing_stage1__\.bin$' "$stderr_with_optin"; then
  log_error "expected normal setup failure after compat opt-in"
  echo "STDERR_LOG(with-optin): $stderr_with_optin"
  exit 1
fi

log_success "phase29bq_selfhost_identity_compat_route_guard_vm: PASS (compat route explicit-only)"
