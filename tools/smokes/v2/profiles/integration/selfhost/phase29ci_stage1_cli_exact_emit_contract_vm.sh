#!/bin/bash
# phase29ci_stage1_cli_exact_emit_contract_vm.sh
# Exact reduced-artifact emit contract smoke for stage1-cli:
# - `run_stage1_cli.sh emit program-json` must succeed on the stage1-cli artifact
# - `run_stage1_cli.sh emit mir-json` must succeed on the same artifact and fixture
# This is narrower than the bootstrap capability smoke:
# it proves the reduced artifact's own exact shell contract, not stage0 bootstrap proof.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)"
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

BIN="${1:-$NYASH_ROOT/target/selfhost/hakorune.stage1_cli}"
ENTRY="${2:-$NYASH_ROOT/apps/tests/hello_simple_llvm.hako}"

if [[ "$BIN" != /* ]]; then
  BIN="$NYASH_ROOT/$BIN"
fi
if [[ "$ENTRY" != /* ]]; then
  ENTRY="$NYASH_ROOT/$ENTRY"
fi

if [ ! -x "$BIN" ]; then
  log_error "stage1-cli exact emit smoke binary not found/executable: $BIN"
  exit 2
fi
if [ ! -f "$ENTRY" ]; then
  log_error "stage1-cli exact emit smoke entry not found: $ENTRY"
  exit 2
fi
if [ -f "${BIN}.artifact_kind" ]; then
  if ! rg -q '^artifact_kind=stage1-cli$' "${BIN}.artifact_kind"; then
    log_error "stage1-cli exact emit smoke requires stage1-cli artifact: ${BIN}.artifact_kind"
    exit 2
  fi
fi

PROG_OUT="$(mktemp --suffix .stage1_cli_program.json)"
PROG_ERR="$(mktemp --suffix .stage1_cli_program.err)"
MIR_OUT="$(mktemp --suffix .stage1_cli_mir.json)"
MIR_ERR="$(mktemp --suffix .stage1_cli_mir.err)"
trap 'rm -f "$PROG_OUT" "$PROG_ERR" "$MIR_OUT" "$MIR_ERR"' EXIT

if ! bash "$ROOT_DIR/selfhost/run_stage1_cli.sh" --bin "$BIN" emit program-json "$ENTRY" >"$PROG_OUT" 2>"$PROG_ERR"; then
  log_error "stage1-cli exact emit smoke: emit program-json failed"
  sed -n '1,80p' "$PROG_ERR" >&2 || true
  exit 1
fi

if ! rg -q '"version":0' "$PROG_OUT" || ! rg -q '"kind":"Program"' "$PROG_OUT"; then
  log_error "stage1-cli exact emit smoke: program-json output missing Program(JSON v0) markers"
  sed -n '1,40p' "$PROG_OUT" >&2 || true
  sed -n '1,40p' "$PROG_ERR" >&2 || true
  exit 1
fi

if ! bash "$ROOT_DIR/selfhost/run_stage1_cli.sh" --bin "$BIN" emit mir-json "$ENTRY" >"$MIR_OUT" 2>"$MIR_ERR"; then
  log_error "stage1-cli exact emit smoke: emit mir-json failed"
  sed -n '1,80p' "$MIR_ERR" >&2 || true
  exit 1
fi

if ! rg -q '"functions"' "$MIR_OUT"; then
  log_error "stage1-cli exact emit smoke: mir-json output missing functions marker"
  sed -n '1,40p' "$MIR_OUT" >&2 || true
  sed -n '1,40p' "$MIR_ERR" >&2 || true
  exit 1
fi

log_success "phase29ci_stage1_cli_exact_emit_contract_vm: PASS ($(basename "$BIN"))"
