#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

if (cd "$ROOT" && cargo build -q --release -p nyash-kernel-min-c >/dev/null 2>&1); then
  echo "[PASS] kernel_min_c_build_canary"
  exit 0
fi
echo "[FAIL] kernel_min_c_build_canary" >&2
exit 1

