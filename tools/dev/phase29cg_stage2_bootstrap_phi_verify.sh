#!/usr/bin/env bash
# phase29cg_stage2_bootstrap_phi_verify.sh
# Reproduce the current Stage2 bootstrap dominance/PHI blocker through the
# stage1-cli -> Program(JSON v0) -> MIR(JSON v0) -> ny_mir_builder pipeline.
#
# This probe requires an emit-capable Stage1 env artifact. The current reduced
# `stage1-cli` artifact is a run-only bootstrap output built from the thin
# entry stub, so fail before taking the stale Program(JSON) bridge path when
# that artifact is selected.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

# shellcheck source=/dev/null
source "$ROOT/tools/selfhost/lib/stage1_contract.sh"

STAGE1_BIN="${STAGE1_BIN:-$ROOT/target/selfhost/hakorune.stage1_cli}"
ENTRY="${ENTRY:-$ROOT/lang/src/runner/stage1_cli_env.hako}"
OUT_DIR="${OUT_DIR:-$(mktemp -d)}"
KEEP_OUT_DIR="${KEEP_OUT_DIR:-0}"
mkdir -p "$OUT_DIR"

TMP_PROG="$OUT_DIR/stage1_cli_env.program.json"
TMP_MIR="$OUT_DIR/stage1_cli_env.mir.json"
TMP_OBJ="$OUT_DIR/stage1_cli_env.o"
TMP_IR="${NYASH_LLVM_DUMP_IR:-$OUT_DIR/stage1_cli_env.ll}"
TMP_EMIT_ERR="$OUT_DIR/stage1_cli_env.emit.err"
TMP_LLVM_ERR="$OUT_DIR/stage1_cli_env.llvm.err"
TMP_VERIFY_ERR="$OUT_DIR/stage1_cli_env.verify.err"

cleanup() {
  if [[ "$KEEP_OUT_DIR" != "1" ]]; then
    rm -rf "$OUT_DIR" 2>/dev/null || true
  fi
}
trap cleanup EXIT

if [[ ! -x "$STAGE1_BIN" ]]; then
  echo "[FAIL] phase29cg_stage2_bootstrap_phi_verify: missing stage1 binary: $STAGE1_BIN" >&2
  exit 1
fi
if [[ ! -f "$ENTRY" ]]; then
  echo "[FAIL] phase29cg_stage2_bootstrap_phi_verify: missing entry: $ENTRY" >&2
  exit 1
fi
if stage1_contract_artifact_is_reduced_stage1_cli "$STAGE1_BIN"; then
  echo "[FAIL] phase29cg_stage2_bootstrap_phi_verify: reduced run-only stage1-cli artifact cannot emit Program/MIR payloads: $STAGE1_BIN" >&2
  echo "       artifact_entry=$(stage1_contract_artifact_entry "$STAGE1_BIN")" >&2
  echo "       required: emit-capable Stage1 env artifact for $ENTRY" >&2
  echo "       next: keep this bridge proof until stage1_contract_exec_mode emit-mir is green for the Stage1 artifact" >&2
  exit 1
fi
if ! command -v opt >/dev/null 2>&1; then
  echo "[FAIL] phase29cg_stage2_bootstrap_phi_verify: llvm opt not found" >&2
  exit 1
fi

SOURCE_TEXT="$(stage1_contract_source_text "$ENTRY")"

set +e
stage1_contract_exec_mode "$STAGE1_BIN" emit-program "$ENTRY" "$SOURCE_TEXT" >"$TMP_PROG" 2>"$TMP_EMIT_ERR"
emit_program_rc=$?
set -e
if [[ "$emit_program_rc" -ne 0 ]]; then
  echo "[FAIL] phase29cg_stage2_bootstrap_phi_verify: emit-program rc=$emit_program_rc" >&2
  sed -n '1,120p' "$TMP_EMIT_ERR" >&2 || true
  exit 1
fi
if ! [[ -s "$TMP_PROG" ]]; then
  echo "[FAIL] phase29cg_stage2_bootstrap_phi_verify: program json is empty" >&2
  exit 1
fi

# Keep the Program(JSON)->MIR bridge out of the reduced-artifact fail-fast path.
# It is still the explicit compatibility bridge until the emit-mir replacement
# proof from P106 is green.
# shellcheck source=/dev/null
source "$ROOT/tools/selfhost/lib/program_json_mir_bridge.sh"

set +e
program_json_mir_bridge_emit "$ROOT/target/release/hakorune" "$TMP_PROG" "$TMP_MIR" "[phase29cg]" >/dev/null 2>"$TMP_EMIT_ERR"
emit_mir_rc=$?
set -e
if [[ "$emit_mir_rc" -ne 0 ]]; then
  echo "[FAIL] phase29cg_stage2_bootstrap_phi_verify: Program(JSON)->MIR bridge rc=$emit_mir_rc" >&2
  sed -n '1,120p' "$TMP_EMIT_ERR" >&2 || true
  exit 1
fi
if ! [[ -s "$TMP_MIR" ]]; then
  echo "[FAIL] phase29cg_stage2_bootstrap_phi_verify: mir json is empty" >&2
  exit 1
fi

set +e
NYASH_LLVM_BACKEND="${NYASH_LLVM_BACKEND:-crate}" \
NYASH_LLVM_SKIP_BUILD=1 \
NYASH_LLVM_DUMP_IR="$TMP_IR" \
NYASH_LLVM_ROUTE_TRACE="${NYASH_LLVM_ROUTE_TRACE:-1}" \
bash "$ROOT/tools/ny_mir_builder.sh" --in "$TMP_MIR" --emit obj -o "$TMP_OBJ" --quiet >/dev/null 2>"$TMP_LLVM_ERR"
llvm_rc=$?
set -e

set +e
opt -passes=verify "$TMP_IR" -disable-output >/dev/null 2>"$TMP_VERIFY_ERR"
verify_rc=$?
set -e

verify_count="$(rg -c 'Instruction does not dominate all uses!' "$TMP_VERIFY_ERR" || true)"
first_failures="$(rg -n -m 6 'Instruction does not dominate all uses!|PHI nodes not grouped at top of basic block|does not dominate' "$TMP_VERIFY_ERR" || true)"

echo "[phase29cg] out_dir=$OUT_DIR"
echo "[phase29cg] emit_program_rc=$emit_program_rc emit_mir_rc=$emit_mir_rc llvm_rc=$llvm_rc verify_rc=$verify_rc verify_count=$verify_count"
if [[ -n "$first_failures" ]]; then
  echo "[phase29cg] first_failures:"
  echo "$first_failures"
fi

if [[ "$llvm_rc" -eq 0 && "$verify_rc" -eq 0 ]]; then
  echo "[PASS] phase29cg_stage2_bootstrap_phi_verify"
  exit 0
fi

if [[ "$llvm_rc" -ne 0 && -s "$TMP_LLVM_ERR" ]]; then
  echo "[phase29cg] llvm stderr:" >&2
  sed -n '1,120p' "$TMP_LLVM_ERR" >&2 || true
fi
if [[ "$verify_rc" -ne 0 && -s "$TMP_VERIFY_ERR" ]]; then
  echo "[phase29cg] verify stderr:" >&2
  sed -n '1,80p' "$TMP_VERIFY_ERR" >&2 || true
fi
echo "[FAIL] phase29cg_stage2_bootstrap_phi_verify llvm_rc=$llvm_rc verify_rc=$verify_rc verify_count=$verify_count" >&2
exit 1
