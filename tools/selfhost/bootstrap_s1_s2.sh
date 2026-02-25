#!/usr/bin/env bash
set -euo pipefail

# bootstrap_s1_s2.sh — Self‑Hosting S1/S2 helper
# S1: emit‑only 再生成 → 正規化ハッシュ一致
# S2: 1‑pass Fixpoint → もう一度一致
#
# 使い方（2通り）
# 1) 既存の v1 JSON ファイル同士を比較（S1/S2の結果を与える）
#    tools/selfhost/bootstrap_s1_s2.sh --in1 a.json --in2 b.json
#
# 2) コマンドから v1 JSON を生成して比較（emit コマンドを渡す）
#    tools/selfhost/bootstrap_s1_s2.sh --cmd1 'bash gen_v1_once.sh' --cmd2 'bash gen_v1_twice.sh'
#    （各コマンドは v1 JSON を stdout に出力すること）

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
LIB_TR="$ROOT/tools/smokes/v2/lib/test_runner.sh"
if [ -f "$LIB_TR" ]; then source "$LIB_TR"; else echo "[FAIL] missing $LIB_TR" >&2; exit 2; fi

require_env >/dev/null || true

IN1=""
IN2=""
CMD1=""
CMD2=""
while [ $# -gt 0 ]; do
  case "$1" in
    --in1) IN1="$2"; shift 2 ;;
    --in2) IN2="$2"; shift 2 ;;
    --cmd1) CMD1="$2"; shift 2 ;;
    --cmd2) CMD2="$2"; shift 2 ;;
    *) echo "[FAIL] unknown arg: $1" >&2; exit 2 ;;
  esac
done

tmp1="/tmp/hako_s1_$$.json"
tmp2="/tmp/hako_s2_$$.json"
cleanup(){ rm -f "$tmp1" "$tmp2" 2>/dev/null || true; }
trap cleanup EXIT

if [ -n "$IN1" ] && [ -n "$IN2" ]; then
  cp "$IN1" "$tmp1"
  cp "$IN2" "$tmp2"
elif [ -n "$CMD1" ] && [ -n "$CMD2" ]; then
  eval "$CMD1" > "$tmp1"
  eval "$CMD2" > "$tmp2"
else
  echo "[FAIL] provide --in1/--in2 or --cmd1/--cmd2" >&2
  exit 2
fi

if ! jq -e . < "$tmp1" >/dev/null 2>&1 || ! jq -e . < "$tmp2" >/dev/null 2>&1; then
  echo "[FAIL] invalid JSON input(s)" >&2
  exit 2
fi

echo "[S1/S2] hashing v1 JSON (normalized) ..." >&2
h1=$(v1_normalized_hash "$tmp1")
h2=$(v1_normalized_hash "$tmp2")
echo "[S1] $h1" >&2
echo "[S2] $h2" >&2

if [ "$h1" = "$h2" ]; then
  echo "[PASS] S1/S2 normalized hashes match"
  exit 0
else
  echo "[FAIL] S1/S2 normalized hashes differ" >&2
  exit 1
fi

