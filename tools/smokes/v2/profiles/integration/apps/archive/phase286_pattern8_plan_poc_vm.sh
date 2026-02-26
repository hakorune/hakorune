#!/usr/bin/env bash
# Phase 286 P2.4: Pattern8 BoolPredicateScan Plan化 PoC (non-static box)
# Expected: exit 7 (is_integer("123") == true)

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../../../.." && pwd)"
HAKORUNE="$REPO_ROOT/target/release/hakorune"

# Test: Pattern8 Plan line routing
# Note: set -e disabled because hakorune exits with 7 (success return value from .hako)
set +e
"$HAKORUNE" --backend vm "$REPO_ROOT/apps/tests/phase286_pattern8_plan_poc.hako" 2>/dev/null
exit_code=$?
set -e

if [ $exit_code -eq 7 ]; then
    exit 0
else
    echo "ERROR: Expected exit 7, got $exit_code" >&2
    exit 1
fi
