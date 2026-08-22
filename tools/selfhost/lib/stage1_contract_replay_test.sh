#!/usr/bin/env bash
# Focused contract tests for the Stage1 replay admission boundary.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
# shellcheck source=/dev/null
source "$ROOT/tools/selfhost/lib/stage1_contract.sh"

unset HAKO_BACKEND_COMPAT_REPLAY
[[ "$(stage1_contract_resolve_backend_replay '')" == "none" ]]
[[ "$(stage1_contract_resolve_backend_replay 'none')" == "none" ]]
[[ "$(stage1_contract_resolve_backend_replay 'harness')" == "harness" ]]

HAKO_BACKEND_COMPAT_REPLAY=none
[[ "$(stage1_contract_resolve_backend_replay '')" == "none" ]]

HAKO_BACKEND_COMPAT_REPLAY=harness
if stage1_contract_resolve_backend_replay '' >/dev/null 2>&1; then
  echo "inherited harness replay was accepted without Stage1 admission" >&2
  exit 1
fi
[[ "$(stage1_contract_resolve_backend_replay 'harness')" == "harness" ]]
if stage1_contract_resolve_backend_replay 'none' >/dev/null 2>&1; then
  echo "CLI/env replay mismatch was accepted" >&2
  exit 1
fi

if stage1_contract_resolve_backend_replay 'native' >/dev/null 2>&1; then
  echo "invalid replay value was accepted" >&2
  exit 1
fi

echo "[stage1-contract-replay-test] PASS"
