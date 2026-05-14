#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="vm-lim-001-object-queue-identity-probe"
cd "$ROOT_DIR"

APP="apps/vm-lim-object-queue-identity-probe/main.hako"
LIMITS="docs/development/current/main/design/vm-known-limitations-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
TIMEOUT_VALUE="${VM_LIM_001_TIMEOUT:-8s}"

fail() {
  echo "[$TAG] ERROR: $*" >&2
  exit 1
}

[[ -f "$APP" ]] || fail "missing app: $APP"
[[ -f "$LIMITS" ]] || fail "missing known limitation SSOT: $LIMITS"
rg -F -q 'VM-LIM-001 object-heavy page queue/facade route' "$LIMITS" || fail "missing VM-LIM-001 docs"
rg -F -q 'tools/checks/vm_lim_001_object_queue_identity_probe.sh' "$INDEX" || fail "probe must be indexed"
rg -F -q 'pages.push(page)' "$APP" || fail "probe must retain page object in ArrayBox"
rg -F -q 'pages.get(0)' "$APP" || fail "probe must retrieve retained page object"
rg -F -q 'retained.freeCount()' "$APP" || fail "probe must call page method on retained object"

out="$(mktemp)"
err="$(mktemp)"
trap 'rm -f "$out" "$err"' EXIT

set +e
timeout --kill-after=2s "$TIMEOUT_VALUE" env NYASH_DISABLE_PLUGINS=1 HAKO_VM_MAX_STEPS="${HAKO_VM_MAX_STEPS:-200000}" cargo run -q --bin hakorune -- --backend vm "$APP" >"$out" 2>"$err"
rc=$?
set -e

if [[ "$rc" == "0" ]]; then
  cat "$out"
  rg -F -q 'summary=ok' "$out" || fail "VM completed but output contract failed"
  echo "[$TAG] VM completed object queue identity route; VM-LIM-001 may be a retirement candidate"
  exit 0
fi

if [[ "$rc" == "124" || "$rc" == "137" ]]; then
  cat "$out" || true
  cat "$err" || true
  echo "[$TAG] known limitation observed: VM timed out after $TIMEOUT_VALUE"
  exit 0
fi

cat "$out" >&2 || true
cat "$err" >&2 || true
fail "VM route failed with rc=$rc"
