#!/usr/bin/env bash
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
cd "$ROOT"

INPUT="lang/src/runner/launcher.hako"
OUT="lang/bin/hakorune"

if [[ ! -f "$INPUT" ]]; then
  echo "error: launcher not found: $INPUT" >&2
  exit 1
fi

mkdir -p "$(dirname "$OUT")"

if ! command -v llvm-config-18 >/dev/null 2>&1; then
  echo "[skip] llvm-config-18 not found; cannot build launcher (install LLVM 18 dev)" >&2
  exit 90
fi

echo "[build] compiling $INPUT → $OUT"
tools/build_llvm.sh "$INPUT" -o "$OUT"
echo "[ok] $OUT"

