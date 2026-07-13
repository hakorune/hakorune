#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
source "$ROOT_DIR/tools/checks/lib/dev_gate_group.sh"

SUCCESS_STEPS="$ROOT_DIR/tools/checks/fixtures/dev_gate_group_success_steps.sh"
FAILURE_STEPS="$ROOT_DIR/tools/checks/fixtures/dev_gate_group_failure_steps.sh"
TMP_DIR="$(mktemp -d /tmp/hakorune-dev-gate-group-test.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT
export TMPDIR="$TMP_DIR"

compact_out="$TMP_DIR/compact.out"
dev_gate_group_run "compact-test" "$SUCCESS_STEPS" >"$compact_out" 2>&1
grep -Fq '[compact-test] ok  quiet success' "$compact_out"
grep -Fq '[compact-test] PASS 1/1' "$compact_out"
if grep -Eq 'hidden-(stdout|stderr)' "$compact_out"; then
  echo "[dev-gate-group-test] compact success leaked child output" >&2
  exit 1
fi

verbose_out="$TMP_DIR/verbose.out"
DEV_GATE_VERBOSE=1 dev_gate_group_run "verbose-test" "$SUCCESS_STEPS" \
  >"$verbose_out" 2>&1
grep -Fq '[verbose-test] >>> quiet success' "$verbose_out"
grep -Fq 'hidden-stdout' "$verbose_out"
grep -Fq 'hidden-stderr' "$verbose_out"
grep -Fq '[verbose-test] PASS 1/1' "$verbose_out"

failure_out="$TMP_DIR/failure.out"
set +e
dev_gate_group_run "failure-test" "$FAILURE_STEPS" >"$failure_out" 2>&1
failure_status=$?
set -e
if [[ "$failure_status" != "7" ]]; then
  echo "[dev-gate-group-test] failure status drifted: $failure_status" >&2
  exit 1
fi
grep -Fq '[failure-test] FAIL expected failure (exit=7' "$failure_out"
grep -Fq 'failure-marker' "$failure_out"
grep -Fq '[failure-test] full_log=' "$failure_out"
if grep -Fq 'post-failure-marker' "$failure_out"; then
  echo "[dev-gate-group-test] step executed after the first failure" >&2
  exit 1
fi

full_log="$(sed -n 's/^\[failure-test\] full_log=//p' "$failure_out")"
if [[ ! -f "$full_log" ]]; then
  echo "[dev-gate-group-test] retained failure log missing: $full_log" >&2
  exit 1
fi
grep -Fq 'failure-marker' "$full_log"

echo "[dev-gate-group-test] ok"
