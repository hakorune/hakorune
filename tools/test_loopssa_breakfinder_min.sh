#!/bin/bash
# test_loopssa_breakfinder_min.sh — LoopSSA/BreakFinder 最小 JSON ハーネステスト
#
# 目的:
#   - lang/src/compiler/tests/loopssa_breakfinder_min.hako を直接 VM で実行し、
#     LoopSSA.stabilize_merges が単純な loop_header/loop_exit パターンで安全に動くことを確認する。
#   - LoopSSA v2 / BreakFinderBox の変更時に、まずここで赤が出ないことをチェックするための軽量テストだよ。
#
# 将来:
#   - Stage‑B 最小サンプルから抽出した JSON をこのハーネスに貼り替えることで、
#     ValueId(50) 系の問題を再現するための土台として使う予定。
#
set -e

NYASH_BIN="${NYASH_BIN:-./target/release/hakorune}"
TEST_FILE="lang/src/compiler/tests/loopssa_breakfinder_min.hako"

echo "=== LoopSSA BreakFinder Minimal JSON Test ==="
echo "Test file: $TEST_FILE"
echo "Binary: $NYASH_BIN"
echo ""

# LoopSSA v2 の EXIT PHI を有効化して実行（BreakFinder/PhiInjector 経由）
HAKO_LOOPSSA_EXIT_PHI=1 \
NYASH_DISABLE_PLUGINS=1 NYASH_FEATURES=stage3 \
  "$NYASH_BIN" --backend vm "$TEST_FILE" 2>&1

echo ""
echo "=== Test complete ==="

