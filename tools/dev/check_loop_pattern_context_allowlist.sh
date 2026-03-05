#!/usr/bin/env bash
set -euo pipefail

if rg -n "\\bLoopPatternContext\\b" src --glob '*.rs' >/tmp/loop_pattern_context_hits.txt; then
  echo "[FAIL] LoopPatternContext must be fully removed from src/**/*.rs" >&2
  cat /tmp/loop_pattern_context_hits.txt >&2
  exit 1
fi

echo "[PASS] LoopPatternContext is absent from src/**/*.rs"
