#!/usr/bin/env bash
# selfhost_exe_stageb.sh — Stage‑B → MirBuilder → ny‑llvmc (crate) → EXE
# Purpose: Build a native EXE from a Nyash .hako source using Stage‑B+MirBuilder (selfhost‑first)
# Usage: tools/selfhost_exe_stageb.sh <input.hako> [-o <out>] [--run]
#
# Prerequisites (one-time setup):
#   cargo build --release -p nyash-llvm-compiler
#   (cd crates/nyash_kernel && cargo build --release)
#   cargo build --release
set -euo pipefail

OUT="a.out"; DO_RUN=0
if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <input.hako> [-o <out>] [--run]" >&2; exit 2
fi
INPUT=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    -o) OUT="$2"; shift 2;;
    --run) DO_RUN=1; shift;;
    *) INPUT="$1"; shift;;
  esac
done
if [[ ! -f "$INPUT" ]]; then echo "error: input not found: $INPUT" >&2; exit 2; fi

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# 1) Emit MIR(JSON) via Stage‑B → MirBuilder (selfhost‑first)
TMP_JSON=$(mktemp --suffix .json)
HAKO_SELFHOST_BUILDER_FIRST="${HAKO_SELFHOST_BUILDER_FIRST:-0}" \
HAKO_MIR_BUILDER_LOOP_JSONFRAG="${HAKO_MIR_BUILDER_LOOP_JSONFRAG:-0}" \
HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE="${HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE:-0}" \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
NYASH_JSON_ONLY=1 bash "$ROOT_DIR/tools/hakorune_emit_mir.sh" "$INPUT" "$TMP_JSON" >/dev/null
echo "[emit] MIR JSON: $TMP_JSON ($(wc -c < "$TMP_JSON") bytes)"

# 2) Build EXE via crate backend (ny-llvmc) using helper
NYASH_LLVM_BACKEND=crate \
NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT_DIR/target/release/ny-llvmc}" \
NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT_DIR/target/release}" \
  bash "$ROOT_DIR/tools/ny_mir_builder.sh" --in "$TMP_JSON" --emit exe -o "$OUT" --quiet >/dev/null
echo "[link] EXE: $OUT"

if [[ "$DO_RUN" = "1" ]]; then
  set +e
  _silent="${NYASH_NYRT_SILENT_RESULT:-}"
  if [[ -n "$_silent" ]]; then
    "$OUT"; rc=$?
  else
    NYASH_NYRT_SILENT_RESULT=1 "$OUT"; rc=$?
  fi
  set -e
  echo "[run] exit=$rc"
fi

exit 0
