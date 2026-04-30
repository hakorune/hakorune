#!/bin/bash
# test_loopssa_breakfinder_slot.sh — LoopSSA/BreakFinder 「最小失敗 JSON」スロットテスト
#
# 目的:
#   - lang/src/compiler/tests/loopssa_breakfinder_slot.hako を直接 VM で実行し、
#     Program(JSON v0) スロットに貼り付けた JSON に対する LoopSSA.stabilize_merges
#     の挙動を観測するよ。
#   - Stage‑B 最小サンプルから抽出した「失敗する JSON v0」をここに貼っておけば、
#     Stage‑B 全体を回さずに LoopSSA/BreakFinderBox 周辺だけを再現できる。
#
set -e

cd "$(dirname "$0")/../../.."

NYASH_BIN="${NYASH_BIN:-./target/release/hakorune}"
TEST_FILE="lang/src/compiler/tests/loopssa_breakfinder_slot.hako"

echo "=== LoopSSA BreakFinder Slot JSON Test ==="
echo "Test file: $TEST_FILE"
echo "Binary: $NYASH_BIN"
echo ""

# LoopSSA v2 の EXIT PHI を有効化して実行（BreakFinder/PhiInjector 経由）
HAKO_LOOPSSA_EXIT_PHI=1 \
NYASH_DISABLE_PLUGINS=1 NYASH_FEATURES=stage3 \
  "$NYASH_BIN" --backend vm "$TEST_FILE" 2>&1

echo ""
echo "=== Test complete ==="
