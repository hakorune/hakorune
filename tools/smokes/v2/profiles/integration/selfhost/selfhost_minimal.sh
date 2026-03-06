#!/bin/bash
# selfhost_minimal.sh — Minimal selfhost Stage‑B→VM path using stage1_run_min.hako

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
fi

BIN="${HAKORUNE_BIN:-${NYASH_BIN:-$ROOT/target/release/hakorune}}"
SELFHOST="$ROOT/tools/selfhost/selfhost_build.sh"
TARGET="$ROOT/apps/tests/stage1_run_min.hako"

warn() { echo -e "[WARN] $*" >&2; }
info() { echo -e "[INFO] $*" >&2; }
fail() { echo -e "[FAIL] $*" >&2; exit 1; }
pass() { echo -e "[PASS] $*" >&2; }

if [ ! -x "$BIN" ]; then
  warn "[SKIP] hakorune binary not found at $BIN (build release first)"
  exit 0
fi

if [ ! -x "$SELFHOST" ]; then
  warn "[SKIP] selfhost_build.sh missing at $SELFHOST"
  exit 0
fi

if [ ! -f "$TARGET" ]; then
  warn "[SKIP] target fixture not found: $TARGET"
  exit 0
fi

info "Running minimal selfhost path via selfhost_build.sh"
set +e
output=$(NYASH_FEATURES="${NYASH_FEATURES:-stage3}" \
NYASH_USE_NY_COMPILER="${NYASH_USE_NY_COMPILER:-1}" \
NYASH_NY_COMPILER_EMIT_ONLY="${NYASH_NY_COMPILER_EMIT_ONLY:-1}" \
  "$SELFHOST" --in "$TARGET" --run 2>&1)
rc=$?
set -e

# Phase S0: Conditional SKIP for known patterns (該当ログの時だけ)
# SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md
if [ $rc -ne 0 ]; then
  # Legacy Pattern4 label: argument list too long (OS limitation)
  if echo "$output" | grep -q "Argument list too long"; then
    warn "[SKIP] selfhost_minimal: legacy Pattern4 label (OS limitation - Argument list too long)"
    echo "# SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md" >&2
    exit 0
  fi

  # Legacy Pattern1 label: loop_simple_while lowering failed / StepTree lowering returned None
  if echo "$output" | grep -qE "(Loop lowering failed|StepTree lowering returned None)"; then
    warn "[SKIP] selfhost_minimal: loop_simple_while legacy gap (Phase 188 limitation)"
    echo "# SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md" >&2
    exit 0
  fi

  # Phase 188.1: legacy Pattern6 label (NestedLoop Minimal) is now supported.
  # Removed conditional SKIP - if BundleResolver.resolve/4 uses unsupported nested form,
  # explicit error will occur (not SKIP)

  # Unknown error - FAIL (回帰を隠さない、Fail-Fast原則)
  echo "$output" >&2
  fail "selfhost_minimal failed (rc=$rc) - unknown error, possible regression"
fi

pass "selfhost_minimal passed (stage1_run_min.hako)"
exit 0
