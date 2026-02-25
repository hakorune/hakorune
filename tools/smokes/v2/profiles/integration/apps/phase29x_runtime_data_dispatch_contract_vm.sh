#!/bin/bash
# Phase 29x: RuntimeData dispatch contract smoke
#
# Contract:
# - nyash_kernel runtime_data dispatch tests must pass.
# - Pins ArrayBox/MapBox runtime_data routes used by LLVM RuntimeDataBox lowering.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29x_runtime_data_dispatch_contract_vm"

set +e
OUTPUT=$(cargo test -p nyash_kernel runtime_data_dispatch -- --nocapture 2>&1)
RC=$?
set -e

if [[ "${RC}" -ne 0 ]]; then
  echo "${OUTPUT}" | tail -n 60 || true
  test_fail "${SMOKE_NAME}: cargo test failed rc=${RC}"
  exit 1
fi

if ! printf '%s\n' "${OUTPUT}" | grep -q "runtime_data_dispatch_array_push_get_index_zero ... ok"; then
  echo "${OUTPUT}" | tail -n 80 || true
  test_fail "${SMOKE_NAME}: missing array dispatch pass line"
  exit 1
fi

if ! printf '%s\n' "${OUTPUT}" | grep -q "runtime_data_dispatch_map_set_get_has ... ok"; then
  echo "${OUTPUT}" | tail -n 80 || true
  test_fail "${SMOKE_NAME}: missing map dispatch pass line"
  exit 1
fi

test_pass "${SMOKE_NAME}: runtime_data dispatch contract pinned"
