#!/bin/bash
# stageb_loopssa_debug.sh — Stage‑B LoopSSA 専用トレースプリセット
#
# 目的:
#   - Stage‑B 最小ハーネス（tools/test_stageb_min.sh）を、
#     LoopSSA v2 / BreakFinderBox / PhiInjectorBox / LocalSSA / receiver 実体化の
#     デバッグに適した ENV セットで実行するよ。
#   - 「いつも同じ環境変数を手で並べる」手間を減らし、ログの再現性を高める。
#
# 実行内容:
#   - HAKO_LOOPSSA_EXIT_PHI=1          : .hako LoopSSA Exit PHI を有効化
#   - HAKO_LOOPSSA_TRACE=1             : LoopSSA/BreakFinder/PhiInjector 専用トレース ON
#   - HAKO_COMPILER_BUILDER_TRACE=0    : Stage‑B 本体の builder トレースは既定で OFF（必要なら手動で1にする）
#   - NYASH_VM_TRACE=1                 : Rust VM 実行トレース（Call/Branch 等）
#   - NYASH_LOCAL_SSA_TRACE=1          : LocalSSA (recv/arg/cond) の Copy 発行トレース
#   - NYASH_BUILDER_TRACE_RECV=1       : receiver 実体化（pin_to_slot/LocalSSA）のトレース
#   ※ 既に値が設定されている場合はそちらを優先するよ。
#
set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

export HAKO_LOOPSSA_EXIT_PHI="${HAKO_LOOPSSA_EXIT_PHI:-1}"
export HAKO_LOOPSSA_TRACE="${HAKO_LOOPSSA_TRACE:-1}"
# Builder 側はデフォルト OFF。既に値があればそれを尊重する。
export HAKO_COMPILER_BUILDER_TRACE="${HAKO_COMPILER_BUILDER_TRACE:-0}"
export NYASH_VM_TRACE="${NYASH_VM_TRACE:-1}"
export NYASH_LOCAL_SSA_TRACE="${NYASH_LOCAL_SSA_TRACE:-1}"
export NYASH_BUILDER_TRACE_RECV="${NYASH_BUILDER_TRACE_RECV:-1}"

echo "=== Stage-B LoopSSA Debug Profile ==="
echo "HAKO_LOOPSSA_EXIT_PHI=${HAKO_LOOPSSA_EXIT_PHI}"
echo "HAKO_LOOPSSA_TRACE=${HAKO_LOOPSSA_TRACE}"
echo "HAKO_COMPILER_BUILDER_TRACE=${HAKO_COMPILER_BUILDER_TRACE}"
echo "NYASH_VM_TRACE=${NYASH_VM_TRACE}"
echo "NYASH_LOCAL_SSA_TRACE=${NYASH_LOCAL_SSA_TRACE}"
echo "NYASH_BUILDER_TRACE_RECV=${NYASH_BUILDER_TRACE_RECV}"
echo ""

exec "${ROOT}/tools/test_stageb_min.sh"
