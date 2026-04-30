#!/bin/bash
# phase29bq_selfhost_stage1_contract_smoke_vm.sh
# Contract smoke for Stage1 bootstrap capability:
# - stage0 bootstrap route emits Program/MIR payloads
# - reduced stage1-cli artifact is runnable bootstrap output
# - legacy env route is diagnostics-only
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2
source "$NYASH_ROOT/tools/selfhost/lib/stage1_contract.sh"

BIN="${1:-$NYASH_ROOT/target/selfhost/hakorune.stage1_cli}"
ENTRY="${2:-$NYASH_ROOT/apps/tests/hello_simple_llvm.hako}"

if [[ "$BIN" != /* ]]; then
  BIN="$NYASH_ROOT/$BIN"
fi
if [[ "$ENTRY" != /* ]]; then
  ENTRY="$NYASH_ROOT/$ENTRY"
fi

if [ ! -x "$BIN" ]; then
  log_error "stage1 contract smoke requires a prebuilt stage1-cli artifact: $BIN"
  log_error "build first: tools/selfhost/mainline/build_stage1.sh --artifact-kind stage1-cli"
  exit 2
fi
if [ ! -f "$ENTRY" ]; then
  log_error "stage1 contract smoke entry not found: $ENTRY"
  exit 2
fi
if [ -f "${BIN}.artifact_kind" ]; then
  if ! rg -q '^artifact_kind=stage1-cli$' "${BIN}.artifact_kind"; then
    log_error "stage1 contract smoke requires stage1-cli artifact: ${BIN}.artifact_kind"
    exit 2
  fi
fi

cleanup() {
  :
}
trap cleanup EXIT

if ! stage1_contract_verify_stage1_cli_bootstrap_capability \
  "$NYASH_ROOT/target/release/hakorune" \
  "$ENTRY" \
  "$BIN"; then
  log_error "stage1 contract smoke: bootstrap capability failed"
  exit 1
fi

log_success "phase29bq_selfhost_stage1_contract_smoke_vm: PASS ($(basename "$BIN"))"
