#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/debug/hakorune --backend vm apps/hako-alloc-segment-lifecycle-scalar-state-proof/main.hako

