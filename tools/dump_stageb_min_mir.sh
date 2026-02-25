#!/bin/bash
# dump_stageb_min_mir.sh — Stage‑B minimal harnessの Program(JSON v0) → MIR(JSON) ダンプ
#
# 目的:
#   - lang/src/compiler/tests/stageb_min_sample.hako を Stage‑B 経由で Program(JSON v0) に変換し、
#     その JSON を Rust 側 MirBuilder で MIR(JSON) に変換してファイルに落とすよ。
#   - LoopSSA の挙動や SSA/PHI 問題を、MIR レベルでデバッグしやすくするための小さなハーネスだよ。
#
# 使い方:
#   tools/dump_stageb_min_mir.sh [PROGRAM_JSON_OUT] [MIR_JSON_OUT]
#   例:
#     tools/dump_stageb_min_mir.sh tmp/stageb_min_program.json tmp/stageb_min_mir.json
#
# 環境変数:
#   NYASH_BIN               — nyash/hakorune バイナリ（既定: ./target/release/hakorune）
#   HAKO_LOOPSSA_EXIT_PHI   — 1: LoopSSA ON（LoopSSA v2 の挙動観測用） / 0: OFF（既定: 0）
#   HAKO_COMPILER_BUILDER_TRACE — 1: LoopSSA/BreakFinder/PhiInjector まわりのトレース
#   NYASH_VM_TRACE          — 1: Rust VM の binop/SSA トレース（重いので必要時のみ）
#
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${NYASH_BIN:-$ROOT_DIR/target/release/hakorune}"
STAGEB="$ROOT_DIR/lang/src/compiler/entry/compiler_stageb.hako"
TEST_FILE="$ROOT_DIR/lang/src/compiler/tests/stageb_min_sample.hako"

OUT_PROGRAM="${1:-$ROOT_DIR/tmp/stageb_min_program.json}"
OUT_MIR="${2:-$ROOT_DIR/tmp/stageb_min_mir.json}"

mkdir -p "$ROOT_DIR/tmp"

echo "[stageb/dump] emitting Program(JSON v0) → $OUT_PROGRAM"
NYASH_JSON_ONLY=1 \
NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
HAKO_STAGEB_FUNC_SCAN=1 \
NYASH_FEATURES=stage3 NYASH_FEATURES=stage3 \
NYASH_PARSER_ALLOW_SEMICOLON=1 \
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
HAKO_LOOPSSA_EXIT_PHI="${HAKO_LOOPSSA_EXIT_PHI:-0}" \
HAKO_COMPILER_BUILDER_TRACE="${HAKO_COMPILER_BUILDER_TRACE:-0}" \
NYASH_VM_TRACE="${NYASH_VM_TRACE:-0}" \
  "$BIN" --backend vm "$STAGEB" -- --source "$(cat "$TEST_FILE")" > "$OUT_PROGRAM"

echo "[stageb/dump] converting Program(JSON v0) → MIR(JSON) → $OUT_MIR"
cat "$OUT_PROGRAM" | \
  "$BIN" --backend vm --ny-parser-pipe --program-json-to-mir "$OUT_MIR"

echo "[stageb/dump] done."
